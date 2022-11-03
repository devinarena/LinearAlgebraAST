use std::io;
use std::fs;

fn main() {
    let file_path = "src/main.rs";
    // read a file
    println!("In file {}", file_path);

    let data = fs::read_to_string(file_path);
    match data {
        Ok(data) => println!("With text: {}", data),
        Err(error) => println!("Error: {}", error),
    }
}
