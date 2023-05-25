use std::io::{self, Write};

use rand::Rng;

fn main() {
    /* Variables */
    // let apples = 5; // immutable
    // let mut bananas = 5; // mutable
    // let apples = apples + 1; // shadowing
    // bananas += 1;
    // println!("bananas: {1} and apples: {0}", apples, bananas);

    /* Constants */
    // const THREE_HOURS_IN_SECONDS: u16 = 60 * 60 * 3;
    // // let THREE_HOURS_IN_SECONDS = 3; // `THREE_HOURS_IN_SECONDS` is interpreted as a constant pattern
    // println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS}");

    /* Tuple */
    // let tup = (1, 2.3, 'a', "ABC", true);
    // println!("{} {} {} {} {}", tup.0, tup.1, tup.2, tup.3, tup.4); // 1 2.3 'a' "ABC" true
    // let (_, _, a, _, _) = tup; // destructuring the tuple via pattern-matching
    // println!("{a}"); // 'a'

    /* Array */
    // let a: [u8; 3] = [1, 2, 3];
    // let [_, b, _] = a;
    // println!("{b}");
    // let c = [true; 3]; // [true, true, true]
    // println!("{}", c[1]);

    let secret_numbers: [u8; 5] = [
        rand::thread_rng().gen_range(1..=100),
        rand::thread_rng().gen_range(1..=100),
        rand::thread_rng().gen_range(1..=100),
        rand::thread_rng().gen_range(1..=100),
        rand::thread_rng().gen_range(1..=100),
    ];
    let mut selection = String::new();
    print_flush("Please select a secret number [1-5]: ");
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line!");
    let selection: usize = selection.trim().parse().expect("Please type a number!");

    let secret_number: u8 = secret_numbers[selection - 1]; // rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!"); // {secret_number}");

    let mut guess_count = 1;
    loop {
        println!();
        print_flush("Please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        // println!("You guessed: {guess}");

        // match eq(&secret_number.to_string(), &guess)

        /* Convert String to Integer */
        // let guess: u8 = guess.trim().parse().expect("Please type a number!");
        let guess: u8 = match guess.trim().parse() {
            Ok(g) => g,
            Err(_) => {
                println!("Please type a number between [1-100]");
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
