// use directives bring in scope
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    let z: i32 = 42;
    let tup: (char, i32) = ('a', 17);
    let (some_char, some_int) = ('a', 17);

    let x = "out";
    {
        // this is a different `x`
        let x = {
            let y = 1; // first statement
            let z = 2; // second statement
            y + z // this is the *tail* - what the whole block will evaluate to (and return)
        };
    }
    println!("{}", x);

    // In this example, std is a crate (~ a library), cmp is a module (~ a source file), and min is a function:
    let least = std::cmp::min(-3, 8);
    println!("{}", least.is_strictly_negative()); // prints "true"

    // structs (like objects in JS)
    struct Vec2 {
        x: f64, // 64-bit floating point, aka "double precision"
        y: f64,
    }

    // we can instantiate them using struct literals
    let v1 = Vec2 { x: 1.0, y: 3.0 };
    let v2 = Vec2 { y: 2.0, x: 4.0 };

    // we can use 'rest' for 'struct update syntax'
    let v3 = Vec2 { x: 14.0, ..v2 };

    // or rest all
    let v4 = { ..v3 };

    let one = Number {
        odd: true,
        value: 1,
    };
    let two = Number {
        odd: false,
        value: -2,
    };
    print_number(&one); // one is borrowed for the call
    print_number(&two); // so is two

    print_match_number(&two);

    one.is_strictly_positive();
    two.is_strictly_negative();

    let mut mut_num = Number {
        odd: true,
        value: 1,
    };
    println!("mut_num is {}", mut_num.value);

    invert(&mut mut_num);
    println!("mut_num after invert {}", mut_num.value);
}

// a struct is like an object
struct Number {
    odd: bool,
    value: i32,
}

// declare a method for our Number type/struct (the implementation of Number)
impl Number {
    fn is_strictly_positive(self) -> bool {
        self.value > 0
    }
}

// traits can be assigned to multiple types (it's like an interface you can implement)
/*
You can implement:

one of your traits on anyone's type
anyone's trait on one of your types
but not a foreign trait on a foreign type
These are called the "orphan rules".

*/
trait Signed {
    fn is_strictly_negative(self) -> bool;
}

impl Signed for Number {
    fn is_strictly_negative(self) -> bool {
        self.value < 0
    }
}

// foreign type (even a primative!)
impl Signed for i32 {
    fn is_strictly_negative(self) -> bool {
        self < 0
    }
}

// An impl block is always for a type, so, inside that block, Self means that type:
impl std::ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            value: -self.value,
            odd: self.odd,
        }
    }
}

// let patterns can be used as conditionals
fn print_number(n: &Number) {
    if let Number { odd: true, value } = n {
        println!("Odd number: {}", value);
    } else if let Number { odd: false, value } = n {
        println!("Even number: {}", value);
    }
}

fn print_match_number(n: &Number) {
    match n {
        Number { odd: true, value } => println!("Odd number: {}", value),
        Number { odd: false, value } => println!("Even number: {}", value),
        Number { value, .. } => println!("Catch the rest: {}", value),
    }
}

// a function takes a mutable reference - but only if our variable binding is also mut.
fn invert(n: &mut Number) {
    n.value = -n.value;
}
