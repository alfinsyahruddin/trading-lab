# Trading Lab Backend

Start PostgreSQL and Redis with `docker compose up postgres redis -d` from the project root,
then run the API from this directory with `cargo run`. The application applies SQLx migrations at startup.

Alternatively, run the entire stack (Postgres, Redis, Backend, Frontend) via `docker compose up -d --build` from the repository root.
The local backend on the host reads `.env`, while Docker Compose uses `backend/.env.docker`.

The seeded administrator is `admin@mail.com` with password `admin123`. Change the development
credentials and JWT secret before using this project outside local development.

## API

- `POST /api/users/register` — public member registration
- `POST /api/users/login`, `POST /api/users/refresh`, `POST /api/users/logout` — session auth
- `GET|POST /api/users`, `GET|PATCH|DELETE /api/users/{id}` — administrator user management

Send access tokens in `Authorization: Bearer <access_token>`. Successful and failed requests use
the same `data`, `status`, `message`, and `timestamp` JSON envelope.
