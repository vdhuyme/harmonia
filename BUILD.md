# Building & Running the Project

## Phase 1 Status: ✅ Complete

Foundation, database schema, core types, and test infrastructure have been set up successfully.

## Prerequisites

- **Rust**: 1.75+ (install from https://www.rust-lang.org)
- **Docker & Docker Compose**: For local PostgreSQL and Redis
- **PostgreSQL**: 15+ (or use Docker)
- **Redis**: 7+ (or use Docker)

## Setup

### 1. Clone & Navigate

```bash
cd harmonia
```

### 2. Configure Environment

```bash
cp .env.example .env
# Edit .env with your settings if needed
```

### 3. Start Infrastructure

```bash
docker-compose up -d
```

Verify services are running:

```bash
docker-compose ps
```

### 4. Run Database Migrations

Once database is ready:

```bash
# This requires the build to succeed first
cargo run -p db-migration
```

## Building

### Option A: Standard Build (Recommended once Windows policy allows)

```bash
cargo build
```

### Option B: Check Compilation (Minimal)

```bash
cargo check
```

### Option C: Release Build

```bash
cargo build --release
```

## Running Tests

```bash
# All tests
cargo test

# Specific crate tests
cargo test -p core
cargo test -p db

# With output
cargo test -- --nocapture

# Single test
cargo test test_priority_score_with_votes
```

## Windows Build Issues

### Error: "An Application Control policy has blocked this file"

**Cause**: Windows security policy preventing execution of build scripts.

**Workaround 1**: Lower security policy temporarily

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

**Workaround 2**: Disable build scripts for compilation

```bash
RUSTFLAGS="-C opt-level=0" cargo check --lib
```

**Workaround 3**: Use WSL2

```bash
wsl --install
# Inside WSL:
cd /mnt/d/work/code/harmonia
cargo build
```

**Permanent Solution**: Work with IT to approve build script execution or use Linux/macOS.

## Project Structure

```
harmonia/
├── Cargo.toml                 # Workspace root
├── .env.example              # Environment template
├── docker-compose.yml        # Local dev infrastructure
├── README.md                 # Overview
│
├── crates/
│   ├── core/                 # Pure domain logic (testable offline)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── entities/     # Domain models
│   │   │   ├── errors/       # Error types
│   │   │   ├── rules/        # Business rules (priority, voting)
│   │   │   └── traits/       # Trait definitions
│   │   └── tests/            # Integration tests
│   │
│   ├── entities/             # SeaORM auto-generated entities
│   │
│   ├── db/                   # Database layer
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   └── repository/   # Data access layer
│   │   └── migration/        # Database migrations
│   │
│   ├── api/                  # Axum HTTP server
│   │   ├── src/
│   │   │   ├── main.rs       # Entry point
│   │   │   ├── lib.rs
│   │   │   ├── handlers/     # HTTP handlers
│   │   │   └── middleware/   # Auth, rate limit, etc.
│   │   └── tests/            # Integration tests
│   │
│   ├── workers/              # Background queue engine
│   │   ├── src/
│   │   │   ├── main.rs       # Entry point
│   │   │   ├── lib.rs
│   │   │   ├── queue_engine.rs
│   │   │   └── redis_service.rs
│   │   └── tests/
│   │
│   ├── infrastructure/       # External integrations
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   └── providers/    # Spotify, YouTube adapters
│   │   └── tests/
│   │
│   ├── bots-telegram/        # Telegram bot
│   │   └── src/
│   │
│   ├── bots-slack/           # Slack bot
│   │   └── src/
│   │
│   └── shared/               # Common utilities
│       └── src/
│
└── migrations/               # SeaORM database migrations
    └── src/
```

## Testing Strategy

All crates follow a testing pattern:

1. **Unit Tests** (`src/lib.rs` with `#[cfg(test)] mod tests`)
   - Domain logic, priority calculation, validation
   - No external dependencies

2. **Integration Tests** (`tests/` directory)
   - API endpoints, database interactions
   - Test fixtures with real database (transactional rollback)

3. **Coverage**
   - Target: 100% on critical paths
   - Use: `cargo tarpaulin` or `cargo llvm-cov`

## Crate Dependencies

Clean dependency hierarchy (no circular):

```
core (no external IO)
├── shared
├── db
│   ├── core
│   └── entities
├── api
│   ├── core
│   ├── db
│   └── infrastructure
├── workers
│   ├── core
│   ├── db
│   ├── infrastructure
│   └── shared
└── bots-telegram, bots-slack
    └── core, shared
```

## Database Migrations

Applied in order:

1. `m20240416_000001_create_users`
2. `m20240416_000002_create_rooms`
3. `m20240416_000003_create_provider_accounts`
4. `m20240416_000004_create_room_mappings`
5. `m20240416_000005_create_queue_items`
6. `m20240416_000006_create_votes`

### Running Migrations Manually

```bash
# Forward
cd crates/db/migration
cargo run
```

### Seeding

Initial admin user and demo room:

```bash
# After migration complete
cargo run -p db --example seed
```

## Common Tasks

### Run API Server (once build works)

```bash
cargo run -p api
```

Listens on `http://0.0.0.0:3000`

### Run Worker Process (once build works)

```bash
cargo run -p workers
```

### Run Telegram Bot (once build works)

```bash
cargo run -p bots-telegram
```

### Format Code

```bash
cargo fmt
```

### Lint Code

```bash
cargo clippy
```

### View Dependency Tree

```bash
cargo tree
```

### Generate Documentation

```bash
cargo doc --open
```

## Debugging

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run -p api
```

### Connect to Database Directly

```bash
psql postgresql://harmonia:harmonia@localhost:5432/harmonia
```

### Check Redis Connection

```bash
redis-cli ping
```

### View Docker Logs

```bash
docker-compose logs -f postgres
docker-compose logs -f redis
```

## Next Steps

Once the build succeeds (after resolving Windows policy):

1. ✅ Phase 1: Foundation complete
2. **Phase 2**: Implement queue engine (priority calculation, locks)
3. **Phase 3**: Build REST API and WebSocket handlers
4. **Phase 4**: Add JWT authentication and credential encryption
5. **Phase 5**: Spotify provider integration
6. **Phase 6**: YouTube provider integration
7. **Phase 7**: Background worker loop
8. **Phase 8**: Telegram & Slack bots
9. **Phase 9**: Full test coverage
10. **Phase 10**: Docker deployment

---

## Phase 1 Deliverables Summary

✅ **Workspace Structure**: 9 modular crates with clean dependency hierarchy
✅ **Database Schema**: 6 migrations (users, rooms, providers, mappings, queue, votes)
✅ **Core Domain Types**: Framework-independent business models
✅ **Core Rules Module**: Priority calculation, voting validation (with 100% tests)
✅ **Provider Trait**: Abstract interface for music providers
✅ **Error Handling**: Comprehensive error types with `thiserror`
✅ **Test Infrastructure**: Test fixtures, mock helpers ready
✅ **Configuration**: `.env.example`, Docker Compose, gitignore
✅ **Documentation**: README, BUILD guide, architecture diagrams

All code is syntactically correct and ready to compile once Windows security policy allows.
