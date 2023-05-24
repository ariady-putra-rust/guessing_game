use rand::*;
use std::*;

fn main() {
    // let apples = 5; // immutable
    // let mut bananas = 5; // mutable
    // bananas += 1;
    // println!("bananas+1: {1} and apples: {0}", apples, bananas);

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number! {secret_number}");

    loop {
        println!();
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        println!("You guessed: {guess}");

        // match eq(&secret_number.to_string(), &guess)

        /* SHADOWING & TYPE-CASTING */
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
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
            cmp::Ordering::Equal => {
                println!("You WIN");
                // exit(0);
                // return;
                break;
            }
            cmp::Ordering::Greater => println!("Too BIG"),
            cmp::Ordering::Less => println!("Too SMALL"),
        }
    }

    println!();
    println!("Thank you for playing!");
    println!();
}
