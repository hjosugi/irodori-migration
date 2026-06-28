# Contributing

`irodori-migration` is an execution-free Rust library. Keep changes
deterministic, previewable, and safe to review.

## Local Checks

Run the same checks as CI before committing:

```sh
cargo fmt -- --check
cargo test
cargo test --all-features
cargo clippy --all-features --all-targets -- -D warnings
rm -f Cargo.lock
cargo publish --dry-run
```

Or run:

```sh
scripts/verify.sh
```

For live database smoke tests, see [docs/testing.md](docs/testing.md).

## Design Rules

- Do not add database connections, credential storage, or direct DDL execution to
  this crate.
- Prefer pure functions that turn specs into plans, SQL, previews, or reports.
- Destructive migration output must be tagged or called out before a host can
  execute it.
- Cross-engine row comparison must make canonicalization explicit.
- New behavior should include focused unit tests and, when it teaches usage, an
  example under `examples/`.
- Generated SQL that is intended to execute on a real engine should have either
  a unit-level SQL-shape test or a live smoke test under `tests/live_sql.rs`.
- Keep public APIs small and stable enough for host applications to wrap.

## Release Checklist

1. Update `Cargo.toml` version.
2. Update `CHANGELOG.md`.
3. Run all local checks.
4. Remove the generated library lockfile with `rm -f Cargo.lock`.
5. Confirm `cargo package --list` contains only intended files. Cargo-generated
   `.cargo_vcs_info.json`, `Cargo.lock`, and `Cargo.toml.orig` are expected in
   the package listing.
6. Commit with a clear message.
7. Tag with `vX.Y.Z`.
8. Push `main` and the tag.
9. Run `cargo publish`.
10. Confirm `cargo search irodori-migration --limit 3` shows the new version.
