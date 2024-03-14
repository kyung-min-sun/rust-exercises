use std::{env, process};

fn main() {
    let config = grep::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });

    match grep::run(config) {
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
        Ok(_) => process::exit(0),
    };
}
