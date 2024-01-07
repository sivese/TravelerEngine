#![warn(missing_docs)]
pub mod pong;
mod app;

#[test]
fn module_test() {
    pong::entry();
}
