use std::error::Error;

use irodori_migration::{
    chunk_checksum_select_sql, CanonicalColumn, CanonicalType, ChecksumAggregate, ChecksumFunction,
    ChunkBounds, ChunkChecksumConfig, MigrationEngine,
};
use mysql::prelude::Queryable;
use testcontainers_modules::{
    mysql::Mysql, postgres::Postgres, testcontainers::runners::SyncRunner,
};

type TestResult = Result<(), Box<dyn Error + Send + Sync + 'static>>;

#[test]
#[ignore = "requires Docker; run in CI with --ignored --test-threads=1"]
fn postgres_chunk_checksum_sql_executes_in_container() -> TestResult {
    let node = Postgres::default().start()?;
    let connection_string = format!(
        "postgres://postgres:postgres@{}:{}/postgres",
        node.get_host()?,
        node.get_host_port_ipv4(5432)?
    );
    let mut client = postgres::Client::connect(&connection_string, postgres::NoTls)?;

    client.batch_execute(
        "
        CREATE TABLE public.irodori_container_orders (
          order_id BIGINT PRIMARY KEY,
          amount NUMERIC(12,2),
          status TEXT
        );
        INSERT INTO public.irodori_container_orders(order_id, amount, status)
        VALUES (1, 10.25, 'paid'), (2, 0.00, NULL), (3, 7.50, 'open');
        ",
    )?;

    let sql = chunk_checksum_select_sql(
        MigrationEngine::Postgres,
        &ChunkChecksumConfig::new(
            "public.irodori_container_orders",
            vec![
                CanonicalColumn::new("order_id", CanonicalType::Integer),
                CanonicalColumn::new("amount", CanonicalType::Decimal { scale: 2 }),
                CanonicalColumn::new("status", CanonicalType::Text),
            ],
        )
        .with_bounds(ChunkBounds {
            column: "order_id".to_string(),
            lower: Some("1".to_string()),
            upper: Some("4".to_string()),
            include_upper: false,
        })
        .with_function(ChecksumFunction::Md5)
        .with_aggregate(ChecksumAggregate::Sum),
    );

    client.simple_query(&sql)?;
    Ok(())
}

#[test]
#[ignore = "requires Docker; run in CI with --ignored --test-threads=1"]
fn mysql_chunk_checksum_sql_executes_in_container() -> TestResult {
    let node = Mysql::default().start()?;
    let connection_string = format!(
        "mysql://root@{}:{}/test",
        node.get_host()?,
        node.get_host_port_ipv4(3306)?
    );
    let mut conn = mysql::Conn::new(mysql::Opts::from_url(&connection_string)?)?;

    conn.query_drop(
        "CREATE TABLE irodori_container_orders (
          order_id BIGINT PRIMARY KEY,
          amount DECIMAL(12,2),
          status VARCHAR(32)
        )",
    )?;
    conn.query_drop(
        "
        INSERT INTO irodori_container_orders(order_id, amount, status)
        VALUES (1, 10.25, 'paid'), (2, 0.00, NULL), (3, 7.50, 'open')
        ",
    )?;

    let sql = chunk_checksum_select_sql(
        MigrationEngine::MySql,
        &ChunkChecksumConfig::new(
            "irodori_container_orders",
            vec![
                CanonicalColumn::new("order_id", CanonicalType::Integer),
                CanonicalColumn::new("amount", CanonicalType::Decimal { scale: 2 }),
                CanonicalColumn::new("status", CanonicalType::Text),
            ],
        )
        .with_bounds(ChunkBounds {
            column: "order_id".to_string(),
            lower: Some("1".to_string()),
            upper: Some("4".to_string()),
            include_upper: false,
        })
        .with_function(ChecksumFunction::Crc32)
        .with_aggregate(ChecksumAggregate::BitXor),
    );

    conn.query_drop(sql)?;
    Ok(())
}
