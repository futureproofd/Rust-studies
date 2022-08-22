fn main() {
    // while let conditional loops
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loop pattern
    let v = vec![1, 2, 3];
    // enumerate adapts the iterator into a tuple (idx, val)
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        1 | 2 => println!("one or two"),
        4..=6 => println!("four to six"),
        _ => println!("something else"),
    }

    let y = 'c';

    match y {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 10, y: 5 };

    // destructuring creates variables a and b
    let Point { x: a, y: b } = p;
    assert_eq!(10, a);
    assert_eq!(5, b);

    // short form:
    let Point { x, y } = p;
    assert_eq!(10, x);
    assert_eq!(5, y);

    // mega destructure for convenience
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    let axis = Point { x: 0, y: 7 };

    match axis {
        Point { x, y: 0 } => println!("on the x axis at {}", x),
        Point { x: 0, y } => println!("on the y axis at {}", y),
        Point { x, y } => println!("on neither axis, ({},{})", x, y),
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    // matching enum variant patterns
    enum Message {
        Quit,
        Move { x: i32, y: i32 }, // a struct variant
        Write(String),           // a tuple variant
        ChangeColor(Color),      // nested tuple variant
    }

    let msg = Message::ChangeColor(Color::Hsv(123, 111, 90));

    // each variant pattern is described in each match arm:
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to red {}, green {}, and blue {}", h, s, v)
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    // ignore pattern:
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    let origin = Point { x: 0, y: 0 };

    // .. pattern ignores remaining parts (similar to x: i32, y: _, but expands to many values)
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}
