extern crate getopts;
extern crate itertools;

use std::env;
use std::fs::File;
// use std::io;
use std::io::prelude::*;

use getopts::Options;

pub mod errors;
pub mod scanner;

fn process_file<F>(input_path: &str, log_error: F) where F: Fn(&str) -> () {
    // Open the input file (read-only)
    let mut input_file = File::open(input_path).unwrap_or_else(|reason| {
        log_error(&format!("Unable to open input file '{}'", input_path));
        log_error(&reason.to_string());
        std::process::exit(1);
    });

    let mut buf = String::new();

    // Read the contents of the input file into buffer.
    input_file.read_to_string(&mut buf).unwrap_or_else(|reason| {
        log_error(&format!("Unable to read contents of input file '{}'", input_path));
        log_error(&reason.to_string());
        std::process::exit(1);
    });

    // Create a scanner over the input.
    let _scanner = scanner::Scanner::new(input_path, &buf);

    // TODO: parse scan would be called here.  See scanner.rs todo comment!
    //
    // For now, we just exit the process as good citizens.
    std::process::exit(1);
}

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
    let input_file_path = match matches.free.len() {
        1 => &matches.free[0],
        _ => {
            print_usage(&program, options);
            std::process::exit(1);
        }
    };

    // Execute the file handler or log an error and exit in the event of a failure.
    process_file(input_file_path, |reason| {
        println!("{}: error: {}", program, reason);
    });
}
