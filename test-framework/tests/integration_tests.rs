use macro_demo::test_custom;
use testframework::register_test;

testframework::setup!();

#[test_custom]
fn passes() {
    assert!(true);
}

#[test_custom]
fn fails() {
    assert!(false);
}

#[test_custom]
fn also_fails() {
    assert!(false);
}
