use std::io;                                                               //Input Output functionality
use rand::Rng;                                                             //Random functionality                       
use std::cmp::Ordering;                                                    //Ordering type functionality

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);        //Generates random number

    //println!("The secret number is: {}", secret_number);                 //For testing purposes

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();                             //Stores user input
    
        io::stdin()
           .read_line(&mut guess)
           .expect("Failed to read line");
           
        let guess: u32 = match guess.trim().parse() {                     //Converts guess from string to integer
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);                               //Prints user input
        
        match guess.cmp(&secret_number) {                                 //Compares guess to the secret number and matches with Ordering pattern
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}