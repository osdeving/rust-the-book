use std::fs::File;
use std::io::{ErrorKind, Read, Write};

fn main() {
    //panic!("crash and burn");

    //let v = vec![1, 2, 3];

    //v[99];

    let greeting_file_result = File::open("hello.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) =>  match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    greeting_file.write("Some text into file".as_bytes()).expect("Failed to write to file");

    let mut s = String::new();

    greeting_file.read_to_string(&mut s).expect("Failed to read file");

    println!("{s}");
}