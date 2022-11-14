use clap::{Arg, Command};
use std::process::exit;

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand_required(true)
        .subcommand(
            Command::new("set")
                .about("Set the value of a string key to a string")
                .arg(
                    Arg::new("key")
                        .value_name("KEY")
                        .num_args(1)
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::new("value")
                        .num_args(1)
                        .value_name("VALUE")
                        .index(2)
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("get")
                .about("Get the string value of a given string key")
                .arg(Arg::new("key").value_name("KEY").index(1).required(true)),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a given key")
                .arg(Arg::new("key").value_name("KEY").index(1).required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("set", _)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(("get", _)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(("rm", _)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
