# Postgres -> ClickHouse templates

Refer to [config details](/docs/en/config.md) for explanations of common fields.

# Struct
```
[extractor]
extract_type=struct
db_type=pg
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s

[sinker]
sink_type=struct
db_type=clickhouse
url=http://admin:123456@127.0.0.1:8123
conflict_policy=interrupt

[filter]
do_dbs=test_db
ignore_dbs=
do_tbs=
ignore_tbs=
do_events=
do_structures=

[router]
db_map=
tb_map=
col_map=

[runtime]
log_level=info
log4rs_file=./log4rs.yaml
log_dir=./logs

[parallelizer]
parallel_type=serial

[pipeline]
checkpoint_interval_secs=10
buffer_size=100
```

# Snapshot
```
[extractor]
db_type=pg
extract_type=snapshot
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s
batch_size=10000

[sinker]
db_type=clickhouse
sink_type=write
url=http://admin:123456@127.0.0.1:8123
conflict_policy=interrupt
batch_size=5000

[filter]
do_dbs=
ignore_dbs=
do_tbs=test_db.*
ignore_tbs=
do_events=insert

[router]
db_map=
tb_map=
col_map=

[parallelizer]
parallel_type=snapshot
parallel_size=8

[pipeline]
buffer_size=100000
buffer_memory_mb=200
checkpoint_interval_secs=10

[runtime]
log_level=info
log4rs_file=./log4rs.yaml
log_dir=./logs
```

# CDC
```
[extractor]
db_type=pg
extract_type=cdc
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s
start_lsn=0/406DE430
slot_name=ape_test

[sinker]
db_type=clickhouse
sink_type=write
url=http://admin:123456@127.0.0.1:8123
conflict_policy=interrupt
batch_size=5000

[filter]
ignore_dbs=
do_dbs=
do_tbs=test_db.*
ignore_tbs=
do_events=insert,update,delete

[router]
tb_map=
col_map=
db_map=

[parallelizer]
parallel_type=table
parallel_size=8

[pipeline]
buffer_size=100000
buffer_memory_mb=200
checkpoint_interval_secs=10

[runtime]
log_dir=./logs
log_level=info
log4rs_file=./log4rs.yaml
```