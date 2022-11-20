use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {

    let mut user_name = String::new();
    //println!("{}",encryption(test_data.to_string()));
    //println!("{}",decryption(encryption(test_data.to_string())));

    create_file("passwords.txt");
    write_file("[USERNAME:]\n");
    let contents = read_file("passwords.txt");
    println!("{contents}");
    let mut tokens = parser(contents);
    let mut line = 0;
   
    for token in &tokens {
        if token.0 == "USERNAME" && token.1 == ""{
            println!("password-manager>Enter the user name : ");
            std::io::stdin().read_line(&mut user_name).unwrap();
            user_name = user_name.trim().to_string();
            //tokens[0].1 = user_name;
            //create_file("passwords.txt");
            //write_file(tokens.to_string());
        }
        line = line + 1;
    }
    println!("{:?}",tokens);

    loop {
        let mut command = String::new();
        println!("password-manager>{}>",user_name);
        std::io::stdin().read_line(&mut command).unwrap();
        command = (&command.to_string().trim()).to_string();
        match command.as_str() {
            "exit" => {  break; }
            "new user" => {
                println!("Enter the user name : ");
                std::io::stdin().read_line(&mut user_name).unwrap();
                write_file(format!("[USERNAME:{}]",user_name.trim()).as_str());
                println!("New user is created successfully!");
            }
            &_ => { println!("Unable to find the command {command}.") }
        }
    }
}

// Parsing the raw string each by line
// Token_Identifier will recognize the tokens and return them
// 
// @params contents : String
// @return tokens : Vec<(String, String)>

fn parser(contents: String) -> Vec<(String, String)>{

    let mut tokens: Vec<(String, String)> = Vec::new();
    let mut lines: Vec<char> = Vec::new();

    // Iterating the raw string as a char
    for chars in contents.chars() {

        // Checking for new line sperator
        // for spliting the lines
        if chars == '\n' {

            // Getting the key and value from the current line
            tokens.push(token_identifier(&lines));
            // Clearing the line for storing the next line
            lines.clear();
            continue; 
        }

        // Constructing the line until \n
        lines.push(chars);
    }

    // @return
    tokens
}

// Token identifier is used to identify the token.
//
// @params line : Vec<char>
// @returns key : String, value : String

fn token_identifier(line: &[char]) -> (String, String) {

    // For token recognize
    let mut start_token:bool = false;
    let mut value_token:bool = false;

    // For storing the key and value
    let mut key = String::new();
    let mut value = String::new();
    
    for character in line {

        // Token recognizer
        // If token is recognized then set the value as true
        match character {
            //'\n' => {break;}
            '[' => {start_token = true; continue;}
            ']' => {start_token = false;}
            ':' => {value_token = true; continue;}
            _=> {start_token = true;}
        }

        // Constructing the token
        // As key and value
        if start_token {
            if value_token {
                value = value + &character.to_string();
                continue;
            }
            key = key + &character.to_string();
        }
    }
    // @return
    (key, value)
}

// Creating a file for storing the encrypted passwords
//
// @params Path, Content

fn create_file(path: &str) {
    let mut my_file = File::create(path).expect("Could not create the file");
    my_file.write_all(b"[Encrypted File has been created]\n").expect("Failed to write into the file.");
}

fn write_file(content: &str) {
    let mut my_file = OpenOptions::new().append(true).open("passwords.txt").expect("Failed to open the file");
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