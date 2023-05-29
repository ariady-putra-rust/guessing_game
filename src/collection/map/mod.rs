pub mod hash_map {
    use std::collections::{BTreeMap, HashMap};
    // See also:
    // cargo add linked-hash-map

    pub fn create() {
        let mut map: HashMap<String, i32> = HashMap::new();
        (0..10).for_each(|i| {
            map.insert(format!("Key{i}"), i);
        });
        dbg!(&map);

        let map: HashMap<String, i32> = HashMap::from_iter((0..10).map(|i| (format!("Key{i}"), i)));
        dbg!(&map);
    }

    pub fn ownership() {
        // For types that implement the Copy trait, like i32, the values are copied into the hash map.
        // For owned values like String, the values will be moved and the hash map will be the owner of those values.
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value are invalid at this point, try using them and
        // see what compiler error you get!
    }

    pub fn access_values() {
        let mut map: HashMap<String, i32> = HashMap::new();
        for i in 0..10 {
            map.insert(format!("Key{i}"), i);
        }

        for (k, v) in &map {
            println!("{}: {}", *k, 0 + *v);
        }
        dbg!(&map);

        for i in 0..map.len() {
            let k = format!("Key{i}");
            let v = map.get(&k);
            // map.get(..)          returns Option<&v>
            // map.get(..).copied() returns Option<v>
            match v {
                Some(&v) => println!("{}: {}", k, 0 + v),
                None => println!("None"),
            }
        }
        dbg!(&map);
    }

    pub fn update_values() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25); // overwrite

        // adding a Key and Value only if the Key isn't present
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);

        // updating a Value based on the Old Value
        let text = "the quick brown fox jumps over the lazy dog";
        let mut map: BTreeMap<String, i32> = BTreeMap::new();
        for word in text.split_whitespace() {
            let key = String::from(word);
            let count = map.entry(key).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
        //
        let mut map: BTreeMap<String, i32> = BTreeMap::new();
        for word in text.split_whitespace() {
            let key = String::from(word);
            let count = map.get(&key).copied().unwrap_or(0);
            map.insert(key, count + 1);
        }
        println!("{:?}", map);
    }
}
