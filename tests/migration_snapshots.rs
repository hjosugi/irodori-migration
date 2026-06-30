use irodori_migration::{
    build_migration_plan, chunk_checksum_select_sql, keyed_diff_sql, CanonicalColumn,
    CanonicalType, ChecksumAggregate, ChecksumFunction, ChunkBounds, ChunkChecksumConfig,
    MigrationEngine, MigrationExportFormat, MigrationSpec,
};

fn postgres_to_mysql_spec() -> MigrationSpec {
    MigrationSpec {
        source_engine: MigrationEngine::Postgres,
        target_engine: MigrationEngine::MySql,
        source_table: "public.orders".to_string(),
        target_table: "warehouse.orders".to_string(),
        key_columns: vec!["order_id".to_string(), "line_id".to_string()],
        compare_columns: vec![
            "order_id".to_string(),
            "line_id".to_string(),
            "amount".to_string(),
            "status".to_string(),
        ],
        partition_column: "sales_dt".to_string(),
        partition_predicate: "sales_dt >= '2026-01-01'".to_string(),
        export_format: MigrationExportFormat::Csv,
        diff_limit: 25,
        hash_bucket_prefix_len: 3,
        ..MigrationSpec::default()
    }
}

#[test]
fn snapshots_plan_sql_by_engine() {
    let plan = build_migration_plan(&postgres_to_mysql_spec());

    insta::assert_snapshot!("postgres_to_mysql_source_sql", plan.source_sql);
    insta::assert_snapshot!("postgres_to_mysql_target_sql", plan.target_sql);
    insta::assert_snapshot!("postgres_to_mysql_diff_sql", plan.diff_sql);
}

#[test]
fn snapshots_standalone_generated_sql() {
    let keys = vec!["order_id".to_string(), "line id".to_string()];
    insta::assert_snapshot!(
        "mysql_keyed_diff",
        keyed_diff_sql(MigrationEngine::MySql, &keys, 25)
    );

    let config = ChunkChecksumConfig::new(
        "public.orders",
        vec![
            CanonicalColumn::new("order_id", CanonicalType::Integer),
            CanonicalColumn::new("amount", CanonicalType::Decimal { scale: 2 }),
            CanonicalColumn::new("status", CanonicalType::Text),
        ],
    )
    .with_bounds(ChunkBounds {
        column: "order_id".to_string(),
        lower: Some("1".to_string()),
        upper: Some("1000".to_string()),
        include_upper: false,
    })
    .with_function(ChecksumFunction::Md5)
    .with_aggregate(ChecksumAggregate::Sum);
    insta::assert_snapshot!(
        "postgres_chunk_checksum",
        chunk_checksum_select_sql(MigrationEngine::Postgres, &config)
    );
}
