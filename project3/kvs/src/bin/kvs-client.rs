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

    match matches.subcommand() {
        Some(("set", matches)) => {
            let key: &String = matches.get_one("key").expect("expect one key for set");
            let value: &String = matches.get_one("value").expect("expect one value to set");
            println!("set {} {}", key, value);
            todo!()
        }
        Some(("get", matches)) => {
            let key: &String = matches.get_one("key").expect("expect one key for get");
            println!("get {}", key);
            todo!()
        }
        Some(("rm", matches)) => {
            let key: &String = matches.get_one("key").expect("expect one key for rm");
            println!("rm {}", key);
            todo!()
        }
        _ => std::process::exit(1),
    }
}
