# Trading Lab Contributor Guide

## Repository Layout

- `backend/` is the Rust API. It is the only implemented application today.
- `frontend/` is reserved for the web client; do not add backend concerns there.
- Keep backend code in the established layers: `entities`, `enums`, `guards`, `helpers`,
  `repositories`, `services`, `routes`, and `setup`.
- SQL migrations belong in `backend/migrations/` and are automatically applied at API startup.

## Backend Workflow

Run commands from `backend/`:

```sh
docker compose --env-file .env.docker up -d
cargo run
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features --locked -- -D warnings
```

Docker Compose intentionally starts only PostgreSQL and Redis. Run the Rust API on the host; it
loads `backend/.env`.

## Configuration and Secrets

- `backend/.env` configures host-run development.
- `backend/.env.docker` configures the PostgreSQL and Redis containers.
- Keep both files synchronized for database and Redis credentials: host URLs use `localhost` /
  `127.0.0.1`, Docker URLs use service names.
- Update `.env.example` and `.env.docker.example` whenever configuration keys change.
- Never log JWTs, passwords, password hashes, refresh tokens, or full connection strings.
- Do not embed environment-specific credentials in Rust code, migrations, tests, or README
  examples. The administrator seed is the sole deliberate development exception.

## API and Authentication Rules

- Preserve the shared JSON envelope: `data`, `status`, `message`, and `timestamp`.
- Keep route handlers thin. Return service results with the response traits, e.g.
  `user_service.list().await.json()`, rather than manually constructing envelopes.
- Public registration always assigns `MEMBER`; only authenticated `ADMIN` users may use user CRUD
  endpoints or create administrators.
- Passwords must use Argon2 helpers. Do not replace them with fast hashes or plaintext storage.
- JWTs include a Redis-backed session ID. Any password, role, or deletion change must revoke the
  affected sessions.

## Database and Redis Changes

- Add a new timestamp-prefixed migration; never edit a migration that may already have run.
- Preserve the PostgreSQL `user_role` enum values `ADMIN` and `MEMBER` unless an explicit data
  migration accompanies the change.
- Use parameterized SQLx queries and map duplicate email errors to the API conflict response.
- Treat Redis as the authoritative store for active session validity; namespace new keys under
  `auth:`.

## Rust Conventions

- Use `Result<T, AppError>` for fallible backend operations and propagate with `?`.
- Avoid `unwrap()` and `expect()` outside tests.
- Borrow strings as `&str` where ownership is unnecessary; avoid incidental cloning.
- Add focused unit tests for pure helpers and route/integration tests for public behavior.
- Before handoff, run formatting, tests, and strict Clippy. Do not silence a lint without a
  documented, justified `#[expect(...)]`.
