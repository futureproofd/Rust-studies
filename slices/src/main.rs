fn main() {
    let mut s = String::from("hello world");
    let word_position = first_word(&s);

    // s.clear();

    // issue here is that word_position still has a reference to 5 if we clear it
    println!("{}", word_position);

    // concept of range slicing:
    let hello = &s[0..5];
    let world = &s[6..11];

    let word_slice: &str = first_word_slice(&s);
    println!("{}", word_slice);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_slice(&my_string_literal[0..6]);
    let word = first_word_slice(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slice(my_string_literal);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // enumerate creates a tuple version of the iteration
    // &item is a reference to each element
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // pattern indicates a space
        if item == b' ' {
            return &s[0..i];
        }
    }
    // return everything
    &s[..]
}
