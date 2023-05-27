use crate::exercises::exercise_1::Day;

/* if-let(-else) */
pub fn if_let_else() {
    let day = Day::Zero;
    if let Day::Zero = day {
        println!("Invalid Day");
    }
    // else {}
    /*
    is equivalent to:
    */
    let day = Day::Zero;
    match day {
        Day::Zero => println!("Invalid Day"),
        _ => (),
    }
}
