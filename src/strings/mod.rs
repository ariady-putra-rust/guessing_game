/* The String Type */
pub fn string() -> String {
    let mut s1 = String::new();
    s1.push_str("Lorem ipsum");
    dbg!(&s1);
    s1 = String::from("Dolor sit amet");
    s1.push_str(", consectetur adipiscing elit");
    dbg!(&s1);

    return s1;
}

/* String Slices */
pub fn string_slices() {
    let s = String::from("hello world");
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    println!("{hello} {world}");
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
}
