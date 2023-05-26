use std::io::{self, Write};

use rand::Rng;

fn main() {
    println!();

    /* Ownership Rules */
    // 1. Each value in Rust has an owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

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

    /* The String Type */
    // let mut s1 = String::new();
    // s1.push_str("Lorem ipsum");
    // println!("1:{s1}");
    // s1 = String::from("Dolor sit amet");
    // s1.push_str(", consectetur adipiscing elit");
    // println!("2:{s1}");

    // /* shallow copy */
    // let s2 = s1;
    // // println!("s1:{s1}"); // `s1` is no longer valid here!
    // println!("s2:{s2}");

    // /* deep copy */
    // let s3 = s2.clone();
    // println!("s2:{s2}");
    // println!("s3:{s3}");

    /* ownership & functions */
    // ownership_and_functions();

    /* transferring ownership of return values */
    // transferring_ownership_of_return_values();

    /* Exercises */
    exercise(0);

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

fn exercise(e: usize) {
    match e {
        1 => exercise_1(),
        _ => return,
    }
}

fn exercise_1() {
    /* 1. Convert temperatures between Fahrenheit and Celsius. */
    println!("212F = {}C", to_celsius(212.0));
    println!("100C = {}F", to_fahrenheit(100.0));

    /* 2. Generate the nth Fibonacci number. */
    println!();
    println!("Fibonacci [0-10]:");
    for n in 0..=10 {
        print!(" {} ", fibonacci(n));
    }
    println!();

    /* 3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
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

fn ownership_and_functions() {
    let s = String::from("hello"); // `s` comes into scope

    takes_ownership(s); // `s`'s value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // `x` comes into scope

    makes_copy(x); // `x` would move into the function,
                   // but `u8` is `Copy`, so it's okay to still
                   // use `x` afterward
}

fn takes_ownership(some_string: String) {
    // `some_string` comes into scope
    println!("{some_string}");
} // Here, `some_string` goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: u8) {
    // `some_integer` comes into scope
    println!("{some_integer}");
} // Here, `some_integer` goes out of scope. Nothing special happens.

fn transferring_ownership_of_return_values() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    println!("s1:{s1}");
    // println!("s2:{s2}"); // `s2` is no longer valid here!
    println!("s3:{s3}");
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
