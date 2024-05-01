-- From https://clickhouse.com/docs/en/getting-started/quick-start
CREATE TABLE my_first_table
(
    `user_id` UInt32,
    `message` String,
    `timestamp` DateTime,
    `metric` Float32
)
ENGINE = MergeTree
PRIMARY KEY (user_id, timestamp)
;

INSERT INTO my_first_table (user_id, message, timestamp, metric) VALUES (101, 'Hello, ClickHouse!', now(), -1.0), (102, 'Insert a lot of rows per batch', yesterday(), 1.41421 ), (102, 'Sort your data based on your commonly-used queries', today(), 2.718 ), (101, 'Granules are the smallest chunks of data read',now() + 5, 3.14159 );

SELECT * FROM my_first_table ORDER BY timestamp ASC;

SELECT
    passenger_count,
    avg(toFloat32(total_amount))
FROM s3('https://datasets-documentation.s3.eu-west-3.amazonaws.com/nyc-taxi/trips_0.gz', 'TabSeparatedWithNames')
GROUP BY passenger_count
ORDER BY passenger_count ASC
;

CREATE TABLE postgres
(
    `user_id` UInt32
)
ENGINE = PostgreSQL(creds, database = postgres, `table` = postgres)
PRIMARY KEY (user_id, timestamp)
;

CREATE TABLE postgres
(
    `user_id` UInt32
)
ENGINE = PostgreSQL(creds, database = postgres, `table` = postgres)
;

