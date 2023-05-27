use crate::strings::string;

/* shallow copy */
pub fn shallow_copy() {
    let s1 = string();
    let s2 = s1;
    // println!("s1:{s1}"); // `s1` is no longer valid here!
    println!("s2:{s2}");
}

/* deep copy */
pub fn deep_copy() {
    let s1 = string();
    let s2 = s1.clone();
    println!("s1:{s1}");
    println!("s2:{s2}");
}
