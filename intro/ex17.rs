// Author: Moisés Adame-Aguilar
// Date: January 12, 2023
// Description: Using traits & generics for functions.

// We use Copy because when max was made generic it was impossible
// to handle types whose size is unknown.

// We can ommit using the Copy trait by returning a reference (&T),
// but this implies using lifetimes.
fn max<T: PartialOrd + Copy>(val1: &T, val2: &T) -> T{
	// Default operator >=,  needs we need to specify PartialOrd
	let res = if val1 >= val2{
		val1
	}else{
		val2
	};

	*res
}

fn main(){
	let num1: u8 = 32;
	let num2: u8 = 200;

	let biggest_num: u8 = max::<u8>(&num1, &num2);

	println!("Biggest: {}", biggest_num);

	println!("Num1: {}", num1);
	println!("Num2: {}", num2);
}