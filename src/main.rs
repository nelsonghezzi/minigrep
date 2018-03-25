extern crate clap;
extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

use clap::{App, Arg};

fn main() {
    const CASE_SENSITIVE_ARG: &str = "case-sensitive";
    const CASE_INSENSITIVE_ARG: &str = "case-insensitive";
    const FILE_ARG: &str = "FILE";

    let args = App::new("minigrep")
        .version("0.1.0")
        .about("grep-like command-line program written in Rust.")
        .author("Nelson G. Ghezzi")
        .arg(
            Arg::with_name(CASE_SENSITIVE_ARG)
                .help("Performs the search in a case-sensitive manner")
                .short("s")
                .long("case-sensitive")
                .conflicts_with("case-insensitive"),
        )
        .arg(
            Arg::with_name(CASE_INSENSITIVE_ARG)
                .help("Performs the search in a case-insensitive manner")
                .short("i")
                .long("case-insensitive"),
        )
        .arg(
            Arg::with_name("query")
                .help("Pattern to search for")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name(FILE_ARG)
                .help("File to search on")
                .required(true)
                .index(2),
        )
        .get_matches();

    let case_sensitive;

    if args.is_present(CASE_SENSITIVE_ARG) {
        case_sensitive = true;
    } else if args.is_present(CASE_INSENSITIVE_ARG) {
        case_sensitive = false;
    } else {
        case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    }

    let config = Config::new(
        args.value_of("query").unwrap().to_string(),
        args.value_of(FILE_ARG).unwrap().to_string(),
        case_sensitive,
    ).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
