use OOP_concepts::{Button, Screen};

mod blog;
pub use crate::blog::Post;

fn main() {
    // By specifying Box<dyn Draw> as the type of the values in the components vector,
    // we’ve defined Screen to need values that we can call the draw method on.
    // We have to implement the trait in order to store it as a component.
    let screen = Screen {
        components: vec![Box::new(Button {
            width: 50,
            height: 50,
            label: String::from("OK"),
        })],
    };

    screen.run();

    // State pattern implementation:
    /*
    The changes we needed to make to main to reassign post mean that this implementation doesn’t quite
     follow the object-oriented state pattern anymore: the transformations between the states are no longer
      encapsulated entirely within the Post implementation. However, our gain is that invalid states are now
       impossible because of the type system and the type checking that happens at compile time!
    */
    let mut post = Post::new();

    post.add_content("I ate a salad for lunch today");
    assert_eq!("", post.get_content());

    post.request_review();
    assert_eq!("", post.get_content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.get_content());
}
