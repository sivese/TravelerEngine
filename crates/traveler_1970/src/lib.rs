#![warn(missing_docs)]
pub mod pong;

mod app;
mod vector;

use test_log::test;

#[test]
fn module_test() {
    pong::entry();
}
