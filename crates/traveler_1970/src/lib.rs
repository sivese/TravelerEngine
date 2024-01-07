#![warn(missing_docs)]
pub mod pong;
mod app;

use test_log::test;

#[test]
fn module_test() {
    pong::entry();
}
