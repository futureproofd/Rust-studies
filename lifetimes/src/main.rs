fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // different concrete lifetimes: borrow checker is valid here
    let string3 = String::from("long string is long");

    {
        // smaller lifetime of references passed in to 'longest' in this case is string4.
        // therefore, the lifetime of the reference returned by the function has to be within the same scope
        let string4 = String::from("xyz");
        let result = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result);
    }

    // another example:
    // for result to be valid for the println! statement, string6 would need to be valid until the end of the outer scope.
    // Rust knows this because we annotated the lifetimes of the function parameters and return values
    // using the same lifetime parameter 'a.
    let string5 = String::from("long string is long");
    let result;
    {
        // the lifetime of the reference returned by the function is the same
        // as the smaller of the lifetimes of the references passed in (string6).
        let string6 = String::from("xyz");
        result = longest(string5.as_str(), string6.as_str()); // <- string6 does not live long enough
    }
    println!("The longest string is {}", result);
}

// return type has a missing lifetime specifier:
// we cannot determine what variable the returned reference refers to (x or y)
// neither do we know the concrete lifetime of the variables passed in. Will the references
// we return even be valid?
/*

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

// Lifetime annotation syntax:
/*
    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime
*/

// We’ve told Rust that the lifetime of the reference returned by the 'longest'
// function is the same as the smaller of the lifetimes of the references passed in.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
