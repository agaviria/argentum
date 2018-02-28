use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub mod errors;

fn run(source: &String) -> Result<(), errors::InputError> {
    unimplemented!();
}

fn run_file(archive: &str) -> Result<(), errors::InputError> {
    let f = match open_file(&archive.to_owned()) {
        Ok(f) => parse_file(f)?,
        Err(e) => {
            println!("Error opening file: {}", e)
        }
    };
    Ok(())
}


fn open_file(file_name: &String) -> Result<File, io::Error> {
    let mut path = env::home_dir().unwrap();
    path.push(file_name);
    println!("path : {:?}", path);
    File::open(path)
}


fn parse_file(file: File) -> Result<(), errors::ParseError> {
    unimplemented!();
}

fn run_prompt() -> Result<(), errors::InputError> {
    loop {
        print!("> ");
        io::stdout().flush();
        let mut source = String::new();
        io::stdin().read_line(&mut source);
        try!(run(&source));
        ()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: ag [script]");
            Ok(())
        }
    };
    let control_flow = match result {
        Ok(_) => 0,
        Err(e) => {
            println!("{:?}", e);
            1
        }
    };
    std::process::exit(control_flow)
}
