use crate::shared::SOCKET_PATH;

use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixListener;

use rkv::backend::{SafeMode, SafeModeEnvironment};
use rkv::{Manager, Rkv, StoreOptions, Value};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const DB_PATH: &str = "asd.db";

pub fn app_server() {
    let socket = Path::new(SOCKET_PATH);
    // Delete old socket if necessary
    if socket.exists() {
        // TODO: not sure about that. Probably unsafe
        std::fs::remove_file(&socket).unwrap();
    }
    // Bind to socket
    let listener = match UnixListener::bind(&socket) {
        Err(_) => panic!("failed to bind socket"),
        Ok(stream) => stream,
    };

    // db open

    let path = Path::new(DB_PATH);
    fs::create_dir_all(path).unwrap();

    // The `Manager` enforces that each process opens the same environment at most once by
    // caching a handle to each environment that it opens. Use it to retrieve the handle
    // to an opened environment—or create one if it hasn't already been opened:
    let mut manager = Manager::<SafeModeEnvironment>::singleton().write().unwrap();
    let created_arc = manager.get_or_create(path, Rkv::new::<SafeMode>).unwrap();
    let env = created_arc.read().unwrap();

    // Then you can use the environment handle to get a handle to a datastore:
    let store = env.open_single("mydb", StoreOptions::create()).unwrap();

    // db open end

    println!("Server started");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut bufreader = BufReader::new(&stream);

                let mut mode = String::new();
                bufreader.read_line(&mut mode).unwrap();

                match mode.trim_end() {
                    "put" => {
                        println!("put");

                        let mut text = String::new();
                        bufreader.read_to_string(&mut text).unwrap();
                        println!("text is: {:?}", text);

                        let mut writer = env.write().unwrap();
                        store
                            .put(&mut writer, get_timestamp(), &Value::Str(&text))
                            .unwrap();
                        writer.commit().unwrap();
                    }
                    "list" => {
                        println!("list");

                        let reader = env.read().expect("reader");
                        store
                            .iter_start(&reader)
                            .unwrap()
                            .for_each(|x| match x.unwrap() {
                                (key, Value::Str(val)) => {
                                    let key = String::from_utf8_lossy(key);
                                    let val = val.replace("\n", " ");
                                    let row = format!("{}:{}\n", key, val);
                                    stream.write(row.as_bytes()).unwrap();
                                }
                                _ => {}
                            });
                    }
                    "pick" => {
                        println!("pick");

                        let mut id = String::new();
                        bufreader.read_to_string(&mut id).unwrap();

                        let reader = env.read().expect("reader");
                        if let Some(val) = store.get(&reader, &id).unwrap() {
                            if let Value::Str(val_str) = val {
                                let mut writer = env.write().unwrap();
                                store.delete(&mut writer, &id).unwrap();
                                store.put(&mut writer, get_timestamp(), &val).unwrap();
                                writer.commit().unwrap();

                                stream.write(val_str.as_bytes()).unwrap();
                                continue;
                            }
                        }

                        stream
                            .write(format!("invalid id: {}", id).as_bytes())
                            .unwrap();
                    }
                    "delete" => {
                        println!("delete");

                        let ids = bufreader.lines();

                        let mut writer = env.write().unwrap();
                        for id in ids {
                            store.delete(&mut writer, id.unwrap()).unwrap();
                        }
                        writer.commit().unwrap();
                    }
                    "peek" => {
                        println!("peek");

                        let mut id = String::new();
                        bufreader.read_to_string(&mut id).unwrap();

                        let reader = env.read().expect("reader");
                        if let Some(val) = store.get(&reader, &id).unwrap() {
                            if let Value::Str(val_str) = val {
                                stream.write(val_str.as_bytes()).unwrap();
                                continue;
                            }
                        }

                        stream
                            .write(format!("invalid id: {}", id).as_bytes())
                            .unwrap();
                    }
                    mode => unimplemented!("mode {}", mode),
                }
            }
            Err(err) => {
                eprintln!("error: {:?}", err);
                break;
            }
        }
    }
}

fn get_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}
