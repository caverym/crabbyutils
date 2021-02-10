use std::env;

// logic
fn multiple(args: Vec<String>) {
    let length = args.len();

    for i in 2..length {
        print!("{}\n", args[i].split('/').last().unwrap());
    }
}

fn suffix(args: Vec<String>) {
    let _suf = args[2].as_str();
    let length = args.len();

    for i in 3..length {
        let mut _str = args[i].as_str();
        _str = _str.split('/').last().unwrap();
        _str = _str.strip_suffix(_suf).unwrap();
        print!("{}\n", _str);
    }
}

fn suffix_basic(args: Vec<String>) {
    let _suf = args[2].as_str();
    let _str = args[1].split('/').last().unwrap().strip_suffix(_suf).unwrap();
    println!("{}", _str);
}

// messages
fn if_arg(the_arg: String) -> i8 {
    let chr: Vec<char> = the_arg.chars().collect();
    if chr[0] == '-' {
        println!("basename: invalide option -- '{}'", chr[1]);
        println!("Try 'basename --help' for more information");
        return 1;
    }
    return 0;
}

fn version() {
    println!("  basename 0.1 in Rust  Copyright (C) 2021  Avery Murray");
    println!("This program comes with ABSOLUTELY NO WARRANTY.");
    println!("This is free software, and you are welcome to redistribute it");
    println!("under certain conditions.")
}

fn help() {
    println!("Usage: basename NAME [SUFFIX]");
    println!("   or: basename OPTION... NAME...");
    println!("Print NAME with leading directory removed");
    println!("If specified, also remove SUFFIX.\n");
    println!("  -a, --multiple      support multiple directory as NAME");
    println!("  -s, --suffix        remove following string as SUFFIX; implies -a");
    println!("  --help              display this help message");
    println!("  --version           display version information\n");
    version();
}

fn missing_args() {
    println!("basneme: missing argument");
    println!("Try 'basename --help' for more information");
}

// startup
fn parse_args(args: Vec<String>) {
    match args[1].as_str() {
        "-a" => multiple(args),
        "--multiple" => multiple(args),

        "-s" => suffix(args),
        "--suffix" => suffix(args),

        "--version" => version(),
        "--help" => help(),

        _ => {
            if if_arg(args[1].to_string()) == 1 {
                return;
            }
            suffix_basic(args);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => missing_args(),
        2 => {
            match args[1].as_str() {
                "--help" => help(),
                "--version" => version(),
                _ => {
                    if if_arg(args[1].to_string()) == 1 {
                        return;
                    }
                    println!("{}", args[1].split('/').last().unwrap());
                }
            }
        }
        _ => parse_args(args)
    }
}
