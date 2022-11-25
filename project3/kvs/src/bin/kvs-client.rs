use std::net::SocketAddr;

use clap::{self, Arg, Command};
use kvs::{KvsClient, Result};
use log::LevelFilter;

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let matches = get_matches()?;
    let addr: &String = matches.get_one("addr").expect("expect an addr");
    let addr: SocketAddr = addr.parse().unwrap();

    match matches.subcommand() {
        Some(("set", matches)) => {
            let key: &String = matches.get_one("key").expect("expect one key for set");
            let value: &String = matches.get_one("value").expect("expect one value to set");
            let mut client = KvsClient::connect(addr)?;
            client.set(key.to_owned(), value.to_owned())?;
        }
        Some(("get", matches)) => {
            let key: &String = matches.get_one("key").expect("expect one key for get");

            let mut client = KvsClient::connect(addr)?;
            let value = client.get(key.to_owned())?;
            if let Some(value) = value {
                println!("{}", value);
            } else {
                println!("err: no value for {}", key);
            }
        }
        Some(("rm", matches)) => {
            let key: &String = matches.get_one("key").expect("expect one key for rm");
            let mut client = KvsClient::connect(addr)?;
            client.remove(key.to_owned())?;
        }
        _ => std::process::exit(1),
    }
    Ok(())
}

fn get_matches() -> Result<clap::ArgMatches> {
    let matches = Command::new("kvs-client")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("addr")
                .long("addr")
                .value_name("IP-PORT")
                .num_args(1)
                .default_value("127.0.0.1:4000"),
        )
        .subcommand_required(true)
        .subcommand(
            Command::new("set")
                .about("Set the value of a string key to a string.")
                .arg(Arg::new("key").value_name("KEY").required(true).index(1))
                .arg(
                    Arg::new("value")
                        .value_name("VALUE")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            Command::new("get")
                .about("Get the string value of a given string key.")
                .arg(Arg::new("key").value_name("KEY").required(true).index(1)),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a given string key.")
                .arg(Arg::new("key").value_name("KEY").required(true).index(1)),
        )
        .get_matches();

    Ok(matches)
}
