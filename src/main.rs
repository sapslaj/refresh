use std::{process::Command, thread};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'n', long = "interval", default_value = "2s")]
    interval: humantime::Duration,

    #[arg(
        trailing_var_arg = true,
        num_args = 1..,
        value_name = "COMMAND",
    )]
    cmd: Vec<String>,
}

fn main() {
    let args = Args::parse();

    loop {
        clearscreen::clear().unwrap();
        let program = args
            .cmd
            .first()
            .expect("command must be given. see --help for more info.");
        let res = Command::new(program).args(args.cmd[1..].to_vec()).spawn();
        match res {
            Ok(_) => (),
            Err(err) => println!("err: {}", err),
        }
        thread::sleep(*args.interval);
    }
}
