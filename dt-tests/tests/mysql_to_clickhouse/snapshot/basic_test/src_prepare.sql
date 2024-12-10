DROP DATABASE IF EXISTS test_db_1;

CREATE DATABASE test_db_1;

```
CREATE TABLE test_db_1.one_pk_no_uk ( 
    pk tinyint, 
    tinyint_col tinyint, 
    tinyint_col_unsigned tinyint unsigned, 
    smallint_col smallint DEFAULT NULL, 
    smallint_col_unsigned smallint unsigned DEFAULT NULL, 
    mediumint_col mediumint DEFAULT NULL, 
    mediumint_col_unsigned mediumint unsigned DEFAULT NULL, 
    int_col int DEFAULT NULL, 
    int_col_unsigned int unsigned DEFAULT NULL, 
    bigint_col bigint DEFAULT NULL, 
    bigint_col_unsigned bigint unsigned DEFAULT NULL, 
    decimal_col decimal(10,4) DEFAULT NULL, 
    float_col float(6,2) DEFAULT NULL, 
    double_col double(8,3) DEFAULT NULL, 
    bit_col bit(64) DEFAULT NULL,
    datetime_col datetime(6) DEFAULT NULL, 
    time_col time(6) DEFAULT NULL, 
    date_col date DEFAULT NULL, 
    year_col year DEFAULT NULL, 
    timestamp_col timestamp(6) NULL DEFAULT NULL, 
    char_col char(255) DEFAULT NULL, 
    varchar_col varchar(255) DEFAULT NULL, 
    binary_col binary(255) DEFAULT NULL, 
    varbinary_col varbinary(255) DEFAULT NULL, 
    tinytext_col tinytext, 
    text_col text, 
    mediumtext_col mediumtext, 
    longtext_col longtext, 
    tinyblob_col tinyblob, 
    blob_col blob, 
    mediumblob_col mediumblob, 
    longblob_col longblob, 
    enum_col enum('x-small','small','medium','large','x-large') DEFAULT NULL, 
    set_col set('a','b','c','d','e') DEFAULT NULL, 
    json_col json DEFAULT NULL,
    PRIMARY KEY (pk) ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4; 
```