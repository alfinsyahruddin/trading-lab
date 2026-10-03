# Trading Lab — Agent & Contributor Guide

Welcome to the **Trading Lab** codebase. This document serves as the primary entry point and architectural hub for AI agents and human contributors. It defines core architectural principles, inviolable rules, and verification procedures.

Detailed domain specifications and guidelines are under [`docs/`](./docs).

---

## 1. Documentation Map

For in-depth explanations, refer directly to the specialized guides:

| Guide | Content Overview |
| :--- | :--- |
| 🦀 [**Backend Architecture**](./docs/backend.md) | Layered architecture, Sectors.app 90-day chunking & 60s 429 retries, rule engine, captcha verification, Redis session tracking, and REST endpoints. |
| ⚡ [**Frontend Architecture**](./docs/frontend.md) | SvelteKit 3 SPA, Svelte 5 Runes conventions, complete UI component catalog (including landing & modals), CSR route guards, and API client. |
| ⚙️ [**Environment & Credentials**](./docs/environment.md) | Complete environment variable reference (host & Docker), default seeded credentials (`admin@mail.com` / `admin123`), and security policies. |
| 🧪 [**Testing Guide**](./docs/testing.md) | Rust unit & HTTP contract tests, Vitest unit/component suites, and Playwright E2E testing workflows. |
| 🛠️ [**Contributor Workflow**](./docs/workflow.md) | Development workflows, Docker Compose operations, verification checklists, and git commit guidelines. |

---

## 2. Repository Layout

```
trading-lab/
├── backend/                  # Rust API service (Actix-web 4, SQLx, Redis, Reqwest)
│   ├── migrations/           # SQLx migration files (sequential PostgreSQL schema)
│   ├── src/
│   │   ├── clients/          # Sectors.app (global GCRA rate limiter, 60s retry, 90d chunks) & Gemini LLM clients
│   │   ├── constants/        # Global constants (Redis prefixes)
│   │   ├── entities/         # Domain models, requests/responses, errors, configuration
│   │   ├── enums/            # System enums (UserRole, BacktestStatus, TokenType)
│   │   ├── guards/           # Actix extractors (AuthenticatedUser, RequireAdmin)
│   │   ├── helpers/          # Hash, token, math, date, and prompt helpers
│   │   ├── repositories/     # PostgreSQL SQLx query layer
│   │   ├── routes/           # REST route controllers
│   │   ├── services/         # Business logic, simulation engine, and rules parser
│   │   │   └── backtest/     # Backtest engine (engine.rs) & rule evaluator (rules.rs)
│   │   ├── setup/            # Infrastructure setup (DB, Redis, HTTP client)
│   │   ├── di.rs             # AppDependencies dependency injection container
│   │   ├── http.rs           # Middleware, CORS, and JSON error handling
│   │   ├── lib.rs            # Backend library entrypoint
│   │   └── main.rs           # Server executable entrypoint
│   ├── tests/                # Integration and HTTP contract tests (http_contract.rs)
│   ├── Cargo.toml            # Rust dependencies & workspace configuration
│   ├── Dockerfile            # Container build specification
│   ├── .env.example          # Local host environment template
│   └── .env.docker.example   # Docker container environment template
│
├── frontend/                 # SvelteKit 3 SPA (CSR-only, Bun, TailwindCSS v4)
│   ├── src/
│   │   ├── app.css           # Tailwind v4 theme tokens & CSS variables
│   │   ├── app.html          # Shell HTML template
│   │   ├── lib/
│   │   │   ├── api.ts        # Typed API client with auto-refresh deduplication
│   │   │   ├── constants/    # Landing content & metadata constants (landing.ts)
│   │   │   ├── constants.ts  # Shared financial indicators & storage keys
│   │   │   ├── types.ts      # Domain TypeScript interfaces
│   │   │   ├── components/   # Svelte 5 UI components (dashboard, strategy, backtest, landing, shared)
│   │   │   └── helpers/      # Client session, theme, date, config, reveal, and toast helpers
│   │   └── routes/           # CSR client routes (+layout.svelte, +page.svelte)
│   ├── tests/
│   │   ├── unit/             # Vitest unit and Svelte 5 component tests (jsdom)
│   │   └── e2e/              # Playwright browser end-to-end user journey tests
│   ├── playwright.config.ts  # Playwright test configuration
│   ├── Dockerfile            # Multi-stage production build (Bun -> Nginx Alpine)
│   ├── nginx.conf            # Nginx SPA fallback configuration
│   ├── package.json          # Frontend dependencies & Bun scripts
│   ├── .env.example          # Local host environment template
│   └── .env.docker.example   # Docker container environment template
│
├── docs/                     # Modular documentation guidelines
│   ├── backend.md            # In-depth backend architecture, domains, and rules
│   ├── frontend.md           # In-depth frontend architecture, runes, and components
│   ├── environment.md        # Complete environment variables and credentials
│   ├── testing.md            # Testing strategies and commands
│   └── workflow.md           # Developer workflows, checklists, and git guidelines
│
├── docker-compose.yml        # PostgreSQL, Redis, Backend, and Frontend containers
├── AGENTS.md                 # Agent & contributor index and golden rules
└── README.md                 # Project overview and public documentation
```

---

## 3. Golden Rules

When contributing or modifying code, agents and contributors MUST adhere to these invariants:

### Architecture & Layering
1. **Strict Layer Segregation**: Routes only extract parameters and invoke services. Repositories only query PostgreSQL. Services orchestrate business logic. All domain structs and DTOs strictly reside in [`backend/src/entities/`](./backend/src/entities).
2. **Unified API Envelope**: Every backend endpoint must return `AppResponse<T>` (`Result<Json<BaseResponse<T>>, AppError>`) via `.json()` or `.json_data()`. Never return ad-hoc JSON structures.
3. **No Unwraps in Production**: Never use `unwrap()` or `expect()` in production Rust code. Use `Result<T, AppError>` and propagate with `?`.
4. **Svelte 5 Runes Only**: All Svelte components must use Svelte 5 Runes (`$props`, `$state`, `$derived`, `$effect`). Never use legacy Svelte syntax (`let:`, `export let`, `$:`).
5. **Pure CSR Mode**: The frontend is strictly a single-page application (`export const ssr = false;` in [`src/routes/+layout.ts`](./frontend/src/routes/+layout.ts)). Never create SvelteKit server routes (`+page.server.ts`).

### Domain & Security Invariants
1. **Redis Session Tracking**: User sessions are mapped in Redis as `auth:session:{session_id}` → `user_id` and tracked in `auth:user-sessions:{user_id}`. Password changes, role changes, and account deletions must revoke all active sessions via `revoke_all_user_sessions`.
2. **Registration Captcha**: `POST /api/users/register` requires `captcha_id` and `captcha_code` (generated via `GET /api/users/captcha`). Automated tests may use the `TEST_CAPTCHA` bypass code.
3. **Sectors.app Resilience**: [`SectorsClient`](./backend/src/clients/sectors_client.rs) and [`SectorsRateLimiter`](./backend/src/clients/sectors_rate_limiter.rs) enforce a global GCRA rate limit across all endpoints and concurrent backtests, retrying HTTP 429 rate limits with a **60-second cooldown** (matching Sectors quota resets) and chunking daily transaction queries into segments of ≤ 90 days.
4. **Database Migrations**: Never modify existing migrations that have already run. Always create a new sequential file under [`backend/migrations/`](./backend/migrations/).
5. **Git Commit Guidelines**: **Do NOT use Conventional Commits** (never use prefixes like `feat:`, `fix:`, `chore:`); write concise, descriptive natural language summaries in title or sentence case.

---

## 4. Verification

Before completing any task, run the verification commands documented in [**Contributor Workflow → Verification Checklist**](./docs/workflow.md#2-verification-checklist-for-tasks).

