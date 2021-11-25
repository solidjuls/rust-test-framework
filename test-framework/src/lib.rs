use std::fmt;
use std::panic::catch_unwind;
use std::panic::RefUnwindSafe;

pub struct Test {
    pub name: &'static str,
    pub line: u32,
    pub file: &'static str,
    pub handler: Box<dyn Fn() + RefUnwindSafe>,
}

impl Test {
    pub fn run(&self) -> TestOutput {
        let result = catch_unwind(|| {
            (self.handler)();
        });

        match result {
            Ok(_) => {
                println!("OK -- {}", self.name);
                return TestOutput::Pass;
            }
            Err(_) => {
                println!("KO -- {}", self.name);
                return TestOutput::Fail;
            }
        }
    }
}

#[derive(Debug)]
pub enum TestOutput {
    Pass,
    Fail,
}
impl fmt::Debug for Test {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Test")
            .field("name", &self.name)
            .field("line", &self.line)
            .field("file", &self.file)
            .field("handler", &"{closure}")
            .finish()
    }
}

inventory::collect!(Test);

pub fn run_all_tests() {
    let mut all_tests = Vec::new();

    for test in inventory::iter::<Test> {
        all_tests.push(test);
    }
    if let Ok(file) = std::env::var("TEST_FILE") {
        all_tests = all_tests
            .into_iter()
            .filter(|test| test.file == file)
            .collect()
    }

    run_all_tests_and_print_output(all_tests);
}

fn run_all_tests_and_print_output(tests: Vec<&Test>) {
    let mut failed_test = Vec::new();
    let mut passed_test = Vec::new();
    for test in tests {
        let output = test.run();
        match output {
            TestOutput::Pass => passed_test.push(test),
            TestOutput::Fail => failed_test.push(test),
        }
    }

    if !failed_test.is_empty() {
        println!(
            "{} test failures, {} passed",
            failed_test.len(),
            passed_test.len()
        );
    } else {
        println!("EVERYTHING WORKS!")
    }
}

#[macro_export]
macro_rules! setup {
    () => {
        #[test]
        fn test_wrapper_run_all_tests() {
            $crate::run_all_tests();
        }
    };
}

#[macro_export]
macro_rules! register_test {
    ( $( $t:tt )*) => {
        inventory::submit! {
            $($t)*
        }
    };
}
