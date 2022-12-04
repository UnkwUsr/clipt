mod client;
mod server;
mod shared;

use clap::{Arg, Command};

fn main() {
    let cmd = Command::new(env!("CARGO_CRATE_NAME"))
        // .arg_required_else_help(true)
        // .subcommand_value_name("APPLET")
        // .subcommand_help_heading("APPLETS")
        .subcommand(Command::new("server").about("server"))
        .subcommand(Command::new("put"))
        .subcommand(Command::new("list"))
        .subcommand(Command::new("pick").arg(Arg::new("id").index(1).required(true)))
        .get_matches();

    match cmd.subcommand_name() {
        Some("server") => server::app_server(),
        Some("put") => client::client_put(),
        Some("list") => client::client_list(),
        Some("pick") => {
            let asd = cmd
                .subcommand_matches("pick")
                .unwrap()
                .get_one::<String>("id")
                .unwrap();

            client::client_pick(asd);
        }
        _ => unreachable!("parser should ensure only valid subcommand names are used"),
    }

    // match matches.subcommand() {
    //     Some(("server", _)) => server::app_server(),
    //     Some(("put", _)) => client::client_put(),
    //     Some(("list", _)) => client::client_list(),
    //     Some(("pick", _)) => client::client_pick(&23.to_string()),
    //     _ => {}
    // }
}
