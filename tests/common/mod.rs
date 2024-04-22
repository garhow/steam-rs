#[macro_export]
/// DO NOT USE THIS MACRO!! USE `#[tokio::test]` INSTEAD!
macro_rules! async_test {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}
