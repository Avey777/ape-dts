```
INSERT INTO test_db_1.full_column_type (
    id,
    char_col, char_col_2, character_col, character_col_2,
    varchar_col, varchar_col_2, character_varying_col, character_varying_col_2,
    bpchar_col, bpchar_col_2,
    text_col,
    real_col, float4_col,
    double_precision_col, float8_col,
    numeric_col, numeric_col_2, decimal_col, decimal_col_2,
    smallint_col, int2_col, smallserial_col, serial2_col,
    integer_col, int_col, int4_col, serial_col, serial4_col,
    bigint_col, int8_col, bigserial_col, serial8_col,
    bit_col, bit_col_2, bit_varying_col, bit_varying_col_2, varbit_col, varbit_col_2,
    time_col, time_col_2, time_col_3, time_col_4,
    timez_col, timez_col_2, timez_col_3, timez_col_4,
    timestamp_col, timestamp_col_2, timestamp_col_3, timestamp_col_4,
    timestampz_col, timestampz_col_2, timestampz_col_3, timestampz_col_4,
    date_col,
    bytea_col,
    boolean_col, bool_col,
    json_col, jsonb_col,
    interval_col, interval_col_2,
    array_float4_col, array_float8_col,
    array_int2_col, array_int4_col, array_int8_col, array_int8_col_2,
    array_text_col,
    array_boolean_col, array_boolean_col_2,
    array_date_col,
    array_timestamp_col, array_timestamp_col_2, array_timestamptz_col, array_timestamptz_col_2,
    box_col, cidr_col, circle_col, inet_col,
    line_col, lseg_col, macaddr_col, macaddr8_col, money_col,
    path_col, pg_lsn_col, pg_snapshot_col, polygon_col, point_col,
    tsquery_col, tsvector_col, txid_snapshot_col,
    uuid_col, xml_col
) VALUES (
    1,
    'A', 'FirstChar255', 'B', 'SecondChar255',
    'FirstVarchar', 'FirstVarchar255', 'FirstCharVarying', 'FirstCharVarying255',
    'C', 'BPChar10',
    'FirstTextContent',
    1.1, 2.2,
    3.3, 4.4,
    5.5, 6.6, 7.7, 8.8,
    9, 10, 11, 12,
    13, 14, 15, 16, 17,
    18, 19, 20, 21,
    B'1', B'1000000000', B'1111', B'1111111111', B'1010', B'0101010101',
    '01:02:03', '01:02:03.123456', '01:02:03', '01:02:03.123456',
    '01:02:03+00', '01:02:03.123456+00', '01:02:03+00', '01:02:03.123456+00',
    '2024-01-01 00:00:00', '2024-01-01 00:00:00.123456', '2024-01-01 00:00:00', '2024-01-01 00:00:00.123456',
    '2024-01-01 00:00:00+00', '2024-01-01 00:00:00.123456+00', '2024-01-01 00:00:00+00', '2024-01-01 00:00:00.123456+00',
    '2024-01-01',
    E'\\xdeadbeef',
    TRUE, TRUE,
    '{"key": "value"}'::json, '{"key": "value"}'::jsonb,
    '1 day 2 hours'::interval, '1 year 2 months'::interval,
    ARRAY[1.1, 2.2], ARRAY[3.3, 4.4],
    ARRAY[1, 2], ARRAY[3, 4], ARRAY[5, 6], ARRAY[7, 8],
    ARRAY['FirstText', 'SecondText'],
    ARRAY[TRUE, FALSE], ARRAY[TRUE, FALSE],
    ARRAY['2024-01-01'::date, '2024-01-02'::date],
    ARRAY['2024-01-01 00:00:00'::timestamp, '2024-01-02 00:00:00'::timestamp], ARRAY['2024-01-01 00:00:00.123456'::timestamp, '2024-01-02 00:00:00.123456'::timestamp], ARRAY['2024-01-01 00:00:00+00'::timestamptz, '2024-01-02 00:00:00+00'::timestamptz], ARRAY['2024-01-01 00:00:00.123456+00'::timestamptz, '2024-01-02 00:00:00.123456+00'::timestamptz],
    '(0,0),(1,1)', '192.168.100.128/25', '(0,0),1', '192.168.1.1',
    '[(0,0),(1,1)]', '[(0,0),(1,1)]', '08:00:2f:00:00:00', '08:00:2f:00:00:00:00:00', '92233720368547758.07',
    '((0,0),(1,1))', NULL, NULL, '((0,0),(1,1))', '(0,0)',
    'fat & rat', 'a fat cat sat on a mat', NULL,
    '550e8400-e29b-41d4-a716-446655440000', '<note>a</note>'
);
```

```
INSERT INTO test_db_1.full_column_type (
    id,
    char_col, char_col_2, character_col, character_col_2,
    varchar_col, varchar_col_2, character_varying_col, character_varying_col_2,
    bpchar_col, bpchar_col_2,
    text_col,
    real_col, float4_col,
    double_precision_col, float8_col,
    numeric_col, numeric_col_2, decimal_col, decimal_col_2,
    smallint_col, int2_col, smallserial_col, serial2_col,
    integer_col, int_col, int4_col, serial_col, serial4_col,
    bigint_col, int8_col, bigserial_col, serial8_col,
    bit_col, bit_col_2, bit_varying_col, bit_varying_col_2, varbit_col, varbit_col_2,
    time_col, time_col_2, time_col_3, time_col_4,
    timez_col, timez_col_2, timez_col_3, timez_col_4,
    timestamp_col, timestamp_col_2, timestamp_col_3, timestamp_col_4,
    timestampz_col, timestampz_col_2, timestampz_col_3, timestampz_col_4,
    date_col,
    bytea_col,
    boolean_col, bool_col,
    json_col, jsonb_col,
    interval_col, interval_col_2,
    array_float4_col, array_float8_col,
    array_int2_col, array_int4_col, array_int8_col, array_int8_col_2,
    array_text_col,
    array_boolean_col, array_boolean_col_2,
    array_date_col,
    array_timestamp_col, array_timestamp_col_2, array_timestamptz_col, array_timestamptz_col_2,
    box_col, cidr_col, circle_col, inet_col,
    line_col, lseg_col, macaddr_col, macaddr8_col, money_col,
    path_col, pg_lsn_col, pg_snapshot_col, polygon_col, point_col,
    tsquery_col, tsvector_col, txid_snapshot_col,
    uuid_col, xml_col
) VALUES (
    2,
    NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL,
    NULL, NULL,
    NULL,
    NULL, NULL,
    NULL, NULL,
    NULL, NULL, NULL, NULL,
    NULL, NULL, DEFAULT, DEFAULT,
    NULL, NULL, NULL, DEFAULT, DEFAULT,
    NULL, NULL, DEFAULT, DEFAULT,
    NULL, NULL, NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL,
    NULL,
    NULL,
    NULL, NULL,
    NULL, NULL,
    NULL, NULL,
    NULL, NULL,
    NULL, NULL,
    NULL, NULL, NULL, NULL, NULL,
    NULL,
    NULL, NULL, 
    NULL, 
    NULL, NULL, NULL, NULL, 
    NULL, NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL,
    NULL, NULL);
```