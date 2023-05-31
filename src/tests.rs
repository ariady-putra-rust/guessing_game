use crate::{
    generics, lifetimes,
    structs::XYZ,
    traits::{Printable, Show},
};
use file_access::FilePath;
use std::{fs, io::Result};

pub fn run(test: usize) {
    if let Err(x) = match test {
        // Files
        1 => read_file("Cargo.toml"),
        2 => pretty_print_with_line_numbers(),
        3 => test_error(), // file doesn't exist
        4 => file_metadata(),
        5 => shortest_way_to_read_a_file(),

        // Generics, Traits & Lifetimes
        6 => generic(),
        7 => traits(),
        8 => lifetime(),

        // else
        _ => Ok(Default::default()),
    } {
        panic!("{x}");
    }
}

fn read_file(file_path: &str) -> Result<()> {
    Ok({
        /* file_access::read_string(&str) */
        // {
        //     let path_str = file_path;
        //     let text = file_access::read_string(&path_str)?;
        //     println!("{path_str}");
        //     println!("{text}");
        // }

        /* file_access::read_string(&String) */
        // {
        //     let path_string = String::from(file_path);
        //     let text = file_access::read_string(&path_string)?;
        //     println!("{path_string}");
        //     println!("{text}");
        // }

        /* file_access::read_string(&FilePath) */
        // {
        //     let path = FilePath::access(&file_path);
        //     let text = file_access::read_string(&path)?;
        //     println!("{file_path}");
        //     println!("{text}");
        // }

        /* FilePath::access(&str).read_string() */
        // {
        //     let path_str = file_path;
        //     let file = FilePath::access(&path_str);
        //     let text = file.read_string()?;
        //     println!("{path_str}");
        //     println!("{text}");
        // }

        /* FilePath::access(&String).read_string() */
        // {
        //     let path_string = String::from(file_path);
        //     let file = FilePath::access(&path_string);
        //     let text = file.read_string()?;
        //     println!("{path_string}");
        //     println!("{text}");
        // }

        let file = FilePath::access(&file_path);
        let text = file_access::read_string(&file)?;
        println!("{}:", file.get_full_path()?);
        println!("{text}");
    })
}

fn pretty_print_with_line_numbers() -> Result<()> {
    Ok({
        let file = FilePath::access(&"Cargo.toml");
        let lines = file.read_lines()?;
        let len = lines.len();
        let w = len.to_string().len();
        println!("{}", file.get_full_path()?);
        for line in 0..len {
            // println!("{:w$}: {line}", line_number, w = 5, line = "str");
            println!("{:w$}: {line}", line + 1, w = w, line = lines[line]);
        }
    })
}

fn test_error() -> Result<()> {
    read_file("FILE_DOES_NOT_EXIST")
}

fn file_metadata() -> Result<()> {
    Ok({
        let file = FilePath::access(&"Cargo.toml");
        let metadata = file.get_metadata()?;
        println!("{}", file.get_full_path()?);
        println!("{:#?}", metadata);
    })
}

fn shortest_way_to_read_a_file() -> Result<()> {
    Ok({
        let file = fs::read_to_string("Cargo.toml")?;
        println!("{file}");
    })
}

fn generic() -> Result<()> {
    Ok(generics::generic())
}

fn traits() -> Result<()> {
    Ok({
        let qty: usize = 3;
        println!("{qty} is {}", qty.show()); // use crate::..::Show

        let she: bool = true;
        println!("{she} is {}", she.show());

        let c: char = 'c';
        println!("{c} says, \"{}\"", c.show());
        println!("{c} screams, \"{}\"", c.show_uppercase());
        println!(
            "{c} pretends to whisper, \"ssh...SURPRISE! {}\"",
            c.show_lowercase()
        );

        let xyz = XYZ::new(3, 6, 9);
        println!("{}", xyz.print());
    })
}

fn lifetime() -> Result<()> {
    Ok({
        lifetimes::lifetime();
        lifetimes::lifetime_annotations_in_struct_definitions();
    })
}
