# Implementation Roadmap: Phases 5-10

**Current Status**: Phase 4 Complete (40% done)
**Remaining Phases**: 6 (60% of project)
**Estimated Timeline**: 4-6 weeks

---

## Phase 5: Spotify Provider Integration (Week 5)

### Objective

Implement Spotify OAuth authentication and music provider functionality.

### Deliverables

#### 1. Spotify OAuth Module (`crates/infrastructure/src/providers/spotify.rs`)

```rust
pub struct SpotifyProvider {
    http_client: reqwest::Client,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

#[async_trait]
impl MusicProvider for SpotifyProvider {
    async fn search(&self, query: &str) -> Result<Vec<Track>> { ... }
    async fn play(&self, device_id: &str, track_id: &str) -> Result<()> { ... }
    async fn pause(&self, device_id: &str) -> Result<()> { ... }
    async fn skip(&self, device_id: &str) -> Result<()> { ... }
    async fn get_current_playback(&self, device_id: &str) -> Result<Track> { ... }
}
```

#### 2. OAuth Flow Handler (`crates/api/src/handlers/oauth.rs`)

```
GET /auth/spotify/login
  ↓ Redirects to Spotify
  ↓
GET /auth/spotify/callback?code=...&state=...
  ↓ Exchange code for access token
  ↓ Store encrypted token in database
  ↓ Redirect to frontend with session
```

#### 3. Provider Connection Endpoint

```
POST /admin/provider/spotify/connect
{
  "access_token": "encrypted-spotify-token",
  "refresh_token": "encrypted-refresh-token"
}
```

#### 4. Spotify API Integration

- Search tracks by query
- Get available devices
- Control playback (play, pause, skip)
- Get current playback status
- Handle token refresh

#### 5. Error Handling

- Rate limiting (60 requests/min)
- Invalid device handling
- Token expiration
- Network timeouts

### Test Coverage

- OAuth token exchange
- Track search
- Playback control
- Error scenarios
- Token refresh

### Database Changes

- Add `provider_type` to `provider_accounts`
- Add `device_id` to `room_mappings`
- Add `expiry` timestamp

### Estimated Effort

- 250 lines of code
- 20 tests
- 3-4 days

### Success Criteria

- ✅ User can authenticate with Spotify
- ✅ Platform can search tracks
- ✅ Platform can play/pause/skip
- ✅ Token refresh works automatically
- ✅ All 20 tests passing

---

## Phase 6: YouTube Provider Integration (Week 5-6)

### Objective

Add YouTube Music as a provider for compatibility.

### Deliverables

#### 1. YouTube OAuth Module (`crates/infrastructure/src/providers/youtube.rs`)

```rust
pub struct YouTubeProvider {
    http_client: reqwest::Client,
    api_key: String,
    // Similar structure to Spotify
}

#[async_trait]
impl MusicProvider for YouTubeProvider { ... }
```

#### 2. YouTube Music API Integration

- Search videos
- Get channel information
- Queue management
- Playback control (via connected device)

#### 3. Device Support

- Integrate with YouTube TV
- YouTube Music web player support
- Smart display support

#### 4. Error Handling

- Quota limits
- Video availability (region-locked)
- Age-restricted content

### Estimated Effort

- 200 lines of code
- 15 tests
- 2-3 days

### Success Criteria

- ✅ User can search YouTube Music
- ✅ Videos queue successfully
- ✅ Playback control works
- ✅ All 15 tests passing

---

## Phase 7: Background Workers & WebSocket Realtime (Weeks 6-7)

### Objective

Implement the worker loop for autonomous queue playback and realtime updates.

### Deliverables

#### 1. Worker Loop (`crates/workers/src/main.rs`)

```rust
#[tokio::main]
async fn main() {
    loop {
        for room in get_all_rooms().await {
            // 1. Acquire distributed lock
            if !redis.acquire_lock(&room.id).await { continue; }

            // 2. Get current playback status
            let current = get_current_playback(&room).await?;

            // 3. If finished, select next song
            if current.is_finished() {
                let next = select_next_song(&room).await?;
                play_song(&room, &next).await?;
                broadcast_event("song_started", &next).await?;
            }

            // 4. Release lock
            redis.release_lock(&room.id).await?;
        }
        sleep(Duration::from_secs(5)).await;
    }
}
```

#### 2. WebSocket Handler (`crates/api/src/handlers/websocket.rs`)

```rust
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(room_id): Path<String>,
    auth: AuthUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, room_id, auth, state))
}

async fn handle_socket(
    socket: WebSocket,
    room_id: String,
    auth: AuthUser,
    state: AppState,
) {
    // 1. Check auth can access room
    if !auth.can_access_room(&room_id) { return; }

    // 2. Subscribe to Redis pub/sub for room
    let mut rx = state.redis.subscribe(format!("room:{}", room_id)).await;

    // 3. Send initial queue state
    let queue = state.db.queue_items().find_by_room(&room_id).await?;
    socket.send(Message::Text(serde_json::to_string(&queue)?)).await?;

    // 4. Forward events to client
    while let Ok(msg) = rx.recv().await {
        let _ = socket.send(Message::Text(msg)).await;
    }
}
```

#### 3. Event Broadcasting

- Queue updated
- Song started
- Song finished
- Vote received
- Error occurred

#### 4. Connection Management

- Track active connections per room
- Handle disconnections
- Clean up subscriptions

### Test Coverage

- Worker loop execution
- Lock acquisition/release
- WebSocket connection lifecycle
- Event broadcasting
- Error recovery

### Estimated Effort

- 300 lines of code
- 15 tests
- 4-5 days

### Success Criteria

- ✅ Worker automatically plays next song
- ✅ WebSocket clients receive real-time updates
- ✅ Multiple rooms don't interfere
- ✅ Reconnection works
- ✅ All 15 tests passing

---

## Phase 8: Telegram & Slack Bot Integration (Weeks 7-8)

### Objective

Enable users to interact with the platform through chat applications.

### Deliverables

#### 1. Telegram Bot (`crates/bots-telegram/src/main.rs`)

```
/play <query> – Queue a song
/queue – Show current queue
/skip – Skip current song
/vote <song_number> – Vote for a song
/status – Show current playing song
```

#### 2. Slack Bot (`crates/bots-slack/src/main.rs`)

```
/music play <query> – Queue a song
/music queue – Show queue
/music vote <number> – Vote for song
```

#### 3. Webhook Handlers

- Handle incoming messages
- Parse commands
- Call API Gateway
- Format responses

#### 4. Database Integration

- Store bot user mappings
- Track command history
- Log interactions

### Test Coverage

- Command parsing
- API calls
- Error responses
- Rate limiting

### Estimated Effort

- 400 lines of code
- 20 tests
- 4-5 days

### Success Criteria

- ✅ Users can queue songs via Telegram
- ✅ Users can queue songs via Slack
- ✅ Queue display shows properly
- ✅ Voting works from chat apps
- ✅ All 20 tests passing

---

## Phase 9: Comprehensive Testing & Polish (Weeks 8-9)

### Objective

Achieve 100% test coverage on critical paths and polish the system.

### Deliverables

#### 1. Additional Test Coverage

- E2E test scenarios (user flow)
- Database transaction tests
- Error recovery tests
- Concurrency tests
- Performance benchmarks

#### 2. Security Auditing

- SQL injection prevention
- XSS prevention
- CSRF protection
- Rate limiting enforcement
- Input validation

#### 3. Documentation

- API OpenAPI/Swagger spec
- Deployment guide
- Admin guide
- User guide
- Architecture documentation

#### 4. Performance Optimization

- Database query optimization
- Caching strategy
- Connection pooling
- Memory profiling

### Test Coverage Target

- 100% of core logic
- 95%+ of API handlers
- 90%+ of infrastructure

### Estimated Effort

- 500 lines of test code
- Documentation updates
- 4-5 days

---

## Phase 10: Deployment & Production Hardening (Week 9-10)

### Objective

Package the system for production deployment.

### Deliverables

#### 1. Docker Images

```dockerfile
# Dockerfile for API
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release -p api

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/api /usr/local/bin/
CMD ["api"]
```

#### 2. Docker Compose

```yaml
services:
  api:
    build: .
    ports:
      - "3000:3000"
    environment:
      DATABASE_URL: postgresql://...
      REDIS_URL: redis://...
      JWT_SECRET: ${JWT_SECRET}

  postgres:
    image: postgres:15
    environment:
      POSTGRES_PASSWORD: ...

  redis:
    image: redis:7

  workers:
    build:
      context: .
      dockerfile: Dockerfile.workers
    environment:
      DATABASE_URL: postgresql://...
      REDIS_URL: redis://...
```

#### 3. Kubernetes Manifests (Optional)

- Deployment configs
- Service definitions
- ConfigMaps for configuration
- Secrets for credentials

#### 4. CI/CD Pipeline

- GitHub Actions workflow
- Build and test automation
- Docker image publishing
- Deployment scripts

#### 5. Monitoring & Logging

- Prometheus metrics
- Grafana dashboards
- ELK stack setup
- Alert configuration

#### 6. Security Hardening

- HTTPS configuration
- Rate limiting enforcement
- DDoS protection
- SQL injection prevention
- Authentication enforcement

### Estimated Effort

- 300 lines of config
- Docker + Kubernetes setup
- CI/CD pipeline
- 3-4 days

---

## Phase Dependencies

```
Phase 1 (Foundation)
    ↓
Phase 2 (Queue Engine)
    ↓
Phase 3 (API Layer)
    ↓
Phase 4 (Auth)
    ↓
    ├─→ Phase 5 (Spotify)
    │       ↓
    │   Phase 7 (Workers)
    │
    ├─→ Phase 6 (YouTube)
    │       ↓
    │   Phase 7 (Workers)
    │
    └─→ Phase 8 (Bots)
            ↓
        Phase 9 (Testing)
            ↓
        Phase 10 (Deployment)
```

---

## Resource Estimates

| Phase     | Lines     | Tests  | Days      | Priority |
| --------- | --------- | ------ | --------- | -------- |
| 5         | 250       | 20     | 3-4       | HIGH     |
| 6         | 200       | 15     | 2-3       | HIGH     |
| 7         | 300       | 15     | 4-5       | CRITICAL |
| 8         | 400       | 20     | 4-5       | MEDIUM   |
| 9         | 500       | -      | 4-5       | HIGH     |
| 10        | 300       | -      | 3-4       | HIGH     |
| **Total** | **1,950** | **70** | **21-26** | -        |

---

## Success Criteria for Each Phase

### Phase 5: Spotify

- ✅ Search results return tracks
- ✅ Play control works
- ✅ Token refresh is automatic
- ✅ Multiple devices supported

### Phase 6: YouTube

- ✅ Video search works
- ✅ Queue functionality
- ✅ Comparable feature set to Spotify

### Phase 7: Workers & WebSocket

- ✅ Autonomous queue playback
- ✅ Real-time updates to all clients
- ✅ <100ms message latency
- ✅ Graceful error recovery

### Phase 8: Bots

- ✅ Telegram integration working
- ✅ Slack integration working
- ✅ Commands functional
- ✅ Response formatting clear

### Phase 9: Testing

- ✅ 100% coverage on core
- ✅ No critical bugs
- ✅ Performance acceptable
- ✅ Documentation complete

### Phase 10: Deployment

- ✅ Docker builds successfully
- ✅ Can run on single server
- ✅ Can run in Kubernetes
- ✅ Monitoring configured
- ✅ Logging functional

---

## Risk Mitigation

| Risk                      | Probability | Mitigation                            |
| ------------------------- | ----------- | ------------------------------------- |
| Spotify API changes       | Medium      | Use versioned APIs, monitor changelog |
| Performance degradation   | Medium      | Profile early, benchmark              |
| WebSocket connection loss | Low         | Implement reconnect with backoff      |
| Database deadlocks        | Low         | Proper transaction handling           |
| Token expiration issues   | Low         | Proactive refresh before expiry       |

---

## Rollout Strategy

### Staging Environment

1. Deploy Phase 5-6 to staging
2. Run full test suite
3. Performance testing
4. Load testing

### Production Rollout

1. Deploy Phase 7-8 (workers first)
2. Monitor for 24 hours
3. Deploy bots
4. Announce to users

### Backward Compatibility

- Maintain API versioning
- Support both Spotify and YouTube
- Graceful fallbacks for missing features

---

## Next Immediate Step

**Start Phase 5**: Implement Spotify OAuth and track search functionality.

Estimated time: 3-4 days
Key files to create:

- `crates/infrastructure/src/providers/spotify.rs` (~250 lines)
- `crates/api/src/handlers/oauth.rs` (~150 lines)
- `crates/infrastructure/tests/spotify.rs` (~200 lines)

---

**Roadmap Created**: April 16, 2026
**Last Updated**: April 16, 2026
**Status**: Ready for Phase 5

Questions? See PHASE4_AUTH_SECURITY.md for security architecture details.
