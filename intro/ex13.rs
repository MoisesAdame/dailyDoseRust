// Author: Moisés Adame-Aguilar
// Date: January 11, 2023
// Description: Chapter 8, Common collections (String).

fn main(){
	// Creating a String with some data in it.
	let mut s1: String = String::from("Hello World!");
	println!("{}", s1);

	// To concatenate it we can use +
	s1 = s1 + &String::from(" This is an append.");
	println!("{}", s1);

	// Indexing into strings. Rust doesn't support indexing.
	// It doesn't allow it because some strings, like "hola"
	// and "друг", even tho they have te same number of le-
	// tters, their lenght is different being 4 for the first
	// str and 8 for the second because of the UTF-8 encoding.

	// String slicing uses bytes as parameters, also, it's
	// return value is &str.
	let s2: String = String::from("你好世界");
	let s2_substr: &str = &s2[0..3];
	let s1_substr: &str = &s1[0..1];
	println!("s1_substr = {}", s1_substr);
	println!("s2_substr = {}", s2_substr);

	// Iterating over Strings.
	for i in s2.chars(){
		print!("{} ", i);
	}
	println!();

	//最后，Strings很难啊。
}