# Trading Lab

> High-performance trading strategy builder and historical backtesting platform for Indonesia Stock Exchange (IDX) traders.

Trading Lab is a full-stack platform built with a high-performance **Rust backend** (Actix-web, SQLx, Redis) and a modern **SvelteKit 5 single-page application** (CSR, Bun, Tailwind CSS v4, TradingView lightweight-charts). It enables Indonesia Stock Exchange (IDX) traders to define complex multi-condition trading strategies, backtest them against Indonesian historical market data, analyze detailed risk and performance metrics, and share winning strategies with the community.

---

## Features

- **Visual Strategy Builder**: Construct dynamic multi-group trading rules (`AND`/`OR` grouping) across financial metrics, price/volume indicators, and valuation parameters.
- **Risk & Target Controls**: Configure Take Profit %, Stop Loss %, live Risk-to-Reward ratio calculations, and maximum holding periods.
- **Simulation & Backtest Engine**: Run asynchronous historical simulations with customizable cash allocations, portfolio holding limits, broker fees, and duration spans (1, 3, 6, 12 months).
- **Comprehensive Analytics**: Track real-time equity curves (Net & Gross), Sharpe Ratio (2% risk-free rate), Profit Factor, Win/Loss Rate, Max Drawdown/Volatility, Average Holding Times, Top Gainers/Losers, and Most Traded tickers.
- **Community Leaderboard & Starring**: Explore public backtests ranked by return percentage or popularity, star favorite strategies, and inspect trade breakdowns.
- **Authentication & RBAC**: Secure Argon2id password hashing, JWT access/refresh token rotation with Redis session revocation, user profile controls, and Admin user management.

---

## Tech Stack

### Backend (Rust)
- **Framework**: [Actix-web 4](https://actix.rs/)
- **Database**: PostgreSQL 16 via [SQLx 0.8](https://github.com/launchbadge/sqlx) (async, parameterized queries)
- **Session Cache**: Redis 7 via `redis-rs` (Tokio connection manager)
- **Market Data**: [Reqwest](https://docs.rs/reqwest/) client with Redis caching and 429 retry backoff (Sectors.app API)
- **Security**: Argon2id (`argon2`), JWT (`jsonwebtoken`)
- **Validation**: `validator` crate

### Frontend (SvelteKit & Svelte 5)
- **Framework**: [SvelteKit 2](https://kit.svelte.dev/) with **Svelte 5 Runes** (`$state`, `$derived`, `$props`, `$effect`)
- **Mode**: Pure Client-Side Rendering (CSR / SPA)
- **Runtime & Package Manager**: [Bun](https://bun.sh/)
- **Styling**: [Tailwind CSS v4](https://tailwindcss.com/) with CSS design token variables
- **Charts**: [TradingView Lightweight Charts](https://tradingview.github.io/lightweight-charts/) & native SVG charts
- **Icons**: `@iconify/svelte` (Lucide icon set)
- **Testing**: [Vitest](https://vitest.dev/) with `@testing-library/svelte` and `jsdom`

---

## Project Structure

```
trading-lab/
├── backend/                  # Rust API service
│   ├── migrations/           # SQLx migration files (PostgreSQL schema)
│   ├── src/
│   │   ├── clients/          # Sectors.app financial market data client
│   │   ├── entities/         # Domain models, requests, responses, errors
│   │   ├── guards/           # Actix-web request extractors (JWT & RBAC)
│   │   ├── helpers/          # Cryptography, JWT, hashing helpers
│   │   ├── repositories/     # PostgreSQL SQLx query layer
│   │   ├── routes/           # REST API endpoints
│   │   ├── services/         # Business logic & simulation engine
│   │   └── setup/            # Infrastructure setup (DB, Redis, HTTP)
│   ├── tests/                # Contract & integration tests
│   ├── Cargo.toml            # Rust dependencies & profiles
│   ├── Dockerfile            # Multi-stage Rust build container
│   ├── .env.example          # Host environment template
│   └── .env.docker.example   # Docker container environment template
│
├── frontend/                 # SvelteKit 5 SPA
│   ├── src/
│   │   ├── lib/
│   │   │   ├── api.ts        # Typed API client with auto-refresh deduplication
│   │   │   ├── constants.ts  # Financial indicator definitions & storage keys
│   │   │   ├── components/   # UI components (dashboard, strategy, backtest, common)
│   │   │   └── helpers/      # Client session, theme, and reactive toast helpers
│   │   └── routes/           # CSR page routes (Dashboard, Strategies, Backtests, Admin)
│   ├── tests/unit/           # Vitest component and unit test suites
│   ├── Dockerfile            # Multi-stage Bun build -> Nginx Alpine container
│   ├── nginx.conf            # Nginx SPA fallback configuration
│   ├── package.json          # Frontend dependencies & scripts
│   ├── .env.example          # Frontend environment template
│   └── .env.docker.example   # Frontend Docker environment template
│
├── docker-compose.yml        # Root Docker Compose (PostgreSQL, Redis, Backend, Frontend)
├── AGENTS.md                 # Developer & AI Agent architectural guidelines
└── README.md                 # Project overview and setup guide
```

---

## Getting Started

### Prerequisites

Ensure you have the following installed:
- [Rust](https://www.rust-lang.org/) (latest stable toolchain)
- [Bun](https://bun.sh/) (v1.2+)
- [Docker](https://www.docker.com/) & Docker Compose

---

### Quick Start with Docker (Full Stack)

To run the complete platform (PostgreSQL, Redis, Rust Backend, and SvelteKit Frontend) with Docker Compose:

1. **Configure Docker environment files**:
   ```sh
   cp backend/.env.docker.example backend/.env.docker
   cp frontend/.env.docker.example frontend/.env.docker
   ```
   *Edit `backend/.env.docker` to provide your `SECTORS_API_KEY` and secret keys.*

2. **Start all services**:
   ```sh
   docker compose up -d --build
   ```

3. **Access the application**:
   - Frontend UI: `http://localhost:3000`
   - Backend API: `http://localhost:8000`

---

### Local Development Setup

#### 1. Backend Setup

1. **Start PostgreSQL & Redis infrastructure**:
   ```sh
   docker compose up postgres redis -d
   ```

2. **Configure backend environment variables**:
   ```sh
   cd backend
   cp .env.example .env
   ```
   *Update `SECTORS_API_KEY` and `JWT_SECRET` in `backend/.env`.*

3. **Run the API server**:
   ```sh
   cargo run
   ```
   *Migrations are automatically executed on startup. The API will listen on `http://127.0.0.1:8000`.*

---

#### 2. Frontend Setup

1. **Navigate to the frontend directory**:
   ```sh
   cd frontend
   ```

2. **Configure environment variables**:
   ```sh
   cp .env.example .env
   ```
   *Default points to `PUBLIC_API_BASE_URL=http://127.0.0.1:8000`.*

3. **Install dependencies**:
   ```sh
   bun install
   ```

4. **Start local development server**:
   ```sh
   bun run dev
   ```
   *The web application will be accessible at `http://localhost:3000`.*

---

## Default Credentials

A default administrator account is seeded upon initial database migration:

| Role | Email | Password |
| :--- | :--- | :--- |
| **Administrator** | `admin@mail.com` | `admin123` |

> *Note: Remember to update default credentials before deploying to production.*

---

## Development & Verification Commands

### Backend (Rust)
```sh
cd backend

# Check formatting
cargo fmt --check

# Run test suite
cargo test

# Run strict Clippy lints
cargo clippy --all-targets --all-features --locked -- -D warnings
```

### Frontend (SvelteKit / Bun)
```sh
cd frontend

# Svelte & TypeScript type checks
bun run check

# Run ESLint
bun run lint

# Auto-fix linting issues
bun run lint:fix

# Run unit and component tests
bun run test:unit

# Check Prettier formatting
bun run format:check

# Format code & Tailwind class order
bun run format
```

---

## Contributor & Agent Guidelines

For detailed architectural principles, coding conventions, domain invariants, and API response structures, refer to [`AGENTS.md`](AGENTS.md).
