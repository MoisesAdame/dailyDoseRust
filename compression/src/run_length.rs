// 作家：摩西
// 日期：二零二三年一月二十五日
// Description：Encoding and Decoding using the Run Lenght Algorithm.

use std::fs::File;
//use std::io::prelude::*;
use std::error::Error;

// Funtion that creates the encoded file
pub fn encode(contents: &String) -> Result<(), Box<dyn Error>>{
	let mut filename_encoded: &str = contents;

	filename_encoded = &filename_encoded[0..filename_encoded.len() - 4];

	let filename_encoded: String = filename_encoded.to_string() + "_encoded.txt";

	let  _file_encoded = File::create(&filename_encoded)?;

	Ok(())
}

#[cfg(test)]
mod test{
	use super::*;

	#[test]
	#[ignore]
	fn encode_test(){
		// The filename of the original file.
		let filename: String = String::from("random.txt");

		// Encoding the file
		encode(&filename);

		// The filename of the encoded file.
		let filename: String = String::from("random_encoded.txt");

		// Looking for the file.
    	let _file_open: File = File::open(filename).unwrap_or_else(|err|{
    		panic!("Error finding the encoded file: {}", err);
    	});
	}
}
