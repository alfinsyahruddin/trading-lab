# Trading Lab — Agent & Contributor Guide

Welcome to the **Trading Lab** codebase. This document outlines the architectural principles, workflows, coding conventions, and verification steps for AI agents and human contributors working across the backend and frontend.

---

## 1. Repository Overview & Architecture

Trading Lab is a full-stack trading analysis and backtesting platform for Indonesia Stock Exchange (IDX) traders, consisting of a high-performance **Rust backend** and a modern **SvelteKit 5 single-page application (SPA)**.

```
trading-lab/
├── backend/                  # Rust API service (Actix-web, SQLx, Redis, Reqwest)
│   ├── migrations/           # SQLx database migration files (sequential SQL)
│   ├── src/                  # Layered backend source code
│   │   ├── clients/          # External API clients (Sectors.app integration)
│   │   ├── constants/        # Global constants
│   │   ├── entities/         # Domain models, error types, responses, configuration
│   │   ├── enums/            # System enums (e.g., UserRole, BacktestStatus, TokenType)
│   │   ├── guards/           # Actix-web request extractors (auth & RBAC)
│   │   ├── helpers/          # Cryptography, JWT, Argon2 hashing utilities
│   │   ├── repositories/     # Database access layer (PostgreSQL / SQLx)
│   │   ├── routes/           # HTTP route handlers & endpoints
│   │   ├── services/         # Business logic and simulation orchestration
│   │   ├── setup/            # Infrastructure initializers (DB, Redis, HTTP client)
│   │   ├── di.rs             # Dependency injection container
│   │   ├── http.rs           # HTTP middleware, CORS, JSON configuration
│   │   ├── lib.rs            # Library entrypoint
│   │   └── main.rs           # Application server entrypoint
│   ├── tests/                # Integration and contract tests
│   ├── Cargo.toml            # Rust dependencies & profiles
│   ├── Dockerfile            # Container build specification
│   ├── docker-compose.yml    # Docker Compose for PostgreSQL & Redis
│   ├── .env.example          # Host environment template
│   └── .env.docker.example   # Docker container environment template
│
├── frontend/                 # SvelteKit 5 SPA (CSR only, Bun, TailwindCSS v4)
│   ├── src/
│   │   ├── app.css           # TailwindCSS v4 & CSS design token theme
│   │   ├── app.html          # HTML template shell
│   │   ├── lib/
│   │   │   ├── api.ts        # Typed REST API client & response unwrapper
│   │   │   ├── constants.ts  # Shared constants, storage keys, indicator variables
│   │   │   ├── types.ts      # TypeScript interfaces and domain types
│   │   │   ├── components/   # Reusable Svelte 5 UI components
│   │   │   │   ├── backtest/ # Backtest preview, chart, and status components
│   │   │   │   ├── dashboard/# Dashboard stats and leaderboard cards
│   │   │   │   └── strategy/ # Strategy form, rule builder, and variable picker
│   │   │   └── helpers/      # Client session, theme, and reactive toast helpers
│   │   └── routes/           # SvelteKit client-side routes (+layout, +page)
│   ├── tests/unit/           # Vitest unit & component tests
│   ├── eslint.config.js      # ESLint flat config (Svelte 5, TS, Tailwind plugin)
│   ├── .prettierrc           # Prettier config with Svelte & Tailwind sorting plugins
│   ├── package.json          # Frontend dependencies & scripts
│   ├── tsconfig.json         # TypeScript configuration
│   ├── vite.config.ts        # Vite & Tailwind setup
│   └── .env.example          # Frontend environment template
│
├── AGENTS.md                 # Developer & AI Agent architectural guidelines
└── README.md                 # Project overview and setup guide
```

---

## 2. Backend Guidelines (Rust)

### Tech Stack
- **Framework**: [Actix-web 4](https://actix.rs/)
- **Database**: PostgreSQL 16 via [SQLx 0.8](https://github.com/launchbadge/sqlx) (async, compile-time checked / parameterized queries)
- **Session Cache**: Redis 7 via `redis-rs` (connection manager with Tokio)
- **External Integration**: `reqwest` with Redis caching and retry backoff
- **Security**: Argon2id for password hashing, `jsonwebtoken` for access/refresh tokens
- **Validation**: `validator` crate for request payload validation

### Layered Architecture
Backend logic is strictly segregated into distinct layers:
1. **Entities & DTOs** (`src/entities/`): Request/response structs, configurations (`AppConfig`), and errors (`AppError`).
2. **Guards / Extractors** (`src/guards/`):
   - `AuthenticatedUser`: Validates JWT access token and active session in Redis; extracts user claims (`sub`, `sid`, `role`).
   - `RequireAdmin`: Ensures the authenticated user possesses the `ADMIN` role.
3. **Helpers** (`src/helpers/`): Pure utility functions (`hash_helper` for Argon2, `token_helper` for JWT signing/decoding).
4. **Clients** (`src/clients/`): External third-party integrations (e.g. `SectorsClient` for Sectors.app financial data with Redis caching and 429 retry backoff).
5. **Repositories** (`src/repositories/`): Direct SQLx queries against PostgreSQL. Repositories handle database interactions only.
6. **Services** (`src/services/`): Business logic, calculation routines, background task execution, and orchestration between repositories, Redis, and external clients.
7. **Routes** (`src/routes/`): Thin HTTP controllers that extract parameters/guards, invoke services, and transform results into standard responses.
8. **Dependency Injection** (`src/di.rs`): `AppDependencies` struct registered with Actix `app_data`.

### API Envelope & Contract
All responses must strictly adhere to the unified JSON envelope:
```json
{
  "data": T | null,
  "status": 200,
  "message": "Optional message" | null,
  "timestamp": "2026-08-30T00:00:00.000Z"
}
```
- Handlers should return `AppResponse<T>` (defined as `Result<Json<BaseResponse<T>>, AppError>`).
- Use the `IntoResponseTrait` helper `service_call().await.json()` or string helper `.json()` instead of manually constructing JSON envelopes.
- Route handlers must remain thin without business logic.

### Error Handling & Invariants
- Use `Result<T, AppError>` for all fallible backend operations and propagate with `?`.
- `AppError` maps domain errors (`BadRequest`, `Unauthorized`, `Forbidden`, `NotFound`, `Conflict`, `Internal`) to standard HTTP statuses and unified error responses.
- Database, Redis, and JWT errors implement `From<...>` for `AppError`, logging details to stderr and masking internal implementation details as `AppError::Internal`.
- Avoid `unwrap()` and `expect()` in production code. Only use them in tests where panic on failure is expected.

### Core Backend Domains

#### 1. Authentication & Users (`src/routes/user_route.rs`)
- **Password Hashing**: Argon2id via `hash_helper`.
- **Session Lifecycle**: Each login creates a JWT access/refresh pair containing a session ID (`sid`). The session is recorded in Redis under `auth:session:{user_id}:{session_id}` with TTL.
- **Session Revocation**: Any critical mutation (logout, password update, role change, account deletion) revokes active sessions in Redis.
- **RBAC**: Public registration `/api/users/register` assigns the `MEMBER` role. Only authenticated `ADMIN` users can access user management or create `ADMIN` accounts.
- **Endpoints**:
  - `POST /api/users/register`: Register a new member account.
  - `POST /api/users/login`: Authenticate and obtain token pair + user profile.
  - `POST /api/users/refresh`: Exchange refresh token for new access token.
  - `POST /api/users/logout`: Invalidate current active session in Redis.
  - `GET /api/users/me`: Retrieve current user profile.
  - `PATCH /api/users/me`: Update current user profile name/email.
  - `POST /api/users/me/password`: Change current user password.
  - `GET /api/users`: List all users (Admin only).
  - `GET /api/users/{id}`: Get user by ID (Admin only).
  - `POST /api/users`: Create user with explicit role (Admin only).
  - `PATCH /api/users/{id}`: Update user profile/role/password (Admin only).
  - `DELETE /api/users/{id}`: Delete user (Admin only, self-deletion prevented).

#### 2. Trading Strategy Domain (`src/routes/trading_strategy_route.rs`)
- **Data Model**: `trading_strategies` stores strategy metadata (name, description, take-profit %, stop-loss %, max holding period in days) and dynamic multi-group filtering rules in `rules JSONB`.
- **Unique Name Constraint**: Strategy names are unique per user via `UNIQUE(user_id, name)`. Duplicate naming attempts return `AppError::Conflict` (409).
- **User Scoping**: User strategy listing is strictly scoped to the authenticated user (`WHERE user_id = $1`).
- **Endpoints**:
  - `GET /api/strategies`: List all strategies owned by the authenticated user.
  - `GET /api/strategies/{id}`: Retrieve strategy details.
  - `POST /api/strategies`: Create a new strategy.
  - `PATCH /api/strategies/{id}`: Update an existing strategy.
  - `DELETE /api/strategies/{id}`: Delete a strategy.
  - `POST /api/strategies/{id}/duplicate`: Duplicate an existing strategy with a new unique name.

#### 3. Backtest Domain (`src/routes/backtest_route.rs`)
- **Data Model**: `backtest_jobs`, `backtest_results`, `backtest_portfolio_history`, `backtest_trades`, and `backtest_stars`.
- **Unique Name Constraint**: Backtest job names are unique per user via `UNIQUE(user_id, name)`.
- **Third-Party Client**: `SectorsClientTrait` and `SectorsClient` fetch data from the Sectors.app API (`/v2/companies/` screener and `/v2/daily/{symbol}/` daily transactions) with Redis caching (no expiration), 429 retries (2x with 1s delay), and console logging on cache misses.
- **Simulation Engine**: Asynchronous simulation via `tokio::spawn` calculating P/L, win rate, profit factor, Sharpe ratio (2% risk-free rate), portfolio volatility, and tracking daily net/gross equity curve values.
- **Endpoints**:
  - `GET /api/backtests`: List all backtests owned by the authenticated user with sparkline history.
  - `GET /api/backtests/{id}`: Retrieve full backtest details, metrics, portfolio history, top gainers/losers, most traded, and trade history.
  - `POST /api/backtests`: Create and trigger background backtest simulation.
  - `PATCH /api/backtests/{id}`: Update backtest metadata (e.g. toggle `is_public` visibility).
  - `DELETE /api/backtests/{id}`: Delete a backtest job and its cascade data.

#### 4. Dashboard & Community Domain (`src/routes/dashboard_route.rs`)
- **Community Leaderboard**: Discovers and ranks public backtests (`is_public = true` and `status = 'DONE'`) by net return percentage or total community stars.
- **Starring System**: Users can star/unstar public backtests (`backtest_stars` table with `UNIQUE(user_id, backtest_id)`).
- **User KPI Aggregates**: Provides aggregate metrics including total stars received across user's public backtests, total strategies created, total backtests run, and registration timestamp.
- **Endpoints**:
  - `GET /api/dashboard/stats`: Retrieve current user stats and metrics.
  - `GET /api/dashboard/leaderboard`: Retrieve top public backtests ranked by return percentage.
  - `GET /api/dashboard/top-stars`: Retrieve top public backtests ranked by star count.
  - `POST /api/dashboard/stars/{backtest_id}`: Star a public backtest.
  - `DELETE /api/dashboard/stars/{backtest_id}`: Unstar a public backtest.

### Database & Migrations
- Migrations reside in `backend/migrations/` with format `YYYYMMDDNNNN_description.sql`.
- Migrations are applied automatically on startup (`setup_db.rs`).
- **Never modify existing migration files** that have already run. Always create a new sequential migration.
- Always use parameterized queries in SQLx. Map unique constraint violations (e.g. duplicate email, duplicate strategy name) to `AppError::conflict`.

### Backend Development Workflow
Run all backend commands from the `backend/` directory:

```sh
# Start PostgreSQL & Redis services
docker compose --env-file .env.docker up -d

# Run API server on host
cargo run

# Check formatting
cargo fmt --check

# Run tests
cargo test

# Run strict Clippy lints
cargo clippy --all-targets --all-features --locked -- -D warnings
```

---

## 3. Frontend Guidelines (SvelteKit & Svelte 5)

### Tech Stack
- **Framework**: [SvelteKit 2](https://kit.svelte.dev/) with **Svelte 5 Runes**
- **Mode**: Pure Client-Side Rendering (CSR / SPA mode: `export const ssr = false;` in `src/routes/+layout.ts`)
- **Package Manager & Runtime**: [Bun](https://bun.sh/)
- **Styling**: [TailwindCSS v4](https://tailwindcss.com/) with CSS custom property tokens
- **Linting & Formatting**: ESLint 9 (with `eslint-plugin-tailwindcss` & `eslint-plugin-svelte`) + Prettier (with `prettier-plugin-tailwindcss` & `prettier-plugin-svelte`)
- **Icons**: `@iconify/svelte` (Lucide icon set)
- **Charts**: `lightweight-charts` (TradingView) for equity curves, native SVG for semicircular win/loss doughnuts and sparklines
- **Testing**: [Vitest](https://vitest.dev/) with `@testing-library/svelte` and `jsdom`

### State & Runes Conventions
- All Svelte components must strictly use **Svelte 5 Runes**:
  - `$props()` for typed component properties.
  - `$state()` for reactive variables and objects.
  - `$derived()` or `$derived.by()` for computed values.
  - `$effect()` for side effects and DOM subscriptions.
- Reactive helper modules outside `.svelte` files must use the `.svelte.ts` extension (e.g., `src/lib/helpers/toast.svelte.ts`).
- Always specify unique keys for each blocks: `{#each items as item (item.id)}`.

### Client-Side Routing & Auth Guards
Since the application runs in pure CSR mode, route protection executes on the client:
- `src/routes/+page.ts`, `login/+page.ts`, `register/+page.ts`: Redirect authenticated users to `/dashboard`.
- `src/routes/dashboard/+layout.ts`: Checks `getToken()`; redirects unauthenticated visitors to `/login`.
- **Routes Breakdown**:
  - `src/routes/dashboard/+page.svelte`: Dashboard home overview featuring user KPI stats cards, community backtest leaderboard (Highest Return & Top Starred), and star interaction.
  - `src/routes/dashboard/strategies/+page.svelte`: Strategy list cards with duplicate, edit, and delete workflows.
  - `src/routes/dashboard/strategies/new/+page.svelte`: Create trading strategy page.
  - `src/routes/dashboard/strategies/[id]/+page.svelte`: Strategy detail view displaying strategy rules, parameters, and associated backtests.
  - `src/routes/dashboard/strategies/[id]/edit/+page.svelte`: Edit trading strategy page.
  - `src/routes/dashboard/backtests/+page.svelte`: Backtest list cards with sparkline previews and status polling.
  - `src/routes/dashboard/backtests/new/+page.svelte`: Run backtest configuration form (with year, duration, fees, cash, and public/private visibility).
  - `src/routes/dashboard/backtests/[id]/+page.svelte`: Comprehensive backtest results, performance metrics, equity chart, top gainers/losers, most traded stocks, and trade log.
  - `src/routes/dashboard/users/+page.svelte`: Admin user management view (role assignment, create user modal, password reset, account deletion).

### UI Component Architecture

#### 1. Common & Shared UI Components (`src/lib/components/`)
- `TopBar.svelte`: Sticky app navigation bar with user dropdown, edit profile modal, change password modal, and theme toggle.
- `Modal.svelte` & `ConfirmModal.svelte`: Accessible dialog modal containers with backdrop blur and action hooks.
- `DataTable.svelte`: Generic responsive tabular view for data lists.
- `SegmentedControl.svelte`: Tab-like button group switch (e.g. for charts, leaderboard tabs, visibility).
- `SelectField.svelte` & `TextField.svelte`: Form input fields with error states, icons, and labels.
- `RoleBadge.svelte`: Badge rendering for `ADMIN` and `MEMBER` roles.
- `ThemeToggle.svelte`: Dark / light mode toggle synced to localStorage and `data-theme`.
- `ToastViewport.svelte`: Fixed viewport for reactive notification toasts.
- `EmptyState.svelte`: Empty placeholder view for lists.

#### 2. Strategy Components (`src/lib/components/strategy/`)
- `StrategyForm.svelte`: Core strategy parameters (name, description, TP%, SL%, live Risk-to-Reward ratio, max holding period days) and rule builder integration.
- `WhereConditionsBuilder.svelte`: Dynamic multi-group rule builder supporting intra-group and inter-group `AND`/`OR` connectors and adaptive operator inputs.
- `VariablePickerModal.svelte`: Categorized financial variable catalog with live search, variable descriptions, selection checkmarks, and category filtering.

#### 3. Backtest Components (`src/lib/components/backtest/`)
- `StatusBadge.svelte`: Indicator badge for `PENDING`, `PROCESSING`, `DONE`, and `FAILED` states.
- `HalfDoughnutChart.svelte`: Semicircle SVG doughnut chart visualizing win vs loss ratio with centered total trade count.
- `PortfolioChart.svelte`: Interactive baseline equity curve powered by `lightweight-charts` with Net/Gross value toggle and Jakarta time rendering.
- `BacktestResultPreview.svelte`: Compact SVG sparkline area chart preview for backtest list cards.

#### 4. Dashboard Components (`src/lib/components/dashboard/`)
- `StatsCard.svelte`: Glassmorphism KPI statistic card with glowing accent orbs, icons, and subtitles.
- `LeaderboardCard.svelte`: Community leaderboard entry card displaying return percentage, win rate, duration, author, sparkline equity chart, and star button.

### API Client & Network Handling
- `src/lib/api.ts` wraps all network requests, attaches `Authorization: Bearer <token>`, automatically refreshes expired access tokens via `/api/users/refresh` with request deduplication, unwraps `BaseResponse<T>`, and raises typed `ApiError` instances on failure.
- Do not make raw `fetch` calls in components; use or extend the typed methods in `src/lib/api.ts`.

### Design System & Styling
Defined in `src/app.css` via Tailwind v4 `@theme` tokens:
- `--color-brand-cyan`: Primary accent `#30B4C9` (buttons, active states, highlights)
- `--color-brand-navy-dark`: Dark background `#2A344C`
- `--color-brand-navy-card`: Dark card background `#3C486A`
- `--color-brand-light-bg`: Light background `#EEF0F6`
- `--color-brand-light-card`: Light card/modal background `#FFFFFF`
- Font: `Montserrat`
- Always use semantic CSS variables (`var(--accent)`, `var(--bg-card)`, `var(--fg)`, `var(--border)`) and utility classes rather than hardcoded hex values.

### Frontend Development Workflow
Run all frontend commands from the `frontend/` directory:

```sh
# Install dependencies
bun install

# Start local development server (port 3000)
bun run dev

# Run Svelte and TypeScript diagnostics
bun run check

# Run ESLint (checks Tailwind semantics, CSS conflicts, shorthands, and Svelte/TS code style)
bun run lint

# Auto-fix ESLint issues
bun run lint:fix

# Run unit and component tests (Vitest)
bun run test:unit

# Check Prettier formatting & Tailwind class order
bun run format:check

# Auto-format codebase & Tailwind class order
bun run format
```

> **Note on Testing**: Always run tests using `bun run test:unit` (or `bun run test`). Avoid running `bun test` directly, as Vitest is configured with `@testing-library/svelte` and `jsdom` for DOM testing.

---

## 4. Environment & Secrets Management

- **Backend Host**: Configured via `backend/.env` (reads `127.0.0.1` / `localhost` for local services).
- **Backend Docker**: Configured via `backend/.env.docker` (used by Docker Compose for container port/credential setup).
- **Frontend**: Configured via `frontend/.env` (`PUBLIC_API_BASE_URL=http://127.0.0.1:8000`).
- **Key Environment Variables**:
  - `DATABASE_URL`: PostgreSQL connection string (`postgresql://postgres:postgres@127.0.0.1:5432/trading_lab`).
  - `REDIS_URL`: Redis connection string (`redis://127.0.0.1:6379`).
  - `JWT_SECRET`: Secret key for signing and verifying JWT tokens.
  - `SECTORS_API_KEY`: API key for Sectors.app financial market data.
  - `SECTORS_API_URL`: Base URL for Sectors.app API (`https://api.sectors.app`).
  - `HOST` & `PORT`: Host and port for Actix-web server (`127.0.0.1:8000`).
  - `CORS_ORIGIN`: Allowed origins for CORS (e.g. `http://localhost:3000`).
- Synchronize `.env.example` and `.env.docker.example` whenever new environment variables are introduced.
- **Never commit secrets, passwords, API tokens, or production credentials to git.**
- Never log full JWT tokens, hashes, or database connection strings containing passwords.

---

## 5. Verification Checklist for Contributors & Agents

Before completing any task, ensure you have verified your changes against the appropriate toolchains:

### For Backend Changes:
1. `cargo fmt --check` passes cleanly.
2. `cargo test` passes all unit and integration tests.
3. `cargo clippy --all-targets --all-features --locked -- -D warnings` completes with zero warnings/errors.
4. Database migrations have been tested against local PostgreSQL if schema was modified.

### For Frontend Changes:
1. `bun run check` produces zero errors and zero warnings.
2. `bun run lint` completes with zero errors.
3. `bun run test:unit` passes all test suites.
4. `bun run format:check` reports clean formatting.

---

## 6. Git & Commit Guidelines

- **Do NOT use Conventional Commits**: Never use prefixes like `feat:`, `fix:`, `chore:`, `refactor:`, `docs:`, `style:`, `test:`, etc.
- **Message Style**: Write concise, clear, and descriptive natural language summaries in title or sentence case (e.g., `Add trading strategy duplication`, `Implement community leaderboard on dashboard`, `Fix navbar alignment on mobile`, `Update ESLint and Prettier configurations`).

