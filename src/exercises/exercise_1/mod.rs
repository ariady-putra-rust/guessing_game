pub fn exercise_1() {
    /* 1. Convert temperatures between Fahrenheit and Celsius. */
    println!("212F = {}C", to_celsius(212.0));
    println!("100C = {}F", to_fahrenheit(100.0));

    /* 2. Generate the nth Fibonacci number. */
    println!();
    println!("Fibonacci [0-10]:");
    for n in 0..=10 {
        print!(" {} ", fibonacci(n));
    }
    println!();

    /* 3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
    taking advantage of the repetition in the song. */
    println!();
    println!("Christmas Carol");
    println!();
    sing_christmas_carol();
}

fn to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn to_fahrenheit(celsius: f32) -> f32 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fibonacci(n: u8) -> u16 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub enum Day {
    Zero,
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
    Tenth,
    Eleventh,
    Twelfth,
}
impl Day {
    pub fn to_string(&self) -> String {
        String::from(match self {
            Self::Zero => "zero",
            Self::First => "first",
            Self::Second => "second",
            Self::Third => "third",
            Self::Fourth => "fourth",
            Self::Fifth => "fifth",
            Self::Sixth => "sixth",
            Self::Seventh => "seventh",
            Self::Eighth => "eighth",
            Self::Ninth => "ninth",
            Self::Tenth => "tenth",
            Self::Eleventh => "eleventh",
            Self::Twelfth => "twelfth",
        })
    }

    pub fn from_usize(u: usize) -> Self {
        match u {
            1 => Self::First,
            2 => Self::Second,
            3 => Self::Third,
            4 => Self::Fourth,
            5 => Self::Fifth,
            6 => Self::Sixth,
            7 => Self::Seventh,
            8 => Self::Eighth,
            9 => Self::Ninth,
            10 => Self::Tenth,
            11 => Self::Eleventh,
            12 => Self::Twelfth,
            _ => Self::Zero,
        }
    }
}
fn sing_christmas_carol() {
    for d in 1..=12 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            Day::from_usize(d).to_string()
        );
        send_gift(d);
    }
}

fn send_gift(day: usize) {
    let gift = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    match day {
        1 => {
            println!("{}", gift[0]);
            println!();
        }
        2 => {
            println!("{}, and", gift[1]);
            send_gift(day - 1);
        }
        _ => {
            println!("{}", gift[day - 1]);
            send_gift(day - 1);
        }
    }
}
