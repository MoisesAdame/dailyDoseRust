// Author: Moisés Adame-Aguilar
// Date: January 11, 2023
// Description: Chapter 8, Common collections (Vec<T>).

fn print_vector(v: &Vec<i32>){
	if v.len() == 0{
		print!("Empty Vector!");
	}else{
		print!("v = ");
		for n in 0..v.len(){
			print!("{}, ", v[n]);
		}
	}
	println!();
}

fn main(){
	// Rust’s standard library includes a number of very useful data structures called collections.

	//- Vector
	let mut v1: Vec<i32> = Vec::new();
	let mut v2: Vec<i32> = vec![1, 2, 3];

	print_vector(&v1);
	print_vector(&v2);

	v2[0] = 5;

	let some_num: i32 = v2[0];

	println!("This num is: {}", some_num);

	v1.push(3);

	v2.push(4);

	// The folloiwng code doesn't work because adding a new element, 
	// no matter where, requieres allocating memory, something that
	// could raise and error if is done in the first variable.
	// let first: &i32 = &v2[0];
	// v2.push(6);
	// println!("{}", first)

	// Changing values and printing (made easy).
	let mut v3: Vec<i32> = Vec::new();
	for n in 0..25{
		v3.push(n)
	}

	// The & is used to copy v3, making the for not consume it.
	for i in &v3{
		print!("{}, ", i)
	}
	println!();

	v3.push(100);

	// Because i is a refrence, to use it we have to dereference(*) it.
	for i in &mut v3{
		*i += 50
	}

	for i in &v3{
		print!("{}, ", i)
	}
	println!();
}