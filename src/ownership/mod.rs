/* Ownership Rules */
// 1. Each value in Rust has an owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
pub fn main() {
    ownership_and_functions();
    transferring_ownership_of_return_values();
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
