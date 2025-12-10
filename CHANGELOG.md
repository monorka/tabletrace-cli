# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

### Changed

### Fixed

---

## [0.1.4] - 2025-12-10

### Changed
- Change notifications now display full history instead of inline diff
- Unified change display format between real-time notifications and history view
- Added ASCII art banner to README

### Fixed
- Fixed unused code warnings with `#[allow(dead_code)]` annotations

---

## [0.1.3] - 2025-12-10

### Fixed
- Install progress now displays during `npm install` (output to stderr)
- Corrected `cargo install` command to use `--git` option

---

## [0.1.2] - 2025-12-10

### Added
- Update detection with different message for installs vs updates

### Fixed
- Version display now reads from `Cargo.toml` instead of hardcoded value
- Synced `Cargo.toml` and `package.json` versions

---

## [0.1.1] - 2025-12-10

### Added
- Progress display during binary download in install script
- Responsive ASCII banner that adapts to terminal width
- Fallback installation methods (Cargo, manual download) when npm scripts are disabled
- Helpful error message when binary is not found

### Fixed
- Execute permission on `bin/tabletrace` wrapper script
- Cross-compilation for Linux ARM64 using musl target

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
[Unreleased]: https://github.com/monorka/tabletrace-cli/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/monorka/tabletrace-cli/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/monorka/tabletrace-cli/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/monorka/tabletrace-cli/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/monorka/tabletrace-cli/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/monorka/tabletrace-cli/releases/tag/v0.1.0
