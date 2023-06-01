pub fn panic() {
    panic!("at the disco");
}

/* Unit Tests */
// The purpose of unit tests is to test each unit of code in isolation from the rest of the code to
// quickly pinpoint where code is and isn't working as expected. You'll put unit tests in the `src` directory
// in each file with the code that they're testing. The convention is to create a module named `tests`
// in each file to contain the test functions and to annotate the module with `cfg(test)`
#[cfg(test)]
mod tests {
    use std::io::{Error, ErrorKind, Result};

    #[test]
    fn unit_test() {
        // Arrange
        let actual = 1 + 2;

        // Action

        // Assert
        let expect = 3;
        assert_eq!(
            actual, expect,
            /* OPTIONAL Panic message: */ "got {{{}}} while expecting {{{}}}",
            /* Panic message param(s): */ actual, expect,
        );
    }

    #[test]
    fn test_fail() {
        assert_eq!(1 + 2, 4, "this test will fail");
    }

    #[test]
    fn bool_test() {
        assert!(true, "checking bool with assert!");
    }

    #[test]
    fn negative_test() {
        assert_ne!(1, 2, "1 != 2");
    }

    // checking for panics with `should_panic`
    #[test]
    #[should_panic(expected = "disco")]
    fn panic_test() {
        super::panic();
    }

    #[test]
    #[should_panic(expected = "anywhere")]
    fn panic_test_fail() {
        super::panic();
    }

    // using Result<T, E> in tests
    #[test]
    fn result_t_e() -> Result<()> {
        Ok(())
    }

    #[test]
    fn result_t_e_fail() -> Result<()> {
        Err(Error::new(ErrorKind::Other, "other error"))
    }

    // ignoring some tests unless specifically requested
    #[test]
    #[ignore]
    // test unit_test::test_ignore ... ignored
    fn test_ignore() {
        // this takes a long time to run
    }
}
