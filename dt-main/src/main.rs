use std::env;

use dt_precheck::{config::task_config::PrecheckTaskConfig, do_precheck};
use dt_task::task_runner::TaskRunner;

#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let task_config = env::args().nth(1).expect("no task_config provided in args");
    if PrecheckTaskConfig::new(&task_config).is_ok() {
        do_precheck(&task_config).await;
    } else {
        let runner = TaskRunner::new(&task_config).unwrap();
        runner.start_task(true).await.unwrap()
    }
}
