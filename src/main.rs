extern crate rand;

use std::io; //input/output library comming out of the standard library
use std::cmp::Ordering;
use rand::Rng;

fn main() { // fn declares new function, main function without parameters
    println!("Guess the number!"); // macro printing a string

    let secret_number = rand::thread_rng().gen_range(1, 101);


    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        //let creates variable, guess which is mutable (mut).
        //The variable is bound to a new empty instance of a String.
        //:: indicates, that new is an associated function of the String type

        io::stdin().read_line(&mut guess)
        //stdin function creates a new instance of std::io::Stdin. This is a type
        //that represents a handle to the standart input for your terminal.
        //on this standard input, the method read_line is called.
        //One argument: &mut guess is passed to the method.
        //read_line puts the user input into a string. The string needs to be
        //mutable, so the user content can be added.
        //&
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        //shadowing
        //trim method on a string eliminates whitespace, because the number entered is 5/n because ENTER is pressed
        //u32 can only contain numerical characters, 32bit integer
        //parse returns an error, if string is not convertable to number

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
