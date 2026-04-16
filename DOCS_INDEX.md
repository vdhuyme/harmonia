# Documentation Index

Welcome to the Music Queue Platform! Here's how to navigate the project documentation:

## Start Here

1. **[README.md](README.md)** – Project overview, architecture, quick start
2. **[QUICKSTART.md](QUICKSTART.md)** – Developer guide for extending the system
3. **[BUILD.md](BUILD.md)** – Build instructions and troubleshooting

## Architecture & Design

- **[IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md)** – Detailed progress report with code metrics
- **Design decisions**: See README.md section "Key Design Decisions"

## Source Code

### Core Domain (Framework-Independent)

- `crates/core/src/` – Pure business logic
  - `entities/` – Domain models (User, Room, QueueItem, etc.)
  - `rules/` – Priority calculation, voting logic
  - `errors/` – Error type definitions
  - `traits/` – Interface definitions (MusicProvider)

### Database Layer

- `crates/db/` – Database access
  - `src/repository/` – Repository trait definitions
  - `migration/src/` – 6 database migrations
- `crates/entities/` – SeaORM auto-generated models (placeholder)

### API Server

- `crates/api/src/`
  - `main.rs` – Server entry point
  - `lib.rs` – Router configuration
  - `error.rs` – Error handling
  - `state.rs` – Application state
  - `handlers/` – HTTP request handlers

### Background Processing

- `crates/workers/src/`
  - `queue_engine.rs` – Core queue logic
  - `redis_service.rs` – Distributed operations
  - `main.rs` – Worker process entry point

### Infrastructure (External Integrations)

- `crates/infrastructure/src/providers/` – Spotify, YouTube adapters (to be implemented)

### Clients

- `crates/bots-telegram/src/` – Telegram bot (to be implemented)
- `crates/bots-slack/src/` – Slack bot (to be implemented)

### Utilities

- `crates/shared/src/`
  - `test_utils.rs` – Test fixtures and helpers

## Tests

Run tests with:

```bash
cargo test              # All tests
cargo test -p core     # Specific crate
cargo test --lib       # Unit tests only
```

Test files:

- `crates/core/tests/` – Priority, voting, entity tests (12 tests)
- `crates/workers/tests/` – Queue engine integration tests (4 tests)
- `crates/api/tests/` – API handler tests
- `crates/shared/src/test_utils.rs` – Test fixture creation

## Configuration

- `.env.example` – Environment variable template
- `docker-compose.yml` – Local PostgreSQL and Redis setup
- `Cargo.toml` – Workspace and dependency management

## Database

See `crates/db/migration/src/` for migrations:

1. `m20240416_000001` – users table
2. `m20240416_000002` – rooms table
3. `m20240416_000003` – provider_accounts table
4. `m20240416_000004` – room_mappings table
5. `m20240416_000005` – queue_items table
6. `m20240416_000006` – votes table

## API Endpoints

Current implemented endpoints:

- `GET /health` – Server health status

Upcoming endpoints (handlers ready, need DB):

- `POST /songs/request` – Queue a song
- `GET /queue/:room_id` – Get queue
- `POST /queue/:room_id/vote` – Vote for song
- `POST /admin/provider/connect` – Connect provider
- `POST /admin/room/map` – Map room to provider

Future (Phase 7+):

- `GET /ws/:room_id` – WebSocket real-time updates

## Development Workflow

1. **Setup**: `cp .env.example .env && docker-compose up -d`
2. **Code**: Edit files in `crates/*/src/`
3. **Test**: `cargo test`
4. **Format**: `cargo fmt && cargo clippy`
5. **Build**: `cargo build` (once Windows policy resolved)

## Common Questions

### Where's the database integration?

Database layer is architected but not yet connected to handlers. Phase 3+ will implement:

- SeaORM repository implementations
- Integration with API handlers
- Real database operations

### How do I add a new provider?

See QUICKSTART.md section "How to Extend: Step-by-Step Example" (Discord provider example).

### How's the priority algorithm work?

See README.md "Design Highlights" → "Priority Algorithm"

Score = `base_priority + (votes * 10) - (minutes_since_creation / 60)`

### What about authentication?

Planned for Phase 4. Currently stubs in place.

### Is there WebSocket support?

Scaffolding ready; full implementation in Phase 7.

## Project Timeline

- **Week 1**: Phases 1–2 (Foundation + Queue Engine) ✅
- **Week 2**: Phase 3 (API Layer) ✅ (in progress)
- **Week 3**: Phase 4 (Auth)
- **Week 4**: Phases 5–6 (Providers)
- **Week 5**: Phases 7–8 (Workers + Bots)
- **Week 6**: Phases 9–10 (Testing + Deployment)

## Support

For questions about:

- **Architecture**: See IMPLEMENTATION_STATUS.md
- **Building**: See BUILD.md
- **Development**: See QUICKSTART.md
- **Design patterns**: See code comments and examples in QUICKSTART.md

---

**Last Updated**: April 16, 2026 – Phase 3 in progress

Next milestone: Phase 4 (Authentication & Security)
