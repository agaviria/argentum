extern crate getopts;
extern crate itertools;

use std::env;

use getopts::Options;

pub mod errors;
pub mod scanner;
pub mod utils;
pub mod token;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("USAGE: {} [options] <input>", program);
    print!("{}", opts.usage(&brief))
}

fn print_version() {
    println!("AG version {}", env!("CARGO_PKG_VERSION"));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // flag options
    let mut options = Options::new();
    options.optopt("s", "script", "Run in script mode <file>", "<file>");
    options.optflag("h", "help", "Display available options");
    options.optflag("v", "version", "Display version information");

    // parse cli arguments and exit in the event argument provided is invalid.
    let matches = options.parse(&args[1..]).unwrap_or_else(|reason| {
        println!("{}: error: {}", program, reason.to_string());
        std::process::exit(1);
    });

    // help requested
    if matches.opt_present("h") {
        print_usage(&program, options);
        return;
    }

    // version info requested
    if matches.opt_present("v") {
        print_version();
        return;
    }

    // allow a single input file name parameter.
    // ! Pending change upon Page trait impl.
    let _input_file_path = match matches.free.len() {
        1 => &matches.free[0],
        _ => {
            print_usage(&program, options);
            std::process::exit(1);
        }
    };

    // TODO:
    // Execute the file handler page collection.
    // Better error reporting:  WARN, INFO, ERROR, etc..,
    // exit in the event of a failure.

}
