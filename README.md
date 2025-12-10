# tabletrace

Real-time PostgreSQL change monitoring CLI.

[![npm version](https://badge.fury.io/js/%40monorka%2Ftabletrace.svg)](https://www.npmjs.com/package/@monorka/tabletrace)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**English** | [日本語](./README.ja.md)

## Features

- 🔍 **Real-time monitoring** - Watch INSERT, UPDATE, DELETE operations as they happen
- 📊 **Multiple tables** - Monitor multiple tables simultaneously
- 🎨 **Color-coded output** - Green for INSERT, yellow for UPDATE, red for DELETE
- 🔄 **Interactive mode** - View change details, history, and switch tables on the fly
- ⚡ **Lightweight** - No triggers, no schema changes, minimal performance impact

> 💡 **Note**: This CLI is a **development tool** designed for local environments. For GUI version, see [TableTrace OSS](https://github.com/monorka/tabletrace-oss). For team development or staging environments, see TableTrace Pro(coming soon).

## Installation

### npm (recommended)

```bash
npm install -g @monorka/tabletrace
```

Or use directly with npx:

```bash
npx @monorka/tabletrace watch --preset postgres
```

### Cargo (Rust)

```bash
cargo install tabletrace
```

### Manual download

Download the binary for your platform from [GitHub Releases](https://github.com/monorka/tabletrace-cli/releases).

## Quick Start

### Using presets (recommended for local development)

```bash
# Local PostgreSQL (localhost:5432)
tabletrace watch --preset postgres

# Supabase local (localhost:54322)
tabletrace watch --preset supabase
```

### Custom connection

```bash
# Basic connection
tabletrace watch -d mydb -u postgres -W mypassword

# Full options
tabletrace watch -H localhost -P 5432 -d mydb -u postgres -W mypassword
```

### Using environment variable (recommended for security)

```bash
export PGPASSWORD=mypassword
tabletrace watch -d mydb -u postgres
```

## Usage

```
tabletrace watch [OPTIONS]

Options:
      --preset <PRESET>      Preset: 'supabase' (localhost:54322) or 'postgres' (localhost:5432)
  -H, --host <HOST>          Database host [default: localhost]
  -P, --port <PORT>          Database port [default: 5432]
  -d, --database <DATABASE>  Database name (required unless using --preset)
  -u, --user <USER>          Database user [default: postgres]
  -W, --password <PASSWORD>  Database password (or use PGPASSWORD env var)
  -s, --schema <SCHEMA>      Schema to watch (use 'all' for all schemas) [default: public]
  -i, --interval <INTERVAL>  Polling interval in milliseconds [default: 1000]
      --interactive          Enable interactive mode [default: true]
  -h, --help                 Print help
  -V, --version              Print version
```

## Interactive Commands

When running in interactive mode, you can use these commands:

| Key | Command |
|-----|---------|
| `1`, `2`, ... | Show details of change #N |
| `l` | List all recorded changes |
| `c` | Clear change history |
| `w` | Show currently watching tables |
| `r` | Reset/reselect tables to watch |
| `h` | Show help |
| `q` | Quit |

## Example Output

```
╔══════════════════════════════════════════════════════════╗
║           TableTrace - Real-time DB Monitor              ║
╚══════════════════════════════════════════════════════════╝

👁 Watching (2 tables)
  [1] public.users
  [2] public.orders

+ #1 [14:23:45] INSERT public.users (1 row)
    + id=123 { name=Alice, email=alice@example.com }

~ #2 [14:23:52] UPDATE public.orders (1 row)
    ~ id=456 { status: pending → completed }

- #3 [14:24:01] DELETE public.users (1 row)
    - id=123 { name=Alice, email=alice@example.com }
```

## How it Works

TableTrace monitors PostgreSQL's `pg_stat_user_tables` system view to detect changes:

- ✅ **No triggers required** - Works with any PostgreSQL database
- ✅ **No schema changes** - Read-only monitoring
- ✅ **Minimal impact** - Uses lightweight polling
- ✅ **Row-level diffs** - Shows exactly what changed

## Security

- Passwords can be passed via `PGPASSWORD` environment variable (recommended)
- Connection credentials are never logged
- Only reads from system catalogs and user tables

## Requirements

- PostgreSQL 9.6 or higher
- Node.js 16 or higher (for npm installation)

## Related

- [TableTrace OSS](https://github.com/monorka/tabletrace-oss) - Desktop GUI application for local development
- [TableTrace Pro](https://tabletrace.dev) - Team collaboration & staging support (coming soon)

## License

MIT © [Monorka Inc.](https://github.com/monorka)
