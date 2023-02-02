use std::env;
use std::process;

use minicut:: {Config, cut};


fn main() {

    let config: Config = Config::build(env::args())
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

    if let Err(e) = cut(config) {
        eprintln!("Application Error {e}");
        process::exit(1);
    }
}
