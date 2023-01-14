// Author: Moisés Adame-Aguilar
// Date: January 13, 2023
// Description: Uses of the LinkedList

#![allow(dead_code)]

mod linked_list;

fn main(){
	let mut list1: linked_list::LinkedList<u32> = linked_list::LinkedList::new();

	for i in 0..10 {
		list1.add_first(i);
	}

	list1.print();

	println!("Size: {:?}", list1.len());
}