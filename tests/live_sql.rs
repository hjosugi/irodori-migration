use std::env;
use std::process::Command;

use irodori_migration::{
    chunk_checksum_select_sql, CanonicalColumn, CanonicalType, ChecksumAggregate, ChecksumFunction,
    ChunkBounds, ChunkChecksumConfig, MigrationEngine,
};

#[test]
#[ignore = "requires psql and IRODORI_POSTGRES_URL"]
fn postgres_chunk_checksum_sql_executes() {
    run_postgres(
        "
        DROP TABLE IF EXISTS public.irodori_live_orders;
        CREATE TABLE public.irodori_live_orders (
          order_id BIGINT PRIMARY KEY,
          amount NUMERIC(12,2),
          status TEXT
        );
        INSERT INTO public.irodori_live_orders(order_id, amount, status)
        VALUES (1, 10.25, 'paid'), (2, 0.00, NULL), (3, 7.50, 'open');
        ",
    );

    let sql = chunk_checksum_select_sql(
        MigrationEngine::Postgres,
        &ChunkChecksumConfig::new(
            "public.irodori_live_orders",
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

    run_postgres(&sql);
}

#[test]
#[ignore = "requires mysql client and IRODORI_MYSQL_* environment variables"]
fn mysql_chunk_checksum_sql_executes() {
    run_mysql(
        "
        DROP TABLE IF EXISTS irodori_live_orders;
        CREATE TABLE irodori_live_orders (
          order_id BIGINT PRIMARY KEY,
          amount DECIMAL(12,2),
          status VARCHAR(32)
        );
        INSERT INTO irodori_live_orders(order_id, amount, status)
        VALUES (1, 10.25, 'paid'), (2, 0.00, NULL), (3, 7.50, 'open');
        ",
    );

    let sql = chunk_checksum_select_sql(
        MigrationEngine::MySql,
        &ChunkChecksumConfig::new(
            "irodori_live_orders",
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

    run_mysql(&sql);
}

fn run_postgres(sql: &str) {
    let url = required_env("IRODORI_POSTGRES_URL");
    let psql = env::var("IRODORI_PSQL").unwrap_or_else(|_| "psql".to_string());
    let mut command = Command::new(psql);
    command
        .arg("-v")
        .arg("ON_ERROR_STOP=1")
        .arg("-X")
        .arg("-q")
        .arg("-d")
        .arg(&url)
        .arg("-c")
        .arg(sql);
    assert_success(command, "postgres");
}

fn run_mysql(sql: &str) {
    let host = required_env("IRODORI_MYSQL_HOST");
    let port = required_env("IRODORI_MYSQL_PORT");
    let user = required_env("IRODORI_MYSQL_USER");
    let password = required_env("IRODORI_MYSQL_PASSWORD");
    let database = required_env("IRODORI_MYSQL_DATABASE");
    let mysql = env::var("IRODORI_MYSQL").unwrap_or_else(|_| "mysql".to_string());
    let mut command = Command::new(mysql);
    command
        .arg("--protocol=tcp")
        .arg("-h")
        .arg(host)
        .arg("-P")
        .arg(port)
        .arg("-u")
        .arg(user)
        .arg(format!("-p{password}"))
        .arg(database)
        .arg("-e")
        .arg(sql);
    assert_success(command, "mysql");
}

fn required_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("{name} must be set when running live SQL tests"))
}

fn assert_success(mut command: Command, label: &str) {
    let output = command
        .output()
        .unwrap_or_else(|error| panic!("failed to spawn {label} client: {error}"));
    assert!(
        output.status.success(),
        "{label} command failed with status {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
