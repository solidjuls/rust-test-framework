//use macro_demo::test;
use testframework::{run_all_tests, Test};

#[test]
fn bar() {
    // assert!(false)
    run_all_tests();
}

#[test]
fn foo() {
    assert!(false);
}

inventory::submit! {
    Test {
        name: "foo",
        file: std::file!(),
        line: std::line!(),
        handler: Box::new(foo),
    }
}
