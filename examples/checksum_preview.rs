use irodori_migration::{
    canonicalization_warnings, chunk_checksum_select_sql, CanonicalColumn, CanonicalType,
    CanonicalizationPolicy, ChecksumAggregate, ChecksumFunction, ChunkBounds, ChunkChecksumConfig,
    MigrationEngine, TimestampMode,
};

fn main() {
    let columns = vec![
        CanonicalColumn::new("order_id", CanonicalType::Integer),
        CanonicalColumn::new("amount", CanonicalType::Decimal { scale: 2 }),
        CanonicalColumn::new(
            "updated_at",
            CanonicalType::Timestamp {
                fractional_digits: 3,
                mode: TimestampMode::Utc,
            },
        ),
    ];
    let policy = CanonicalizationPolicy::default();
    let config = ChunkChecksumConfig::new("public.orders", columns.clone())
        .with_function(ChecksumFunction::Md5)
        .with_aggregate(ChecksumAggregate::Sum)
        .with_bounds(ChunkBounds {
            column: "order_id".into(),
            lower: Some("1000".into()),
            upper: Some("2000".into()),
            include_upper: false,
        });

    for warning in canonicalization_warnings(MigrationEngine::Postgres, &columns, &policy) {
        eprintln!("warning: {warning}");
    }
    println!(
        "{}",
        chunk_checksum_select_sql(MigrationEngine::Postgres, &config)
    );
}
