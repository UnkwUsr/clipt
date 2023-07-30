mod client;
mod server;
mod shared;

use clap::{Arg, ArgAction, Command};

fn main() {
    let cmd = Command::new(env!("CARGO_CRATE_NAME"))
        .arg_required_else_help(true)
        .subcommand(Command::new("server").about("Start server"))
        .subcommand(Command::new("put").about("Put new item"))
        .subcommand(Command::new("list").about("List all items"))
        .subcommand(
            Command::new("pick")
                .about("Print item by id and make it newest")
                .arg(
                    Arg::new("id")
                        .index(1)
                        .required(true)
                        .value_parser(clap::value_parser!(usize)),
                ),
        )
        .subcommand(
            Command::new("peek").about("Print item by id").arg(
                Arg::new("id")
                    .index(1)
                    .required(true)
                    .value_parser(clap::value_parser!(usize)),
            ),
        )
        .subcommand(
            Command::new("delete").about("Delete items by ids").arg(
                Arg::new("id")
                    .index(1)
                    .required(true)
                    .value_parser(clap::value_parser!(usize))
                    .action(ArgAction::Append),
            ),
        )
        .get_matches();

    match cmd.subcommand_name() {
        Some("server") => server::app_server(),
        Some("put") => client::client_put(),
        Some("list") => client::client_list(),
        Some("pick") => {
            let id = cmd
                .subcommand_matches("pick")
                .unwrap()
                .get_one::<usize>("id")
                .unwrap();

            client::client_pick(id);
        }
        Some("peek") => {
            let id = cmd
                .subcommand_matches("peek")
                .unwrap()
                .get_one::<usize>("id")
                .unwrap();

            client::client_peek(id);
        }
        Some("delete") => {
            let ids: Vec<&usize> = cmd
                .subcommand_matches("delete")
                .unwrap()
                .get_many::<usize>("id")
                .unwrap()
                .collect();

            client::client_delete(ids);
        }
        _ => unreachable!("unreachable because arg_required_else_help() is used"),
    }
}
