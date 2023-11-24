use std::collections::HashMap;

use async_trait::async_trait;
use futures::TryStreamExt;
use sqlx::{MySql, Pool};

use crate::{
    call_batch_fn, close_conn_pool,
    meta_fetcher::mysql::mysql_struct_fetcher::MysqlStructFetcher,
    rdb_query_builder::RdbQueryBuilder,
    sinker::{base_checker::BaseChecker, base_sinker::BaseSinker, rdb_router::RdbRouter},
    Sinker,
};

use dt_common::error::Error;

use dt_meta::{
    ddl_data::DdlData,
    mysql::{mysql_meta_manager::MysqlMetaManager, mysql_tb_meta::MysqlTbMeta},
    row_data::RowData,
};

#[derive(Clone)]
pub struct MysqlChecker {
    pub conn_pool: Pool<MySql>,
    pub meta_manager: MysqlMetaManager,
    pub router: RdbRouter,
    pub batch_size: usize,
}

#[async_trait]
impl Sinker for MysqlChecker {
    async fn sink_dml(&mut self, mut data: Vec<RowData>, batch: bool) -> Result<(), Error> {
        if data.is_empty() {
            return Ok(());
        }

        if !batch {
            self.serial_check(data).await.unwrap();
        } else {
            call_batch_fn!(self, data, Self::batch_check);
        }
        Ok(())
    }

    async fn close(&mut self) -> Result<(), Error> {
        return close_conn_pool!(self);
    }

    async fn sink_ddl(&mut self, data: Vec<DdlData>, _batch: bool) -> Result<(), Error> {
        if data.is_empty() {
            return Ok(());
        }

        self.serial_ddl_check(data).await.unwrap();
        Ok(())
    }
}

impl MysqlChecker {
    async fn serial_check(&mut self, data: Vec<RowData>) -> Result<(), Error> {
        for row_data_src in data.iter() {
            let tb_meta = self.get_tb_meta(row_data_src).await?;
            let query_builder = RdbQueryBuilder::new_for_mysql(&tb_meta);

            let (sql, cols, binds) = query_builder.get_select_query(row_data_src)?;
            let query = query_builder.create_mysql_query(&sql, &cols, &binds);

            let mut rows = query.fetch(&self.conn_pool);
            if let Some(row) = rows.try_next().await.unwrap() {
                let row_data_dst = RowData::from_mysql_row(&row, &tb_meta);
                if !BaseChecker::compare_row_data(row_data_src, &row_data_dst) {
                    BaseChecker::log_diff(row_data_src, &tb_meta.basic);
                }
            } else {
                BaseChecker::log_miss(row_data_src, &tb_meta.basic);
            }
        }
        Ok(())
    }

    async fn batch_check(
        &mut self,
        data: &mut [RowData],
        start_index: usize,
        batch_size: usize,
    ) -> Result<(), Error> {
        let tb_meta = self.get_tb_meta(&data[0]).await?;
        let query_builder = RdbQueryBuilder::new_for_mysql(&tb_meta);

        // build fetch dst sql
        let (sql, cols, binds) =
            query_builder.get_batch_select_query(data, start_index, batch_size)?;
        let query = query_builder.create_mysql_query(&sql, &cols, &binds);

        // fetch dst
        let mut dst_row_data_map = HashMap::new();
        let mut rows = query.fetch(&self.conn_pool);
        while let Some(row) = rows.try_next().await.unwrap() {
            let row_data = RowData::from_mysql_row(&row, &tb_meta);
            let hash_code = row_data.get_hash_code(&tb_meta.basic);
            dst_row_data_map.insert(hash_code, row_data);
        }

        BaseChecker::batch_compare_row_datas(
            data,
            &dst_row_data_map,
            &tb_meta.basic,
            start_index,
            batch_size,
        );
        Ok(())
    }

    async fn serial_ddl_check(&mut self, data: Vec<DdlData>) -> Result<(), Error> {
        for data_src in data {
            if let Some(data_model_src) = data_src.meta {
                let mysql_struct_fetcher = MysqlStructFetcher {
                    conn_pool: self.conn_pool.to_owned(),
                    db: String::from(""),
                    filter: None,
                };
                let model_dst_option = mysql_struct_fetcher
                    .fetch_with_model(&data_model_src)
                    .await
                    .ok()
                    .flatten();

                if let Some(data_model_dst) = model_dst_option {
                    if !BaseChecker::compare_ddl_data(&data_model_src, &data_model_dst) {
                        BaseChecker::log_diff_struct(&data_model_src, &data_model_dst);
                    }
                } else {
                    BaseChecker::log_miss_struct(&data_model_src);
                }
            }
        }
        Ok(())
    }

    #[inline(always)]
    async fn get_tb_meta(&mut self, row_data: &RowData) -> Result<MysqlTbMeta, Error> {
        BaseSinker::get_mysql_tb_meta(&mut self.meta_manager, &mut self.router, row_data).await
    }
}