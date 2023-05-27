/* Unit Struct */
pub struct Unit;
/*
Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself
*/

/* Tuple Struct */
pub struct XYZ(u8, u8, u8);
pub fn xyz() {
    let xyz = XYZ(5, 4, 3);
    println!("{} {} {}", xyz.0, xyz.1, xyz.2);
}
