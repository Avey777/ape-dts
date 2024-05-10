use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use mongodb::bson::Document;
use serde::{Deserialize, Serialize, Serializer};

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "type", content = "value")]
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[allow(dead_code)]
pub enum ColValue {
    None,
    Bool(bool),
    Tiny(i8),
    UnsignedTiny(u8),
    Short(i16),
    UnsignedShort(u16),
    Long(i32),
    UnsignedLong(u32),
    LongLong(i64),
    UnsignedLongLong(u64),
    Float(f32),
    Double(f64),
    Decimal(String),
    Time(String),
    Date(String),
    DateTime(String),
    Timestamp(String),
    Year(u16),
    String(String),
    RawString(Vec<u8>),
    Blob(Vec<u8>),
    Bit(u64),
    Set(u64),
    Enum(u32),
    Set2(String),
    Enum2(String),
    Json(Vec<u8>),
    Json2(String),
    MongoDoc(Document),
}

impl std::fmt::Display for ColValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.to_option_string().unwrap_or("NULL".to_string())
        )
    }
}

impl ColValue {
    pub fn hash_code(&self) -> u64 {
        match self {
            ColValue::None => 0,
            _ => {
                let mut hasher = DefaultHasher::new();
                self.to_option_string().hash(&mut hasher);
                hasher.finish()
            }
        }
    }

    /// return: (str, is_hex_str)
    pub fn to_mysql_string(&self) -> (Option<String>, bool) {
        match self {
            // varchar, char, tinytext, mediumtext, longtext, text
            ColValue::RawString(v) => {
                let (str, is_hex_str) = Self::binary_to_str(v);
                (Some(str), is_hex_str)
            }

            // tinyblob, mediumblob, longblob, blob, varbinary, binary
            ColValue::Blob(v) => (Some(Self::binary_to_hex_str(v)), true),

            _ => (self.to_option_string(), false),
        }
    }

    pub fn to_option_string(&self) -> Option<String> {
        match self {
            ColValue::Tiny(v) => Some(v.to_string()),
            ColValue::UnsignedTiny(v) => Some(v.to_string()),
            ColValue::Short(v) => Some(v.to_string()),
            ColValue::UnsignedShort(v) => Some(v.to_string()),
            ColValue::Long(v) => Some(v.to_string()),
            ColValue::UnsignedLong(v) => Some(v.to_string()),
            ColValue::LongLong(v) => Some(v.to_string()),
            ColValue::UnsignedLongLong(v) => Some(v.to_string()),
            ColValue::Float(v) => Some(v.to_string()),
            ColValue::Double(v) => Some(v.to_string()),
            ColValue::Decimal(v) => Some(v.to_string()),
            ColValue::Time(v) => Some(v.to_string()),
            ColValue::Date(v) => Some(v.to_string()),
            ColValue::DateTime(v) => Some(v.to_string()),
            ColValue::Timestamp(v) => Some(v.to_string()),
            ColValue::Year(v) => Some(v.to_string()),
            ColValue::String(v) => Some(v.to_string()),
            ColValue::RawString(v) => Some(Self::binary_to_str(v).0),
            ColValue::Bit(v) => Some(v.to_string()),
            ColValue::Set(v) => Some(v.to_string()),
            ColValue::Set2(v) => Some(v.to_string()),
            ColValue::Enum(v) => Some(v.to_string()),
            ColValue::Enum2(v) => Some(v.to_string()),
            ColValue::Json(v) => Some(format!("{:?}", v)),
            ColValue::Json2(v) => Some(v.to_string()),
            ColValue::Blob(v) => Some(Self::binary_to_str(v).0),
            ColValue::MongoDoc(v) => Some(v.to_string()),
            ColValue::Bool(v) => Some(v.to_string()),
            ColValue::None => Option::None,
        }
    }

    /// return: (str, is_hex_str)
    fn binary_to_str(v: &[u8]) -> (String, bool) {
        if let Ok(str) = String::from_utf8(v.to_owned()) {
            (str, false)
        } else {
            // charsets like: gbk, big5, ujis, euckr
            (Self::binary_to_hex_str(v), true)
        }
    }

    fn binary_to_hex_str(v: &[u8]) -> String {
        let hex_str = v
            .iter()
            .fold(String::new(), |hex_str, &x| format!("{hex_str}{:02X}", x));
        format!("x'{}'", hex_str)
    }

    pub fn is_nan(&self) -> bool {
        match &self {
            ColValue::Float(v) => v.is_nan(),
            ColValue::Double(v) => v.is_nan(),
            _ => false,
        }
    }

    pub fn get_malloc_size(&self) -> usize {
        match self {
            ColValue::Tiny(_) | ColValue::UnsignedTiny(_) | ColValue::Bool(_) => 1,
            ColValue::Short(_) | ColValue::UnsignedShort(_) | ColValue::Year(_) => 2,
            ColValue::Long(_)
            | ColValue::UnsignedLong(_)
            | ColValue::Float(_)
            | ColValue::Enum(_) => 4,
            ColValue::LongLong(_)
            | ColValue::UnsignedLongLong(_)
            | ColValue::Double(_)
            | ColValue::Bit(_)
            | ColValue::Set(_) => 8,
            ColValue::Decimal(v)
            | ColValue::Time(v)
            | ColValue::Date(v)
            | ColValue::DateTime(v)
            | ColValue::Timestamp(v)
            | ColValue::String(v)
            | ColValue::Set2(v)
            | ColValue::Enum2(v)
            | ColValue::Json2(v) => v.len(),
            ColValue::Json(v) | ColValue::Blob(v) | ColValue::RawString(v) => v.len(),
            ColValue::MongoDoc(v) => v.to_string().len(),
            ColValue::None => 0,
        }
    }
}

impl Serialize for ColValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // serde json serializer
        // case 1: #[derive(Serialize)]
        //   output: {"title":{"String":"C++ primer"},"author":{"String":"avc"}}
        // case 2: #[derive(Serialize)]
        //         #[serde(tag = "type", content = "value")]
        //   output: {"title":{"type":"String","value":"C++ primer"},"author":{"type":"String","value":"avc"}}
        // case 3: this impl
        //   output: {"title":"C++ primer","author":"avc"}
        match self {
            ColValue::Bool(v) => serializer.serialize_bool(*v),
            ColValue::Tiny(v) => serializer.serialize_i8(*v),
            ColValue::UnsignedTiny(v) => serializer.serialize_u8(*v),
            ColValue::Short(v) => serializer.serialize_i16(*v),
            ColValue::UnsignedShort(v) => serializer.serialize_u16(*v),
            ColValue::Long(v) => serializer.serialize_i32(*v),
            ColValue::UnsignedLong(v) => serializer.serialize_u32(*v),
            ColValue::LongLong(v) => serializer.serialize_i64(*v),
            ColValue::UnsignedLongLong(v) => serializer.serialize_u64(*v),
            ColValue::Float(v) => serializer.serialize_f32(*v),
            ColValue::Double(v) => serializer.serialize_f64(*v),
            ColValue::Decimal(v) => serializer.serialize_str(v),
            ColValue::Time(v) => serializer.serialize_str(v),
            ColValue::Date(v) => serializer.serialize_str(v),
            ColValue::DateTime(v) => serializer.serialize_str(v),
            ColValue::Timestamp(v) => serializer.serialize_str(v),
            ColValue::Year(v) => serializer.serialize_u16(*v),
            ColValue::String(v) => serializer.serialize_str(v),
            ColValue::RawString(v) => serializer.serialize_bytes(v),
            ColValue::Blob(v) => serializer.serialize_bytes(v),
            ColValue::Bit(v) => serializer.serialize_u64(*v),
            ColValue::Set(v) => serializer.serialize_u64(*v),
            ColValue::Set2(v) => serializer.serialize_str(v),
            ColValue::Enum(v) => serializer.serialize_u32(*v),
            ColValue::Enum2(v) => serializer.serialize_str(v),
            ColValue::Json(v) => serializer.serialize_bytes(v),
            ColValue::Json2(v) => serializer.serialize_str(v),
            // not supported
            ColValue::MongoDoc(_) => serializer.serialize_none(),
            ColValue::None => serializer.serialize_none(),
        }
    }
}
