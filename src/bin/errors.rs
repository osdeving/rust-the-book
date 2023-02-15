#![allow(unused, dead_code)]

use core::panic;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};

fn main() {
    //panic!("crash and burn");

    //let v = vec![1, 2, 3];

    //v[99];

    let greeting_file_result = File::open("hello.txt");

    // let mut greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) =>  match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    let mut greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // greeting_file.write("Some text into file".as_bytes()).expect("Failed to write to file");

    // let mut s = String::new();

    // greeting_file.read_to_string(&mut s).expect("Failed to read file");

    // println!("{s}");

    // unwrap: if not error, return a value of Ok Result variant, if error then panic!
    //let file = File::open("other-file.txt").unwrap();

    // expect: if not error, return a value of Ok Result variant, if error then panic! expect accepts a message
    let file = File::open("other-file.txt").expect("other-file.txt should be included in this project");



}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}