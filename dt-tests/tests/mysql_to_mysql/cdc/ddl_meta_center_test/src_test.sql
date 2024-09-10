
INSERT INTO test_db_1.tb_1 VALUES (1,1);

-- add column
ALTER TABLE test_db_1.tb_1 ADD COLUMN f_2 smallint DEFAULT NULL;
ALTER TABLE test_db_1.tb_1 ADD COLUMN f_3 smallint DEFAULT NULL;

INSERT INTO test_db_1.tb_1 VALUES (2,2,2,2);

-- drop column (breaking change)
INSERT INTO test_db_1.tb_1 VALUES (3,3,3,3);

ALTER TABLE test_db_1.tb_1 DROP COLUMN f_1;
ALTER TABLE test_db_1.tb_1 DROP COLUMN f_2;

INSERT INTO test_db_1.tb_1 VALUES (4,4);

-- truncate table
TRUNCATE test_db_1.truncate_tb_1;
TRUNCATE TABLE test_db_1.truncate_tb_2;
-- truncate table in another database
TRUNCATE TABLE test_db_2.truncate_tb_1; 

-- rename table (breaking change)
INSERT INTO test_db_1.rename_tb_1 VALUES(1, 1);
INSERT INTO test_db_1.rename_tb_2 VALUES(1, 1);
INSERT INTO test_db_1.rename_tb_3 VALUES(1, 1);

ALTER TABLE test_db_1.rename_tb_1 RENAME test_db_1.dst_rename_tb_1;
RENAME TABLE test_db_1.rename_tb_2 TO test_db_1.dst_rename_tb_2, test_db_1.rename_tb_3 TO test_db_1.dst_rename_tb_3;

INSERT INTO test_db_1.dst_rename_tb_1 VALUES(2, 2);
INSERT INTO test_db_1.dst_rename_tb_2 VALUES(2, 2);
INSERT INTO test_db_1.dst_rename_tb_3 VALUES(2, 2);

-- drop table (breaking change)
INSERT INTO test_db_1.drop_tb_1 VALUES(1, 1);

DROP TABLE test_db_1.drop_tb_1;
DROP TABLE IF EXISTS test_db_1.drop_tb_2, test_db_1.drop_tb_3;

-- drop database (breaking change)
INSERT INTO test_db_3.tb_1 VALUES(1, 1);

DROP DATABASE test_db_3;
DROP DATABASE IF EXISTS test_db_3;

-- create database
CREATE DATABASE test_db_4;
CREATE DATABASE IF NOT EXISTS test_db_4;

-- create table (breaking change)
CREATE TABLE test_db_2.tb_1 ( f_0 tinyint, f_1 smallint DEFAULT NULL, f_2 smallint DEFAULT NULL, PRIMARY KEY (f_0) ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4; 

INSERT INTO test_db_2.tb_1 VALUES (1,1,1);

-- add index
ALTER TABLE test_db_2.tb_1 ADD INDEX idx_f_1 (f_1);
CREATE INDEX idx_f_2 ON test_db_2.tb_1 (f_2);

-- drop index
ALTER TABLE test_db_2.tb_1 DROP INDEX idx_f_1;
DROP INDEX idx_f_2 ON test_db_2.tb_1;

-- create database with special character
CREATE DATABASE `中文database!@#$%^&*()_+`;

-- create table with chinese character
CREATE TABLE `中文database!@#$%^&*()_+`.`中文` ( f_0 tinyint, f_1 smallint DEFAULT NULL, f_2 smallint DEFAULT NULL, PRIMARY KEY (f_0) ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4; 

INSERT INTO `中文database!@#$%^&*()_+`.`中文` VALUES(1, 1, 1);