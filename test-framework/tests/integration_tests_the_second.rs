use macro_demo::test_custom;
use testframework::register_test;

testframework::setup!();

#[test_custom]
fn foo() {
    assert!(true);
}