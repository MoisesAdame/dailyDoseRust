// Author: Moisés Adame-Aguilar
// Date: January 18, 2023
// Description: Reproduction of the command line program grep.

use std::env;
use std::fs::File;
use std::io::Read;
use std::io;
use std::io::prelude::*;

// Type that groups both of the arguments passed.
struct Config{
    query: String,
    file_name: String
}

// Methods for the Config type.
impl Config{
    fn new(query: String, file_name: String) -> Self{
        println!("Query: {:?}", &query);
        println!("File Name: {:?}", &file_name);
        Self{
            query,
            file_name,
        }
    }
}

// Propagating function that reads the text contents of a file.
fn read_file(config: &Config) -> Result<String, io::Error>{
    // Opening the file.
    let mut read_file: File = File::open(&config.file_name)?;

    // String for storing the text contets of the file.
    let mut string_read: String = String::new();

    // Converting the insides of the file into a sting.
    read_file.read_to_string(&mut string_read)?;

    // Returning what was read inside a Result::Ok type.
    Ok(string_read)
}

// Parse arguments and error handling.
fn parse_args(some_vec: &Vec<String>) -> Result<Config, &'static str>{
    // If the user inputs more or less than 2 inputs, 
    // the program gets broken.
    if some_vec.len() != 3{
        return Err("Invalid number of command line arguments.");
    }

    // Storing the command line arguments as variables.
    let query: String = some_vec[1].to_string();
    let file_name: String = some_vec[2].to_string();

    // Returning the values.
    Ok(Config::new(query, file_name))
}

fn run(some_vec: &Vec<String>){
    // Storing the command line arguments as variables.
    let config: Config = match parse_args(some_vec){
        Ok(expr) => expr,
        Err(e) => {
            panic!("Prblem in parse_args(). {:?}", e);
        }
    };

    // Storing the text contets inside a variable.
    let file_contents: String = match read_file(&config) {
        Ok(expr) => expr,
        Err(e) => {
            panic!("An error occured: {:?}", e);
        }
    };

    println!("{}", file_contents);
}


fn main() {
    // Allowing the program to read the command line arguments.
    let args: Vec<String> = env::args().collect();

    run(&args);
}