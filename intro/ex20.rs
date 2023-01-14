// Author: Moisés Adame-Aguilar
// Date: January 13, 2023
// Description: Linked List.

#![allow(dead_code)]

use std::mem::replace;

#[derive(Debug)]
enum Option<T>{
	None,
	Some(Box<Node<T>>)
}

#[derive(Debug)]
struct Node<T> {
	data: T,
	next: Option<T>
}

impl<T: std::fmt::Debug> Node<T> {
	fn new(data: T, next: Option<T>) -> Self{
		Self{
			data,
			next,
		}
	}

	fn print(&self){
		println!("{:?}", self.data);
	}
}

#[derive(Debug)]
struct LinkedList<T> {
	head: Option<T>
}

impl<T: std::fmt::Debug> LinkedList<T> {
	// Function for instatinating Linked Lists.
	fn new() -> Self{
		Self{
			head: Option::None
		}
	}

	// Function for adding an element to the head of the Linked List.
	fn add_first(&mut self, data: T){
		let new_head: Node<T> = Node::new(data, replace(&mut self.head, Option::None));

		self.head = Option::Some(Box::new(new_head));
	}

	// Function for printing each elemt in the Linked List.
	fn print(&self){
		let mut temp: &Option<T> = &self.head;
		loop{
			match &mut temp {
				Option::Some(temp_out) => {
					(**temp_out).print();
					temp = &(**temp_out).next;

				},Option::None => {break;}
			}
		}
	}
}