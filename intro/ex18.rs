// Author: Moisés Adame-Aguilar
// Date: January 1w, 2023
// Description: Chapter 10, Trait Bounds to Conditionally Implement Methods.

use std::fmt::Display;

#[derive(Debug)]
struct Point<T>{
	x: T,
	y: T,
}

// The new() method is always implemented.
impl<T> Point<T>{
	fn new(x: T, y: T) -> Self{
		Self{
			x,
			y,
		}
	}
}

// The bigger_side() method is implemented only if T has 
// implemented the PartialOrd and Display traits.
impl<T: PartialOrd + Display> Point<T>{
	fn bigger_side(&self){
		if self.x >= self.y {
			println!("Big side is x: {}", self.x);
		}else{
			println!("Big side is y: {}", self.y);
		}
	}
}

fn main(){
	let my_point: Point<f32> = Point::new(3.0, 2.0);

	my_point.bigger_side();
}






