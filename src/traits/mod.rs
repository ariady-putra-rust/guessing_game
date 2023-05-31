use std::fmt::Display;

pub mod implementations;

pub trait Printable {
    fn print(&self) -> String;
}

// `trait` with default implementations
pub trait Show
where
    Self: Display,
{
    fn show(&self) -> String {
        format!("It's {{{}}}", self)
    }

    fn show_uppercase(&self) -> String {
        self.show().to_uppercase()
    }

    fn show_lowercase(&self) -> String {
        self.show().to_lowercase()
    }
}

/* Trait Bound Syntax:
pub fn only_accept_printable_trait<T: Printable>(param: &T)
sugar:
*/
pub fn only_accept_printable_trait(param: &impl Printable) {
    println!("{}", param.print());
}
