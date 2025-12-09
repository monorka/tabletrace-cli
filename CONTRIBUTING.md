# Contributing to TableTrace CLI

Thank you for your interest in contributing to TableTrace CLI!

## How to Contribute

### Reporting Bugs

1. Check if the issue already exists
2. Create a new issue with:
   - Clear title
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment (OS, PostgreSQL version, etc.)

### Submitting Pull Requests

1. Fork the repository
2. Create a feature branch from `develop`
   ```bash
   git checkout develop
   git checkout -b feature/your-feature
   ```
3. Make your changes
4. Run tests and linting
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   ```
5. Commit with a clear message
   ```bash
   git commit -m "feat: add new feature"
   ```
6. Push and create a PR to `develop`

### Commit Message Format

We use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `style:` - Formatting (no code change)
- `refactor:` - Code refactoring
- `test:` - Tests
- `chore:` - Maintenance

### Code Style

- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes with no warnings
- Add tests for new functionality

## Development Setup

### Prerequisites

- Rust 1.70 or higher
- PostgreSQL 9.6 or higher (for testing)

### Build

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run directly
cargo run -- watch --preset postgres
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

### Local PostgreSQL for Testing

```bash
# Using Docker
docker run -d \
  --name tabletrace-test \
  -e POSTGRES_PASSWORD=postgres \
  -p 5432:5432 \
  postgres:16

# Connect
cargo run -- watch -d postgres -u postgres -W postgres
```

## Project Structure

```
src/
├── main.rs          # Entry point
├── cli.rs           # CLI argument parsing
├── types.rs         # Data structures
├── db.rs            # Database operations
├── diff.rs          # Change detection logic
├── error.rs         # Custom error types
├── input.rs         # User input handling
├── state.rs         # Global state
├── constants.rs     # Constants
├── display/         # Output formatting
│   ├── mod.rs
│   ├── banner.rs
│   ├── change.rs
│   ├── diff.rs
│   ├── history.rs
│   └── messages.rs
└── watcher/         # Main monitoring logic
    ├── mod.rs
    ├── changes.rs
    ├── handlers.rs
    ├── snapshot.rs
    └── stats.rs
```

## Questions?

Feel free to open an issue or discussion.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
