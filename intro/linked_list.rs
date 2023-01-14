// Author: Moisés Adame-Aguilar
// Date: January 13, 2023
// Description: Linked List.

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
pub struct LinkedList<T> {
	head: Option<T>,
	size: usize
}

impl<T: std::fmt::Debug> LinkedList<T> {
	// Function for instatinating Linked Lists.
	pub fn new() -> Self{
		Self{
			head: Option::None,
			size: 0
		}
	}

	// Function for adding an element to the head of the Linked List.
	pub fn add_first(&mut self, data: T){
		let new_head: Node<T> = Node::new(data, replace(&mut self.head, Option::None));

		self.head = Option::Some(Box::new(new_head));

		self.size += 1;
	}

	// Function that returns the number of nodes in the Linked List.
	pub fn len(&self) -> usize{
		self.size
	}

	// Function that returns the nth element inside the Linked List.
	pub fn get(&self, n: usize) -> &T{
		let mut temp: &Option<T> = &self.head;
		for _i in 0..n{
			match &mut temp {
				Option::Some(temp_out) => {
					temp = &(**temp_out).next;
				},
				Option::None => {
					break;
				},
			}
		}
		// Return is missing.
	}

	// Function for printing each elemt in the Linked List.
	pub fn print(&self){
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