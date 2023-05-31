use crate::structs::XYZ;

use super::{Printable, Show};

impl Show for usize {
    fn show(&self) -> String {
        String::from(match *self {
            0 => "None",
            _ => "Some",
        })
    }
}

impl Show for bool {
    fn show(&self) -> String {
        String::from(if *self { "Right" } else { "Wrong" })
    }
}

// use some default implementations of `Show`
impl Show for char {
    // just implement this 1 method only
    fn show_lowercase(&self) -> String {
        self.show_uppercase()
    }
}

impl Printable for XYZ {
    fn print(&self) -> String {
        format!("XYZ = [x:{}; y:{}; z:{}]", self.x(), self.y(), self.z())
    }
}
