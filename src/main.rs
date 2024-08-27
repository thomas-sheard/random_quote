use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("not enough arguments! expected a path to a file.");
    } else if args.len() > 2 {
        panic!("too many arguments! only expected a path to a file.");
    }

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("should have been able to read the file");
    let contents = contents.split("\n\n");

    for line in contents {
        println!("quote: {line}");
    }

}
