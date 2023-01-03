use std::{env, error::Error, process::exit};

use error::FlattenError;
use flatten::FlattenExecutor;

mod error;
mod flatten;

fn main() {
    let executor = parse_args().unwrap_or_else(|error| {
        println!("{error}");
        print_usage();
        exit(1);
    });

    executor.flatten().unwrap_or_else(|error| {
        println!("flatten failed: {}", error);
        exit(1);
    });
}

fn parse_args() -> Result<FlattenExecutor, Box<dyn Error>> {
    let mut args = env::args().skip(1); // skip binary name (arg0)

    let arg1 = args.next().unwrap_or(String::from("-h"));
    match arg1.as_str() {
        "-h" | "--help" => print_usage_and_exit(),
        "-v" | "--version" => print_version_and_exit(),
        _ => (),
    }

    let mut cmd_args = FlattenExecutor::new(arg1)?;

    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => print_usage_and_exit(),
            "-v" | "--version" => print_version_and_exit(),
            "-c" | "--copy" => cmd_args.copy = true,
            "--keep-dirs" => cmd_args.keep_dirs = true,
            _ => {
                let error_msg = String::from("Unrecognized option: ") + arg.as_str();
                return Err(Box::new(FlattenError::new(&error_msg)));
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
    println!(
        "flatten v{} by vollkorntomate (https://github.com/vollkorntomate/flatten)",
        env!("CARGO_PKG_VERSION")
    );
}

fn print_version_and_exit() {
    print_version();
    exit(0);
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
