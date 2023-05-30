use self::{exercise_1::exercise_1, exercise_2::exercise_2, exercise_3::exercise_3};

pub mod exercise_1;
pub mod exercise_2;
pub mod exercise_3;

pub fn exercise(e: usize) {
    match e {
        1 => exercise_1(),
        2 => exercise_2(),
        3 => exercise_3(),
        _ => return,
    }
}
