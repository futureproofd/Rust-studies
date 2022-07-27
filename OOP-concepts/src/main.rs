use OOP_concepts::{Button, Screen};

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
}
