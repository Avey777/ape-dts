use super::redis_client::RedisClient;
use super::redis_psync_extractor::RedisPsyncExtractor;
use super::redis_resp_types::Value;
use crate::extractor::base_extractor::BaseExtractor;
use crate::Extractor;
use async_trait::async_trait;
use dt_common::config::config_enums::DbType;
use dt_common::config::config_token_parser::ConfigTokenParser;
use dt_common::error::Error;
use dt_common::log_error;
use dt_common::log_info;
use dt_common::log_warn;
use dt_common::utils::rdb_filter::RdbFilter;
use dt_common::utils::sql_util::SqlUtil;
use dt_common::utils::time_util::TimeUtil;
use dt_meta::position::Position;
use dt_meta::redis::redis_entry::RedisEntry;
use dt_meta::redis::redis_object::RedisCmd;
use dt_meta::syncer::Syncer;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub struct RedisCdcExtractor {
    pub base_extractor: BaseExtractor,
    pub conn: RedisClient,
    pub run_id: String,
    pub repl_offset: u64,
    pub repl_port: u64,
    pub now_db_id: i64,
    pub keepalive_interval_secs: u64,
    pub heartbeat_interval_secs: u64,
    pub heartbeat_key: String,
    pub syncer: Arc<Mutex<Syncer>>,
    pub filter: RdbFilter,
}

#[async_trait]
impl Extractor for RedisCdcExtractor {
    async fn extract(&mut self) -> Result<(), Error> {
        log_info!(
            "RedisCdcExtractor starts, keepalive_interval_secs: {}, heartbeat_interval_secs: {}, heartbeat_key: {}",
            self.keepalive_interval_secs,
            self.heartbeat_interval_secs,
            self.heartbeat_key
        );

        let mut psync_extractor = RedisPsyncExtractor {
            base_extractor: &mut self.base_extractor,
            conn: &mut self.conn,
            run_id: self.run_id.clone(),
            repl_offset: self.repl_offset,
            repl_port: self.repl_port,
            now_db_id: self.now_db_id,
            filter: self.filter.clone(),
        };

        // receive rdb data if needed
        psync_extractor.extract().await?;
        self.run_id = psync_extractor.run_id;
        self.repl_offset = psync_extractor.repl_offset;

        self.receive_aof().await
    }

    async fn close(&mut self) -> Result<(), Error> {
        self.conn.close().await
    }
}

impl RedisCdcExtractor {
    async fn receive_aof(&mut self) -> Result<(), Error> {
        let heartbeat_db_key = ConfigTokenParser::parse(
            &self.heartbeat_key,
            &vec!['.'],
            &SqlUtil::get_escape_pairs(&DbType::Redis),
        );
        let heartbeat_db_id = if heartbeat_db_key.len() == 2 {
            heartbeat_db_key[0].parse().unwrap()
        } else {
            i64::MIN
        };

        // start hearbeat
        if heartbeat_db_key.len() == 2 {
            self.start_heartbeat(heartbeat_db_id, &heartbeat_db_key[1])
                .await
                .unwrap();
        } else {
            log_warn!("heartbeat disabled, heartbeat_tb should be like db.key");
        }

        let mut heartbeat_timestamp = String::new();
        let mut start_time = Instant::now();
        loop {
            // heartbeat
            if start_time.elapsed().as_secs() >= self.keepalive_interval_secs {
                self.keep_alive_ack().await?;
                start_time = Instant::now();
            }

            let (value, n) = self.conn.read_with_len().await.unwrap();
            if Value::Nil == value {
                continue;
            }

            self.repl_offset += n as u64;
            let cmd = self.handle_redis_value(value).await.unwrap();

            if !cmd.args.is_empty() {
                // switch db
                if cmd.get_name().eq_ignore_ascii_case("select") {
                    self.now_db_id = String::from_utf8(cmd.args[1].clone())
                        .unwrap()
                        .parse::<i64>()
                        .unwrap();
                    continue;
                }

                // get timestamp generated by heartbeat
                if self.now_db_id == heartbeat_db_id
                    && cmd
                        .get_str_arg(1)
                        .eq_ignore_ascii_case(&heartbeat_db_key[1])
                {
                    heartbeat_timestamp = cmd.get_str_arg(2);
                    continue;
                }

                // build entry and push it to buffer
                let mut entry = RedisEntry::new();
                entry.cmd = cmd;
                entry.db_id = self.now_db_id;
                let position = Position::Redis {
                    run_id: self.run_id.clone(),
                    repl_offset: self.repl_offset,
                    now_db_id: self.now_db_id,
                    timestamp: heartbeat_timestamp.clone(),
                };

                RedisPsyncExtractor::push_to_buf(
                    &mut self.base_extractor,
                    &mut self.filter,
                    entry,
                    position,
                )
                .await
                .unwrap();
            }
        }
    }

    async fn handle_redis_value(&mut self, value: Value) -> Result<RedisCmd, Error> {
        let mut cmd = RedisCmd::new();
        match value {
            Value::Bulk(values) => {
                for v in values {
                    match v {
                        Value::Data(data) => cmd.add_arg(data),
                        _ => {
                            log_error!("received unexpected value in aof bulk: {:?}", v);
                            break;
                        }
                    }
                }
            }
            v => {
                return Err(Error::RedisRdbError(format!(
                    "received unexpected aof value: {:?}",
                    v
                )));
            }
        }
        Ok(cmd)
    }

    async fn keep_alive_ack(&mut self) -> Result<(), Error> {
        // send replconf ack to keep the connection alive
        let mut position_repl_offset = self.repl_offset;
        if let Position::Redis { repl_offset, .. } = self.syncer.lock().unwrap().committed_position
        {
            if repl_offset >= self.repl_offset {
                position_repl_offset = repl_offset
            }
        }

        let repl_offset = &position_repl_offset.to_string();
        let args = vec!["replconf", "ack", repl_offset];
        let cmd = RedisCmd::from_str_args(&args);
        log_info!("replconf ack cmd: {}", cmd.to_string());
        if let Err(err) = self.conn.send(&cmd).await {
            log_error!("replconf ack failed, error: {:?}", err);
        }
        Ok(())
    }

    async fn start_heartbeat(&self, db_id: i64, key: &str) -> Result<(), Error> {
        log_info!(
            "try starting heartbeat, heartbeat_interval_secs: {}, db_id: {}, key: {}",
            self.heartbeat_interval_secs,
            db_id,
            key
        );

        if self.heartbeat_interval_secs == 0 || db_id == i64::MIN || key.is_empty() {
            log_warn!("heartbeat disabled, heartbeat_tb should be like db_id.key");
            return Ok(());
        }

        let mut conn = RedisClient::new(&self.conn.url).await?;
        let heartbeat_interval_secs = self.heartbeat_interval_secs;
        let key = key.to_string();

        let _ = tokio::spawn(async move {
            // set db
            let cmd = RedisCmd::from_str_args(&vec!["SELECT", &db_id.to_string()]);
            if let Err(err) = conn.send(&cmd).await {
                log_error!(
                    "heartbeat failed, cmd: {}, error: {:?}",
                    cmd.to_string(),
                    err
                );
            }

            let mut start_time = Instant::now();
            loop {
                if start_time.elapsed().as_secs() >= heartbeat_interval_secs {
                    Self::heartbeat(&key, &mut conn).await.unwrap();
                    start_time = Instant::now();
                }
                TimeUtil::sleep_millis(1000 * heartbeat_interval_secs).await;
            }
        });
        log_info!("heartbeat started");
        Ok(())
    }

    async fn heartbeat(key: &str, conn: &mut RedisClient) -> Result<(), Error> {
        // send `SET heartbeat_key current_timestamp` by another connecion to generate timestamp
        let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let timestamp =
            since_epoch.as_secs() * 1000 + since_epoch.subsec_nanos() as u64 / 1_000_000;
        let heartbeat_value = Position::format_timestamp_millis(timestamp as i64);

        let cmd = RedisCmd::from_str_args(&vec!["SET", key, &heartbeat_value]);
        log_info!("heartbeat cmd: {}", cmd.to_string());
        if let Err(err) = conn.send(&cmd).await {
            log_error!("heartbeat failed, error: {:?}", err);
        }
        Ok(())
    }
}
