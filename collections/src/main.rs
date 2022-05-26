use std::{hash::Hash, ptr::NonNull};

fn main() {
    // type annotation because we haven't inserted any elements yet.
    let _vec1: Vec<i32> = Vec::new();
    // infer type and initialize via macro.
    let vec2 = vec![1, 2, 3];
    // mutable to allow adding elements (infer type from push)
    let mut vec3 = Vec::new();
    vec3.push(1);

    // get elements (two ways:)
    let third: &i32 = &vec2[2];
    println!("The third element is {}", third);

    // the get method gives us an Option<T>. Getting by [] out of range will result in an error
    match vec2.get(2) {
        Some(third) => println!("Get > The third element is {}", third),
        None => println!("there is no third element!"),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let _first = &v[0];
    // v.push(6); <- this breaks
    /*
    because vectors put the values next to each other in memory, adding a new element onto the end of the vector might
     require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all
      the elements next to each other where the vector is currently stored. In that case, the reference to the first
       element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.
    */

    // Iteration
    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50; // use dereference operator to get to the value in i
    }

    // enums in vecs
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let enum_vec: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(12.34),
        SpreadsheetCell::Text(String::from("hey")),
    ];

    // Strings:
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
                       // we can see in the signature that add takes ownership of self, because self does not have an &.
                       // This means s1 will be moved into the add call and will no longer be valid after that.
                       // using + is the same as calling the add method: fn add(self, s: &str) -> String {}
                       // So although let s3 = s1 + &s2; looks like it will copy both strings and create a new one,
                       //this statement actually takes ownership of s1, appends a copy of the contents of s2, and then
                       //returns ownership of the result. In other words, it looks like it’s making a lot of copies but isn’t;

    let s4: String = String::from("1");
    let s5: String = String::from("2");
    let s6: String = String::from("3");
    // format! uses references so it doesn't take ownership.
    let s7 = format!("{}-{}-{}", s4, s5, s6);

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    // strings allocate bytes.
    // Here, s will be a &str that contains the first 4 bytes of the string.
    // Earlier, we mentioned that each of these characters was 2 bytes, which means s will be Зд.

    for c in s.chars() {
        // print chars
        println!("{}", c);
    }

    //Hashmaps:
    use std::collections::HashMap;

    let mut hash1 = HashMap::new();
    hash1.insert(String::from("Red"), 10);
    hash1.insert(String::from("Blue"), 12);

    // combine / iterate two vectors into a hashmap:
    let vect1 = vec![String::from("Red"), String::from("Blue")];
    let vect2 = vec![12, 13];
    //use the zip method to create an iterator of tuples where “Blue” is paired with 10,
    // and so forth. Then we could use the collect method to turn that iterator of tuples into a hash map,
    let mut scores1: HashMap<_, _> = vect1.into_iter().zip(vect2.into_iter()).collect();

    //The type annotation HashMap<_, _> is needed here because it’s possible to collect into
    // many different data structures and Rust doesn’t know which you want unless you specify.
    // For the parameters for the key and value types, however, we use underscores, and Rust can
    // infer the types that the hash map contains based on the types of the data in the vectors. I

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue1");
    let score = scores.get(&team_name);

    match score {
        Some(s) => println!("score is {}", s),
        None => println!("not found."),
    }

    // iterate
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // overwrite:
    scores.insert(String::from("Blue"), 25);

    // or_insert: (returns a mutable reference to the value of the Entry key, or else insert a new one)
    scores.entry(String::from("Blue")).or_insert(50);
}
