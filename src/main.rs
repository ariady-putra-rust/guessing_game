use std::io::{self, Write};

use rand::Rng;

fn main() {
    println!();

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

    /* Loop label */
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("End count = {count}");

    /* While loop */
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index += 1;
    // }

    /* For-each loop */
    // let a = [10, 20, 30, 40, 50];
    // for element in a {
    //     println!("the value is: {element}");
    // }

    /* Exercises */
    exercise_1();

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

    let mut guess_count = 0;
    let guess_count = loop {
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
            Ok(g) => {
                guess_count += 1;
                g // expression
            }
            Err(_) => {
                println!("Please type a number between [1-100]");
                continue; // statement
            }
        };

        // match secret_number == guess {
        //     true => println!("You WIN"),
        //     false => println!("Try again!"),
        // }
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Equal => {
                // exit(0);
                // return;
                break guess_count;
            }
            std::cmp::Ordering::Greater => println!("Too BIG"),
            std::cmp::Ordering::Less => println!("Too SMALL"),
        }
    };

    println!(
        "You WIN after {guess_count} guess{}!",
        if guess_count == 1 { "" } else { "es" }
    );

    println!();
    println!("Thank you for playing!");
    println!();
}

fn print_flush(s: &str) {
    print!("{}", s);
    io::stdout().flush().expect("Failed to flush!")
}

fn exercise_1() {
    /* Convert temperatures between Fahrenheit and Celsius. */
    println!("212F = {}C", to_celsius(212.0));
    println!("100C = {}F", to_fahrenheit(100.0));

    /* Generate the nth Fibonacci number. */
    println!();
    println!("Fibonacci [0-10]:");
    for n in 0..=10 {
        print!(" {} ", fibonacci(n));
    }
    println!();

    /* Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
    taking advantage of the repetition in the song. */
    println!();
    println!("Christmas Carol");
    println!();
    sing_christmas_carol();
}

fn to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn to_fahrenheit(celsius: f32) -> f32 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fibonacci(n: u8) -> u16 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn sing_christmas_carol() {
    let day = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    for d in 0..12 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            day[d]
        );
        send_gift(d);
    }
}

fn send_gift(day: usize) {
    let gift = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    match day {
        0 => {
            println!("{}", gift[0]);
            println!();
        }
        1 => {
            println!("{}, and", gift[1]);
            send_gift(day - 1);
        }
        _ => {
            println!("{}", gift[day]);
            send_gift(day - 1);
        }
    }
}
