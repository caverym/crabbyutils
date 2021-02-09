use std::env;

fn version() {
    print!(
        "           basename in Rust  Copyright (C) 2021  Avery Murray
        This program comes with ABSOLUTELY NO WARRANTY.
        This is free software, and you are welcome to redistribute it
        under certain conditions.
        \n"
    )
}

fn help() {
    print!(
        "Usage: basename NAME [SUFFIX]
            or: basename OPTION... NAME...
        Print NAME with leading directory removed
        If specified, also remove SUFFIX.
        
            -a, --multiple      support multiple directory as NAME
            -s, --suffix        remove following string as SUFFIX; implies -a
            --help              display this help message
            --version           display version information
            \n"
    );
    version();
}

fn suffix(args: Vec<String>) {
    let _suf = args[2].as_str();
    let length = args.len();

    for i in 3..length {
        let _str = args[i].split('/').last().unwrap().strip_suffix(_suf).unwrap();

        print!("{} ", _str);
    }

    print!("\n");
}

fn multiple(args: Vec<String>) {
    let length = args.len();

    for i in 2..length {
        print!("{} ", args[i].split('/').last().unwrap());
    }
    print!("\n");
}

fn parse_args(args: Vec<String>) {

    match args[1].as_str() {
        "-a" => {
            multiple(args);
        }

        "--multiple" => {
            multiple(args);
        }

        "-s" => {
            suffix(args);
        }

        "--suffix" => {
            suffix(args);
        }

        "--help" => {
            help();
        }

        "--version" => {
            version();
        }

        _ => {
            match args.len() {
                3 => {
                    suffix(args);
                }
                _ => {
                    println!("{}", args[1].split('/').last().unwrap());
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            help();
        }
        _ => parse_args(args)
    }
    return;
}
