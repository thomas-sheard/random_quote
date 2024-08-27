use std::env;
use std::fs;
use rand::Rng;

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

    let contents = contents.split("\r\r\n");

    let quotes = contents.collect::<Vec<&str>>();

    let random_number = rand::thread_rng().gen_range(1..=quotes.len()-1);

    println!("{}", quotes[random_number]);

}
