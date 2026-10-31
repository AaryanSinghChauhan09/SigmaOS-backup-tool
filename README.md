# SigmaOS Backup Tool

This crate exposes backup and snapshot APIs, but the current
`SovereignBackupToolEngine` stores archive records in memory. Its archive
creation uses a fixed timestamp and estimated deduplication ratio; restore only
checks whether an ID is present. It does not read user files, encrypt archive
contents, or restore data.

Treat record-level tests as model checks, not backup integrity or recovery
evidence. Real use needs durable storage, cryptographic implementation, and a
restore drill against isolated test data.

## License

GPL-3.0-or-later. See [`LICENSE`](LICENSE) and [`Cargo.toml`](Cargo.toml).
