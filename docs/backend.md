# Backend Architecture & Engineering Guidelines

This document provides the in-depth architectural reference for the Rust backend of Trading Lab. For golden rules and invariants, see [AGENTS.md §3](../AGENTS.md#3-golden-rules).

---

## 1. Layered Architecture

Backend code strictly separates concerns into distinct layers under [`backend/src/`](../backend/src):

1. **Entities & DTOs** ([`src/entities/`](../backend/src/entities)):
   - Request and response structs (`RegisterRequest`, `CreateBacktestJobRequest`, etc.).
   - Database row structs (`UserRecord`, `BacktestJobRecord`, `MostTradedRow`, etc.).
   - System configuration ([`AppConfig`](../backend/src/entities/app_config.rs)), unified responses ([`BaseResponse`](../backend/src/entities/base_response.rs), [`AppResponse`](../backend/src/entities/app_response.rs)), pagination ([`PaginationQuery`](../backend/src/entities/pagination.rs), [`PaginatedData`](../backend/src/entities/pagination.rs)), and errors ([`AppError`](../backend/src/entities/app_error.rs)).
   - External API models ([`sectors.rs`](../backend/src/entities/sectors.rs), [`strategy_suggestion.rs`](../backend/src/entities/strategy_suggestion.rs)).
2. **Guards / Extractors** ([`src/guards/`](../backend/src/guards)):
   - [`AuthenticatedUser`](../backend/src/guards/authenticated_user.rs): Validates the JWT Bearer token and checks for an active session in Redis; extracts claims (`sub`, `sid`, `role`).
   - [`RequireAdmin`](../backend/src/guards/require_admin.rs): Enforces that the caller possesses the `ADMIN` role.
3. **Helpers** ([`src/helpers/`](../backend/src/helpers)):
   - [`hash_helper.rs`](../backend/src/helpers/hash_helper.rs): Argon2id password hashing and verification.
   - [`token_helper.rs`](../backend/src/helpers/token_helper.rs): JWT token pair creation, signing, and decoding.
   - [`math_helper.rs`](../backend/src/helpers/math_helper.rs): Financial calculations (Sharpe ratio, volatility, PnL percentages).
   - [`prompt_helper.rs`](../backend/src/helpers/prompt_helper.rs): Prompt template generation for AI summaries and strategy refinement recommendations.
   - [`date_helper.rs`](../backend/src/helpers/date_helper.rs): Date formatting, Indonesian trading calendar, and range utilities.
4. **Clients** ([`src/clients/`](../backend/src/clients)):
   - [`SectorsClient`](../backend/src/clients/sectors_client.rs): Sectors.app integration with 90-day chunking, 60s 429 retry backoff, and Redis caching.
   - [`GeminiLLM`](../backend/src/clients/llm_client.rs): Google Gemini client for structured LLM evaluations.
5. **Repositories** ([`src/repositories/`](../backend/src/repositories)):
   - Pure database access layer executing SQLx queries against PostgreSQL.
   - Derived with `Clone` and stored by value in services (`PgPool` uses internal `Arc`).
6. **Services** ([`src/services/`](../backend/src/services)):
   - Domain business logic, orchestration between repositories, Redis, and external clients.
   - Heavy simulation routines modularized in [`src/services/backtest/`](../backend/src/services/backtest) ([`engine.rs`](../backend/src/services/backtest/engine.rs) and [`rules.rs`](../backend/src/services/backtest/rules.rs)).
7. **Routes** ([`src/routes/`](../backend/src/routes)):
   - Thin HTTP controllers that extract parameters/guards, invoke services, and return responses.
8. **Dependency Injection** ([`src/di.rs`](../backend/src/di.rs)):
   - [`AppDependencies`](../backend/src/di.rs) initializes pools, clients, repositories, and services, registering them into Actix `app_data`.

---

## 2. API Contract & Envelope

All API endpoints strictly adhere to the unified envelope:

```json
{
  "data": T | null,
  "status": 200,
  "message": "Optional message" | null,
  "timestamp": "2026-09-17T15:00:00.000Z"
}
```

- Handlers return `AppResponse<T>` (`Result<Json<BaseResponse<T>>, AppError>`).
- Use the `IntoResponseTrait` helper `service_call().await.json()` or `String.json_data()` rather than manually building response envelopes.
- System root endpoints:
  - `GET /`: Returns API name (`"Trading Lab API"`).
  - `GET /health`: Health check (`"ok"`).
  - Unmatched routes default to 404 JSON via `routes::not_found`.

---

## 3. Error Handling

- Fallible operations return `Result<T, AppError>` and propagate with `?`.
- [`AppError`](../backend/src/entities/app_error.rs) maps domain errors to standard HTTP status codes:
  - `BadRequest` (400)
  - `Unauthorized` (401)
  - `Forbidden` (403)
  - `NotFound` (404)
  - `Conflict` (409)
  - `Internal` (500)
- Database (`sqlx::Error`), Redis (`redis::RedisError`), and JWT errors implement `From<...>` for `AppError`, logging error traces to stderr while masking internal implementation details from clients as `AppError::Internal`.

---

## 4. Domain Implementations

### A. Authentication & Users ([`routes/user_route.rs`](../backend/src/routes/user_route.rs))

1. **Password Hashing**: Argon2id via [`hash_helper`](../backend/src/helpers/hash_helper.rs).
2. **Captcha Verification**:
   - `GET /api/users/captcha`: Generates a 5-character distorted image via `captcha::Captcha` and stores the solution in Redis under `auth:captcha:{captcha_id}` with a 300-second (5 min) TTL.
   - `POST /api/users/register`: Requires `captcha_id` and `captcha_code`. Validates against Redis and deletes the key immediately upon verification.
   - In automated test environments, the bypass code `TEST_CAPTCHA` is permitted.
3. **Session Lifecycle in Redis**:
   - On login, a new session UUID (`sid`) is embedded in the JWT access and refresh token claims.
   - The session is stored in Redis:
     - `auth:session:{session_id}` → stores `user_id` string with refresh TTL.
     - `auth:user-sessions:{user_id}` → Redis `SET` tracking all active `session_id`s for that user.
4. **Session Revocation**:
   - Single session logout: deletes `auth:session:{session_id}` and removes `session_id` from `auth:user-sessions:{user_id}`.
   - Batch revocation: Any password update, role change, or user deletion invokes `revoke_all_user_sessions(user_id)` to invalidate all active client sessions immediately.
5. **RBAC & Seed Account**:
   - Default registration assigns the `MEMBER` role.
   - Administrative endpoints require the [`RequireAdmin`](../backend/src/guards/require_admin.rs) guard.
   - Database migration seeds a default administrator: `admin@mail.com` / `admin123`.

**Endpoints**:
- `GET /api/users/captcha`: Generate new captcha challenge.
- `POST /api/users/register`: Register new member account (captcha required).
- `POST /api/users/login`: Authenticate and obtain JWT token pair.
- `POST /api/users/refresh`: Exchange refresh token for a new access token.
- `POST /api/users/logout`: Invalidate current active session in Redis.
- `GET /api/users/me`: Get current user profile.
- `PATCH /api/users/me`: Update current user profile.
- `POST /api/users/me/password`: Change current user password (revokes all user sessions).
- `GET /api/users`: List all users (Admin only).
- `GET /api/users/{id}`: Get user details (Admin only).
- `POST /api/users`: Create user with explicit role (Admin only).
- `PATCH /api/users/{id}`: Update user profile/role/password (Admin only).
- `DELETE /api/users/{id}`: Delete user (Admin only, self-deletion prevented).

---

### B. Trading Strategy Domain ([`routes/trading_strategy_route.rs`](../backend/src/routes/trading_strategy_route.rs))

1. **Data Model**: `trading_strategies` table stores `take_profit_percentage`, `stop_loss_percentage`, `max_holding_period_days`, and multi-group condition logic in `rules JSONB`.
2. **Name Uniqueness**: Enforced per-user by `UNIQUE(user_id, name)`. Duplicate naming attempts return `AppError::Conflict` (409).
3. **Duplication**: `POST /api/strategies/{id}/duplicate` creates a copy with `{Original Name} (Copy)` naming logic, ensuring uniqueness.

**Endpoints**:
- `GET /api/strategies`: List all strategies owned by the user.
- `GET /api/strategies/{id}`: Retrieve strategy details.
- `POST /api/strategies`: Create a strategy.
- `PATCH /api/strategies/{id}`: Update an existing strategy.
- `DELETE /api/strategies/{id}`: Delete a strategy.
- `POST /api/strategies/{id}/duplicate`: Duplicate a strategy.
- `POST /api/strategies/ai-suggestions`: Request AI refinement recommendations for rules and parameters.

---

### C. Backtest Simulation Domain ([`routes/backtest_route.rs`](../backend/src/routes/backtest_route.rs))

1. **Data Model**:
   - `backtest_jobs`: Job metadata, configuration (`initial_cash`, `max_holding_stocks`, `max_stocks`, `duration_months`, fees, `is_public`, `status`).
   - `backtest_results`: Simulation aggregates (PnL, win rate, Sharpe ratio, volatility, max/avg profit & loss, holding times, AI insights).
   - `backtest_portfolio_history`: Daily net and gross equity values.
   - `backtest_trades`: Individual trade records with entry/exit prices, fees, exit reason, and `query_values` (indicators snapshot at entry).
   - `backtest_stars`: Community stars for public backtests.
2. **Candidate Pool vs Holding Capacity**:
   - `max_holding_stocks` (1–50, default 4): Maximum concurrent portfolio holdings.
   - `max_stocks` (default 12): Top screener candidates considered for purchase on any given day.
3. **Sectors.app Client Architecture** ([`src/clients/sectors_client.rs`](../backend/src/clients/sectors_client.rs)):
   - **Rate Limiting**: HTTP 429 errors trigger a 60-second backoff (`RETRY_DELAY_SECS = 60`) with up to 2 retries, aligning with the Sectors.app 60s quota window.
   - **Date Chunking**: Daily transaction requests spanning more than 90 days are chunked into ≤ 90-day segments via [`compute_date_chunks`](../backend/src/clients/sectors_client.rs).
   - **Caching**:
     - Screener responses cached under `sectors:screener:{where_query}:{limit}`.
     - Daily transaction chunks cached under `sectors:daily:{symbol}:{chunk_start}:{chunk_end}`.
   - **Screener Telemetry**: Queries pass `include_query_values=true` and `order_by=last_close_price` to capture indicator values for trade logging.
4. **Rule Parsing & Simulation Engine** ([`src/services/backtest/`](../backend/src/services/backtest)):
   - Fundamental metrics and market cap are converted into Sectors API screener queries (`{variable}[{year}]`, `last_close_price`, `market_cap`).
   - Daily variables (`volume`, `value`, `price`) cannot be screened via the Sectors company API and are separated into manual filter groups evaluated dynamically during the daily simulation loop ([`rules.rs`](../backend/src/services/backtest/rules.rs)).
   - Before creating batch requests for daily transactions, the backtest engine retrieves cached chunks from Redis first via `get_cached_stock_transactions`; only stocks with missing chunks are queued into rate-limited API batches.
   - Simulation runs asynchronously in a Tokio background task, updating job status from `PROCESSING` to `DONE` or `FAILED`.

**Endpoints**:
- `GET /api/backtests`: List current user's backtests with sparklines.
- `GET /api/backtests/{id}`: Retrieve full backtest report, metrics, equity curve, and trade history.
- `POST /api/backtests`: Queue and start a new backtest simulation.
- `POST /api/backtests/{id}/rerun`: Rerun an existing failed backtest.
- `PATCH /api/backtests/{id}`: Update metadata (e.g. toggle `is_public`).
- `DELETE /api/backtests/{id}`: Delete backtest and cascaded records.

---

### D. Dashboard & Community Domain ([`routes/dashboard_route.rs`](../backend/src/routes/dashboard_route.rs))

- **Community Leaderboard**: Discovers public backtests (`is_public = true` and `status = 'DONE'`), ranking by net return % or community stars.
- **Starring System**: Star/unstar public backtests (`UNIQUE(user_id, backtest_id)` in `backtest_stars`).
- **User KPI Metrics**: Aggregates total stars received, total strategies created, total backtests run, and account creation date.

**Endpoints**:
- `GET /api/dashboard/stats`: Retrieve current user statistics.
- `GET /api/dashboard/leaderboard`: Top public backtests by return %.
- `GET /api/dashboard/top-stars`: Top public backtests by star count.
- `POST /api/dashboard/stars/{backtest_id}`: Star a public backtest.
- `DELETE /api/dashboard/stars/{backtest_id}`: Unstar a public backtest.

---

### E. App Settings & AI Intelligence ([`routes/settings_route.rs`](../backend/src/routes/settings_route.rs))

- **Master AI Toggle**: `app_settings` table and Redis key `app_setting:ai_enabled` allow admins to enable/disable AI features platform-wide.
- **AI Insights**: Generates a 4-point qualitative performance analysis on completed backtests, cached permanently in `backtest_results.ai_insights`.
- **Strategy Refinement**: Analyzes TP/SL, holding duration, and rule condition trees against IDX market behaviors, offering "Accept" / "Ignore" adjustments.
- **Prompt Helper**: Centralized prompt templates in [`helpers/prompt_helper.rs`](../backend/src/helpers/prompt_helper.rs).

**Endpoints**:
- `GET /api/settings`: Retrieve platform settings.
- `PATCH /api/settings`: Update settings (Admin only).

> Strategy AI recommendations (`POST /api/strategies/ai-suggestions`) are routed via [`trading_strategy_route.rs`](../backend/src/routes/trading_strategy_route.rs) and documented under Section B above.
