CREATE EXTENSION IF NOT EXISTS plpgsql;
CREATE EXTENSION IF NOT EXISTS ltree;
CREATE EXTENSION IF NOT EXISTS isn;
CREATE EXTENSION IF NOT EXISTS citext;
CREATE EXTENSION IF NOT EXISTS hstore;
CREATE EXTENSION IF NOT EXISTS postgis;

DROP SCHEMA IF EXISTS lua_test_expect CASCADE;
CREATE SCHEMA lua_test_expect;

DROP TABLE IF EXISTS lua_test_expect.default_table;
CREATE TABLE lua_test_expect.default_table(pk serial, val numeric(20,8), created_at timestamp, created_at_tz timestamptz, ctime time , ctime_tz timetz , cdate date , cmoney money , cbits bit(3) , csmallint smallint , cinteger integer , cbigint bigint , creal real , cbool bool , cfloat8 float8 , cnumeric numeric(6,2) , cvarchar varchar(5) , cbox box , ccircle circle , cinterval interval , cline line , clseg lseg , cpath path , cpoint point , cpolygon polygon , cchar char , ctext text , cjson json , cxml xml , cuuid uuid , cvarbit varbit(3) , cinet inet , ccidr cidr , cmacaddr macaddr , PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.numeric_table;
CREATE TABLE lua_test_expect.numeric_table (pk SERIAL, si SMALLINT, i INTEGER, bi BIGINT, r REAL, db DOUBLE PRECISION, r_int REAL, db_int DOUBLE PRECISION, r_nan REAL, db_nan DOUBLE PRECISION, r_pinf REAL, db_pinf DOUBLE PRECISION, r_ninf REAL, db_ninf DOUBLE PRECISION, ss SMALLSERIAL, bs BIGSERIAL, b BOOLEAN, o OID, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.numeric_decimal_table;
CREATE TABLE lua_test_expect.numeric_decimal_table (pk SERIAL, d DECIMAL(3,2), dzs DECIMAL(4), dvs DECIMAL, d_nn DECIMAL(3,2), n NUMERIC(6,4), nzs NUMERIC(4), nvs NUMERIC, d_int DECIMAL(3,2), dzs_int DECIMAL(4), dvs_int DECIMAL, n_int NUMERIC(6,4), nzs_int NUMERIC(4), nvs_int NUMERIC, d_nan DECIMAL(3,2), dzs_nan DECIMAL(4), dvs_nan DECIMAL, n_nan NUMERIC(6,4), nzs_nan NUMERIC(4), nvs_nan NUMERIC, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.string_table;
CREATE TABLE lua_test_expect.string_table (pk SERIAL, vc VARCHAR(2), vcv CHARACTER VARYING(2), ch CHARACTER(4), c CHAR(3), t TEXT, b BYTEA, bnn BYTEA , ct CITEXT, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.network_address_table;
CREATE TABLE lua_test_expect.network_address_table (pk SERIAL, i INET, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.cidr_network_address_table;
CREATE TABLE lua_test_expect.cidr_network_address_table (pk SERIAL, i CIDR, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.macaddr_table;
CREATE TABLE lua_test_expect.macaddr_table(pk SERIAL, m MACADDR, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.cash_table;
CREATE TABLE lua_test_expect.cash_table (pk SERIAL, csh MONEY, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.bitbin_table;
CREATE TABLE lua_test_expect.bitbin_table (pk SERIAL, ba BYTEA, bol BIT(1), bol2 BIT, bs BIT(2), bs7 BIT(7), bv BIT VARYING(2), bv2 BIT VARYING(24), bvl BIT VARYING(64), bvunlimited1 BIT VARYING, bvunlimited2 BIT VARYING, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.bytea_binmode_table;
CREATE TABLE lua_test_expect.bytea_binmode_table (pk SERIAL, ba BYTEA, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.time_table;
CREATE TABLE lua_test_expect.time_table (pk SERIAL, ts TIMESTAMP, tsneg TIMESTAMP(6) WITHOUT TIME ZONE, ts_ms TIMESTAMP(3), ts_us TIMESTAMP(6), tz TIMESTAMPTZ, date DATE, date_pinf DATE, date_ninf DATE, ti TIME, tip TIME(3), ttf TIME, ttz TIME WITH TIME ZONE, tptz TIME(3) WITH TIME ZONE, it INTERVAL, tsp TIMESTAMP (0) WITH TIME ZONE, ts_large TIMESTAMP, ts_large_us TIMESTAMP(6), ts_large_ms TIMESTAMP(3), tz_large TIMESTAMPTZ, ts_max TIMESTAMP(6), ts_min TIMESTAMP(6), tz_max TIMESTAMPTZ, tz_min TIMESTAMPTZ, ts_pinf TIMESTAMP(6), ts_ninf TIMESTAMP(6), tz_pinf TIMESTAMPTZ, tz_ninf TIMESTAMPTZ, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.text_table;
CREATE TABLE lua_test_expect.text_table (pk SERIAL, j JSON, jb JSONB, x XML, u Uuid, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.geom_table;
CREATE TABLE lua_test_expect.geom_table (pk SERIAL, p POINT, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.range_table;
CREATE TABLE lua_test_expect.range_table (pk SERIAL, unbounded_exclusive_tsrange TSRANGE, bounded_inclusive_tsrange TSRANGE, unbounded_exclusive_tstzrange TSTZRANGE, bounded_inclusive_tstzrange TSTZRANGE, unbounded_exclusive_daterange DATERANGE, bounded_exclusive_daterange DATERANGE, int4_number_range INT4RANGE, numerange NUMRANGE, int8_number_range INT8RANGE, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.array_table;
CREATE TABLE lua_test_expect.array_table (pk SERIAL, int_array INT[], bigint_array BIGINT[], text_array TEXT[], char_array CHAR(10)[], varchar_array VARCHAR(10)[], date_array DATE[], numeric_array NUMERIC(10, 2)[], varnumeric_array NUMERIC[3], citext_array CITEXT[], inet_array INET[], cidr_array CIDR[], macaddr_array MACADDR[], tsrange_array TSRANGE[], tstzrange_array TSTZRANGE[], daterange_array DATERANGE[], int4range_array INT4RANGE[],numerange_array NUMRANGE[], int8range_array INT8RANGE[], uuid_array UUID[], json_array json[], jsonb_array jsonb[], oid_array OID[], PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.custom_table;
CREATE TABLE lua_test_expect.custom_table (pk serial, lt ltree, i isbn , n TEXT, lt_array ltree[], PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.hstore_table;
CREATE TABLE lua_test_expect.hstore_table (pk serial, hs hstore, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.hstore_table_mul;
CREATE TABLE lua_test_expect.hstore_table_mul (pk serial, hs hstore, hsarr hstore[], PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.hstore_table_with_special;
CREATE TABLE lua_test_expect.hstore_table_with_special (pk serial, hs hstore, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.circle_table;
CREATE TABLE lua_test_expect.circle_table (pk serial, ccircle circle, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.macaddr8_table;
CREATE TABLE lua_test_expect.macaddr8_table (pk SERIAL, m MACADDR8, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.postgis_table;
CREATE TABLE lua_test_expect.postgis_table (pk SERIAL, p GEOMETRY(POINT,3187), ml GEOGRAPHY(MULTILINESTRING), PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.postgis_array_table;
CREATE TABLE lua_test_expect.postgis_array_table (pk SERIAL, ga GEOMETRY[], gann GEOMETRY[] , PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.timezone_table;
CREATE TABLE lua_test_expect.timezone_table (pk SERIAL, t1 time, t2 timetz, t3 timestamp, t4 timestamptz, PRIMARY KEY(pk));

DROP TABLE IF EXISTS lua_test_expect.col_has_special_character_table;
CREATE TABLE lua_test_expect.col_has_special_character_table ("p:k" SERIAL, "col`1" text, "col,2" text, "col\3" text, PRIMARY KEY("p:k"));