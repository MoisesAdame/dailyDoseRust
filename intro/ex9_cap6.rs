// Author: Moisés Adame-Aguilar
// Date: December 9th, 2022
// Description: Solution of chap5 problems of book "Steve Klabnik, Carol 
// Nichols, Rust Community - The Rust Programming Language-No Starch Press (2018)"

#![allow(dead_code)]

// Define the Option enum w/ T data type
/*
#[derive(Debug)]
enum Option<T>{
	Some(T),
	None,
}
*/

// The match control flow operator
enum Coin{
	Penny,
	Nickel,
	Dime,
	Quarter,
}

impl Coin{
	fn coin2cents(&self) -> u32{
		match self{
			// Every case is called 'arm'
			Coin::Penny => 1,
			Coin::Nickel => 5,
			Coin::Dime => 10,
			Coin::Quarter => 25,
		}
	}
}

#[derive(Clone)]
struct Node<'a, T>{
	data: T,
	next: &'a Option<Node<'a, T>>,
}

impl<'a, T: std::fmt::Debug> Node<'a, T>{
	fn new(data: T, next: &'a Option<Node<'a, T>>) -> Node<'a, T>{
		Node{
			data,
			next,
		}
	}

	fn print(&self){
		println!("{:?}", self.data);
	}
}

struct LinkedList<'a, T>{
	root: Option<Node<'a, T>>,
}

impl<'a, T: std::fmt::Debug> LinkedList<'a, T>{
	fn new() -> LinkedList<'a, T>{
		LinkedList{
			root: Option::None,
		}
	}

	
	fn add_first(&mut self, val: T){
		match &self.root{
			Option::None => {
				self.root = Option::Some(Node::new(val, &Option::None));
			},
			Option::Some(Node) => {
				self.root = Option::Some(Node::new(val, &Option::Some(*Node)));
			},
		}
	}
}


fn main(){

	let mut list1: LinkedList<i32> = LinkedList::new();
	//list1.add_last(32);

	//println!("[*] Situation of root = {}", list1.add_last());




	let abscent_int: Option<i32> = Option::None;

	let some_int:Option<i32>  = Option::Some(23);

	println!("[*] some_int = {:?}", some_int);
	println!("[*] abscent_int = {:?}", abscent_int);

	let one_coin: Coin = Coin::Penny;
	let other_coin: Coin = Coin::Dime;

	println!("[*] a penny is worth = {}c", one_coin.coin2cents());

	println!("[*] a dime is worth = {}c", other_coin.coin2cents());


}