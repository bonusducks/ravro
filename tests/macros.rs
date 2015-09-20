
macro_rules! test {
    ($f:ident, $body:block) => {
        #[test]
        fn $f() {
            use LOGGER_INIT;
            if *LOGGER_INIT == () {
                $body
            } else {
                panic!("failed to init logging system");
            }
        }
    }
}
