use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*; // Importing the colored crate for colored output   

fn main() {
    //generating a random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("input your guess: ");

        //initializing a string to hold guess
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //this line handles message as Ok,Err where it Returns value

        //this hangles guess message as integer not as string
        //this is a process called shadowing the Stirng to u32 convertion
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Please enter a valid number!");
                continue; // Skip to the next iteration of the loop
            }   
        }; 
        // this is guess with u32 the unsigned 32-bit integer
        // we have match which checks if matched
        // also we have Ordering which is an enum with three variants: Less,Greater,Equal
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {

                println!("{}","You win!".green());
                break
            }
        }
    }
}
