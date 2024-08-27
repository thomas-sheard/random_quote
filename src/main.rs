use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("loading file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    println!("which contains:\n{contents}");

}
