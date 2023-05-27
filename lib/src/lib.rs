// cargo new lib --lib

use crate::libutil::print_flush;
use rand::Rng;
use std::io;

pub mod libutil;

#[derive(Debug)]
pub struct Game {
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

    // getters
    pub fn get_secret_numbers(&self) -> [u8; 5] {
        self.secret_numbers
    }
    pub fn get_secret_number(&self) -> u8 {
        self.secret_number
    }
    pub fn get_guess_count(&self) -> u8 {
        self.guess_count
    }
}
/*
There’s no reason to separate these methods into multiple `impl` blocks here,
but this is valid syntax.
*/

pub fn play_game() {
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
        match guess.cmp(&game.get_secret_number()) {
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

    let guess_count = game.get_guess_count();
    println!(
        "You WIN after {guess_count} guess{}!",
        if guess_count == 1 { "" } else { "es" }
    );

    println!();
    println!("Thank you for playing!");
    println!();
}
