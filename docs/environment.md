# Environment Variables & Configuration Guide

This document details the configuration variables, secrets handling, and default credentials for Trading Lab across host development and Docker container environments.

---

## 1. Environment Files Overview

The project relies on four environment template files:

| Environment File | Usage |
| :--- | :--- |
| [`backend/.env.example`](../backend/.env.example) | Host machine backend configuration (`cargo run`). |
| [`backend/.env.docker.example`](../backend/.env.docker.example) | Docker Compose backend & database configuration. |
| [`frontend/.env.example`](../frontend/.env.example) | Host machine frontend configuration (`bun run dev`). |
| [`frontend/.env.docker.example`](../frontend/.env.docker.example) | Docker Compose frontend build configuration. |

---

## 2. Backend Environment Variables

All variables are parsed in [`AppConfig::from_env`](../backend/src/entities/app_config.rs):

| Variable | Required? | Default | Description |
| :--- | :---: | :--- | :--- |
| `APP_NAME` | **Yes** | — | Name of the application (e.g., `"Trading Lab"`). |
| `BIND_ADDRESS` | No | `127.0.0.1` | Network interface IP to bind Actix-web server to. Use `0.0.0.0` in Docker. |
| `PORT` | **Yes** | `8000` | TCP port for the backend HTTP server. |
| `DATABASE_URL` | **Yes** | — | PostgreSQL connection string (`postgres://user:password@host:5432/dbname`). |
| `REDIS_URL` | **Yes** | — | Redis connection URL (`redis://:password@host:6379`). |
| `JWT_SECRET` | **Yes** | — | High-entropy secret key for HMAC SHA-256 JWT signing. |
| `ACCESS_TOKEN_EXPIRATION_SECONDS` | **Yes** | `900` | Lifetime of short-lived JWT access tokens in seconds (15 min). |
| `REFRESH_TOKEN_EXPIRATION_SECONDS` | **Yes** | `604800` | Lifetime of JWT refresh tokens and Redis sessions in seconds (7 days). |
| `CORS_ALLOWED_ORIGIN` | No | `http://localhost:3000` | Allowed origins for CORS headers (comma-separated). |
| `SECTORS_API_KEY` | **Yes** | — | API key for Sectors.app IDX financial market data. |
| `SECTORS_RATE_LIMIT_PER_MINUTE` | No | `25` | Maximum requests per minute allowed across all Sectors API endpoints and concurrent backtests. |
| `GEMINI_API_KEY` | No | `""` | Google Gemini API key for AI executive summaries and strategy recommendations. |
| `GEMINI_MODEL` | No | `gemini-3.1-flash-lite` | Model identifier for Google Gemini calls. |

---

## 3. Frontend Environment Variables

Vite exposes environment variables prefixed with `PUBLIC_` to the client browser:

| Variable | Required? | Default | Description |
| :--- | :---: | :--- | :--- |
| `PUBLIC_API_BASE_URL` | **Yes** | `http://127.0.0.1:8000` | Base URL of the backend REST API endpoint. |
| `PUBLIC_IS_COMING_SOON` | No | `true` | When `true`, intercept public register/login clicks with the Coming Soon modal. |

---

## 4. Docker Infrastructure Variables

Used by [`docker-compose.yml`](../docker-compose.yml) for container health checks and database provisioning:

| Variable | Target Service | Description |
| :--- | :--- | :--- |
| `POSTGRES_USER` | `postgres` | Superuser username for PostgreSQL. |
| `POSTGRES_PASSWORD` | `postgres` | Password for PostgreSQL user. |
| `POSTGRES_DB` | `postgres` | Default database name to create. |
| `REDIS_PASSWORD` | `redis` | Authentication password for Redis server. |

---

## 5. Default Seeded Credentials

Initial database migrations automatically seed a default administrator user:

| Role | Email | Password | Seed Migration |
| :--- | :--- | :--- | :--- |
| **Administrator** | `admin@mail.com` | `admin123` | [`202608300002_seed_administrator.sql`](../backend/migrations/202608300002_seed_administrator.sql) |

> [!WARNING]
> Always change default administrator passwords before publishing any instance to staging or production.

---

## 6. Secrets Policy & Invariants

1. **Never Commit Secrets**: Do not commit active `.env` or `.env.docker` files, API keys, JWT secrets, or production passwords to source control.
2. **Template Synchronization**: When adding or renaming an environment variable in [`AppConfig`](../backend/src/entities/app_config.rs), immediately update `.env.example` and `.env.docker.example`.
3. **Safe Logging**: Never output unmasked connection strings, JWTs, password hashes, or API keys in standard error or standard output logs.
