# Testing Guide

This guide documents the testing strategies, frameworks, and execution commands for Trading Lab. For the full project layout, see [AGENTS.md §2](../AGENTS.md#2-repository-layout).

---

## 1. Backend Testing (Rust)

Backend tests verify domain math, JWT lifecycle, Redis caching, serialization, and HTTP contracts.

### Unit Tests
- Embedded alongside modules under `#[cfg(test)]`:
  - **Helpers**: [`token_helper.rs`](../backend/src/helpers/token_helper.rs), [`prompt_helper.rs`](../backend/src/helpers/prompt_helper.rs), [`math_helper.rs`](../backend/src/helpers/math_helper.rs), [`date_helper.rs`](../backend/src/helpers/date_helper.rs).
  - **Clients**: [`sectors_client.rs`](../backend/src/clients/sectors_client.rs) (verifies 90-day date chunking logic and retry policies).
  - **Services**: [`session_service.rs`](../backend/src/services/session_service.rs), [`backtest/rules.rs`](../backend/src/services/backtest/rules.rs) (verifies where-query compilation and manual indicator evaluation).
  - **Entities**: Validation and pagination logic in [`pagination.rs`](../backend/src/entities/pagination.rs), [`user.rs`](../backend/src/entities/user.rs), [`backtest.rs`](../backend/src/entities/backtest.rs).

### Integration & Contract Tests
- Located in [`backend/tests/http_contract.rs`](../backend/tests/http_contract.rs).
- Spins up an Actix test server instance to validate JSON envelopes, status codes, and error formatting.
- **Captcha Test Bypass**: Use `TEST_CAPTCHA` as the `captcha_code` in registration payloads during automated integration tests to bypass visual image resolution.

### Backend Test Commands
Run commands inside the `backend/` directory:

```sh
# Run all unit and integration tests
cargo test

# Run tests with output printed to console
cargo test -- --nocapture

# Run a specific test suite
cargo test --test http_contract
```

---

## 2. Frontend Unit & Component Testing (Vitest)

Vitest executes unit tests for helper modules and component tests using `@testing-library/svelte` and `jsdom`.

### Test Coverage
- **API Client**: [`tests/unit/api.test.ts`](../frontend/tests/unit/api.test.ts) (auto-refresh deduplication, error unwrapping).
- **Components**: Component render tests, prop bindings, and user interactions across all UI components ([`tests/unit/components/`](../frontend/tests/unit/components)).
- **Helpers**: Session storage, Jakarta date conversions, runtime configs, and toast state.

### Execution Note
> [!IMPORTANT]
> Always execute Vitest via `bun run test:unit` (or `bun run test`). Do **not** run raw `bun test`, as Bun's native test runner does not load the jsdom and `@testing-library/svelte` DOM simulation environment configured in `vite.config.ts`.

### Frontend Unit Test Commands
Run commands inside the `frontend/` directory:

```sh
# Run all unit and component tests once
bun run test:unit

# Run tests in interactive watch mode
bun run test:unit:watch

# Generate code coverage report
bun run test:unit:coverage
```

---

## 3. Frontend End-to-End Testing (Playwright)

Playwright runs full browser simulations against the SPA configured in [`frontend/playwright.config.ts`](../frontend/playwright.config.ts).

### Test Suites ([`frontend/tests/e2e/`](../frontend/tests/e2e))
1. **`landing-and-auth.spec.ts`**: Verifies landing page rendering, theme toggle, interactive charts, and user authentication workflows.
2. **`dashboard-navigation.spec.ts`**: Verifies navigation between strategies, backtests, settings, and profile modals.
3. **`strategy-flow.spec.ts`**: Verifies creating, editing, validating rules, duplicating, and deleting trading strategies.
4. **`backtest-flow.spec.ts`**: Verifies configuring parameters, triggering backtests, and inspecting result charts and metrics.
5. **`helpers/mock-api.ts`**: Intercepts network calls to provide predictable test fixtures without requiring live external Sectors API calls.

### E2E Test Commands
Run commands inside the `frontend/` directory:

```sh
# Run full Playwright test suite headlessly
bun run test:e2e

# Run tests with UI mode
bunx playwright test --ui

# Run a specific test spec
bunx playwright test tests/e2e/strategy-flow.spec.ts
```
