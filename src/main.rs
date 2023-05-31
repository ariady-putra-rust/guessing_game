use crate::exercises::exercise;
use guessing_game_lib::play_game;

pub mod borrowing;
pub mod collection;
pub mod controls;
pub mod copies;
pub mod dangling_references;
pub mod error_handling;
pub mod generics;
pub mod lifetimes;
pub mod loops;
pub mod ownership;
pub mod strings;
pub mod structs;
pub mod traits;
pub mod variables;

pub mod exercises;
pub mod tests;

pub mod integration_test;
pub mod unit_test;

// fn read_only(&self) {..} // borrowing
// fn mutate(&mut self) {..} // pass by reference
// fn consume(self) {..} // take ownership

fn main() {
    println!();

    tests::run(0);

    /* References Rules */
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.

    /* Exercises */
    exercise(0);

    play_game();
}
