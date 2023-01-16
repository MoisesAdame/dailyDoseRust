// Author: Moisés Adame-Aguilar
// Date: January 13, 2023
// Description: Linked List.

use std::mem::replace;

#[derive(Debug, PartialEq)]
enum Option<T>{
	None,
	Some(Box<Node<T>>)
}

#[derive(Debug, PartialEq)]
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

impl<T: std::fmt::Debug + std::cmp::PartialEq> LinkedList<T> {
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

	pub fn add_last(&mut self, data: T){
		let mut temp: &Option<T> = &self.head;
		self.size += 1;
		let mut new_tail: &Option<T> = &Option::Some(Box::new(Node::new(data, Option::None)));

		loop {
			match &mut temp{
				Option::Some(temp_out) => replace(&mut temp, &(**temp_out).next),
				Option::None => &replace(&mut new_tail, &Option::None),
			};

			if *temp == Option::None && *new_tail == Option::None{
				println!("Happend: {:?}", self);
				break;
			}else{
				println!("Not Happend");
			}
		}
	}

	// Function that returns the number of nodes in the Linked List.
	pub fn len(&self) -> usize{
		self.size
	}

	// Function for printing each elemt in the Linked List.
	pub fn print(&self){
		let mut temp: &Option<T> = &self.head;
		loop{
			match &mut temp {
				Option::Some(temp_out) => {
					(**temp_out).print();
					temp = &(**temp_out).next;
				},Option::None => {
					break;
				}
			}
		}
	}
}