# Contributor & Agent Workflow Guide

This document outlines the workflows, verification commands, and git commit guidelines for human contributors and AI agents working on Trading Lab.

---

## 1. Development Workflows

### A. Infrastructure with Docker Compose
Run all Docker Compose commands from the repository root:

```sh
# Start PostgreSQL & Redis services for local host development
docker compose up postgres redis -d

# Start the full stack (Postgres, Redis, Backend, Frontend)
docker compose up -d --build

# View container logs
docker compose logs -f

# Stop and remove all containers
docker compose down
```

### B. Backend Local Workflow (Host)
Run inside the `backend/` directory:

```sh
# Run the API server with auto-migrations
cargo run

# Check formatting
cargo fmt --check

# Auto-format Rust files
cargo fmt

# Run unit and integration tests
cargo test

# Run strict Clippy linter
cargo clippy --all-targets --all-features --locked -- -D warnings
```

### C. Frontend Local Workflow (Host)
Run inside the `frontend/` directory:

```sh
# Install dependencies
bun install

# Start Vite local development server (port 3000)
bun run dev

# Run Svelte and TypeScript typecheck diagnostics
bun run check

# Run ESLint
bun run lint

# Auto-fix ESLint issues
bun run lint:fix

# Run unit and component tests (Vitest)
bun run test:unit

# Run Playwright end-to-end tests
bun run test:e2e

# Check Prettier formatting & Tailwind class sorting
bun run format:check

# Auto-format codebase & Tailwind class order
bun run format
```

---

## 2. Verification Checklist for Tasks

Before completing any task, run the following verification steps:

### Backend Changes Checklist
- [ ] `cargo fmt --check` passes cleanly with zero formatting violations.
- [ ] `cargo test` passes all unit and integration tests.
- [ ] `cargo clippy --all-targets --all-features --locked -- -D warnings` completes with zero warnings/errors.
- [ ] Any new or modified database schema has a corresponding sequential migration file in [`backend/migrations/`](../backend/migrations).
- [ ] Any newly introduced environment variables are documented in [`docs/environment.md`](./environment.md), `.env.example`, and `.env.docker.example`.

### Frontend Changes Checklist
- [ ] `bun run check` produces 0 errors and 0 warnings.
- [ ] `bun run lint` completes with 0 errors.
- [ ] `bun run test:unit` passes all test suites.
- [ ] `bun run format:check` reports clean formatting.
- [ ] Relevant user flow passes Playwright E2E tests (`bun run test:e2e`).
- [ ] All new components use Svelte 5 Runes (`$state`, `$derived`, `$props`, `$effect`) and keyed `#each` blocks.

---

## 3. Git & Commit Guidelines

- **Do NOT Use Conventional Commits**: Never use prefixes such as `feat:`, `fix:`, `chore:`, `refactor:`, `docs:`, `style:`, or `test:`.
- **Message Style**: Write concise, descriptive, natural language summaries in title or sentence case.
- **Examples**:
  - `Add trading strategy duplication`
  - `Implement trade screening info modal`
  - `Refactor sectors client 429 retry backoff to 60 seconds`
  - `Update Playwright E2E test suites for auth flow`
  - `Fix mobile responsive alignment on dashboard topbar`
