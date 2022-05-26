// child module (private by default) pub exposed
pub mod hosting;

// child
pub mod serving {
    fn take_order() {}
    pub fn serve_order() {}
    fn take_payment() {}
}
