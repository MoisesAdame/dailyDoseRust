// Author: Moisés Adame-Aguilar
// Date: January 23, 2023
// Description: Introduction to Automated Tests (Integration Tests).

extern crate adder;

#[test]
fn integration_add(){
	let num1: usize	= 28;
	let num2: usize = 35;

	assert_eq!(num1 + num2, adder::add(num1, num2));
}