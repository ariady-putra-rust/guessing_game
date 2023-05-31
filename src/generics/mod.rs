use crate::traits::{Printable, Show};

pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn move_to(&mut self, x: T, y: T) -> &Self {
        self.x = x;
        self.y = y;
        return self;
    }
}

/* An `impl` block that only applies to a struct with a
particular concrete type for the generic type parameter `T`
*/
impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T: Printable + Show> Point<T> {
    pub fn method_implementation_only_for_point_with_printable_and_show_traits(&self) {
        println!("{}", self.x.print());
        println!("{}", self.y.show());
    }
}

pub fn generic() {
    let p = Point {
        x: "5".to_string(),
        y: "10".to_string(),
    };

    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());

    let q = Point { x: p.x(), y: p.y() };
    println!("q = ({}, {})", q.x(), q.y());

    let r = Point { x: p.x(), y: p.y() };
    println!("r = ({}, {})", r.x(), r.y());

    let x = p.x();
    let y = p.y();
    println!("(x, y) = ({x}, {y})");
    // let z = x + y;
    // println!("x + y = {z}");

    let mut a = Point::new(4, 5);
    let mut b = Point::new(8, 9);
    println!("a = ({}, {})", a.x(), a.y());
    println!("b = ({}, {})", b.x(), b.y());
    a.move_to(b.x, b.y);
    println!("a = ({}, {})", a.x(), a.y());
    println!("b = ({}, {})", b.x(), b.y());
    b.move_to(a.x, a.y);
    println!("a = ({}, {})", a.x(), a.y());
    println!("b = ({}, {})", b.x(), b.y());
}
