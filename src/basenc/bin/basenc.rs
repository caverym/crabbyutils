extern crate base64;
use std::env;

fn help() {
    println!("Help message");
}

fn to_64(args: Vec<String>) {
    let chr: Vec<char> = args[2].chars().collect();
    let mut conv = [0; 1024];
    let length = chr.len();

    for i in 0..length {
        match chr[i] {
            'A' => {conv[i] = 0;}
            'B' => {conv[i] = 1;}
            'C' => {conv[i] = 2;}
            'D' => {conv[i] = 3;}
            'E' => {conv[i] = 4;}
            'F' => {conv[i] = 5;}
            'G' => {conv[i] = 6;}
            'H' => {conv[i] = 7;}
            'I' => {conv[i] = 8;}
            'J' => {conv[i] = 9;}
            'K' => {conv[i] = 10;}
            'L' => {conv[i] = 11;}
            'M' => {conv[i] = 12;}
            'N' => {conv[i] = 13;}
            'O' => {conv[i] = 14;}
            'P' => {conv[i] = 15;}
            'Q' => {conv[i] = 16;}
            'R' => {conv[i] = 17;}
            'S' => {conv[i] = 18;}
            'T' => {conv[i] = 19;}
            'U' => {conv[i] = 20;}
            'V' => {conv[i] = 21;}
            'W' => {conv[i] = 22;}
            'X' => {conv[i] = 23;}
            'Y' => {conv[i] = 24;}
            'Z' => {conv[i] = 25;}
            'a' => {conv[i] = 26;}
            'b' => {conv[i] = 27;}
            'c' => {conv[i] = 28;}
            'd' => {conv[i] = 29;}
            'e' => {conv[i] = 30;}
            'f' => {conv[i] = 31;}
            'g' => {conv[i] = 32;}
            'h' => {conv[i] = 33;}
            'i' => {conv[i] = 34;}
            'j' => {conv[i] = 35;}
            'k' => {conv[i] = 36;}
            'l' => {conv[i] = 37;}
            'm' => {conv[i] = 38;}
            'n' => {conv[i] = 39;}
            'o' => {conv[i] = 40;}
            'p' => {conv[i] = 41;}
            'q' => {conv[i] = 42;}
            'r' => {conv[i] = 43;}
            's' => {conv[i] = 44;}
            't' => {conv[i] = 45;}
            'u' => {conv[i] = 46;}
            'v' => {conv[i] = 47;}
            'w' => {conv[i] = 48;}
            'x' => {conv[i] = 49;}
            'y' => {conv[i] = 50;}
            'z' => {conv[i] = 51;}
            '0' => {conv[i] = 52;}
            '1' => {conv[i] = 53;}
            '2' => {conv[i] = 54;}
            '3' => {conv[i] = 55;}
            '4' => {conv[i] = 56;}
            '5' => {conv[i] = 57;}
            '6' => {conv[i] = 58;}
            '7' => {conv[i] = 59;}
            '8' => {conv[i] = 60;}
            '9' => {conv[i] = 61;}
            '+' => {conv[i] = 62;}
            '/' => {conv[i] = 63;}
            _ => {conv[i] = 100;}
        }
    }

    for i in 0..length {
        print!("{}", chr[i].to_ascii_lowercase());
    }
    print!("\n");

}

fn parse_args(args: Vec<String>) {
    match args[1].as_str() {
        "--base64" => {
            to_64(args);
        }

        _ => {help()}
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {help();}

        _ => {parse_args(args);}
    }
}