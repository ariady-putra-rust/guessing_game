use std::io::{self, Write};

use rand::Rng;

// fn read_only(&self) {..} // borrowing
// fn mutate(&mut self) {..} // pass by reference
// fn consume(self) {..} // take ownership

/* Unit Struct */
//
// struct Unit;
/*
Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself
*/
// fn unit_struct() {
//     let unit: Unit = Unit;
// }

/* Tuple Struct */
// struct XYZ(u8, u8, u8);
// fn xyz() {
//     let xyz = XYZ(5, 4, 3);
//     println!("{} {} {}", xyz.0, xyz.1, xyz.2);
// }

#[derive(Debug)]
struct Game {
    secret_numbers: [u8; 5],
    secret_number: u8,
    guess_count: u8,
}

impl Game {
    /* Associated Function */
    // pub fn init_game() -> Game {
    pub fn init_game() -> Self {
        // Game {
        //     secret_numbers: [
        //         rand::thread_rng().gen_range(1..=100),
        //         rand::thread_rng().gen_range(1..=100),
        //         rand::thread_rng().gen_range(1..=100),
        //         rand::thread_rng().gen_range(1..=100),
        //         rand::thread_rng().gen_range(1..=100),
        //     ],
        //     secret_number: 0,
        //     guess_count: 0,
        // }
        Self::build_game(
            [
                rand::thread_rng().gen_range(1..=100),
                rand::thread_rng().gen_range(1..=100),
                rand::thread_rng().gen_range(1..=100),
                rand::thread_rng().gen_range(1..=100),
                rand::thread_rng().gen_range(1..=100),
            ],
            // 0,
            // 0,
        )
    }

    /* Associated Function */
    // fn build_game(secret_numbers: [u8; 5], secret_number: u8, guess_count: u8) -> Game {
    //     Game {
    //         secret_numbers,
    //         secret_number,
    //         guess_count,
    //     }
    // }
    fn build_game(secret_numbers: [u8; 5]) -> Self {
        let game = Self::default_game();

        // return Game {
        //     secret_numbers,
        //     ..game
        // };
        return Self {
            secret_numbers,
            ..game
        };
    }

    /* Associated Function */
    // fn default_game() -> Game {
    fn default_game() -> Self {
        // Game {
        //     secret_numbers: [0; 5],
        //     secret_number: 0,
        //     guess_count: 0,
        // }
        Self {
            secret_numbers: [0; 5],
            secret_number: 0,
            guess_count: 0,
        }
    }
}

/*
Each struct is allowed to have multiple `impl` blocks.
*/
impl Game {
    /* Method */
    pub fn select_secret_number(&mut self, selected_index: usize) {
        self.secret_number = self.secret_numbers[selected_index];
    }
    // pub fn select_secret_number(self, selected_index: usize) -> Self {
    //     Self {
    //         secret_number: self.secret_numbers[selected_index],
    //         ..self
    //     }
    // }

    /* Method */
    pub fn increment_guess_count(&mut self) {
        self.guess_count += 1;
    }
    // pub fn increment_guess_count(self) -> Self {
    //     Self {
    //         guess_count: self.guess_count + 1,
    //         ..self
    //     }
    // }
}
/*
There’s no reason to separate these methods into multiple `impl` blocks here,
but this is valid syntax.
*/

fn main() {
    println!();

    /* Ownership Rules */
    // 1. Each value in Rust has an owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    /* References Rules */
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.

    /* Dangling References */
    // let reference_to_nothing = dangle();

    /* Borrowing */
    // borrowing();
    // The opposite of referencing by using `&` is dereferencing,
    // which is accomplished with the dereference operator `*`

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

    /* Array Slices */
    // let a = [1, 2, 3, 4, 5];
    // let slice = &a[1..3];
    // assert_eq!(slice, &[2, 3]);

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

    /* String Slices */
    // let s = String::from("hello world");
    // let hello: &str = &s[0..5];
    // let world: &str = &s[6..11];
    // println!("{hello} {world}");
    /*
    With Rust's `..` range syntax,
    if you want to start at index [0],
    you can drop the value before the two periods.
    In other words,
    these are equal:
    */
    // let s = String::from("hello");
    // let slice: &str = &s[0..2];
    // let slice: &str = &s[..2];
    /*
    By the same token,
    if your slice includes the last byte of the String,
    you can drop the trailing number.
    That means these are equal:
    */
    // let s = String::from("hello");
    // let len = s.len();
    // let slice: &str = &s[3..len];
    // let slice: &str = &s[3..];
    /*
    You can also drop both values to take a slice of the entire string.
    So these are equal:
    */
    // let s = String::from("hello");
    // let len = s.len();
    // let slice: &str = &s[0..len];
    // let slice: &str = &s[..];
    /*
    NOTE:
    String slice range indices must occur at valid UTF-8 character boundaries.
    If you attempt to create a string slice in the middle of a multibyte character,
    your program will exit with an error.
    */

    /* shallow copy */
    // let s2 = s1;
    // // println!("s1:{s1}"); // `s1` is no longer valid here!
    // println!("s2:{s2}");

    /* deep copy */
    // let s3 = s2.clone();
    // println!("s2:{s2}");
    // println!("s3:{s3}");

    /* ownership & functions */
    // ownership_and_functions();

    /* transferring ownership of return values */
    // transferring_ownership_of_return_values();

    /* Exercises */
    exercise(0);

    // let game1 = Game {
    //     secret_numbers: [1; 5],
    //     secret_number: 1,
    //     guess_count: 1,
    // };
    // let game2 = Game {
    //     secret_number: 2,
    //     ..game1
    // };
    // println!("{}", game1.guess_count);
    // println!("{}", game2.guess_count);

    let mut game = Game::init_game();
    // println!("{}", game); // in format strings you may be able to use `{:?}` (or {:#?} for pretty-print)
    // println!("{:?}", game); // Game { secret_numbers: [..], secret_number: 0, guess_count: 0 }
    // println!("{:#?}", game); // pretty-print
    // dbg!(&game); // [src/main.rs:line-number] &game = pretty-print

    // let secret_numbers: [u8; 5] = [
    //     rand::thread_rng().gen_range(1..=100),
    //     rand::thread_rng().gen_range(1..=100),
    //     rand::thread_rng().gen_range(1..=100),
    //     rand::thread_rng().gen_range(1..=100),
    //     rand::thread_rng().gen_range(1..=100),
    // ];
    let mut selection = String::new();
    print_flush("Please select a secret number [1-5]: ");
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line!");
    let selection: usize = selection.trim().parse().expect("Please type a number!");

    // let secret_number: u8 = secret_numbers[selection - 1]; // rand::thread_rng().gen_range(1..=100);
    // game.secret_number = game.secret_numbers[selection - 1];

    game.select_secret_number(selection - 1);
    // Game::select_secret_number(&mut game, selection - 1);

    // game = Game::select_secret_number(game, selection - 1);
    // game = game.select_secret_number(selection - 1);

    println!("Guess the number!"); // {secret_number}");

    // let mut guess_count = 0;
    // let guess_count = loop {
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
            Ok(g) => {
                // game.guess_count += 1;

                game.increment_guess_count();
                // Game::increment_guess_count(&mut game);

                // game = Game::increment_guess_count(game);
                // game = game.increment_guess_count();

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
        match guess.cmp(&game.secret_number) {
            std::cmp::Ordering::Equal => {
                // exit(0);
                // return;

                // break guess_count;
                break;
            }
            std::cmp::Ordering::Greater => println!("Too BIG"),
            std::cmp::Ordering::Less => println!("Too SMALL"),
        }
    }

    // println!(
    //     "You WIN after {guess_count} guess{}!",
    //     if guess_count == 1 { "" } else { "es" }
    // );
    println!(
        "You WIN after {} guess{}!",
        game.guess_count,
        if game.guess_count == 1 { "" } else { "es" }
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
        2 => exercise_2(),
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

enum Day {
    Zero,
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
    Tenth,
    Eleventh,
    Twelfth,
}
impl Day {
    pub fn to_string(&self) -> String {
        String::from(match self {
            Self::Zero => "zero",
            Self::First => "first",
            Self::Second => "second",
            Self::Third => "third",
            Self::Fourth => "fourth",
            Self::Fifth => "fifth",
            Self::Sixth => "sixth",
            Self::Seventh => "seventh",
            Self::Eighth => "eighth",
            Self::Ninth => "ninth",
            Self::Tenth => "tenth",
            Self::Eleventh => "eleventh",
            Self::Twelfth => "twelfth",
        })
    }

    pub fn from_usize(u: usize) -> Self {
        match u {
            1 => Self::First,
            2 => Self::Second,
            3 => Self::Third,
            4 => Self::Fourth,
            5 => Self::Fifth,
            6 => Self::Sixth,
            7 => Self::Seventh,
            8 => Self::Eighth,
            9 => Self::Ninth,
            10 => Self::Tenth,
            11 => Self::Eleventh,
            12 => Self::Twelfth,
            _ => Self::Zero,
        }
    }
}
fn sing_christmas_carol() {
    for d in 1..=12 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            Day::from_usize(d).to_string()
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
        1 => {
            println!("{}", gift[0]);
            println!();
        }
        2 => {
            println!("{}, and", gift[1]);
            send_gift(day - 1);
        }
        _ => {
            println!("{}", gift[day - 1]);
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

fn borrowing() {
    // let s = String::from("hello");
    // change(&s); // expected &mut String, found &String
    //             // mismatched types
    //             // expected mutable reference `&mut String`
    //             //            found reference `&String`
    let mut s = String::from("hello");
    change(&mut s);

    println!("{s}");

    /* Mutable references have one big restriction:
    if you have a mutable reference to a value,
    you can have no other references to that value.
    This code that attempts to create two mutable references to `s` will fail:
    */
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s; // cannot borrow `s` as mutable more than once at a time
    // println!("{}, {}", r1, r2);

    /* Rust enforces a similar rule for combining mutable and immutable references.
    This code results in an error:
    */
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s; // cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("{}, {}, and {}", r1, r2, r3);
    /*
    but this code will compile because the last usage of the immutable references,
    the `println!`, occurs before the mutable reference is introduced:
    */
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // println!("{r1}, {r2}"); // variables r1 and r2 will not be used after this point
    // let r3 = &mut s;
    // println!("{r3}");
    /*
    This code will also result in an error:
    */
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &s; // cannot borrow `s` as immutable because it is also borrowed as mutable
    // r1.push_str(", world");
    // println!("{r1}, {r2}");
    /*
    multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data
    */
}

// fn change(some_string: &String) {
//     some_string.push_str(", world"); // cannot borrow `*some_string` as mutable, as it is behind a `&` reference
//                                      // `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
//                                      // consider changing this to be a mutable reference: `&mut String`
// }
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     // `dangle` returns a reference to a String

//     let s = String::from("hello"); // `s` is a new String

//     &s // we return a reference to the String `s`
// } // Here `s` goes out of scope and is dropped. Its memory goes away.
//   // Danger!

fn exercise_2() {
    /* write a function that takes a string of words separated by spaces and returns the first word it finds in that string */
    // If the function doesn’t find a space in the string,
    // the whole string must be one word,
    // so the entire string should be returned.
    let mut sentence = String::from("The quick brown fox jumps over the lazy dog.");
    println!("{sentence}");
    let first_word = first_word(&sentence);
    let last_word = last_word(&sentence);
    sentence.clear();
    println!("{sentence}");
    println!("{} {}", first_word, last_word);
}

fn first_word(sentence: &String) -> String {
    nth_word(0, sentence)
}

fn last_word(sentence: &String) -> String {
    nth_back_word(0, sentence)
}

fn nth_word(n: usize, sentence: &String) -> String {
    let mut words = sentence.split_whitespace();
    return String::from(words.nth(n).unwrap_or(""));
}

fn nth_back_word(n: usize, sentence: &String) -> String {
    let mut words = sentence.split_whitespace();
    return String::from(words.nth_back(n).unwrap_or(""));
}
