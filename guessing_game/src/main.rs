use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");


    loop {
        
    println!("Please input your guess.");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..101);
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line.");

    let guess: u32 =  match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };  
    println!("The secret number is {}",secret_number);
        
        println!("You guessed: {}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("The guessed number is too less!"),
            Ordering::Greater => println!("The guessed number is too great!"),
            Ordering::Equal =>{ 
                println!("You win!");
                break;
            }
        }    
    }

}
