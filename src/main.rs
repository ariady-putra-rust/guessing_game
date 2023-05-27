// cargo new guessing_game

pub mod borrowing;
pub mod controls;
pub mod copies;
pub mod dangling_references;
pub mod loops;
pub mod ownership;
pub mod strings;
pub mod structs;
pub mod variables;

pub mod exercises;

// fn read_only(&self) {..} // borrowing
// fn mutate(&mut self) {..} // pass by reference
// fn consume(self) {..} // take ownership

fn main() {
    println!();

    /* References Rules */
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.

    /* Exercises */
    exercises::exercise(0);

    guessing_game_lib::play_game();
}
