use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Guessing the game");
    
    loop {

        println!("Input your gues..");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read input");

        println!("You guessed:{guess}");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        } ;

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too less"),
            Ordering::Equal => {
                println!("Correct");
                break;
            },
        }
    }   
}
