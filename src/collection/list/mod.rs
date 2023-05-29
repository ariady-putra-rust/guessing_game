pub mod vector {
    pub fn create() {
        let mut list: Vec<i32> = Vec::new();

        list.append(&mut vec![1, 2, 3]);
        dbg!(&list);

        list.copy_from_slice(&[1, 2, 3]);
        dbg!(&list);

        list.clear();
        for i in 1..=3 {
            list.push(i);
        }
        dbg!(&list);

        let list = Vec::from([1, 2, 3]);
        dbg!(&list);

        let list = vec![1, 2, 3];
        dbg!(&list);
    }

    pub fn read_elements() {
        let list = vec![1, 2, 3];

        for i in &list {
            print!(" {} ", 0 + i);
        }
        println!();

        for i in 0..list.len() {
            print!(" {} ", 0 + &list[i]);
        }
        println!();

        for i in 0..list.len() {
            match &list.get(i) {
                // Some(&i) => print!(" {} ", 0 + i),
                Some(i) => print!(" {} ", 0 + *i), // dereferencing
                None => print!(" None "),
            }
        }
        println!();
    }

    pub fn read_mutable_list() {
        let mut list = Vec::from([1, 2, 3]);

        for i in &mut list {
            print!(" {} ", 0 + *i); // dereferencing
        }
        println!();

        for i in 0..list.len() {
            print!(" {} ", 0 + &list[i]);
        }
        println!();

        for i in 0..list.len() {
            match &list.get(i) {
                // Some(&i) => print!(" {} ", 0 + i),
                Some(i) => print!(" {} ", 0 + *i), // dereferencing
                None => print!(" None "),
            }
        }
        println!();
    }

    pub mod store_multiple_types {
        #[derive(Debug)]
        pub enum Trick {
            Int(i32),
            Float(f64),
            Str(String),
            Bool(bool),
        }

        pub fn create() {
            let multitype_list = Vec::from([
                Trick::Int(1),
                Trick::Float(2.3),
                Trick::Str(String::from("ABC")),
                Trick::Bool(true),
            ]);
            for t in &multitype_list {
                dbg!(&t);
            }
        }
    }
}
