// 作家：摩西
// 日期：二零二三年一月二十五日
// Description: Iterators and closures of Rust.

// Closures are anonymous functions.
// Here an example of a funtion and closure that do the same thing.
fn func_addone(num: i32) -> i32{
	num + 1
}

fn main(){
	// Once you use the clousure, its type cannot be changed.
	let clos_addone = |num: i32| -> i32{
		num + 1
	};

	let some_number: i32 = -32;
	println!("Function: {}", func_addone(some_number));
	println!("Function: {}", clos_addone(some_number));
}