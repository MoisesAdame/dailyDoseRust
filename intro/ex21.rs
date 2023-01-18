// Author: Moisés Adame-Aguilar
// Date: January 17, 2023
// Description: Error Handling.

// Rust, as opposed awith other languages, doesn't have exceptions.
// Ryst has the type Result<T, E> for recoverable errors and the
// panic! macro for unrecoverable ones.

#![allow(dead_code)]

use std::fs::File;
use std::io::Read;

// The type Result<T, E> is an enum.
enum MyResult<T, E>{
	Ok(T),
	Err(E)
}

// This is propagating error, namely, to return a result type.
fn read_file1(file_name: &str) -> Result<String, std::io::Error>{
	// First, the file is oppened.
	let file_open: Result<File, std::io::Error> = File::open(file_name.to_string());

	// We open and see if there is an error. If this is true, we return it.
	let mut file_read = match file_open {
		Ok(expr) => expr,
		Err(err) => return Err(err),
	};

	// Returning our string result
	let mut string_read: String = String::new();

	match file_read.read_to_string(&mut string_read){
		Ok(_) => Ok(string_read),
		Err(err) => Err(err),
	}
}

// This is a common patter, so rust implemented the ? operator.
// read_file2() has the same functionaity as read_file1() but using less lines.
// ** Is important to metion that this operator only works inside 
// functions that return a Result<T, E> type.
fn read_file2(file_name: &str) -> Result<String, std::io::Error>{
	// First the file is oppend. The ? deals if an error happend to exist.
	let mut file_open: File = File::open(file_name.to_string())?;

	// Then, we create a string to return it.
	let mut string_read: String = String::new();

	// Finally, we pass what we read, to a string.
	file_open.read_to_string(&mut string_read)?;

	// If there is no error, we return what we read inside an Ok.
	Ok(string_read)
}

fn main(){
	// The panic macro, breaks the program and prints a message.
	let vec1: Vec<u32> = vec![1, 2, 3, 4, 5];
	let some_index: usize = 3;
	if some_index < vec1.len(){
		println!("The value is: {:?}", vec1[some_index]);
	}else{
		panic!("Index out of bounds!");
	}

	// Using the result type is much like using the Option<T> type.
	let file_read: Result<File, std::io::Error> = File::open("hello.txt");

	// let file_open = match file_read {
	// 	Ok(file) => file,
	// 	Err(err) => {
	// 		panic!("File oppening error: {:?}", err);
	// 	},
	// };

	// To do what we did above, but in one line, we can use the unwrap() method.
	//let file_open: File = file_read.unwrap();

	// If we don't want to use the default panic message, we can use expect().
	let file_open: File = file_read.expect("Non default painc message");
}