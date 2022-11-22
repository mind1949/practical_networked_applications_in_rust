use clap::{self, Arg, Command};

fn main() {
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
    let engine: &String = matches.get_one("engine").expect("expect one engine");
    println!("addr: {}, engine: {}", addr, engine);
    todo!()
}
