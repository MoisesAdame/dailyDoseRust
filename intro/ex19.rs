// Author: Moisés Adame-Aguilar
// Date: January 12, 2023
// Description: Chapter 10, Validating References with Lifetimes.

#![allow(dead_code)]

#[derive(Debug)]
struct Node<'a, T>{
	data: T,
	next: Option<&'a Node<'a, T>>
}

impl<'a, T> Node<'a, T>{
	fn new(data: T, next: Option<&'a Node<'a, T>>) -> Self{
		Self{
			data,
			next
		}
	}
}

#[derive(Debug)]
struct LinkedList<'a, T>{
	head: Option<&'a Node<'a, T>>
}

impl<'a, T> LinkedList<'a, T>{
	fn new() -> Self{
		Self{
			head:  None,
		}		
	}

	fn add_first(&self, data: T){
		match self.head {
			Some(expr) => println!("Not Empty LinkedList"),
			None => {
				let new_node: Node<T> = Node::new(data, None);
				self.head = Some(&'a new_node);
			}
		}
	}
}

fn main(){
	let list: LinkedList<u32> = LinkedList::new();

	list.add_first(32);

}