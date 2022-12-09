// Author: Moisés Adame-Aguilar
// Date: December 7th, 2022
// Description: Solution of chap4 problems of book "Steve Klabnik, Carol 
// Nichols, Rust Community - The Rust Programming Language-No Starch Press (2018)"

// ---OWNERSHIP
// The reason why Rust doesn't need a garbage collector.
// Related topics: Borrowing, slices, and how Rust lays data out in memory.
// Rules:
// 1) Each value in Rust has a variable that’s called its owner.
// 2) There can be only one owner at a time.
// 3) When the owner goes out of scope, the value will be dropped.

use std::io;
use std::fmt;

// Debug trait allows printability
#[derive(Debug)]
struct Person{
    name: String,
    age: u8,
}

// Let's make smth so that it prints nicely
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "Name: {} ({})", self.name, self.age)
    }
}

fn own_string(my_string: String){
    println!("Good string ya '{}'!", my_string);
}

fn own_int(my_int: i32){
    println!("Good int ya '{}'!", my_int);
}

fn str_len(some_string: &String) -> usize{
    some_string.len()
}

fn updt_age(p: &mut Person, new_age: u8){
    p.age = new_age;
}

fn first_word(my_str: &str) -> &str{
    let mut cut: &str;
    for i in 0..my_str.len(){
        cut = &my_str[i..i+1];
        if cut == " "{
            return &my_str[..i];
        }
    }
    &my_str[..]
}

fn main(){
	let mut name: String = String::new();
	println!("Give me your name: ");
	io::stdin().read_line(&mut name)
        .expect("Failed to read line");


    let mut salute: String = String::from("Hello, ");
    salute.push_str(&name);


    println!("{}.", salute);

    // If the size of a data type is known, moving is valid
    // otherwise, panic is arised. Ints, floats, bools can
    // be moved, strings and more complex dt, cannot.
    let x: i32 = 5;
    let last_name: String = String::from("adame");
    own_string(last_name.clone());
    own_int(x);

    println!("{}, {}", x, last_name);

    // In order no to have to use clone() everytime, we can use
    // references, which come tu life using '&'
    let my_name: String = String::from("Moisés Adame Aguilar");

    println!("Wow, your name, '{}' is {} characters long!", my_name, str_len(&my_name));

    // If we wanted to change that variable, we'd have to use a mutable reference
    // sooo, let's do it!
    // NOTE: You can only use one mutable refrence at a time.
    // NOTE: We also cannot have a mutable reference while we have an immutable one.
    let mut p1: Person = Person{name: String::from("Mark Zuckerberg"), age: 30};

    println!("{}", p1);

    updt_age(&mut p1, 35);

    println!("{}", p1);

    // Thus, the TWO data types that do not have ownership are 
    // the slice and the reference type.
    // Strring slyces are of type &str
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];

    println!("&s[..5] = {}, &s[6..] = {}", hello, world);


    // Arrays can also have slices
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    let half_array: &[i32] = &my_array[..3];

    println!("{}", half_array.len());


    let part_word: &str = first_word(&s[..]);
    println!("-{}-", part_word);

}