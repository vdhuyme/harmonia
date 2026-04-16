# Music Queue Platform – Full Implementation Plan (Architecture-First)

## Overview

A **distributed music orchestration platform** supporting:

- Multiple providers: Spotify, YouTube (extensible)
- Multiple clients: Web, Telegram, Slack
- Real-time queue management with prioritization
- Room-based playback control
- Scalable, multi-instance architecture

This plan follows a **full-system, architecture-first approach**, building all layers with production readiness in mind from the beginning.

---

## System Architecture

```
[ Web Client ]
[ Telegram Bot ]
[ Slack Bot ]
        |
        v
     Axum API Gateway
        |
---------------------------------------------------
| Application Layer                              |
| Queue Engine | Auth | Room | Provider Resolver |
---------------------------------------------------
        |
---------------------------------------------------
| Infrastructure Layer                           |
| SeaORM (Postgres) | Redis | External APIs    |
---------------------------------------------------
        |
        v
   Spotify / YouTube APIs
```

---

## Phase Breakdown

### Phase 1 – Workspace, Core Types, Migrations

#### Goals

- Establish monorepo structure
- Define domain models
- Setup database schema

#### Tasks

- Create Rust workspace:

```
harmonia/
├── crates/
```

- Implement `core` crate:
  - Entities (User, Room, QueueItem, Vote)
  - Errors
  - Traits (Repository, Provider)
  - Rules (priority, voting)

- Setup SeaORM:
  - Generate entities
  - Write migrations:
    - users
    - rooms
    - provider_accounts
    - room_mappings
    - queue_items
    - votes

---

### Phase 2 – Queue Engine & Redis Service

#### Goals

- Implement prioritized queue system
- Ensure concurrency safety

#### Tasks

- Queue engine:
  - Insert song
  - Recalculate priority
  - Sort queue
  - Handle voting

- Priority algorithm:

```
score = base_priority + (votes * 10) - (minutes_since_creation / 60)
```

- Redis integration:
  - Locking via:

```
SET key value EX ttl NX
```

- Prevent:
  - race conditions
  - duplicate updates

---

### Phase 3 – API Layer (Axum)

#### Goals

- Expose system functionality via HTTP

#### Tasks

- Setup Axum server:
  - Router
  - Middleware
  - Error handling

- Implement endpoints:

```
GET  /health
POST /songs/request
GET  /queue/:room_id
POST /queue/:room_id/vote
```

- State management:
  - DB pool
  - Redis client
  - Services

---

### Phase 4 – Authentication & Encryption

#### Goals

- Secure user access
- Protect provider credentials

#### Tasks

- JWT authentication:
  - access token
  - optional refresh token

- Encryption:
  - Encrypt provider tokens before storing
  - Use secure key management

- Middleware:
  - Auth guard
  - Role-based access (admin/user)

---

### Phase 5 – Spotify Provider Integration

#### Goals

- Enable real playback via Spotify

#### Tasks

- OAuth flow:

```
GET  /auth/spotify/url
POST /auth/spotify/callback
```

- Store tokens securely

- Implement provider adapter:
  - play
  - pause
  - transfer playback
  - get current device

---

### Phase 6 – YouTube Provider Integration

#### Goals

- Add secondary playback provider

#### Tasks

- Implement adapter:
  - search
  - metadata extraction
  - playback (via external player or proxy)

- Normalize track format across providers

---

### Phase 7 – Workers & Real-Time (WebSocket)

#### Goals

- Offload processing from API
- Enable real-time updates

#### Tasks

- Create `workers` crate:
  - queue processor
  - playback orchestrator

- WebSocket:

```
GET /ws/:room_id
```

- Redis Pub/Sub:
  - broadcast queue updates
  - notify song changes

---

### Phase 8 – Telegram & Slack Bots

#### Goals

- Multi-platform interaction

#### Telegram Bot

- Commands:
  - `/play`
  - `/queue`
  - `/vote`

#### Slack Bot

- Slash commands
- Interactive buttons

#### Integration

- Bots call API endpoints
- Receive updates via WebSocket or polling

---

### Phase 9 – Testing & Documentation

#### Goals

- Ensure system correctness
- Improve maintainability

#### Tasks

- Unit tests:
  - queue logic
  - voting
  - priority calculation

- Integration tests:
  - API endpoints
  - DB interactions

- Documentation:
  - API specs
  - architecture docs

---

### Phase 10 – Docker & Deployment

#### Goals

- Production-ready deployment

#### Tasks

- Dockerize:
  - API
  - Workers
  - Redis
  - Postgres

- `docker-compose.yml`

- Environment config:
  - `.env`

---

## Project Structure

```
harmonia/
├── crates/
│   ├── core/
│   ├── entities/
│   ├── db/
│   ├── api/
│   ├── workers/
│   ├── infrastructure/
│   ├── bots-telegram/
│   ├── bots-slack/
│   └── shared/
│
├── docker-compose.yml
├── README.md
├── BUILD.md
├── IMPLEMENTATION_STATUS.md
└── migrations/
```

---

## Database Schema

Tables:

- users
- rooms
- provider_accounts
- room_mappings
- queue_items
- votes

Constraints:

- Foreign keys
- Cascade delete
- Unique vote per user/item

---

## Design Highlights

### Priority System

- Vote-based boosting
- Time-based decay
- Fair queue balancing

---

### Distributed Locking

- Redis atomic lock
- Prevent concurrent writes

---

### Domain Isolation

- `core` crate:
  - no IO
  - fully testable
  - framework-independent

---

### Repository Pattern

- Abstract DB access
- ORM-agnostic
- Easy mocking

---

## Development Commands

```
cargo fmt
cargo clippy
cargo test
cargo build
cargo run -p api
```

---

## Quality Requirements

- Clean modular architecture
- 100% test coverage for core logic
- No business logic in API layer
- Strict separation of concerns

---

## Final Goal

A **production-grade distributed music queue system** with:

- Real-time updates
- Multi-provider support
- Multi-client interaction
- Scalable architecture
- Clean and testable codebase

---
