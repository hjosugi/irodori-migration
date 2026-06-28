# Architecture

`irodori-migration` treats database migration as a verified pipeline, not a
single DDL runner.

## Principles

- **Execution-free core**: the crate generates plans, SQL, import/export streams,
  and verification scripts. Hosts own credentials, connections, scheduling, and
  approval.
- **Recipe-style planning**: plan builders are deterministic functions over a
  spec, similar to OpenRewrite recipes over Lossless Semantic Trees. A second
  run with the same input should produce the same output.
- **Preview before apply**: every schema diff can become a migration script with
  destructive statements tagged before a host asks for approval.
- **Integrity before speed**: counts, key counts, fingerprints, bucket diffs, and
  row-level diffs are first-class gates.
- **Chunk before row diff**: large tables should be narrowed by partition or hash
  bucket before expensive row-level comparison.
- **Normalize explicitly**: cross-engine comparisons must pin NULL markers,
  delimiters, timestamp rendering, numeric scale, char padding, case/whitespace,
  and byte encodings before hashing.
- **Resume by design**: long-running extract, load, backfill, and compare jobs
  must expose progress and cancellation/checkpoint seams.

## Reference Models

- OpenRewrite: recipes, dry-run review, and lossless structured transformation.
- Atlas: declarative desired state, dev-database diffing, linting, and migration
  approval.
- Liquibase and Flyway: versioned migrations, checksums, repeatable changes, and
  controlled rollout.
- Sqitch: dependency-oriented deploy/revert/verify scripts instead of simple
  timestamp ordering.
- reladiff/data-diff: in-database segment checksums with recursive narrowing for
  cross-database diff.
- Percona pt-table-checksum: chunked checksums and throttled online verification
  for MySQL replication.
- AWS DMS and Debezium: full-load plus CDC for ongoing changes.
- gh-ost/pt-online-schema-change/Spirit: shadow-table copy, change capture,
  throttling, and controlled cutover.
- Stripe/Figma-style online migrations: expand/contract, dual-write, backfill,
  shadow-read verification, then cutover.

## Current Modules

- `plan`: migration specs, runbooks, row-hash SQL, bucket-level diff SQL, failed
  bucket diff SQL, manifest DDL, and snippets.
- `schema`: source-agnostic schema snapshots, structural diff, migration preview,
  and destructive statement tagging.
- `io`: tabular import previews and export encoders for CSV, TSV, SQL, JSON,
  NDJSON, Avro, and Parquet.
- `export`: standalone progress/cancellation wrapper for row export streams.
- `dialect`: minimal SQL dialect helpers shared by schema and SQL export.

## Source Links

- OpenRewrite LST and recipes: <https://docs.openrewrite.org/concepts-and-explanations/lossless-semantic-trees>
- Atlas schema apply: <https://atlasgo.io/getting-started>
- Liquibase checksums: <https://www.liquibase.com/blog/what-affects-changeset-checksums>
- Sqitch deploy/revert/verify model: <https://sqitch.org/docs/manual/sqitchtutorial/>
- reladiff: <https://github.com/erezsh/reladiff>
- data-diff technical explanation: <https://github.com/datafold/data-diff/blob/master/docs/technical-explanation.md>
- Percona pt-table-checksum: <https://docs.percona.com/percona-toolkit/pt-table-checksum.html>
- AWS DMS CDC and validation: <https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Task.CDC.html>
- Debezium features: <https://debezium.io/documentation/reference/stable/features.html>
- gh-ost: <https://github.com/github/gh-ost>
- Stripe online migrations: <https://stripe.com/blog/online-migrations>
- Figma database scaling: <https://www.figma.com/blog/how-figmas-databases-team-lived-to-tell-the-scale/>
