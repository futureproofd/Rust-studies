/*
The state pattern is an object-oriented design pattern. The crux of the pattern is that a value has some internal state,
 which is represented by a set of state objects, and the value’s behavior changes based on the internal state.
  The state objects share functionality: in Rust, of course, we use structs and traits rather than objects and inheritance.
   Each state object is responsible for its own behavior and for governing when it should change into another state.
    The value that holds a state object knows nothing about the different behavior of the states or when to transition between states.
*/
pub struct Post {
    // value changes based on internal state (Draft, PendingReview, Published)
    // changes are dynamically dispatched.
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        // This ensures whenever we create a new instance of Post, it will start out as a draft.
        // Because the state field of Post is private, there is no way to create a Post in any other state.
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // mutable because we're changing the Post instance we're calling on
    pub fn add_content(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // gets a reference to self (Post struct) and call use_ref on the Option
    // (we want a ref, not ownership of the value). Because an Option<&Box<dyn State>> is returned,
    // we must then call unwrap (see ch.9), followed by the 'content' method implemented on State.
    // At this point, the deref coercion will take effect, which means the correct content method for the given state object will be called. (See impl State for Published)
    pub fn get_content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    // The request_review method on Post is the same no matter its state value. Each state is responsible for its own rules.
    pub fn request_review(&mut self) {
        // This is because calling State::request_review (In Draft impl) will move Box<self>, which is allocated on the heap,
        // and Rust doesn't allow you to just move values away from heap unless you implement Copy,
        // otherwise what's left there? The book uses Option::take() to move ownership out and leave None in place.
        // We need to set state to None temporarily rather than setting it directly with code like self.state = self.state.request_review();
        // to get ownership of the state value. This ensures Post can’t use the old state value after we’ve transformed it into a new state.
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review()) // returns a new State
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// This syntax means the method is only valid when called on a Box holding the type.
// This syntax takes ownership of Box<Self>, invalidating the old state so the state
// value of the Post can transform into a new state.
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

// our state objects
struct Draft {}
struct PendingReview {}
struct Published {}

// our state implementations
impl State for Draft {
    // The request_review method on Draft needs to return a new, boxed instance of a new PendingReview struct,
    // which represents the state when a post is waiting for a review.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // if we call the approve method on a Draft, it will have no effect
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

impl State for PendingReview {
    // PendingReview struct also implements the request_review method but doesn’t do any transformations.
    // Rather, it returns itself, because when we request a review on a post already in the PendingReview state,
    // it should stay in the PendingReview state.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // When we call approve on PendingReview, it returns a new, boxed instance of the Published struct.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// The Published struct implements the State trait, and for both the request_review method and the approve method,
// it returns itself, because the post should stay in the Published state in those cases.
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Note that we need lifetime annotations on this method, as we discussed in Ch 10.
    // We’re taking a reference to a post as an argument and returning a reference to part of that post,
    // so the lifetime of the returned reference is related to the lifetime of the post argument.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
