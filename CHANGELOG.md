# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

### Changed

### Fixed

---

## [0.1.0] - 2025-12-09

### Added
- Real-time table monitoring with polling-based change detection
- Multi-table watching with color-coded output (INSERT/UPDATE/DELETE)
- Interactive mode with keyboard commands
- Row-level diff display showing exactly what changed
- PostgreSQL preset (`--preset postgres` for localhost:5432)
- Supabase Local preset (`--preset supabase` for localhost:54322)
- Schema filtering support (`-s` / `--schema`)
- Configurable polling interval (`-i` / `--interval`)
- `PGPASSWORD` environment variable support for secure password handling
- Connection error detection with automatic reconnection

### Technical
- Built with Rust + Tokio for async operations
- Uses `pg_stat_user_tables` for non-intrusive monitoring
- No triggers or schema changes required
- Distributed via npm with prebuilt binaries

---

<!-- Links -->
[Unreleased]: https://github.com/monorka/tabletrace-cli/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/monorka/tabletrace-cli/releases/tag/v0.1.0
