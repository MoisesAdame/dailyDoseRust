// Author: Moisés Adame-Aguilar
// Date: January 13, 2023
// Description: Uses of the LinkedList

#![allow(dead_code)]

mod structures;

fn main(){
	let mut list1: structures::linked_list::LinkedList<u32> = structures::linked_list::LinkedList::new();

	for i in 0..10 {
		list1.add_first(i);
	}

	list1.add_last(25);
	// list1.add_last(25);
	// list1.add_last(25);
	// list1.add_last(25);

	list1.print();

	//println!("Size: {:?}", list1);
}