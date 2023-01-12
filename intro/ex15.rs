// Author: Moisés Adame-Aguilar
// Date: January 11, 2023
// Description: Generics for functions & user defined types.

// In Function Definiton.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T{
	let mut res: &T = &list[0];
	for i in list.iter(){
		if res < i{
			res = i;
		}
	}
	res
}

// In Struct Defnition.
struct Point<T>{
	x: T,
	y: T,
}

// In Method Definition.
impl<T> Point<T>{
	fn get_x(&self) -> &T{
		&self.x
	}

	fn get_y(&self) -> &T{
		&self.y
	}
}

// In Enum Definition
// enum Option<T>{
// 	Some(T),
// 	None,
// }

fn main(){
	let my_array: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
	println!("{}", largest::<char>(&my_array));

	let my_point: Point<f32> = Point::<f32>{x: 3.2, y: 2.45};
	println!("Point = ({}, {})", my_point.get_x(), my_point.get_y());
}