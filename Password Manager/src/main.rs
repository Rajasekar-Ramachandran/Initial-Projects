use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {

    let test_data = "Rajasekar";
    println!("{}",encryption(test_data.to_string()));
    println!("{}",decryption(encryption(test_data.to_string())));
    create_file("test.txt","Dummy text");
    let contents = read_file("test.txt");
    println!("{contents}");
    while true {
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();
        command = (&command.to_string().trim()).to_string();
        match command.as_str() {
            "exit" => {  break; }
            &_ => { println!("Unable to find the command {command}.") }
        }
    }
}

// Creating a file for storing the encrypted passwords
//
// @params Path, Content

fn create_file(path: &str, content: &str) {
    let mut my_file = File::create(path).expect("Could not create the file");
    my_file.write_all(content.as_bytes()).expect("Failed to write into the file.");
}

// Reading the encrypted file
//
// @params path
// @return contents

fn read_file(path: &str) -> String {
    let mut my_file = File::open(path).expect("Could not read the file");
    let mut contents = String::new();
    my_file.read_to_string(&mut contents).expect("Failed to read the text file");
    contents
}

// This function will read the original string and
// Converting the string ASCII value
// To encrypt the original string
//
// @params raw_data - String
// @returns encrypted_data - String

fn encryption(raw_data:String) -> String {

    // Initial neccessary value
    let mut length = raw_data.len();
    let mut encrypted_data = String::new().to_owned();

    // Iterating the original string as character 
    for character in raw_data.chars() {

        // Converting the ASCII value of the character
        // By adding the length of the original string
        let data = (character as u8) + length as u8;
        // Adding the character into the encrypted string as a result
        encrypted_data = encrypted_data + &(data as char).to_string();
        length = length - 1;
    }
    // Returning the encrypted string
    encrypted_data
}


// This function will read the encrypted string and
// Converting the encrypted string to the original string
//
// @params encrypted_data - String
// @returns decrypted_data - String

fn decryption(encrypted_data:String) -> String {
    // Initial neccessary value
    let mut length = encrypted_data.len();
    let mut decrypted_data = String::new().to_owned();

    // Iterating the encrypted string as character 
    for character in encrypted_data.chars() {

        // Converting the encrypted character into the original character
        // By subtracting the length of the encrypted string
        let data = (character as u8) - length as u8;
        // Adding the character into the decrypted string as a result
        decrypted_data = decrypted_data + &(data as char).to_string();
        length = length - 1;
    }
    // Returning the decrypted string
    decrypted_data
}