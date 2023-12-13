use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Welcome to my guessing game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The number is {secret_number}");
    loop{
        println!("Please enter some number.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Incorrect");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
        } // end match
    
    } // end loop

}
