use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

// // Verbose version
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     let mut username = String::new();
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// Shorter version
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// Even shorter version
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }

// Shortest version
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    println!("file: {:?}", greeting_file);

    let username = read_username_from_file().unwrap();

    println!("username: {}", username);

    // Return the value inside Ok, rf the Result is the Err variant, unwrap will call panic!
    // let greeting_file = File::open("hello.txt").unwrap();

    // Same as unwrap but with custom message
    // let greeting_file =
    //     File::open("hello.txt").expect("hello.txt should be included in this project");

    let _greeting_file = File::open("hello.txt")?;

    Ok(())
}
