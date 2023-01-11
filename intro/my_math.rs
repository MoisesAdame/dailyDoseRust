// Author: Moisés Adame-Aguilar
// Date: December 9th, 2022
// Description: Chapter 7, Using Modules to Reuse and Organize Code

pub fn factorial(x: u128) -> u128{
	if x == 0 || x == 1{
		return 1
	}else{
		return x * factorial(x - 1)
	}
}

pub fn exp(x: f32) -> f32{
	let mut sum: f32 = 0.0;
	for n in 0..30{
		sum += f32::powf(x, n as f32)/factorial(n) as f32
	}
	sum
}