pub fn exercise_2() {
    /* write a function that takes a string of words separated by spaces and returns the first word it finds in that string */
    // If the function doesn’t find a space in the string,
    // the whole string must be one word,
    // so the entire string should be returned.
    let mut sentence = String::from("The quick brown fox jumps over the lazy dog.");
    println!("{sentence}");
    let first_word = first_word(&sentence);
    let last_word = last_word(&sentence);
    sentence.clear();
    println!("{sentence}");
    println!("{} {}", first_word, last_word);
}

fn first_word(sentence: &String) -> String {
    nth_word(0, sentence)
}

fn last_word(sentence: &String) -> String {
    nth_back_word(0, sentence)
}

fn nth_word(n: usize, sentence: &String) -> String {
    let mut words = sentence.split_whitespace();
    return String::from(words.nth(n).unwrap_or(""));
}

fn nth_back_word(n: usize, sentence: &String) -> String {
    let mut words = sentence.split_whitespace();
    return String::from(words.nth_back(n).unwrap_or(""));
}
