use clap::{self, Arg, ArgMatches, Command};
use pipo::{PipoClient, PipoServer, Result};
use std::process::exit;

fn main() -> Result<()> {
    let matches = clap::Command::new("pipo")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            Command::new("client")
                .about("pipo client")
                .arg(
                    Arg::new("host")
                        .value_name("HOST")
                        .long("host")
                        .num_args(1)
                        .default_value("127.0.0.1"),
                )
                .arg(
                    Arg::new("port")
                        .value_name("PORT")
                        .long("port")
                        .num_args(1)
                        .default_value("6379"),
                ),
        )
        .subcommand(
            Command::new("server")
                .about("pipo server")
                .arg(
                    Arg::new("host")
                        .value_name("HOST")
                        .long("host")
                        .num_args(1)
                        .default_value("127.0.0.1"),
                )
                .arg(
                    Arg::new("port")
                        .value_name("PORT")
                        .long("port")
                        .num_args(1)
                        .default_value("6379"),
                ),
        )
        .subcommand_required(true)
        .get_matches();

    let get_addr = |arg_matches: &ArgMatches| -> (String, i32) {
        let host = arg_matches.get_one::<String>("host").unwrap().to_string();
        let port = arg_matches
            .get_one::<String>("port")
            .unwrap()
            .parse::<i32>()
            .unwrap();
        (host, port)
    };
    match matches.subcommand() {
        Some(("client", matches)) => {
            let (host, port) = get_addr(matches);
            let addr = format!("{}:{}", host, port);
            PipoClient::connect(addr)?.run()?;
        }
        Some(("server", matches)) => {
            let (host, port) = get_addr(matches);
            let addr = format!("{}:{}", host, port);
            PipoServer::bind(addr)?.run()?;
        }
        _ => exit(1),
    }
    Ok(())
}
