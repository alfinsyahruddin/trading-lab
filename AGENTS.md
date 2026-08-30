# Trading Lab — Agent & Contributor Guide

Welcome to the **Trading Lab** codebase. This document outlines the architectural principles, workflows, coding conventions, and verification steps for AI agents and human contributors working across the backend and frontend.

---

## 1. Repository Overview & Architecture

Trading Lab is a full-stack platform consisting of a high-performance **Rust backend** and a modern **SvelteKit 5 single-page application (SPA)**.

```
trading-lab/
├── backend/                  # Rust API service (Actix-web, SQLx, Redis)
│   ├── migrations/           # SQLx database migration files
│   ├── src/                  # Layered backend source code
│   │   ├── constants/        # Global constants
│   │   ├── entities/         # Domain models, error types, responses, configuration
│   │   ├── enums/            # System enums (e.g., UserRole, TokenType)
│   │   ├── guards/           # Actix-web request extractors (auth & RBAC)
│   │   ├── helpers/          # Cryptography, JWT, hashing utilities
│   │   ├── repositories/     # Database access layer (PostgreSQL / SQLx)
│   │   ├── routes/           # HTTP route handlers
│   │   ├── services/         # Business logic and session management
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
│   │   │   ├── constants.ts  # Shared client constants & storage keys
│   │   │   ├── types.ts      # TypeScript interfaces and types
│   │   │   ├── components/   # Reusable Svelte 5 UI components
│   │   │   └── helpers/      # Client-side session, theme, and reactive toast helpers
│   │   └── routes/           # SvelteKit client-side routes (+layout, +page)
│   ├── tests/unit/           # Vitest unit & component tests
│   ├── eslint.config.js      # ESLint flat config (Svelte 5, TS, Tailwind plugin)
│   ├── .prettierrc           # Prettier config with Svelte & Tailwind sorting plugins
│   ├── package.json          # Frontend dependencies & scripts
│   ├── tsconfig.json         # TypeScript configuration
│   ├── vite.config.ts        # Vite & Tailwind setup
│   └── .env.example          # Frontend environment template
│
└── AGENTS.md                 # This guide
```

---

## 2. Backend Guidelines (Rust)

### Tech Stack
- **Framework**: [Actix-web 4](https://actix.rs/)
- **Database**: PostgreSQL 16 via [SQLx 0.8](https://github.com/launchbadge/sqlx) (async, compile-time checked / parameterized queries)
- **Session Cache**: Redis 7 via `redis-rs` (connection manager with Tokio)
- **Security**: Argon2id for password hashing, `jsonwebtoken` for access/refresh tokens
- **Validation**: `validator` crate for request payload validation

### Layered Architecture
Backend logic is strictly segregated into layers:
1. **Entities & DTOs** (`src/entities/`): Request/response structs, configurations (`AppConfig`), and errors (`AppError`).
2. **Guards / Extractors** (`src/guards/`):
   - `AuthenticatedUser`: Validates JWT access token and active session in Redis; extracts user claims.
   - `RequireAdmin`: Ensures the authenticated user possesses the `ADMIN` role.
3. **Helpers** (`src/helpers/`): Pure utility functions (`hash_helper` for Argon2, `token_helper` for JWT signing/decoding).
4. **Repositories** (`src/repositories/`): Direct SQLx queries against PostgreSQL. Repositories handle database interactions only.
5. **Services** (`src/services/`): Business logic, orchestration between repositories and Redis session store (`AuthService`, `UserService`, `SessionService`).
6. **Routes** (`src/routes/`): Thin HTTP controllers that extract parameters/guards, invoke services, and transform results using response traits.
7. **Dependency Injection** (`src/di.rs`): `AppDependencies` struct registered with Actix `app_data`.

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

### Authentication & Sessions
- **Password Hashing**: Always use Argon2id via `hash_helper`. Never use plain hashes or unauthenticated algorithms.
- **Session Lifecycle**: Each login generates a JWT pair containing a session ID (`sid`). The session is recorded in Redis under `auth:session:{user_id}:{session_id}` with TTL.
- **Session Revocation**: Any critical mutation (user logout, password update, role change, or account deletion) **must** revoke all active sessions in Redis.
- **RBAC**: Public registration `/api/users/register` always assigns the `MEMBER` role. Only authenticated `ADMIN` users can access user management or create `ADMIN` accounts.

### Trading Strategy Domain
- **Data Model**: `trading_strategies` stores strategy metadata (name, description, is_public, tp_percentage, sl_percentage, max_holding_period_days) and dynamic multi-group filtering rules in `rules JSONB`.
- **Unique Name Constraint**: Strategy names are unique per user via `UNIQUE(user_id, name)`. Duplicate naming attempts return `AppError::Conflict` (409).
- **User Scoping**: User strategy listing is strictly scoped to the authenticated user (`WHERE user_id = $1`).
- **Endpoints**:
  - `GET /api/strategies`: List all strategies owned by the authenticated user.
  - `GET /api/strategies/{id}`: Retrieve strategy details.
  - `POST /api/strategies`: Create a new strategy.
  - `PATCH /api/strategies/{id}`: Update an existing strategy.
  - `DELETE /api/strategies/{id}`: Delete a strategy.
  - `POST /api/strategies/{id}/duplicate`: Duplicate an existing strategy with a new unique name.

### Backtest Domain
- **Data Model**: `backtest_jobs`, `backtest_results`, `backtest_portfolio_history`, and `backtest_trades` store backtest execution metadata, aggregate performance statistics, equity curves, and individual trade executions.
- **Unique Name Constraint**: Backtest job names are unique per user via `UNIQUE(user_id, name)`.
- **Third-Party Client**: `SectorsClientTrait` and `SectorsClient` fetch data from the Sectors.app API (`/v2/companies/` screener and `/v2/daily/{symbol}/` daily transactions) with Redis caching (no expiration), 429 retries (2x with 1s delay), and console logging on cache misses.
- **Simulation Engine**: Asynchronous simulation via `tokio::spawn` calculating P/L, win rate, profit factor, Sharpe ratio (2% risk-free rate), portfolio volatility, and tracking daily equity curve values.
- **Endpoints**:
  - `GET /api/backtests`: List all backtests owned by the authenticated user with sparkline history.
  - `GET /api/backtests/{id}`: Retrieve full backtest details, metrics, portfolio history, top gainers/losers, most traded, and trade history.
  - `POST /api/backtests`: Create and trigger background backtest simulation.
  - `DELETE /api/backtests/{id}`: Delete a backtest job and its cascade data.

### Database & Migrations
- Migrations reside in `backend/migrations/` with format `YYYYMMDDNNNN_description.sql`.
- Migrations are applied automatically on startup (`setup_db.rs`).
- **Never modify existing migration files** that have already run. Always create a new sequential migration.
- Always use parameterized queries in SQLx. Map unique constraint violations (e.g. duplicate email) to `AppError::conflict`.

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
- **Styling**: [TailwindCSS v4](https://tailwindcss.com/) with CSS variables
- **Linting & Formatting**: ESLint 9 (with `eslint-plugin-tailwindcss` & `eslint-plugin-svelte`) + Prettier (with `prettier-plugin-tailwindcss` & `prettier-plugin-svelte`)
- **Icons**: `@iconify/svelte` (Lucide icon set)
- **Charts**: `lightweight-charts` (TradingView)
- **Testing**: [Vitest](https://vitest.dev/) with `@testing-library/svelte` and `jsdom`

### State & Runes Conventions
- All Svelte components must use **Svelte 5 Runes**:
  - `$props()` for component properties.
  - `$state()` for reactive variables.
  - `$derived()` for computed/derived values.
  - `$effect()` for side effects.
- Reactive modules outside `.svelte` files must use the `.svelte.ts` extension (e.g., `src/lib/helpers/toast.svelte.ts`).

### Routing & Client-Side Auth Guards
- Since the app is pure CSR, route guards run on the client:
  - `src/routes/+page.ts`, `login/+page.ts`, `register/+page.ts`: Redirect authenticated users to `/dashboard`.
  - `src/routes/dashboard/+layout.ts`: Checks `getToken()`; redirects unauthenticated visitors to `/login`.
  - `src/routes/dashboard/users/+page.svelte`: Client-side admin verification (redirects non-admin users to `/dashboard`).
  - `src/routes/dashboard/strategies/+page.svelte`: Card list view for trading strategies with duplicate, edit, and delete workflows.
  - `src/routes/dashboard/strategies/new/+page.svelte`: Create trading strategy page.
  - `src/routes/dashboard/strategies/[id]/edit/+page.svelte`: Edit trading strategy page.
  - `src/routes/dashboard/backtests/+page.svelte`: Grouped card list view for backtests with sparklines and polling.
  - `src/routes/dashboard/backtests/new/+page.svelte`: Run backtest configuration form.
  - `src/routes/dashboard/backtests/[id]/+page.svelte`: Comprehensive backtest results and performance view.
- All backend routes independently enforce authentication and authorization.

### Strategy Components & Rule Builder
- `src/lib/components/strategy/StrategyForm.svelte`: Core parameter controls (name, desc, public/private toggle, TP%, SL%, live Risk-to-Reward calculation, max holding days).
- `src/lib/components/strategy/WhereConditionsBuilder.svelte`: Dynamic multi-group rule builder with intra-group and inter-group `AND`/`OR` connectors and adaptive operator inputs.
- `src/lib/components/strategy/VariablePickerModal.svelte`: Categorized variable catalog with live search, item format `<b><code></b> <description>`, selection checkmarks, and conditional Save enablement.

### Backtest Components
- `src/lib/components/backtest/StatusBadge.svelte`: Theme-aware badge indicator for `PENDING`, `PROCESSING`, `DONE`, and `FAILED` states.
- `src/lib/components/backtest/HalfDoughnutChart.svelte`: Semicircle SVG chart visualizing wins vs losses with centered total trade count.
- `src/lib/components/backtest/PortfolioChart.svelte`: Baseline equity curve with Net/Gross segmented toggle and Jakarta time rendering.
- `src/lib/components/backtest/BacktestResultPreview.svelte`: Compact sparkline area chart preview for backtest list cards.

### API Client
- `src/lib/api.ts` wraps all network requests, attaches `Authorization: Bearer <token>`, unwraps `BaseResponse<T>`, and raises typed `ApiError` instances on failure.
- When calling backend endpoints, do not fetch directly from components; use or extend `src/lib/api.ts`.

### Design System & Styling
- Defined in `src/app.css` via Tailwind v4 `@theme` tokens:
  - `--color-brand-cyan`: Primary accent `#30B4C9` (buttons, active states, badges)
  - `--color-brand-navy-dark`: Dark background `#2A344C`
  - `--color-brand-navy-card`: Dark card background `#3C486A`
  - `--color-brand-light-bg`: Light background `#EEF0F6`
  - `--color-brand-light-card`: Light card/modal background `#FFFFFF`
  - Font: `Montserrat`
- Always use semantic utility classes and custom property tokens rather than arbitrary hardcoded hex codes.

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

> **Note on Testing**: Always run tests using `bun run test:unit` (or `bun run test`). Avoid running `bun test` directly, as Vitest is configured for DOM/Svelte component testing.

---

## 4. Environment & Secrets Management

- **Backend Host**: Configured via `backend/.env` (reads `127.0.0.1` / `localhost` for local services).
- **Backend Docker**: Configured via `backend/.env.docker` (used by Docker Compose for container port/credential setup).
- **Frontend**: Configured via `frontend/.env` (`PUBLIC_API_BASE_URL=http://127.0.0.1:8000`).
- Synchronize `.env.example` and `.env.docker.example` whenever environment variables change.
- **Never commit secrets, passwords, API tokens, or production credentials to git.**
- Never log full JWT tokens, hashes, or database connection strings with passwords.

---

## 5. Verification Checklist for Agents

Before completing any task, ensure you have verified your changes against the appropriate toolchains:

### For Backend Changes:
1. `cargo fmt --check` passes cleanly.
2. `cargo test` passes all tests.
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
- **Message Style**: Write concise, clear, and descriptive natural language summaries (e.g., `Add trading strategy duplication`, `Fix navbar alignment on mobile`, `Update ESLint and Prettier configurations`).

