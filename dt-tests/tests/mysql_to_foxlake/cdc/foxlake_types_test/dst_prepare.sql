DROP TABLE IF EXISTS sync_db_test_types.test_types;

```
CREATE TABLE sync_db_test_types.`test_types` (
  `c_pk` bigint unsigned NOT NULL AUTO_INCREMENT,
  `c_type` varchar(255) DEFAULT NULL,
  `c_bit` bit(1) DEFAULT NULL,
  `c_bit_64` bit(64) DEFAULT NULL,
  `c_bool` tinyint(1) DEFAULT NULL,
  `c_tinyint` tinyint DEFAULT NULL,
  `c_smallint` smallint DEFAULT NULL,
  `c_mediumint` mediumint DEFAULT NULL,
  `c_int` int DEFAULT NULL,
  `c_bigint` bigint DEFAULT NULL,
  `c_utinyint` tinyint unsigned DEFAULT NULL,
  `c_usmallint` smallint unsigned DEFAULT NULL,
  `c_umediumint` mediumint unsigned DEFAULT NULL,
  `c_uint` int unsigned DEFAULT NULL,
  `c_ubigint` bigint unsigned DEFAULT NULL,
  `c_decimal_65_0` decimal(65,0) DEFAULT NULL,
  `c_decimal_65_30` decimal(65,30) DEFAULT NULL,
  `c_float` double DEFAULT NULL,
  `c_double` double DEFAULT NULL,
  `c_date` date DEFAULT NULL,
  `c_datetime` datetime DEFAULT NULL,
  `c_datetime_6` datetime(6) DEFAULT NULL,
  `c_timestamp` timestamp NULL DEFAULT NULL,
  `c_timestamp_6` timestamp(6) NULL DEFAULT NULL,
  `c_time` time DEFAULT NULL,
  `c_time_6` time(6) DEFAULT NULL,
  `c_year` year DEFAULT NULL,
  `c_char_255` char(255) DEFAULT NULL,
  `c_binary_255` binary(255) DEFAULT NULL,
  `c_varchar_5000` varchar(5000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL,
  `c_varbinary_5000` varbinary(5000) DEFAULT NULL,
  `c_tinyblob` tinyblob,
  `c_blob` blob,
  `c_mediumblob` mediumblob,
  `c_longblob` longblob,
  `c_tinytext` tinytext,
  `c_text` text,
  `c_mediumtext` mediumtext,
  `c_longtext` longtext,
  `c_enum` enum('value1','value2') DEFAULT NULL,
  `c_set` set('value1','value2') DEFAULT NULL,
  `c_json` json DEFAULT NULL,
  PRIMARY KEY (`c_pk`)
) AUTO_INCREMENT=394 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci
```