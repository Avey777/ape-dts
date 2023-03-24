use crate::{
    common::sql_util::SqlUtil,
    error::Error,
    meta::{
        col_value::ColValue,
        pg::{pg_meta_manager::PgMetaManager, pg_tb_meta::PgTbMeta},
        row_data::RowData,
        row_type::RowType,
    },
    sinker::{rdb_router::RdbRouter, sinker_util::SinkerUtil},
    traits::Sinker,
};
use log::error;
use sqlx::{Pool, Postgres};

use async_trait::async_trait;

#[derive(Clone)]
pub struct PgSinker {
    pub conn_pool: Pool<Postgres>,
    pub meta_manager: PgMetaManager,
    pub router: RdbRouter,
    pub batch_size: usize,
}

#[async_trait]
impl Sinker for PgSinker {
    async fn sink(&mut self, data: Vec<RowData>) -> Result<(), Error> {
        if data.len() == 0 {
            return Ok(());
        }
        self.serial_sink(data).await
    }

    async fn close(&mut self) -> Result<(), Error> {
        if self.conn_pool.is_closed() {
            return Ok(());
        }
        return Ok(self.conn_pool.close().await);
    }

    async fn batch_sink(&mut self, data: Vec<RowData>) -> Result<(), Error> {
        if data.len() == 0 {
            return Ok(());
        }

        match &data[0].row_type {
            RowType::Delete => self.batch_delete(data).await,
            RowType::Insert => self.batch_insert(data).await,
            _ => self.serial_sink(data).await,
        }
    }
}

impl PgSinker {
    async fn serial_sink(&mut self, data: Vec<RowData>) -> Result<(), Error> {
        for row_data in data.iter() {
            let tb_meta = self.get_tb_meta(&row_data).await?;
            let sql_util = SqlUtil::new_for_pg(&tb_meta);

            let (sql, cols, binds) = if row_data.row_type == RowType::Insert {
                self.get_insert_query(&sql_util, &tb_meta, row_data)?
            } else {
                sql_util.get_query_info(&row_data)?
            };
            let query = SqlUtil::create_pg_query(&sql, &cols, &binds, &tb_meta);
            query.execute(&self.conn_pool).await.unwrap();
        }
        Ok(())
    }

    async fn batch_delete(&mut self, data: Vec<RowData>) -> Result<(), Error> {
        let all_count = data.len();
        let mut sinked_count = 0;
        let tb_meta = self.get_tb_meta(&data[0]).await?;
        let sql_util = SqlUtil::new_for_pg(&tb_meta);

        loop {
            let mut batch_size = self.batch_size;
            if all_count - sinked_count < batch_size {
                batch_size = all_count - sinked_count;
            }

            let (sql, cols, binds) =
                sql_util.get_batch_delete_query(&data, sinked_count, batch_size)?;
            let query = SqlUtil::create_pg_query(&sql, &cols, &binds, &tb_meta);
            query.execute(&self.conn_pool).await.unwrap();

            sinked_count += batch_size;
            if sinked_count == all_count {
                break;
            }
        }
        Ok(())
    }

    async fn batch_insert(&mut self, data: Vec<RowData>) -> Result<(), Error> {
        let all_count = data.len();
        let mut sinked_count = 0;
        let tb_meta = self.get_tb_meta(&data[0]).await?;
        let sql_util = SqlUtil::new_for_pg(&tb_meta);

        loop {
            let mut batch_size = self.batch_size;
            if all_count - sinked_count < batch_size {
                batch_size = all_count - sinked_count;
            }

            let (sql, cols, binds) =
                sql_util.get_batch_insert_query(&data, sinked_count, batch_size)?;
            let query = SqlUtil::create_pg_query(&sql, &cols, &binds, &tb_meta);

            let result = query.execute(&self.conn_pool).await;
            if let Err(error) = result {
                error!(
                    "batch insert failed, will insert one by one, schema: {}, tb: {}, error: {}",
                    tb_meta.basic.schema,
                    tb_meta.basic.tb,
                    error.to_string()
                );
                let sub_data = &data[sinked_count..sinked_count + batch_size];
                self.serial_sink(sub_data.to_vec()).await.unwrap();
            }

            sinked_count += batch_size;
            if sinked_count == all_count {
                break;
            }
        }
        Ok(())
    }

    fn get_insert_query<'a>(
        &self,
        sql_util: &SqlUtil,
        tb_meta: &PgTbMeta,
        row_data: &'a RowData,
    ) -> Result<(String, Vec<String>, Vec<Option<&'a ColValue>>), Error> {
        let (mut sql, mut cols, mut binds) = sql_util.get_insert_query(row_data)?;

        let mut placeholder_index = cols.len() + 1;
        let after = row_data.after.as_ref().unwrap();
        let mut set_pairs = Vec::new();
        for col in tb_meta.basic.cols.iter() {
            let set_pair = format!(
                "{}={}",
                col,
                sql_util.get_placeholder(placeholder_index, col)
            );
            set_pairs.push(set_pair);
            cols.push(col.clone());
            binds.push(after.get(col));
            placeholder_index += 1;
        }

        sql = format!(
            "{} ON CONFLICT ({}) DO UPDATE SET {}",
            sql,
            tb_meta.basic.id_cols.join(","),
            set_pairs.join(",")
        );
        Ok((sql, cols, binds))
    }

    #[inline(always)]
    async fn get_tb_meta(&mut self, row_data: &RowData) -> Result<PgTbMeta, Error> {
        SinkerUtil::get_pg_tb_meta(&mut self.meta_manager, &mut self.router, row_data).await
    }
}