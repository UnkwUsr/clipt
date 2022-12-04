use crate::shared::SOCKET_PATH;

use std::io::{Read, Write};
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
        std::fs::remove_file(&socket).unwrap();
    }
    // Bind to socket
    let listener = match UnixListener::bind(&socket) {
        Err(_) => panic!("failed to bind socket"),
        Ok(stream) => stream,
    };

    // db shit

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

    // db shit end

    println!("Server started, waiting for clients");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = String::new();
                stream.read_to_string(&mut buf).unwrap();
                let mut asd = buf.lines();
                match asd.next() {
                    Some("put") => {
                        let text: String = asd.collect();
                        println!("put");

                        let mut writer = env.write().unwrap();
                        store
                            .put(&mut writer, get_timestamp(), &Value::Str(&text))
                            .unwrap();
                        writer.commit().unwrap();
                    }
                    Some("list") => {
                        println!("list");

                        let reader = env.read().expect("reader");
                        store.iter_start(&reader).unwrap().for_each(|x| {
                            let row = x.unwrap();
                            stream
                                .write(
                                    format!(
                                        "{}: {}\n",
                                        String::from_utf8_lossy(row.0),
                                        String::from_utf8_lossy(
                                            row.1.to_bytes().unwrap().as_slice()
                                        ),
                                    )
                                    .as_bytes(),
                                )
                                .unwrap();
                        });
                        // stream.flush().unwrap();
                    }
                    Some("pick") => {
                        println!("pick");

                        let id: String = asd.collect();
                        eprintln!("id is {}", id);
                    }
                    Some(&_) => todo!(),
                    None => todo!(),
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