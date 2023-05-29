pub fn format() {
    let s = format!("{} {} {} {}", 1, 2.3, 'a', "ABC");
    println!("{s}");
}

// For iterating over strings by `grapheme cluster`
// https://doc.rust-lang.org/book/ch08-02-strings.html#methods-for-iterating-over-strings
// Take a look at:
// cargo add unicode_clusters
