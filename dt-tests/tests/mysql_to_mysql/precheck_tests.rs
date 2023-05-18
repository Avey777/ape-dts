#[cfg(test)]
mod test {

    use std::collections::{HashMap, HashSet};

    use dt_precheck::meta::check_item::CheckItem;
    use serial_test::serial;

    use crate::test_runner::test_base::TestBase;

    #[tokio::test]
    #[serial]
    async fn db_not_exists_test() {
        let test_dir = "mysql_to_mysql/precheck/db_not_exists_test";

        let mut src_expected_results = HashMap::new();
        src_expected_results.insert(CheckItem::CheckIfStructExisted.to_string(), false);

        let mut dst_expected_results = HashMap::new();
        dst_expected_results.insert(CheckItem::CheckIfStructExisted.to_string(), false);

        TestBase::run_precheck_test(
            test_dir,
            &HashSet::new(),
            &src_expected_results,
            &dst_expected_results,
        )
        .await
    }

    #[tokio::test]
    #[serial]
    async fn struct_existed_test() {
        let test_dir = "mysql_to_mysql/precheck/struct_existed_test";

        let mut src_expected_results = HashMap::new();
        src_expected_results.insert(CheckItem::CheckIfStructExisted.to_string(), true);

        let mut dst_expected_results = HashMap::new();
        dst_expected_results.insert(CheckItem::CheckIfStructExisted.to_string(), true);

        TestBase::run_precheck_test(
            test_dir,
            &HashSet::new(),
            &src_expected_results,
            &dst_expected_results,
        )
        .await
    }

    #[tokio::test]
    #[serial]
    async fn struct_supported_basic_test() {
        let test_dir = "mysql_to_mysql/precheck/struct_supported_basic_test";

        let mut src_expected_results = HashMap::new();
        src_expected_results.insert(CheckItem::CheckIfTableStructSupported.to_string(), true);

        let mut dst_expected_results = HashMap::new();
        dst_expected_results.insert(CheckItem::CheckIfTableStructSupported.to_string(), true);

        TestBase::run_precheck_test(
            test_dir,
            &HashSet::new(),
            &src_expected_results,
            &dst_expected_results,
        )
        .await
    }

    #[tokio::test]
    #[serial]
    async fn struct_supported_no_pk_test() {
        let test_dir = "mysql_to_mysql/precheck/struct_supported_no_pk_test";

        let mut src_expected_results = HashMap::new();
        src_expected_results.insert(CheckItem::CheckIfTableStructSupported.to_string(), false);

        let mut dst_expected_results = HashMap::new();
        dst_expected_results.insert(CheckItem::CheckIfTableStructSupported.to_string(), false);

        TestBase::run_precheck_test(
            test_dir,
            &HashSet::new(),
            &src_expected_results,
            &dst_expected_results,
        )
        .await
    }

    #[tokio::test]
    #[serial]
    async fn struct_supported_have_fk_test() {
        let test_dir = "mysql_to_mysql/precheck/struct_supported_have_fk_test";

        let mut src_expected_results = HashMap::new();
        src_expected_results.insert(CheckItem::CheckIfTableStructSupported.to_string(), false);

        let mut dst_expected_results = HashMap::new();
        dst_expected_results.insert(CheckItem::CheckIfTableStructSupported.to_string(), false);

        TestBase::run_precheck_test(
            test_dir,
            &HashSet::new(),
            &src_expected_results,
            &dst_expected_results,
        )
        .await
    }
}
