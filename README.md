# Trading Lab

> Everyone built a stock screener, but no one ever backtested it!

Trading Lab is a full-stack platform built with a high-performance **Rust backend** (Actix-web, SQLx, Redis) and a modern **SvelteKit 5 single-page application** (CSR, Bun, Tailwind CSS v4, TradingView lightweight-charts). It enables Indonesia Stock Exchange (IDX) traders to define complex multi-condition trading strategies, backtest them against Indonesian historical market data, analyze detailed risk and performance metrics, and share winning strategies with the community.

---

## Landing Page

The public landing page (`/`) is a cinematic, parallax-enabled marketing surface with:

- **Visual world**: Deep-space financial observatory — cosmic void background, luminous orbital arc illustration with IDX ticker constellations, WebGL-like CSS shader starfield canvas, and nebula atmosphere.
- **Typography**: [Barlow Condensed](https://fonts.google.com/specimen/Barlow+Condensed) (display / headlines) + [Figtree](https://fonts.google.com/specimen/Figtree) (body / UI copy). Neither is Montserrat, preserving a distinct landing identity from the app's dashboard.
- **Sections**: Hero → Feature highlights (4 cards) → App screenshots → 3-step workflow → Community leaderboard preview → CTA → Footer.
- **Themes**: Dark mode only — immersing visitors in the deep cosmic void aesthetic without distractions.
- **Motion**: Parallax scroll layers on hero orbital image, twinkling starfield canvas animation, staggered section reveals via IntersectionObserver, animated orbital rings on the CTA section, and reduced-motion fallback.
- **Static assets** (in `frontend/static/`):
  - `landing-hero.jpg` — IDX orbital arc constellation illustration
  - `landing-nebula.jpg` — Deep space nebula background
  - `landing-ai.jpg` — AI neural network illustration
  - `screenshot-dashboard.png`, `screenshot-backtest.png`, `screenshot-ai.png` — App UI screenshots

---

## Features

### 🛠️ Strategy Builder & Rule Engine
- **Visual Condition Builder**: Construct dynamic, multi-group screening rules with customizable intra-group and inter-group logical connectors (`AND` / `OR`).
- **Comprehensive IDX Indicator Catalog**: Screen stocks across categorized financial metrics, including Valuation (P/E, P/B), Profitability (ROE, ROA, Net Margin), Solvency (DER, Current Ratio), Dividend Yield, and Technical Price/Volume indicators.
- **Dynamic Comparisons**: Compare variables against fixed numeric thresholds, percentage changes, or cross-metric conditions.
- **One-Click Strategy Duplication**: Easily duplicate existing strategies with automated unique naming (`{Name} (Copy)`) to rapidly iterate on rule variations.

### 🤖 AI-Powered Trading Intelligence (Google Gemini)
- **AI Strategy Refinement**: Intercepts strategy creation and modification to analyze Take-Profit, Stop-Loss, holding duration, and conditional screening rules against IDX market dynamics, offering one-click "Accept" or "Ignore" suggestions to optimize risk-reward parameters and rule conditions (adding variables, modifying thresholds).
- **AI Executive Summary**: Generates a structured, 5-point qualitative breakdown on completed backtests (Market Alignment, Risk-Adjusted Efficiency, Profit Drivers, Drawdown Exposure, and Iteration Advice).
- **Persistent AI Caching**: AI summaries are cached directly in PostgreSQL and Redis for zero-latency retrieval without redundant LLM calls.
- **Platform-Wide Master Toggle**: Administrators can dynamically enable or disable AI capabilities across the entire platform via app settings.

### 🎯 Risk & Trade Management
- **Target & Stop Controls**: Configure explicit Take-Profit (%) and Stop-Loss (%) exit thresholds.
- **Live Risk-to-Reward ($R:R$) Ratio**: Real-time calculated Risk-to-Reward ratio with adaptive visual feedback when risk exceeds reward.
- **Holding Period Limits**: Enforce maximum holding periods (in days) to automatically liquidate stagnant positions and protect capital velocity.

### ⚡ Historical Backtesting Engine
- **Asynchronous Simulation**: High-performance historical simulation engine running in non-blocking Tokio background tasks.
- **Real Market Data Integration**: Powered by Sectors.app IDX market data with permanent Redis caching, cache-miss telemetry, and 429 rate-limit backoff retries.
- **Customizable Simulation Parameters**: Configure starting capital, maximum simultaneous portfolio holdings, broker commission fees (buy/sell), and historical durations (1, 3, 6, 12 months).

### 📊 Deep Analytics & Interactive Visualizations
- **Interactive Equity Curve**: High-performance interactive baseline equity chart powered by **TradingView Lightweight Charts** with Jakarta timezone rendering and Net vs Gross equity toggling.
- **Key Performance Metrics**: Instant calculation of Total Return %, Win Rate %, Profit Factor, Sharpe Ratio (2% risk-free rate), Portfolio Volatility, and Average Holding Time.
- **Win/Loss Doughnut & Sparklines**: Semicircular SVG win vs loss distribution chart and mini sparkline equity previews on list cards.
- **Detailed Trade Telemetry**: In-depth breakdown of Top Gainers, Top Losers, Most Traded Tickers, and a complete trade execution history log with exit reasons (Take-Profit, Stop-Loss, Max Holding Days, or Period End).

### 🌐 Community Leaderboard & Social Collaboration
- **Public Strategy Discovery**: Explore top-performing strategies shared by the community, ranked by **Highest Net Return** or **Top Starred**.
- **Interactive Starring**: Star favorite community backtests to curate a personal collection and highlight leading strategies on the leaderboard.
- **Granular Privacy Controls**: Toggle simulation visibility (`is_public`) at any time to share winning backtests with the community or keep proprietary strategies private.

### 🔒 Enterprise Security & Administration
- **Stateful JWT Session Management**: JWT access and refresh token pair rotation backed by Redis session tracking with instantaneous revocation on logout or password change.
- **Role-Based Access Control (RBAC)**: Distinct permissions for `ADMIN` and `MEMBER` roles with route extractors and client-side guards.
- **Admin Management Console**: Dedicated admin panel for managing registered users, creating accounts with explicit roles, resetting credentials, and moderating content.
- **Profile & Credential Management**: User self-service modal dialogs for updating profile details and securely changing passwords with Argon2id hashing.

---

## Tech Stack

### Backend (Rust)
- **Framework**: [Actix-web 4](https://actix.rs/)
- **Database**: PostgreSQL 16 via [SQLx 0.8](https://github.com/launchbadge/sqlx) (async, parameterized queries)
- **Session Cache**: Redis 7 via `redis-rs` (Tokio connection manager)
- **Market Data**: [Reqwest](https://docs.rs/reqwest/) client with Redis caching and 429 retry backoff (Sectors.app API)
- **AI Intelligence**: Google Gemini API (`gemini-3.1-flash-lite`) via `LLMTrait` / `GeminiLLM`
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
│   │   ├── clients/          # Sectors.app financial market data & Gemini LLM clients
│   │   ├── constants/        # Global constants & financial indicator definitions
│   │   ├── entities/         # Domain models, requests, responses, errors, settings
│   │   ├── enums/            # System enums (UserRole, BacktestStatus, TokenType)
│   │   ├── guards/           # Actix-web request extractors (AuthenticatedUser, RequireAdmin)
│   │   ├── helpers/          # Cryptography, JWT, Argon2 hashing helpers
│   │   ├── repositories/     # PostgreSQL SQLx query layer
│   │   ├── routes/           # REST API endpoints (users, strategies, backtests, dashboard, settings)
│   │   ├── services/         # Business logic, calculation routines & simulation engine
│   │   ├── setup/            # Infrastructure setup (PostgreSQL, Redis, HTTP client)
│   │   ├── di.rs             # AppDependencies container
│   │   ├── http.rs           # Server middleware & CORS configuration
│   │   ├── lib.rs            # Library entrypoint
│   │   └── main.rs           # Application server entrypoint
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
│   │   │   ├── types.ts      # TypeScript interfaces and domain models
│   │   │   ├── components/   # UI components (dashboard, strategy, backtest, common)
│   │   │   └── helpers/      # Client session, theme, and reactive toast helpers
│   │   └── routes/           # CSR page routes (Dashboard, Strategies, Backtests, Settings, Users)
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
