# Testing

Default gate:

```sh
cargo fmt -- --check
cargo test
cargo test --all-features
cargo clippy --all-features --all-targets -- -D warnings
```

Container SQL smoke tests:

```sh
cargo test --test container_sql -- --ignored --test-threads=1
```

Externally managed Postgres/MySQL smoke tests:

```sh
export IRODORI_POSTGRES_URL='postgres://postgres:postgres@127.0.0.1:5432/irodori_migration'
export IRODORI_MYSQL_HOST='127.0.0.1'
export IRODORI_MYSQL_PORT='3306'
export IRODORI_MYSQL_USER='root'
export IRODORI_MYSQL_PASSWORD='mysql'
export IRODORI_MYSQL_DATABASE='irodori_migration'
cargo test --test live_sql -- --ignored --test-threads=1
```

Package check:

```sh
rm -f Cargo.lock
cargo package --list --allow-dirty
cargo publish --dry-run --allow-dirty
```
