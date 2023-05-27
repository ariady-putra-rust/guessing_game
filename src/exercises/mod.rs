pub mod exercise_1;
pub mod exercise_2;

pub fn exercise(e: usize) {
    match e {
        1 => exercise_1::exercise_1(),
        2 => exercise_2::exercise_2(),
        _ => return,
    }
}
