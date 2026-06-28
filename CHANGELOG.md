# Changelog

All notable changes to `irodori-migration` are documented here.

## 0.1.2 - 2026-06-29

- Added CI workflow for formatting, tests, clippy, and package verification.
- Added development documentation, changelog, examples, and rustfmt config.
- Fixed crate metadata, README release instructions, gitignore coverage, and
  license naming.
- Marked the crate as `unsafe_code` forbidden.

## 0.1.1 - 2026-06-29

- Added cross-engine canonicalization policies for checksums.
- Added chunked checksum SQL, checksum manifests, divergent-chunk queries, and
  sync repair-plan scaffolding.
- Added recipe-style dry-run previews for generated artifacts.
- Added expand/contract rollout and shadow-read runbook helpers.

## 0.1.0 - 2026-06-29

- Initial standalone migration core.
- Added schema diff and destructive-change tagging.
- Added migration plans, row-hash SQL, bucket-level diff SQL, and failed-bucket
  row diff SQL.
- Added tabular import previews and CSV, TSV, SQL, JSON, NDJSON, Avro, and
  Parquet encoders.
