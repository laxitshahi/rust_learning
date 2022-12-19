//std is the standard library for Rust:: io is the input/output library
//A number of things are automatically imported into this file when we do std::io --> std::prelude
//or std::io::prelude

use rand::Rng; //:: is the path seperator
use std::{cmp::Ordering, io};

fn main() {
    let min: u32 = 1;
    let max: u32 = 100;

    println!("Guess da number");
    //thread_rng is local to the current thread of execution
    let secret_number = rand::thread_rng().gen_range(min..=max); //inclusive uppper/lower bounds
    println!("Your secret number is {secret_number}");

    loop {
        println!("Please input your guess [between {min} and {max}]");

        //Variables are immutable(cannot be cahnges) by default, hence we need to add mut
        //If we did not use mut here, there would be no way to reflect user inputs
        let mut guess = String::new(); //the new function creates an empty string

        io::stdin()
            //References are also immutable by default
            .read_line(&mut guess) //takes the input and APPENDS to a string and RETURNS 'Result' enum
            .expect("Tailed to read the line."); //if Res == Err return string Res = OK return value

        //Restating this value is called shadowing:: allows us to reuse the variable w/o 2 variables
        //It is general used when you want to manipulate a variable
        let guess: u32 = match guess.trim().parse() {
            //Here we are using match to manually set the return for both enums
            //Rather than using expect like above (stdin)
            Ok(num) => num,
            Err(_) => continue, //Catch all errors regardless of what is in them
                                //if a poor input (negative, string etc) move to next iteration of loop
        };
        //The trim() is used to remove any white space in the start and end of the input
        //This includes the \n that is added whent he user presses enter

        println!("Your guess was {guess}"); //The {} can also be left blank for evaluating expresssions

        //Secret_number is infered to also be u32 due to guess
        match guess.cmp(&secret_number) {
            //Match compares and returns one of the following below
            //We can see each possibility as an "arm"
            //Rust simply checks which arm fits
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You have won a goat");
                break;
            }
        }
    }
}
