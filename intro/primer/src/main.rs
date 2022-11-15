// Allow dead or unused code
#![allow(dead_code)]

// Tipos de datos customizables: stuct y enum
// stuct es como un diccionario en python, enum son opciones que se pueden tomar
#[derive(Debug)]
struct User{
    username: String,
    email: String,
    sign_in_account: u64,
    active: bool
}

impl User{
    fn new(username: String, email: String) -> User{
        let ret_usr = User{
            username,
            email,
            sign_in_account: 1,
            active: true
        };

        return ret_usr;
    }
}

struct Rectangle{
    base: f64,
    height: f64
}

impl Rectangle{
    fn print(&self) -> (){
        println!("[!] Rectangle: ");
        println!("[*] Base: {}", self.base);
        println!("[*] Height: {}", self.height);
        println!("[*] Area: {}", self.area());
    }

    fn area(&self) -> f64{
        let area_res = self.base * self.height;
        return area_res;
    }
}

fn main() {
    // Tipos de datos de forma autónoma
    let mut x = 5;

    println!("First, x is {}", x);

    x = 7;

    println!("Now, x is {}", x);


    // Declaramos tipo de dato
    // Tenemos string, lo parseamos, esto nos da un Result
    // con .expect() manejamos el error, si no es numero, nos regresa un error
    let guess: u32 = "42".parse().expect("Not a number");

    println!("The guess is {}", guess);


    // instanciamos User
    let user1 = User{
        username: String::from("frankwill"), 
        email: "frnak@tec.mx".to_string(), 
        sign_in_account: 23, 
        active: true
    };

    let user2 = User::new("Moy".to_string(), "a01660927“tec.mx".to_string());

    println!("[*] My user1 is: {:?}", user1);
    println!("[*] My user2 is: {:?}", user2);


    let my_rectangle: Rectangle = Rectangle{
        base: 3.0,
        height: 4.0
    };

    my_rectangle.print();
}