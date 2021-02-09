use std::env;

fn version() {
    print!(
        "   basename 0.1 in Rust  Copyright (C) 2021  Avery Murray
This program comes with ABSOLUTELY NO WARRANTY.
This is free software, and you are welcome to redistribute it
under certain conditions.\n"
    );
}

fn help() {
    print!(
        "Usage: basename NAME [SUFFIX]
or: basename OPTION... NAME...
Print NAME with leading directory removed
If specified, also remove SUFFIX.\n
        
    -a, --multiple      support multiple directory as NAME
    -s, --suffix        remove following string as SUFFIX; implies -a
    --help              display this help message
    --version           display version information\n"
    );
    print!("\n");
    version();
}

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

fn if_arg(the_arg: String) -> i8 {
    let chr: Vec<char> = the_arg.chars().collect();

    if chr[0] == '-' {
        print!("basename: invalid option -- '{}'
Try 'basename --help' for more information\n", chr[1]);
        return 1;
    }

    return 0;
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

        "--version" => {
            version();
        }

        "--help" => {
            help();
        }

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
        1 => {
            println!("basneme: missing argument
Try 'basename --help' for more information");
        }

        2 => {
            match args[1].as_str() {
                "--help" => {
                    help();
                }

                "--version" => {
                    version();
                }

                _ => {
                    if if_arg(args[1].to_string()) == 1 {
                        return;
                    }
                    println!("{}", args[1].split('/').last().unwrap());
                }
            }
        }

        _ => {
            parse_args(args);
        }
    }
}
