// Author: Moisés Adame-Aguilar
// Date: December 7th, 2022
// Description: Soluction of chap3 problems of book "Steve Klabnik, Carol 
// Nichols, Rust Community - The Rust Programming Language-No Starch Press (2018)"

fn f2c(fahrenheit_temp: f32) -> f32{
	(fahrenheit_temp - 32.0) * 5.0/9.0
}

fn c2f(celsius_temp: f32) -> f32{
	(celsius_temp * 9.0/5.0) + 32.0
}

fn fib(nth_num: u32) -> u32{
	if nth_num == 0 || nth_num == 1{
		1
	}else{
		fib(nth_num - 1) + fib(nth_num - 2)
	}
}


fn main(){
	// Problem 1: Convert temperatures between Fahrenheit and Celsius.
	let temp: f32 = 100.0;
	let res1: f32 = f2c(temp);
	let res2: f32 = c2f(res1);
	println!("[*] {}°F is {}°C", temp, res1);

	println!("[*] {}°C is {}°F", res1, res2);

	// Problem 2: Generate the nth Fibonacci number.
	for num in 0..10{
		println!("[*] {} = {}", num, fib(num));
	}

	// Problem 3: Print the lyrics to the Christmas carol “The Twelve 
	// Days of Christmas,” taking advantage of the repetition in the song.

	// Not doing this...

}

