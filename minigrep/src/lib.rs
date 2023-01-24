// 作家：摩西
// 日期：二零二三年一月二十三日
// 形容：Minigrep command line tool.

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

// Definition of the atributes of the Config struct.
pub struct Config{
    query: String,
    filename: String
}

// Definition of the methods of the Config struct.
impl Config{
    pub fn new(args: &Vec<String>) -> Result<Self, &'static str>{
        if args.len() > 3{
            Err("Too much arguments!")
        }else if args.len() < 3{
            Err("Very few arguments!")
        }else{
            let query: String = args[1].to_string();
            let filename: String = args[2].to_string();
            Ok(Self{query, filename})
        }
    }
}

// Definition of the run() function.
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // Printing query and filename.
    println!("Searching for: {:?}", config.query);
    println!("In file: {:?}", config.filename);

    // Opennig the file.
    let mut file_open: File = File::open(config.filename)?;

    // File contents.
    let mut contents: String = String::new();

    // Reading the file.
    file_open.read_to_string(&mut contents)?;

    // Printing file contents.
    println!("With text: \n{}", contents);

    Ok(())
}