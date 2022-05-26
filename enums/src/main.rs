enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// we can define methods on enums!
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Quarter(UsState),
    Nickel,
    Dime,
}

fn value_in_cents(coin: Coin) -> u32 {
    // match expression keyword is a coin
    match coin {
        // match arms have a pattern and some code
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
    }
}

fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hey"));
    m.call();

    let num1: i8 = 5;
    let num2: Option<i8> = Some(6);
    let seven = add_one(num2);
    // let sum = num1 + num2; < this won't work because Option<i8> is a different type
    /*
    Intense! In effect, this error message means that Rust doesn’t understand how to add an i8 and an Option<i8>, because they’re different types.
     When we have a value of a type like i8 in Rust, the compiler will ensure that we always have a valid value.
     We can proceed confidently without having to check for null before using that value.
     Only when we have an Option<i8> (or whatever type of value we’re working with)
     do we have to worry about possibly not having a value, and the compiler will make sure we handle that case before using the value.

    In other words, you have to convert an Option<T> to a T before you can perform T operations with it.
    Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

    In general, in order to use an Option<T> value, you want to have code that will handle each variant.
    You want some code that will run only when you have a Some(T) value, and this code is allowed to use the inner T.
     You want some other code to run if you have a None value, and that code doesn’t have a T value available.
     The match expression is a control flow construct that does just this when used with enums: i
     t will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.
    */

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let dice_roll = 9;

    roll_dice(dice_roll);

    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("the max is configured to be {}", max),
        _ => (),
    }
}

fn add_one(num: Option<i8>) -> Option<i8> {
    match num {
        None => None,
        Some(i) => {
            println!("adding one {:?}", num);
            Some(i + 1)
        }
    }
}

fn roll_dice(dice_roll: i32) {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => {
            println!("rerolling with {:?}", dice_roll);
            reroll(dice_roll)
        }
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}

fn reroll(num: i32) {
    roll_dice(num - 1);
}
