// 作家：摩西
// 日期：二零二三年一月二十三日
// 形容：Minigrep command line tool.

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

// Definition of the atributes of the Config struct.
#[derive(Debug, PartialEq)]
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

// Definition of search, a funtions that looks for certain
// query inside a str and returns a vector of it.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    // The vector we are goinf to return.
    let mut res: Vec<&str> = Vec::new();

    // Searching the regular expression.
    for lines in contents.lines(){
        if lines.contains(query){
            res.push(lines);
        }
    }

    // The res vector is returned.
    res
}

// Atomic Tests
#[cfg(test)]
mod test{
    use super::*;

    // Test for Config related methods.
    #[test]
    fn config_new_1_test(){
        // The arguments that will go inside the Vector args.
        let lib_path: String = String::from("some_path");
        let query: String = String::from("some_query");
        let filename: String = String::from("some_filename.txt");

        // Definition of the Vector args.
        let args: Vec<String> = vec![lib_path, query, filename];

        // Generating the Coonfig instance.
        let config = Config::new(&args).unwrap_or_else(|err|{
            panic!("Problem parsing arguments: {}", err);
        });

        let config_theo: Config = Config{query: String::from("some_query"),
                                  filename: String::from("some_filename.txt")};

        assert_eq!(config_theo, config);
    }

    #[test]
    #[should_panic]
    fn config_new_2_test(){
        // The arguments that will go inside the Vector args.
        let lib_path: String = String::from("some_path");
        let query: String = String::from("some_query");
        let filename: String = String::from("some_filename.txt");
        let extra_arg: String = String::from("extra_unnecesary_argument");

        // Definition of the Vector args.
        let args: Vec<String> = vec![lib_path, query, filename, extra_arg];

        // Generating the Coonfig instance.
        let _config = Config::new(&args).unwrap_or_else(|err|{
            panic!("Problem parsing arguments: {}", err)
        });
    }

    #[test]
    #[should_panic]
    fn config_new_3_test(){
        // The arguments that will go inside the Vector args.
        let query: String = String::from("some_query");

        // Definition of the Vector args.
        let args: Vec<String> = vec![query];

        // Generating the Coonfig instance.
        let _config = Config::new(&args).unwrap_or_else(|err|{
            panic!("Problem parsing arguments: {}", err);
        });
    }

    // Tests for utility funtions.
    #[test]
    fn search_test() {
        let query: &str = "duct";
        let contents: &str = "Rust:\nsafe, fast, productive.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}