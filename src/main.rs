use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut number_of_guesses: u32 = 0;

    loop {
        number_of_guesses = number_of_guesses + 1;
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            // Returns Result with type Ok or Err:
            // - Ok holds result data, in this case - number of bytes of user's input;
            // - Err holds stacktrace error information.
            .read_line(&mut guess)
            .expect("Failed to read line!");

        if guess.trim() == "quit" || guess.trim() == "exit" {
            println!("Quitting game.");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! It took you {} guesses.", number_of_guesses);
                break;
            }
        }
    }
}
