# irodori-migration

`irodori-migration` is the standalone migration core extracted from Irodori.
It provides execution-free building blocks for database moves:

- schema snapshots, structural diffs, and destructive-change tagging
- cross-engine migration runbooks and validation SQL
- row-hash manifests, bucket-level diff SQL, and failed-bucket row diff SQL
- tabular import previews and export encoders for CSV, TSV, SQL, JSON, NDJSON,
  Avro, and Parquet
- a small progress/cancellation export runner that host apps can wrap in their
  own job system

The crate does not open database connections, store credentials, or apply DDL.
Applications should preview generated SQL, require explicit approval for
destructive steps, run engine-specific safety checks, and verify data with
counts, hashes, and row-level diffs before cutover.

## Example

```rust
use irodori_migration::{
    build_migration_plan, MigrationEngine, MigrationExportFormat, MigrationSpec,
};

let spec = MigrationSpec {
    source_engine: MigrationEngine::Hive,
    target_engine: MigrationEngine::Snowflake,
    source_table: "legacy.orders".into(),
    target_table: "analytics.orders".into(),
    key_columns: vec!["order_id".into()],
    compare_columns: vec!["order_id".into(), "amount".into(), "updated_at".into()],
    export_format: MigrationExportFormat::Parquet,
    ..MigrationSpec::default()
};

let plan = build_migration_plan(&spec);
assert!(plan.source_sql.contains("irodori_row_hash"));
assert!(plan.diff_sql.contains("Bucket-level diff"));
```

## Repository Status

This repo is publish-ready locally. Push it to
`https://github.com/hjosugi/irodori-migration`, then publish with:

```sh
cargo test --all-features
cargo publish --dry-run
cargo publish
```
