// Author: Moisés Adame-Aguilar
// Date: January 11, 2023
// Description: Chapter 8, Common collections (Hash Map).

#![allow(dead_code)]

// Use the HashMap from the collections.
use std::collections::HashMap;

#[derive(Debug)]
struct RGB{
	red: u8,
	green: u8,
	blue: u8
}

impl RGB{
	// Function that initializes an RGB object with field init shorthand.
	fn new(red: u8, green: u8, blue: u8) -> RGB{
		RGB{
			red,
			green,
			blue
		}
	}

	// Function that prints a color.
	fn print(&self){
		println!("Color = ({}, {}, {})", self.red, self.green, self.blue);
	}

	fn to_string(&self) -> String{
		format!("Color = ({}, {}, {})", self.red, self.green, self.blue)
	}
}

fn main(){
	// Now the Hash Map is created.
	let mut colors: HashMap<String, RGB> = HashMap::new();

	// Inserting some Key-Value Elements.
	colors.insert(String::from("red"), RGB::new(255, 0, 0));
	colors.insert(String::from("green"), RGB::new(0, 255, 0));
	colors.insert(String::from("blue"), RGB::new(0, 0, 255));
	colors.insert(String::from("yellow"), RGB::new(255, 255, 0));
	colors.insert(String::from("cyan"), RGB::new(0, 255, 255));
	colors.insert(String::from("black"), RGB::new(0, 0, 0));
	colors.insert(String::from("white"), RGB::new(255, 255, 255));

	// Accessing values from the Hash Map. Simple Way.
	let _white_rgb: &RGB = &colors["white"];

	// Accessing values from the Hash Map. Hard Way.
	match colors.get(&String::from("cyan")){
		None => {
			println!("None")
		},
		Some(i) => {
			println!("{}", i.to_string())
		}
	};

	for (key, value) in &colors{
		println!("{} -> {}", key, value.to_string());
	}
}