use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = grep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });

    match grep::run(config) {
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
        Ok(_) => return,
    };
}
