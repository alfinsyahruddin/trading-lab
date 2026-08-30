# Trading Lab Frontend

SvelteKit 5 Single Page Application (CSR Only) for the Trading Lab platform.

## Tech Stack

- **SvelteKit 2** + **Svelte 5 Runes** (Client-Side Rendering / SPA mode only, SSR disabled)
- **TypeScript**
- **Bun**
- **TailwindCSS v4**
- **TradingView Lightweight Charts** (installed, ready for future chart pages)
- **Vitest** + **@testing-library/svelte**

## Quick Start

```sh
cd frontend
cp .env.example .env
bun install
bun run dev
```

The dev server runs on `http://localhost:3000`. Expects the backend API at
`http://127.0.0.1:8000` (configure via `PUBLIC_API_BASE_URL` in `.env`).

## Environment Variables

| Variable              | Description          | Default                 |
| --------------------- | -------------------- | ----------------------- |
| `PUBLIC_API_BASE_URL` | Backend API base URL | `http://127.0.0.1:8000` |

## Scripts

| Command                      | Description                          |
| ---------------------------- | ------------------------------------ |
| `bun run dev`                | Start development server (port 3000) |
| `bun run build`              | Production build                     |
| `bun run preview`            | Preview production build             |
| `bun run check`              | Svelte type checking                 |
| `bun run test:unit`          | Run unit tests                       |
| `bun run test:unit:watch`    | Run unit tests in watch mode         |
| `bun run test:unit:coverage` | Run unit tests with coverage report  |
| `bun run format`             | Format code with Prettier            |
| `bun run format:check`       | Check formatting                     |

## Design System

| Token      | Value      | Usage                               |
| ---------- | ---------- | ----------------------------------- |
| Accent     | `#30B4C9`  | Primary buttons, active nav, badges |
| Dark BG    | `#2A344C`  | Page background (dark mode)         |
| Dark Card  | `#3C486A`  | Card background (dark mode)         |
| Light BG   | `#FFFFFF`  | Page background (light mode)        |
| Light Card | `#EEF0F6`  | Card background (light mode)        |
| Font       | Montserrat | All text                            |

## Project Structure

```
src/
├── app.css              # TailwindCSS v4 + CSS design tokens
├── app.html             # HTML shell with FOUC prevention
├── lib/
│   ├── api.ts           # REST API client (wraps fetch, unwraps BaseResponse envelope)
│   ├── types.ts         # TypeScript interfaces (UserResponse, LoginResponse, etc.)
│   ├── constants.ts     # localStorage key names
│   ├── helpers/
│   │   ├── session.ts   # Token persistence (localStorage)
│   │   ├── theme.ts     # Dark/light theme manager
│   │   └── toast.ts     # Toast notification store
│   └── components/
│       ├── TopBar.svelte       # Dashboard navigation bar
│       ├── ThemeToggle.svelte  # Dark/light mode toggle
│       ├── Modal.svelte        # Accessible modal dialog
│       ├── ConfirmModal.svelte # Confirmation dialog
│       ├── TextField.svelte    # Styled text input
│       ├── SelectField.svelte  # Styled select dropdown
│       ├── DataTable.svelte    # Reusable data table
│       ├── RoleBadge.svelte    # User role pill badge
│       ├── ToastViewport.svelte # Fixed toast container
│       └── EmptyState.svelte   # Empty content placeholder
└── routes/
    ├── +layout.ts              # Global SSR disabled (ssr = false)
    ├── +layout.svelte          # Root layout (theme init + toast viewport)
    ├── +page.svelte            # / — Hero landing page
    ├── +page.ts                # Client-side redirect if authenticated
    ├── +error.svelte           # Custom error page
    ├── login/
    │   ├── +page.svelte        # Login form
    │   └── +page.ts            # Client-side redirect if authenticated
    ├── register/
    │   ├── +page.svelte        # Register form
    │   └── +page.ts            # Client-side redirect if authenticated
    └── dashboard/
        ├── +layout.ts          # Client-side auth guard (requires localStorage token)
        ├── +layout.svelte      # Dashboard shell (TopBar + content area)
        ├── +page.svelte        # /dashboard — COMING SOON
        └── users/
            └── +page.svelte    # /dashboard/users — User CRUD table (admin only)

tests/
└── unit/
    ├── setup.ts
    ├── helpers/
    │   ├── session.test.ts
    │   ├── theme.test.ts
    │   └── toast.test.ts
    ├── components/
    │   ├── RoleBadge.test.ts
    │   ├── EmptyState.test.ts
    │   └── TextField.test.ts
    └── routes/
        └── require-auth.test.ts
```

## Auth Flow (CSR Mode)

1. Root layout configures `export const ssr = false;` (pure CSR SPA).
2. User visits `/` — client load checks `getToken()`; if found, redirects to `/dashboard`.
3. User submits login form → `POST /api/users/login`.
4. On success: tokens and user profile stored in `localStorage`.
5. All `/dashboard/*` routes check `getToken()` in `dashboard/+layout.ts` load function.
6. `/dashboard/users` enforces client-side admin role check.
7. On logout: `POST /api/users/logout` → clear `localStorage` → redirect to `/login`.
