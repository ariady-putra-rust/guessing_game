use crate::traits::Printable;
use std::fmt::Display;

/* Lifetime Elision Rules */
// 1. The compiler assigns a lifetime parameter to each parameter that's a reference.
// In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
// a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
// and so on.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters:
// fn foo<'a>(x: &'a i32) -> &'a i32.
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
// the lifetime of `self` is assigned to all output lifetime parameters.
// This third rule makes methods much nicer to read and write because fewer symbols are necessary.
/*
The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes.
If the compiler gets to the end of the three rules and there are still references for which it can't figure out lifetimes,
the compiler will stop with an error. These rules apply to `fn` definitions as well as `impl` blocks.
*/

pub fn lifetime() {
    let string1 = String::from("abcd");

    // using the `longest` function with references to `String` values that have different concrete lifetimes
    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    /* One special lifetime we need to discuss is 'static,
    which denotes that the affected reference can live for the entire duration of the program.
    All string literals have the 'static lifetime, which we can annotate as follows:
    */
    let string3: &'static str = "I have a static lifetime.";
    /* The text of this string is stored directly in the program's binary,
    which is always available. Therefore, the lifetime of all string literals is 'static.
    */
    let result = longest_with_an_announcement(
        string1.as_str(),
        string3,
        "A long time ago in a galaxy far, far away...",
    );
    println!("The longest string is now {}", result);
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    /*
    fn longest(s1: &str, s2: &str) -> &str {
    */
    /*
    Let's apply the first rule: each parameter gets its own lifetime.
    This time we have two parameters instead of one, so we have two lifetimes:
    */
    /*
    fn longest<'a, 'b>(s1: &'a str, s2: &'b str) -> &str {
    */
    /*
    You can see that the second rule doesn't apply because there is more than one input lifetime.
    The third rule doesn't apply either, because `longest` is a function rather than a method,
    so none of the parameters are `self`. After working through all three rules,
    we still haven't figured out what the return type's lifetime is.
    */
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub struct ImportantExcerpt<'a> {
    /*
    This struct has the single field `part` that holds a string slice, which is a reference.
    As with generic data types, we declare the name of the generic lifetime parameter inside
    angle brackets after the name of the struct so we can use the lifetime parameter in the
    body of the struct definition.
    */
    pub part: &'a str,
    /*
    This annotation means an instance of `ImportantExcerpt` can't outlive the reference it
    holds in its `part` field.
    */
}

impl<'a> ImportantExcerpt<'a> {
    pub fn letter_count(&self) -> usize {
        self.part.len()
    }

    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);

        return self.part;
    }
}

impl Printable for ImportantExcerpt<'_> {
    fn print(&self) -> String {
        format!("Important excerpt: \"{}\"", self.part)
    }
}

pub fn lifetime_annotations_in_struct_definitions() {
    let novel = String::from("Call me Ariady. A long time ago in a galaxy far, far away...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.print());
    println!("Letter count is {}", i.letter_count());
    println!(
        "Once again, \"{}\"",
        i.announce_and_return_part("This is an announcement!")
    );
}
