extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 11);


    loop{
        let mut guess = String::new();

        println!("Please input your guess.");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");


        println!("Your guess was: {}", guess); 

        if guess == secret_number{
            break;
        }else if guess > secret_number{
            println!("Your guess was bigger than the secret number!"); 
        }else{
            println!("Your guess was smaller than the secret number!"); 
        }
    }


    println!("Correct, the secret number was: {}", secret_number)    
}