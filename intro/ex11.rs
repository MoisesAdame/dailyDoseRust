// Author: Moisés Adame-Aguilar
// Date: January 11, 2023
// Description: Chapter 7, Using Modules to Reuse and Organize Code

mod my_math;

fn main(){
	for n in 0..5{
		println!("exp({}) = {}", n, my_math::exp(n as f32))
	}
}