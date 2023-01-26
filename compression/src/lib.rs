// 作家：摩西
// 日期：二零二三年一月二十五日
// Description：Modules used in the CLI.

pub mod run_length;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

#[derive(Debug)]
pub struct Query{
	algorithm: String,
	option: String,
	filename: String
}

impl Query{
	pub fn new(args: &Vec<String>) -> Result<Self, &'static str>{
		if args.len() > 4{
			Err("Too many arguments!")
		}else if args.len() < 3{
			Err("Missing arguments!")
		}else if args.len() == 4{
			Ok(
				Self{algorithm: args[1].to_string(), option: args[2].to_string(), filename: args[3].to_string()}
			)
		}else{
			Ok(
				Self{algorithm: args[1].to_string(), option: "encode".to_string(), filename: args[3].to_string()}
			)
		}
	}
}

fn read_file(filename: &str) -> Result<String, Box<dyn Error>>{
	// Opennig the file.
    let mut file_open: File = File::open(filename)?;

    // File contents.
    let mut contents: String = String::new();

    // Reading the file.
    file_open.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn run(query: &Query){
	// Obtaining contents of .txt file
	let contents = read_file(&query.filename).unwrap_or_else(|err|{
		println!("Problem reading file contents: {}", err);
        process::exit(1);
	});

	// Determining which algorithm to use.
	if query.algorithm == "runlength" && query.option == "encode"{
		println!("Using Run Length Compression...");
		println!("Ecoding: {}", query.filename);

		println!("Contents:\n{}", contents);
	}else if query.algorithm == "runlength" && query.option == "decode" {
		println!("Using Run Length Compression...");
		println!("Decoding: {}", query.filename);

		println!("Contents:\n{}", contents);
	}else{
		println!("Invalid arguments");
        process::exit(1);
	}
}






