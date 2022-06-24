use std::{ops::Deref, rc::Rc};

use crate::List::{Cons, Nil};

// recursively-defined types:
// without a Box definition, i.e. Cons(i32, List), this Construct (Cons) has an infinite size
// Note our second argument for Cons is our same 'List', which indicates recursion, hence a possible infinite size.
// Rust has no idea how to allocate memory for something that could run infinitely, so we have
// to provide a 'Box' as a fixed-size (Stack)pointer reference to the value on the Heap
/*

enum List {
    Cons(i32, Box<List>),
    Nil,
}
*/
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

/*
    The Drop trait
    Specify the code to run when a value goes out of scope by implementing the Drop trait
*/
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping custom smart pointer for {}", self.data)
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    //let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    //The variable x holds an i32 value, 5. We set y equal to a reference to x. We can assert that x is equal to 5.
    //However, if we want to make an assertion about the value in y, we have to use *y to follow the reference
    // to the value it’s pointing to (hence dereference).
    assert_eq!(5, *y); // note that if we didn't include a dereference, we'd get an error because a number and a ref to one are different types.

    // we can do the same with Boxes, but there are some differences. See below.
    let var1 = 5;
    let var2 = Box::new(var1);

    assert_eq!(5, var1);
    assert_eq!(5, *var2);

    /*
        Building our own smart-pointer Box
        - how the behave differently than regular &references
    */

    // A tuple struct with one element, T
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            // return and hold the value passed into our struct
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            // returns a reference to the value (first of the tuple) we want to access with the * operator
            &self.0
        }
    }

    let my_var1 = 5;
    let my_var2 = MyBox::new(my_var1);

    assert_eq!(5, my_var1);
    // Rust substitutes the * operator with a call to the deref method we implemented
    assert_eq!(5, *my_var2);

    // deref coercion:
    fn hello(name: &str) {
        println!("hello, {}", name)
    }

    // note our hello function takes a string slice. We're able to coerce our MyBox dereference of a String into a string slice.
    // The standard library provides an implementation of Deref on String that returns a string slice, and this is in the API
    // documentation for Deref. Rust calls deref again to turn the &String into &str
    let m = MyBox::new(String::from("Marcus"));
    hello(&m);
    // without deref coercion we'd have to write it like this:
    // The (*m) dereferences the MyBox<String> into a String. Then the & and [..] take a string slice of the String
    hello(&(*m)[..]);

    // Drop trait will run when this goes out of scope
    let cust1 = CustomSmartPointer {
        data: String::from("My data"),
    };

    /*
        Reference Counted Smart Pointer
         Using Rc<T> allows a single value to have multiple owners, and the count ensures that the value remains valid
         as long as any of the owners still exist.
    */
    // let list_a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // Cons variants own the data they hold (list_a is moved to list_b, and so on)
    // let list_b = Cons(3, Box::new(list_a));
    // let list_c = Cons(3, Box::new(list_a)); <--- won't work because list_a has already been moved to list_b

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("count after creating a = {}", Rc::strong_count(&a));

    // The call to Rc::clone only increments the reference count!
    // share data between multiple parts of your program for reading only
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
    {
        let d = Cons(4, Rc::clone(&a));
        println!("count after creating d = {}", Rc::strong_count(&a));
    }
    // The implementation of the Drop trait decreases the reference count automatically when an Rc<T> value goes out of scope.
    println!("count after d goes out of scope = {}", Rc::strong_count(&a));
}
