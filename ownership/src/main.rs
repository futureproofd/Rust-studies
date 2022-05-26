fn main() {
    // manage data allocated on the heap.
    let mut s = String::from("hello");

    s.push_str(", world!");

    // So, what’s the difference here? Why can String be mutated but literals cannot?
    //The difference is how these two types deal with memory.

    /*
    With the String type, in order to support a mutable, growable piece of text,
    we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

    The memory must be requested from the memory allocator at runtime.
    We need a way of returning this memory to the allocator when we’re done with our String.
    */

    // we make a copy which points to the same pointer in the heap
    //let s2 = s;
    // this results in an error because the String type doesn't implement the Copy trait.
    //println!("{}", s);

    // this works because integers have a known size at compile time and are put on the stack
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    let s1 = String::from("hello");

    // we can create a reference to our string to allow "borrowing"
    // this means we have to give it back:
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // scopes:
    // this next example works because of a non-lexical lifetime
    // i.e. we create references to s3 and print them before we create a mutable reference to s3
    // it will be known at compile time that this is safe because r1 and r2 are 'out of scope'
    let mut s3 = String::from("hello");

    let r1 = &s3; // no problem
    let r2 = &s3; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s3; // no problem
    println!("{}", r3);

    // Another scope example:
    let mut s4 = String::from("hello");

    {
        let r1 = &mut s4;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s4;
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// this wont work with a borrowed variable because we're trying to change it.
fn change(some_string: &String) {
    some_string.push_str(", world");
}

// this version works because we're flagging the variable as mutable
fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}
