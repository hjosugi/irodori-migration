# irodori-migration

`irodori-migration` is the execution-free migration core extracted from
Irodori Table. It builds plans, SQL, previews, manifests, and import/export
streams; host applications own credentials, network access, scheduling,
approval UX, and actual execution.

[![crates.io](https://img.shields.io/crates/v/irodori-migration.svg)](https://crates.io/crates/irodori-migration)
[![docs.rs](https://docs.rs/irodori-migration/badge.svg)](https://docs.rs/irodori-migration)

## What It Provides

- schema snapshots, structural diffs, and destructive-change tagging
- cross-engine migration runbooks and validation SQL
- explicit value canonicalization for decimals, floats, timestamps, NULLs,
  text, booleans, UUIDs, bytes, and JSON
- chunked checksum SQL inspired by pt-table-checksum and reladiff/data-diff
- row-hash manifests, bucket-level diff SQL, and failed-bucket row diff SQL
- recipe-style dry-run previews with before/after text and patch output
- rollout runbooks for expand/contract, dual-write, backfill, shadow-read,
  canary, cutover, and contract phases
- tabular import previews and export encoders for CSV, TSV, SQL, JSON, NDJSON,
  Avro, and Parquet
- a progress/cancellation export runner that host apps can wrap in their own job
  system

## Safety Model

This crate never opens database connections, stores credentials, applies DDL, or
deletes data. Callers should:

1. Preview generated SQL and scripts.
2. Require explicit approval for destructive statements.
3. Pin cross-engine canonicalization rules before hashing.
4. Compare counts and checksums before row-level diff.
5. Use shadow reads, canaries, and rollback gates before cutover.

## Install

```toml
[dependencies]
irodori-migration = "0.1.2"
```

Optional encoders:

```toml
irodori-migration = { version = "0.1.2", features = ["avro", "parquet"] }
```

## Quick Start

```rust
use irodori_migration::{
    build_migration_plan, CanonicalColumn, CanonicalType, ChunkChecksumConfig,
    MigrationEngine, MigrationExportFormat, MigrationSpec,
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

let checksum = irodori_migration::chunk_checksum_select_sql(
    MigrationEngine::Postgres,
    &ChunkChecksumConfig::new(
        "public.orders",
        vec![
            CanonicalColumn::new("order_id", CanonicalType::Integer),
            CanonicalColumn::new("amount", CanonicalType::Decimal { scale: 2 }),
        ],
    ),
);
assert!(checksum.contains("COUNT(*)"));
```

More runnable examples are in [`examples/`](examples).

## Development

Required Rust version: 1.85 or newer.

```sh
cargo fmt -- --check
cargo test
cargo test --all-features
cargo clippy --all-features --all-targets -- -D warnings
rm -f Cargo.lock
cargo package --list
cargo publish --dry-run
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for release steps and contribution rules.

## Architecture

The design is based on deterministic, reviewable transformations:

- OpenRewrite-style recipe previews and dry-run reporting
- Percona/reladiff-style chunked checksums and recursive narrowing
- explicit cross-engine canonicalization before hashing
- expand/contract rollout with dual-write, backfill, shadow-read, canary, and
  cutover gates

See [docs/architecture.md](docs/architecture.md).

## License

Irodori-authored code in this repository is available under `MIT OR 0BSD` unless
a file says otherwise. See [LICENSE](LICENSE).

## Disclaimer

Migration planning and diff helpers can produce destructive or incomplete plans
when used with real systems. Review generated SQL, backups, permissions, and
target connections before execution. For the broader product disclaimer, see
<https://hjosugi.github.io/irodori-docs/disclaimer.html>.
