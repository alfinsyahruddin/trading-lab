# Trading Lab Backend

High-performance REST API and historical backtest simulation service for the **Trading Lab** platform, built with **Rust**, **Actix-web 4**, **PostgreSQL 16** via SQLx, and **Redis 7**.

---

## Features & Architecture

- **High-Performance API Engine**: Powered by Actix-web 4 with asynchronous non-blocking I/O.
- **Layered Clean Architecture**: Strict separation of concerns across Entities/DTOs, Guards, Helpers, External Clients, Repositories (SQLx), Business Services, and Routes.
- **Financial Market Data Client**: Custom `SectorsClient` interfacing with Sectors.app (`/v2/companies/` and `/v2/daily/{symbol}/`) with permanent Redis caching, 429 rate-limit backoff retries, and cache miss diagnostics.
- **Historical Backtest Simulation Engine**: Modular historical simulation engine under `src/services/backtest/` (`engine.rs`, `rules.rs`) calculating trade executions, P/L, win rates, profit factor, Sharpe ratio (2% risk-free rate), max volatility, and daily net/gross equity curves.
- **AI Intelligence & Summary (Google Gemini)**: Automated pre-submission strategy parameter refinement suggestions and 5-dimension qualitative backtest executive summaries with Redis setting toggles and persistent caching.
- **Session & Auth Management**: Argon2id password hashing, JWT access/refresh token pairs, and stateful session tracking in Redis with instantaneous revocation on logout, password change, or admin moderation.
- **Role-Based Access Control (RBAC)**: Fine-grained access extractors (`AuthenticatedUser`, `RequireAdmin`) for protected endpoints and administrative actions.
- **Unified JSON Envelope**: Consistent API response formatting (`data`, `status`, `message`, `timestamp`) across all successful responses and domain errors (`AppError`).

---

## Tech Stack

- **Language**: Rust (latest stable)
- **Web Framework**: [Actix-web 4](https://actix.rs/)
- **Database Access**: [SQLx 0.8](https://github.com/launchbadge/sqlx) (PostgreSQL 16 async driver with parameterized queries)
- **Session Cache**: Redis 7 via `redis-rs` (Tokio connection manager)
- **HTTP Client**: [Reqwest](https://docs.rs/reqwest/) (with JSON serialization and retry logic)
- **Security & Cryptography**: `argon2` (Argon2id), `jsonwebtoken`
- **Validation**: `validator` crate

---

## Directory Structure

```
backend/
├── migrations/               # Sequential SQLx migration files (auto-applied on startup)
├── src/
│   ├── clients/              # External clients (Sectors.app market data, Gemini LLM)
│   ├── constants/            # Global constants & indicator mappings
│   ├── entities/             # Request/response DTOs, domain models, AppConfig, AppError
│   ├── enums/                # UserRole, BacktestStatus, TokenType
│   ├── guards/               # Actix-web request extractors (AuthenticatedUser, RequireAdmin)
│   ├── helpers/              # Cryptographic hashing & JWT token utilities
│   ├── repositories/         # Database persistence layer (PostgreSQL / SQLx)
│   ├── routes/               # Thin HTTP route handlers
│   ├── services/             # Business logic, calculation routines, backtest simulation
│   │   └── backtest/         # Modular backtest engine (simulation & screener rules)
│   ├── setup/                # Infrastructure initializers (PostgreSQL, Redis, Reqwest)
│   ├── di.rs                 # AppDependencies dependency injection container
│   ├── http.rs               # HTTP server middleware, CORS, JSON error extractors
│   ├── lib.rs                # Library entrypoint & route registration
│   └── main.rs               # Application runtime entrypoint
├── tests/                    # Integration and HTTP contract tests
├── Cargo.toml                # Rust dependencies and compiler configuration
├── Dockerfile                # Multi-stage production container build
├── .env.example              # Host environment template
└── .env.docker.example       # Docker container environment template
```

---

## Getting Started

### 1. Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable toolchain)
- [Docker](https://www.docker.com/) & Docker Compose (for PostgreSQL & Redis)

### 2. Infrastructure Setup

From the repository root, launch PostgreSQL and Redis:

```sh
docker compose up postgres redis -d
```

### 3. Environment Configuration

Copy the example environment file and configure secrets:

```sh
cp .env.example .env
```

Key environment variables in `backend/.env`:

| Variable | Description | Default |
| :--- | :--- | :--- |
| `HOST` | API server bind address | `127.0.0.1` |
| `PORT` | API server port | `8000` |
| `DATABASE_URL` | PostgreSQL connection string | `postgresql://postgres:postgres@127.0.0.1:5432/trading_lab` |
| `REDIS_URL` | Redis connection string | `redis://127.0.0.1:6379` |
| `JWT_SECRET` | Secret key for JWT signing & verification | *Required* |
| `SECTORS_API_KEY` | Sectors.app API key for IDX financial data | *Required* |
| `SECTORS_API_URL` | Sectors.app base URL | `https://api.sectors.app` |
| `GEMINI_API_KEY` | Google Gemini API key for AI features | *Optional / Required for AI* |
| `GEMINI_MODEL` | Google Gemini model identifier | `gemini-3.1-flash-lite` |
| `CORS_ORIGIN` | Allowed CORS origin | `http://localhost:3000` |

### 4. Run the API Server

```sh
cargo run
```

*Database migrations (`backend/migrations/`) are executed automatically on server startup. The API will listen on `http://127.0.0.1:8000`.*

---

## Default Administrator Credentials

A default administrator account is automatically seeded upon initial database migration:

| Role | Email | Password |
| :--- | :--- | :--- |
| **Administrator** | `admin@mail.com` | `admin123` |

> *Note: Update development credentials before deploying to staging or production environments.*

---

## API Endpoints Reference

All endpoints return a unified JSON envelope:
```json
{
  "data": T | null,
  "status": 200,
  "message": "Optional message" | null,
  "timestamp": "2026-09-02T00:00:00.000Z"
}
```

### 1. Authentication & User Profile (`/api/users`)
- `POST /api/users/register` — Public user registration (assigns `MEMBER` role)
- `POST /api/users/login` — Authenticate and obtain JWT access/refresh token pair
- `POST /api/users/refresh` — Exchange refresh token for new access token
- `POST /api/users/logout` — Invalidate current active session in Redis
- `GET /api/users/me` — Retrieve current user profile
- `PATCH /api/users/me` — Update current user profile (name, email)
- `POST /api/users/me/password` — Change current user password

### 2. Admin User Management (`/api/users` — Admin Only)
- `GET /api/users` — List all registered users
- `GET /api/users/{id}` — Retrieve user details by ID
- `POST /api/users` — Create user with explicit role (`ADMIN` or `MEMBER`)
- `PATCH /api/users/{id}` — Update user profile, role, or password
- `DELETE /api/users/{id}` — Delete user account (prevents self-deletion)

### 3. Trading Strategies (`/api/strategies`)
- `GET /api/strategies` — List trading strategies owned by authenticated user
- `GET /api/strategies/{id}` — Retrieve strategy details, rules, and parameters
- `POST /api/strategies` — Create a new trading strategy
- `PATCH /api/strategies/{id}` — Update strategy parameters and filtering rules
- `DELETE /api/strategies/{id}` — Delete a trading strategy
- `POST /api/strategies/{id}/duplicate` — Duplicate strategy with a unique copy name
- `POST /api/strategies/ai-suggestions` — Request AI parameter and conditional rule recommendations for trading strategies

### 4. Backtest Simulations (`/api/backtests`)
- `GET /api/backtests` — List backtest jobs owned by authenticated user with sparklines
- `GET /api/backtests/{id}` — Get full backtest results, equity curves, metrics, and trade history
- `POST /api/backtests` — Configure and trigger background backtest simulation
- `PATCH /api/backtests/{id}` — Update backtest metadata (e.g. toggle `is_public` visibility)
- `DELETE /api/backtests/{id}` — Delete backtest job and its historical cascade data

### 5. Dashboard & Community (`/api/dashboard`)
- `GET /api/dashboard/stats` — Retrieve authenticated user KPI statistics
- `GET /api/dashboard/leaderboard` — Discover top public backtests ranked by return percentage
- `GET /api/dashboard/top-stars` — Discover top public backtests ranked by community stars
- `POST /api/dashboard/stars/{backtest_id}` — Star a public backtest
- `DELETE /api/dashboard/stars/{backtest_id}` — Remove star from a public backtest

### 6. App Settings (`/api/settings`)
- `GET /api/settings` — Retrieve current platform settings (e.g. `ai_enabled`)
- `PATCH /api/settings` — Update platform settings (Admin only)

---

## Development & Verification

Run these checks before submitting backend changes:

```sh
# Format check
cargo fmt --check

# Run unit and integration tests
cargo test

# Strict Clippy lints
cargo clippy --all-targets --all-features --locked -- -D warnings
```
