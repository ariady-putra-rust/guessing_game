/* Unit Struct */
pub struct Unit;
/*
Unit-like structs can be useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself
*/

pub struct X(u8);
impl X {
    pub fn new(x: u8) -> Self {
        X(x)
    }

    pub fn x(&self) -> u8 {
        self.0
    }
}

/* Tuple Structs */
pub struct XY(u8, u8);
pub struct XYZ(u8, u8, u8);

impl XYZ {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> u8 {
        self.0
    }

    pub fn y(&self) -> u8 {
        self.1
    }

    pub fn z(&self) -> u8 {
        self.2
    }
}

pub fn xyz() {
    let xyz = XYZ(5, 4, 3);
    println!("{} {} {}", xyz.0, xyz.1, xyz.2);
}
