use std::fmt::Debug;
use std::fmt::Display;

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p2 = Point2 { x: 10, y: 20.2 };
    let p2_mix = Point2 { x: "hey", y: 't' };
    let p2_res = p2.mixup(p2_mix);

    println!("point2 y is now: {}", p2_res.y);

    let article = NewsArticle {
        headline: String::from("Fast fashion is dead!"),
        location: String::from("Worldwide"),
        author: String::from("Zara"),
        content: String::from("The world is on fire."),
    };

    println!("new article: {}", article.summarize());
}

// Traits: similar to a feature often called interfaces in other languages, although with some differences.
pub trait Summary {
    fn summarize(&self) -> String;
    /*
    or we could specify a default behavior:
       fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    then specify an empty impl block with impl Summary for NewsArticle {}.
    */
}

pub trait Displayer {
    fn display(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implement a trait for a type
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
// use the impl Trait syntax: accept any type that implements a trait
pub fn notify(item: &impl Summary) {
    println!("News flash! {}", item.summarize());
}

// same as above, but trait bound syntax. Useful if you have two params that require the same type
// and not just the same trait
pub fn notify_2<T: Summary>(item: &T) {
    println!("News flash! {}", item.summarize());
}

// multiple trait bounds:
pub fn notify_3(item: &(impl Summary + Displayer)) {
    println!("News flash! {}", item.summarize());
    println!("Details {}", item.display());
}

// multiple trait bounds on generic types:
pub fn notify_4<T: Summary + Displayer>(item: &T) {
    println!("News flash! {}", item.summarize());
    println!("Details {}", item.display());
}

// clearer trait bounds with where clause:
// instead of this: fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
fn some_func<T, U>(t: &T, u: &U) -> i32
where
    T: Displayer + Clone,
    U: Clone + Debug,
{
    3
}

// example of trait bounds:
/*
 types like i32 and char that have a known size can be stored on the stack, so they implement the Copy trait.
 But when we made the largest function generic, it became possible for the list parameter to
 have types in it that don’t implement the Copy trait. We enforce that only types with the Copy trait can be used.
*/
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    // iterate item ref
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Using trait bounds to conditionally implement a method
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// only implements the cmp_display method if its inner type T implements the PartialOrd trait
// that enables comparison and the Display trait that enables printing.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest member is x: {}", self.x);
        } else {
            println!("the largest member is y: {}", self.y);
        }
    }
}

// return a type that implements a trait:
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("mad_ficus"),
        content: String::from("elastica"),
        reply: false,
        retweet: false,
    }
}

/** Generics */
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// concrete type implementations will have this method. Generics wont.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    // the function accepts different generics because it's relevant to the method only
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}
