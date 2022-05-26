struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation block for Rectangle
impl Rectangle {
    // also borrowable reference
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // We can define associated functions that don’t have self as their first parameter
    //(and thus are not methods) because they don’t need an instance of the type to work with.
    // We’ve already used one function like this: the String::from function that’s defined on the String type.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user1 = build_user(String::from("nex@6.com"), String::from("1usr"));

    // use struct update syntax to get the rest of user values
    let user2 = User {
        email: String::from("another@test.com"),
        ..user1
    };

    let scale = 2;
    let rect1 = Rectangle {
        width: 30 * scale,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // we use the debug format specifier to see what our object properties are:
    println!("rect1 is {:?}", rect1);

    // call via method syntax
    // The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.
    println!("the area of the rectagle is {}", rect1.area());

    //  This function is namespaced by the struct: the :: syntax is used for both associated functions and namespaces created by modules
    let sq = Rectangle::square(20);
}
/*
we want to borrow the struct rather than take ownership of it.
This way, main retains its ownership and can continue using rect1,
which is the reason we use the & in the function signature and where we call the function.
*/
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// function to build and implicitly return a User struct
// shorthand variables don't have to be repeated!
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
