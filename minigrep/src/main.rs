use std::ffi::OsString;
use std::{env, error::Error, process};

use minigrep::{Config, run};
fn main() {
    // env::args returns and iterator over the args and collect collects them in standard
    // collection
    // we might use
    // let args: Vec<OsString> = env::args_os().collect();
    // to deal with unicode strings
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        // writing to stderr
        eprintln!("There is an error while parsing argments: {}", err);
        process::exit(1);
    });
    println!(
        "Searching for `{}` in `{}`\n",
        config.query, config.filepath
    );
    if let Err(err) = run(config) {
        // writing to stderr
        eprintln!("Application Error: {err}");
        process::exit(1);
    }
}
