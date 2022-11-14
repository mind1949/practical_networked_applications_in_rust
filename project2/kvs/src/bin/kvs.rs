use clap::{Arg, Command};
use kvs::{KvStore, KvsError, Result};
use std::env::current_dir;
use std::path::PathBuf;
use std::process::exit;

fn main() -> Result<()> {
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
        Some(("set", arg_matches)) => {
            let key = arg_matches.get_one::<String>("key").expect("expect a key");
            let value = arg_matches
                .get_one::<String>("value")
                .expect("expect a value");

            let mut store = KvStore::open(get_store_dir()?)?;
            store.set(key.to_string(), value.to_string())
        }
        Some(("get", arg_matches)) => {
            let key = arg_matches.get_one::<String>("key").expect("expect a key");

            let mut store = KvStore::open(get_store_dir()?)?;
            if let Some(value) = store.get(key.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
            Ok(())
        }
        Some(("rm", arg_matches)) => {
            let key = arg_matches
                .get_one::<String>("key")
                .expect("expect get a key");

            let mut store = KvStore::open(get_store_dir()?)?;
            match store.remove(key.to_string()) {
                Ok(()) => {}
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                }
                Err(e) => return Err(e),
            }

            Ok(())
        }
        _ => Err(KvsError::UnexpectedCommandType),
    }
}

fn get_store_dir() -> Result<PathBuf> {
    Ok(current_dir()?)
}
