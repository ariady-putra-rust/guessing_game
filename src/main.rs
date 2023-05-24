use std::io::{self, Write};

use rand::Rng;

fn main() {
    // let apples = 5; // immutable
    // let mut bananas = 5; // mutable
    // bananas += 1;
    // println!("bananas+1: {1} and apples: {0}", apples, bananas);

    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!"); // {secret_number}");

    let mut guess_count = 1;
    loop {
        println!();
        print_flush("Please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        println!("You guessed: {guess}");

        // match eq(&secret_number.to_string(), &guess)

        /* SHADOWING & TYPE-CASTING */
        // let guess: u8 = guess.trim().parse().expect("Please type a number!");
        let guess: u8 = match guess.trim().parse() {
            Ok(g) => g,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        // match secret_number == guess {
        //     true => println!("You WIN"),
        //     false => println!("Try again!"),
        // }
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Equal => {
                println!(
                    "You WIN after {guess_count} guess{}!",
                    match guess_count == 1 {
                        true => "",
                        false => "es",
                    }
                );
                // exit(0);
                // return;
                break;
            }
            std::cmp::Ordering::Greater => println!("Too BIG"),
            std::cmp::Ordering::Less => println!("Too SMALL"),
        }

        guess_count += 1;
    }

    println!();
    println!("Thank you for playing!");
    println!();
}

fn print_flush(s: &str) {
    print!("{}", s);
    io::stdout().flush().expect("Failed to flush!");
}
