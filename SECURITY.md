# Security

`irodori-migration` is an execution-free planning library. It should not store
credentials, open network connections, or apply DDL directly.

## Reporting

Report security issues privately to the repository owner rather than opening a
public issue with exploit details.

Include:

- affected version
- generated SQL or API entry point involved
- expected and actual behavior
- whether credentials, destructive SQL, or data exposure are involved

## Security Boundaries

This crate can generate destructive or high-impact SQL. Host applications must
still enforce:

- credential storage and redaction
- approval prompts for destructive statements
- database permissions and network controls
- backups and restore verification
- engine-specific execution safety checks
- audit logging for generated plans and applied migrations

Generated checksum SQL is for verification. A passing checksum is not a backup,
not a permission model, and not a substitute for rollback planning.
