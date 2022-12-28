use std::{env, process::exit};

use error::FlattenError;
use flatten::FlattenExecutor;

mod error;
mod flatten;

fn main() {
    let executor = parse_args().unwrap_or_else(|error| {
        println!("{}", error.message);
        print_usage();
        exit(1);
    });

    executor.flatten().unwrap_or_else(|error| {
        println!("flatten failed: {}", error);
        exit(1);
    });
}

fn parse_args() -> Result<FlattenExecutor, FlattenError> {
    let mut args = env::args().skip(1); // skip binary name (arg0)

    let arg1 = args.next().unwrap_or(String::from("-h"));
    match arg1.as_str() {
        "-h" | "--help" => print_usage_and_exit(),
        _ => (),
    }

    let mut cmd_args = FlattenExecutor::new(arg1);

    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => print_usage_and_exit(),
            "-c" | "--copy" => cmd_args.copy = true,
            "--keep-dirs" => cmd_args.keep_dirs = true,
            _ => {
                return Err(FlattenError {
                    message: String::from("Unrecognized option: ") + arg.as_str(),
                })
            }
        }
    }

    Ok(cmd_args)
}

pub fn fail(message: &str) {
    println!("flatten failed: {message}");
    exit(1);
}

fn print_version() {
    println!("flatten v{}", env!("CARGO_PKG_VERSION"));
}

fn print_usage() {
    print_version();
    println!(
        "Usage: flatten <dir> [options]
Options:
  -c, --copy:\tCopies the files instead of moving
  --keep-dirs:\tDon't delete the nested directories"
    );
}

fn print_usage_and_exit() {
    print_usage();
    exit(0);
}
