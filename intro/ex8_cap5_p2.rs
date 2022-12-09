// Author: Moisés Adame-Aguilar
// Date: December 9th, 2022
// Description: Solution of chap5 problems of book "Steve Klabnik, Carol 
// Nichols, Rust Community - The Rust Programming Language-No Starch Press (2018)"

// Using debuf in order to print the Vect type.
#[derive(Debug)]
struct Vect{
	x: f32,
	y: f32
}

impl Vect{
	// Associated cosntructor function.
	fn new(x: f32, y:f32) -> Vect{
		// Using --field init shorthand--
		Vect{
			x,
			y
		}
	}

	// Method sum, with parameters.
	fn sum(&self, other_vect: &Vect) -> Vect{
		Vect{
			x: self.x + other_vect.x,
			y: self.y + other_vect.y
		}
	}

	// Methdod mag, without parameters.
	fn mag(&self) -> f32{
		(self.x * self.x + self.y * self.y).sqrt()
	}

	// Method dot, with parameters.
	fn dot(&self, other_vect: &Vect) -> f32{
		self.x * other_vect.x + self.y * other_vect.y
	}
}

fn main(){
	let v1: Vect = Vect::new(3.0, 4.0);

	let v2: Vect = Vect::new(7.0, 12.0);

	let v3: Vect = v1.sum(&v2);

	println!("{:#?}", v3);

	println!("|v1| = {}", v1.mag());

	println!("v1 * v2 = {}", v1.dot(&v2));

}