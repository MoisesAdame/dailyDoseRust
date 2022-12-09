// Author: Moisés Adame-Aguilar
// Date: December 9th, 2022
// Description: Solution of chap5 problems of book "Steve Klabnik, Carol 
// Nichols, Rust Community - The Rust Programming Language-No Starch Press (2018)"

// --STRUCTS
// Structs allow us to package data in a simple manner.
#![allow(dead_code)]

// Normal Struct
struct User{
	name: String,
	age: u8,
	email: String,
	active: bool
}

// Field Init Shorthand allows us to not write everything twice
fn build_user(name: String, age: u8, email: String) -> User{
	User{
		name,
		age,
		email,
		active: true
	}
}

// Tuple Struct
#[derive(Debug)]
struct Point(f32, f32);

// Unit-Like Structs
// They allow us to store data, that has no data (?)
// For example, what happens w/ the unit type ()

// Rectangle
#[derive(Debug)]
struct Rectangle(Point, Point);
fn area(rect: &Rectangle) -> f32{
	let base: f32 = rect.0.0 - rect.1.0;
	let height: f32 = rect.0.1 - rect.1.1;

	if height * base < 0.0 {
		-(height * base)
	}else{
		height * base
	}
}

fn main(){
	let mut user1: User = build_user(String::from("Moises Adame Aguilar"), 19, String::from("steveolf@protonmail.com"));

	user1.name = String::from("Moisés Adame Aguilar");

	println!("user1.name = {}", user1.name);

	let myrect: Rectangle = Rectangle(Point(2.5, 3.0), Point(7.3, 4.1));

	println!("Area(myrect) = {}", area(&myrect));

	println!("myrect = {:?}", myrect);

	println!("myrect = {:#?}", myrect);

}