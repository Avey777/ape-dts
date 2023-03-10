#[cfg(test)]
mod test {
    use crate::test::test_runner::TestRunner;
    use serial_test::serial;
    use tokio::runtime::Runtime;

    #[test]
    #[serial]
    fn cdc_basic_test() {
        let rt = Runtime::new().unwrap();
        let runner = rt
            .block_on(TestRunner::new("mysql_to_mysql/cdc_basic_test"))
            .unwrap();
        rt.block_on(runner.run_cdc_test(3000, 1000, false)).unwrap();
    }

    #[test]
    #[serial]
    fn cdc_uk_changed_test() {
        let rt = Runtime::new().unwrap();
        let runner = rt
            .block_on(TestRunner::new("mysql_to_mysql/cdc_uk_changed_test"))
            .unwrap();
        rt.block_on(runner.run_cdc_test(3000, 1000, false)).unwrap();
    }
}