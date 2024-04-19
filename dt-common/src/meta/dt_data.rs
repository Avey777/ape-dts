use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::meta::{position::Position, redis::redis_entry::RedisEntry};

use super::{ddl_data::DdlData, row_data::RowData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtItem {
    pub dt_data: DtData,
    pub position: Position,
    pub data_origin_node: String,
}

impl DtItem {
    pub fn is_ddl(&self) -> bool {
        self.dt_data.is_ddl()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DtData {
    Ddl {
        ddl_data: DdlData,
    },
    Dml {
        row_data: RowData,
    },
    Begin {},
    Commit {
        xid: String,
    },
    #[serde(skip)]
    Redis {
        entry: RedisEntry,
    },
}

impl DtData {
    pub fn is_begin(&self) -> bool {
        matches!(self, DtData::Begin { .. })
    }

    pub fn is_commit(&self) -> bool {
        matches!(self, DtData::Commit { .. })
    }

    pub fn is_ddl(&self) -> bool {
        matches!(self, DtData::Ddl { .. })
    }

    pub fn get_data_size(&self) -> usize {
        match &self {
            DtData::Dml { row_data } => row_data.data_size,
            DtData::Redis { entry } => entry.data_size,
            // ignore other item types
            _ => 0,
        }
    }
}

impl ToString for DtData {
    fn to_string(&self) -> String {
        json!(self).to_string()
    }
}