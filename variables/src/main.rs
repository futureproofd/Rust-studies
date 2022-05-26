fn main() {
    let mut x = 5;
    println!("x is {}", x);

    x = 6;
    println!("x is now {}", x);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("y is {}", y);
    println!("tuple access at {}", tup.0);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // is a block that, in this case, evaluates to 4. That value gets bound to y as part of the let statement.
    // Note that the x + 1 line doesn’t have a semicolon at the end,
    // unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons.
    // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
    let thing = {
        let x = 3;
        x + 1
    };

    println!("The value of thing is: {}", thing);

    let num: i32 = five();

    if num < 5 {
        println!("less");
    } else {
        println!("equal or more");
    }

    // this works because if block is an expression.
    let condition = true;
    let number = if condition { 5 } else { 6 };

    print_loop();

    let mut counter = 0;

    // assign a loop expression to a variable. Break and return an assignment
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    for element in months {
        println!("the month is {}", element);
    }
}

fn five() -> i32 {
    5
}

fn print_loop() {
    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 8 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}
