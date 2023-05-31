/* Borrowing */
// The opposite of referencing by using `&` is dereferencing,
// which is accomplished with the dereference operator `*`
pub fn borrowing() {
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
    multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else's reading of the data
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
