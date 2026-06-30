# Testing

`irodori-migration` has three test layers.

## 1. Default Library Checks

These do not require databases or credentials:

```sh
cargo fmt -- --check
cargo test
cargo test --all-features
cargo clippy --all-features --all-targets -- -D warnings
```

They cover schema diffing, generated SQL shape, canonicalization choices,
checksum builders, dry-run previews, rollout plans, import/export encoders, and
examples.

## 2. Live SQL Smoke Tests

The live smoke tests are ignored by default and execute generated checksum SQL
against real Postgres and MySQL databases. They are intended for CI or a local
database sandbox.

### Container-managed databases

Use the `testcontainers` suite when CI can run Docker. This starts disposable
Postgres and MySQL instances and connects with native Rust clients, so no
`psql` or `mysql` executables are required:

```sh
cargo test --test container_sql -- --ignored --test-threads=1
```

### Externally managed databases

The legacy live SQL test can still target databases you started yourself:

```sh
export IRODORI_POSTGRES_URL='postgres://postgres:postgres@127.0.0.1:5432/irodori_migration'
export IRODORI_MYSQL_HOST='127.0.0.1'
export IRODORI_MYSQL_PORT='3306'
export IRODORI_MYSQL_USER='root'
export IRODORI_MYSQL_PASSWORD='mysql'
export IRODORI_MYSQL_DATABASE='irodori_migration'

cargo test --test live_sql -- --ignored --test-threads=1
```

Optional client overrides:

- `IRODORI_PSQL`: path to the `psql` executable.
- `IRODORI_MYSQL`: path to the `mysql` executable.

These tests verify that representative generated SQL executes; they do not
replace engine-by-engine production validation for permissions, scale, timezone
settings, collations, or native type edge cases.

## 3. Package Verification

Before publishing:

```sh
rm -f Cargo.lock
cargo package --list --allow-dirty
cargo publish --dry-run --allow-dirty
```

Cargo-generated `.cargo_vcs_info.json`, `Cargo.lock`, and `Cargo.toml.orig` are
expected in package listings for this library crate.
