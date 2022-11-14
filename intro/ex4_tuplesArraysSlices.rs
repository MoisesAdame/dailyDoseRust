// Import `fmt` and `mem`
use std::fmt;

// A structure that holds 4 32-bit float
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// Let's make smth so that it prints nicely
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}

fn main(){
	// --- TUPLES
	// - Constructed with ()
	// - Can store different types
	// - Tuple indexing: tuple_name.index
	// - If the tuple has more than 12 elem it cannot be printed

	let my_tuple = (32u8, -356i32, true);
	println!("[*] My tuple: {:?}", my_tuple);

	let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("[*] My matrix: {}", matrix);

    // --- ARRAYS
    // - Constructed with []
    // - Stores same type
    // - They aren't dynamic, you have to know their lenght: [T; length]

    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    let _my_array2: [i32; 5] = [420;5];
    let index: usize = 0;

    println!("[*] my_array[{0}] = {1}", index, my_array[index]);
    println!("[*] my_array.len() = {0}", my_array.len());

    // --- SLICES
    // - To instatiate them you have to: &[T]
    // - Very similar, but DYNAMIC memory


    // Printing an array
    for i in 0..my_array.len(){
        println!("{}: {}", i, my_array[i]);
    }


    // 我是去年九月来到了中国的


}