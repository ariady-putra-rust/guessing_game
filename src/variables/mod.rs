/* Variables */
pub fn variable() {
    let apples = 5; // immutable
    let mut bananas = 5; // mutable
    let apples = apples + 1; // shadowing
    bananas += 1;
    println!("bananas: {1} and apples: {0}", apples, bananas);
}

/* Constants */
pub fn constants() {
    const THREE_HOURS_IN_SECONDS: u16 = 60 * 60 * 3;
    // let THREE_HOURS_IN_SECONDS = 3; // `THREE_HOURS_IN_SECONDS` is interpreted as a constant pattern
    println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS}");
}

/* Tuple */
pub fn tuple() {
    let tup = (1, 2.3, 'a', "ABC", true);
    println!("{} {} {} {} {}", tup.0, tup.1, tup.2, tup.3, tup.4); // 1 2.3 'a' "ABC" true
    let (_, _, a, _, _) = tup; // destructuring the tuple via pattern-matching
    println!("{a}"); // 'a'
}

/* Array */
pub fn array() {
    let a: [u8; 3] = [1, 2, 3];
    let [_, b, _] = a;
    println!("{b}");
    let c = [true; 3]; // [true, true, true]
    println!("{}", c[1]);
}

/* Array Slices */
pub fn array_slices() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
