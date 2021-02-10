use std::env;
use std::fs;
use std::io::{stdin,stdout,Write};

fn cat_nothing() {
    loop {
        let mut _str = String::new();
        let _=stdout().flush();
        stdin().read_line(&mut _str).expect("Failed to write");
        print!("{}", _str);
    }
}

fn basic_read_file(path: &str) -> String {
    let file = fs::read_to_string(path).expect("Failed to read file");
    return file;

}

fn parse_args(args: Vec<String>) {
    let length = args.len();
    match args[1].as_str() {
        // extra args will be added soon:tm:
        _ => {
            for i in 1..length {
                print!("{}", basic_read_file(args[i].as_str()));
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => cat_nothing(),
        _ => parse_args(args),
    }
}