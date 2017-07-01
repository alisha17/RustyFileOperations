use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path_to_read = Path::new("hello.txt");

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path_to_read) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open due to: {}", why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}", why.description()),
        Ok(_) => print!("{} contains:\n", s),
    }

    let path_to_write = Path::new("hello.txt");

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path_to_write) {
        Err(why) => panic!("couldn't create {}", why.description()),
        Ok(file) => file,
    };

    match file.write_all(&s.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}", why.description())
        },
        Ok(_) => println!("successfully wrote"),
    }




}