use clap::{self, Arg, Command};
/*
<!--  cli.yml -->
name: myapp
version: "1.0"
author: Kevin K. <kbknapp@gmail.com>
about: Does awesome things
args:
    - config:
        short: c
        long: config
        value_name: FILE
        help: Sets a custom config file
        takes_value: true
    - INPUT:
        help: Sets the input file to use
        required: true
        index: 1
    - verbose:
        short: v
        multiple: true
        help: Sets the level of verbosity
subcommands:
    - test:
        about: controls testing features
        version: "1.3"
        author: Someone E. <someone_else@other.com>
        args:
            - debug:
                short: d
                help: print debug information

*/

fn main() {
    let cmd = Command::new("meow")
        .version("1.0")
        .author("mind1949")
        .about("Does awesome things")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Sets a custom config file")
                .value_name("FILE")
                .required(true),
        )
        .arg(Arg::new("input").required(true).index(1))
        .arg(
            Arg::new("verbose")
                .short('v')
                .num_args(0..)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            Command::new("test")
                .about("controls testing features")
                .version("1.3")
                .author("mind1949")
                .arg(
                    Arg::new("debug")
                        .short('d')
                        .help(" print debug information"),
                ),
        );
    let args = cmd.get_matches();
    println!("{:?}", args);
}
