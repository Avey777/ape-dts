use dt_common::error::Error;

use super::{check_util::CheckUtil, rdb_test_runner::RdbTestRunner};

pub struct RdbCheckTestRunner {
    base: RdbTestRunner,
    dst_check_log_dir: String,
    expect_check_log_dir: String,
}

impl RdbCheckTestRunner {
    pub async fn new(relative_test_dir: &str) -> Result<Self, Error> {
        let base = RdbTestRunner::new_default(relative_test_dir).await.unwrap();
        let version = base.get_dst_mysql_version().await;
        let (expect_check_log_dir, dst_check_log_dir) =
            CheckUtil::get_check_log_dir(&base.base, &version);
        Ok(Self {
            base,
            dst_check_log_dir,
            expect_check_log_dir,
        })
    }

    pub async fn run_check_test(&self) -> Result<(), Error> {
        // clear existed check logs
        CheckUtil::clear_check_log(&self.dst_check_log_dir);

        // prepare src and dst tables
        self.base.execute_test_ddl_sqls().await?;
        self.base.execute_test_dml_sqls().await?;

        // start task
        self.base.base.start_task().await?;

        CheckUtil::validate_check_log(&self.expect_check_log_dir, &self.dst_check_log_dir)?;

        self.base.execute_clean_sqls().await?;

        Ok(())
    }

    pub async fn run_revise_test(&self) -> Result<(), Error> {
        CheckUtil::clear_check_log(&self.dst_check_log_dir);
        self.base.run_snapshot_test(true).await
    }

    pub async fn run_review_test(&self) -> Result<(), Error> {
        CheckUtil::clear_check_log(&self.dst_check_log_dir);
        self.run_check_test().await
    }
}
