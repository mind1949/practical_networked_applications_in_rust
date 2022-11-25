use std::net::SocketAddr;
use std::process;

use clap::{self, Arg, Command};
use kvs::{KvStore, KvsServer, Result, SledKvsEngine};
use log::{debug, error, LevelFilter};

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

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
        .arg(
            Arg::new("engine")
                .long("engine")
                .value_name("ENGINE-NAME")
                .num_args(1)
                .default_value("kvs")
                .value_parser([
                    clap::builder::PossibleValue::new("kvs"),
                    clap::builder::PossibleValue::new("sled"),
                ]),
        )
        .get_matches();

    let addr: &String = matches.get_one("addr").expect("expect one addr");
    let addr: SocketAddr = addr.parse().unwrap();
    let engine: &String = matches.get_one("engine").expect("expect one engine");
    debug!("addr: {}, engine: {}", addr, engine);

    if let Err(err) = match engine.as_str() {
        "kvs" => {
            let store = KvStore::open("./kvs/kvs")?;
            let mut server = KvsServer::new(store);
            server.run(addr)
        }
        "sled" => {
            let db = SledKvsEngine::new(sled::open("./kvs/sled")?);
            let mut server = KvsServer::new(db);
            server.run(addr)
        }
        _ => unreachable!(),
    } {
        error!("err: {}", err);
        process::exit(1);
    } else {
        Ok(())
    }
}
