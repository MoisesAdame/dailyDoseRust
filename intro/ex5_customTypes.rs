// Allow dead or unused code
#![allow(dead_code)]

// Debug trait allows printability
#[derive(Debug)]
struct Person{
	name: String,
	age: u8,
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

#[derive(Debug)]
struct Point{
	x: f64,
	y: f64,
}

fn euclidean_dist(p1: &Point, p2: &Point) -> f64{
	let sum: f64 = (p1.x - p2.x)*(p1.x - p2.x) + (p1.y - p2.y)*(p1.y - p2.y);
	return sum.sqrt();
}


fn main(){
	// ————customTypes have 2 keywords: struct and enum.

	// Can only be instantiated using the originally used varNames.
	let name = String::from("Mark Zuckerberg");
	let age: u8 = 30;
	let mark = Person{name, age};

	// You have to use :? to print the debug stuct
	println!("[*] My Person: {:?}", mark);

	// Accesing data
	println!("[*] Detailed info:");
	println!("[*] Name: {}", mark.name);
	println!("[*] Age: {}", mark.age);

	// Another example but only using tuples
	let my_matrix = Matrix(3.33,2.0,1.55,-2.43);
	println!("[*] Tuple Matrix:");
	println!("[*] My Matrix: {:?}", my_matrix);

	// Example using Points
	let point1 = Point{x:0.0, y:0.0};
	let point2 = Point{x:3.0, y:4.0};
	let distance: f64 = euclidean_dist(&point1, &point2);
	println!("[*] The distance between ({}, {}) and ({}, {}) is {}",
			 point1.x, point1.y, point2.x, point2.y, distance);

}