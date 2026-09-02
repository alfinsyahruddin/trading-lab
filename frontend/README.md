# Trading Lab Frontend

Modern SvelteKit 5 Single Page Application (CSR / SPA mode) for the **Trading Lab** platform, built with **Svelte 5 Runes**, **Bun**, **TailwindCSS v4**, and **TradingView Lightweight Charts**.

---

## Tech Stack

- **Framework**: [SvelteKit 2](https://kit.svelte.dev/) with **Svelte 5 Runes** (`$state`, `$derived`, `$props`, `$effect`)
- **Mode**: Pure Client-Side Rendering (CSR / SPA mode only, SSR disabled via `export const ssr = false;`)
- **Runtime & Package Manager**: [Bun](https://bun.sh/)
- **Styling**: [TailwindCSS v4](https://tailwindcss.com/) with CSS design token variables
- **Charts**: [TradingView Lightweight Charts](https://tradingview.github.io/lightweight-charts/) for interactive equity curves & native SVG for sparklines and win/loss half-doughnuts
- **Icons**: `@iconify/svelte` (Lucide icon set)
- **Testing**: [Vitest](https://vitest.dev/) with `@testing-library/svelte` and `jsdom`

---

## Quick Start

### 1. Prerequisites

- [Bun](https://bun.sh/) (v1.2+)
- Running Trading Lab Backend API (`http://127.0.0.1:8000`)

### 2. Installation & Development

```sh
cd frontend

# Copy environment template
cp .env.example .env

# Install dependencies
bun install

# Start development server (port 3000)
bun run dev
```

The application will be accessible at `http://localhost:3000`.

---

## Environment Variables

| Variable              | Description          | Default                 |
| :-------------------- | :------------------- | :---------------------- |
| `PUBLIC_API_BASE_URL` | Backend API base URL | `http://127.0.0.1:8000` |

---

## Available Scripts

| Command                      | Description                                                  |
| :--------------------------- | :----------------------------------------------------------- |
| `bun run dev`                | Start development server (port 3000)                         |
| `bun run build`              | Build production SPA bundle                                  |
| `bun run preview`            | Preview production build locally                             |
| `bun run check`              | Run Svelte and TypeScript diagnostic checks                  |
| `bun run lint`               | Run ESLint (checks Svelte, TypeScript, TailwindCSS v4 rules) |
| `bun run lint:fix`           | Automatically fix ESLint issues                              |
| `bun run test:unit`          | Run unit and component tests via Vitest                      |
| `bun run test:unit:watch`    | Run Vitest in interactive watch mode                         |
| `bun run test:unit:coverage` | Run Vitest and generate test coverage report                 |
| `bun run format`             | Auto-format codebase & sort Tailwind classes with Prettier   |
| `bun run format:check`       | Verify formatting and Tailwind class ordering                |

---

## Design System & Theme Tokens

Defined in `src/app.css` via TailwindCSS v4 `@theme` design tokens:

| Token                      | Value        | Usage                                                       |
| :------------------------- | :----------- | :---------------------------------------------------------- |
| `--color-brand-cyan`       | `#30B4C9`    | Primary accent, CTA buttons, active states, sparkline fills |
| `--color-brand-navy-dark`  | `#2A344C`    | Dark mode primary background                                |
| `--color-brand-navy-card`  | `#3C486A`    | Dark mode card & modal container background                 |
| `--color-brand-light-bg`   | `#EEF0F6`    | Light mode primary background                               |
| `--color-brand-light-card` | `#FFFFFF`    | Light mode card & modal container background                |
| Font                       | `Montserrat` | Primary typography throughout the application               |

---

## Project Structure

```
frontend/
├── src/
│   ├── app.css                  # TailwindCSS v4 theme tokens & global styles
│   ├── app.html                 # HTML shell with theme initialization
│   ├── lib/
│   │   ├── api.ts               # Typed API client with auto-refresh deduplication & error handling
│   │   ├── constants.ts         # Indicator variable definitions, storage keys, default rules
│   │   ├── types.ts             # Domain interfaces, request/response models, and DTOs
│   │   ├── helpers/
│   │   │   ├── session.ts       # JWT token & user profile persistence in localStorage
│   │   │   ├── theme.ts         # Light/dark mode theme manager with data-theme synchronization
│   │   │   └── toast.svelte.ts  # Reactive Svelte 5 toast notification store
│   │   └── components/
│   │       ├── backtest/
│   │       │   ├── BacktestAiSummary.svelte     # 5-point AI qualitative analysis card
│   │       │   ├── BacktestResultPreview.svelte # Sparkline equity curve preview for lists
│   │       │   ├── HalfDoughnutChart.svelte     # Semicircle SVG win/loss ratio chart
│   │       │   ├── PortfolioChart.svelte        # Interactive Lightweight Charts equity curve
│   │       │   └── StatusBadge.svelte           # Backtest job status pill badge
│   │       ├── dashboard/
│   │       │   ├── LeaderboardCard.svelte       # Public leaderboard entry with star action
│   │       │   └── StatsCard.svelte             # Glassmorphism user KPI statistics card
│   │       ├── strategy/
│   │       │   ├── StrategyAiSuggestionsCard.svelte # Floating AI recommendations card for parameters & rules
│   │       │   ├── StrategyForm.svelte          # Strategy config, parameters, and rule editor
│   │       │   ├── VariablePickerModal.svelte   # Financial metric variable catalog modal
│   │       │   └── WhereConditionsBuilder.svelte# Dynamic multi-group rule builder
│   │       ├── ChangePasswordModal.svelte       # User password change modal dialog
│   │       ├── ConfirmModal.svelte              # Generic confirmation modal dialog
│   │       ├── DataTable.svelte                 # Responsive generic tabular data component
│   │       ├── EditProfileModal.svelte          # User profile edit modal dialog
│   │       ├── EmptyState.svelte                # Empty state visual placeholder
│   │       ├── IosSwitch.svelte                 # Smooth iOS-style boolean toggle switch
│   │       ├── Modal.svelte                     # Accessible backdrop modal dialog container
│   │       ├── RoleBadge.svelte                 # ADMIN / MEMBER user role badge
│   │       ├── SegmentedControl.svelte          # Accessible segmented button group switch
│   │       ├── SelectField.svelte               # Form select dropdown with icon and error state
│   │       ├── TextField.svelte                 # Form text input with icon and error state
│   │       ├── ThemeToggle.svelte               # Dark/light mode theme toggle button
│   │       ├── ToastViewport.svelte             # Fixed container for reactive toast notifications
│   │       └── TopBar.svelte                    # Navigation bar with user menu and modal hooks
│   └── routes/
│       ├── +error.svelte                        # Custom error page
│       ├── +layout.svelte                       # Root layout (theme init & toast viewport)
│       ├── +layout.ts                           # Global SSR disabled (ssr = false)
│       ├── +page.svelte                         # Landing page with hero overview
│       ├── +page.ts                             # Client redirect to /dashboard if authenticated
│       ├── login/                               # Authentication login page
│       ├── register/                            # Member registration page
│       └── dashboard/
│           ├── +layout.svelte                   # Dashboard layout with TopBar shell
│           ├── +layout.ts                       # Client-side auth guard (requires active session)
│           ├── +page.svelte                     # Dashboard home (KPI stats, leaderboard, stars)
│           ├── backtests/
│           │   ├── +page.svelte                 # Backtest list with sparklines & polling
│           │   ├── [id]/+page.svelte            # Backtest results, metrics, charts, trade log
│           │   └── new/+page.svelte             # Backtest launch configuration form
│           ├── settings/
│           │   └── +page.svelte                 # Platform settings (Admin AI master switch)
│           ├── strategies/
│           │   ├── +page.svelte                 # Strategy list with duplicate/edit/delete
│           │   ├── [id]/+page.svelte            # Strategy detail view & associated backtests
│           │   ├── [id]/edit/+page.svelte       # Strategy edit form
│           │   └── new/+page.svelte             # Strategy creation with AI suggestions
│           └── users/
│               └── +page.svelte                 # User management table & modals (Admin only)
│
├── tests/
│   └── unit/
│       ├── setup.ts                             # Vitest DOM setup and global mocks
│       ├── api.test.ts                          # API client, envelope unwrapping, auto-refresh tests
│       ├── components/                          # Component unit tests (Vitest + Testing Library)
│       ├── constants/                           # Indicator mapping tests
│       ├── helpers/                             # Session, theme, and toast store tests
│       └── routes/                              # Route navigation and auth guard tests
├── Dockerfile                                   # Multi-stage Bun build -> Nginx SPA container
├── nginx.conf                                   # SPA routing fallback configuration
├── package.json                                 # NPM dependencies and scripts
└── tsconfig.json                                # TypeScript compiler configuration
```

---

## Client-Side Authentication Flow (CSR Mode)

1. Root layout disables SSR (`export const ssr = false;`).
2. Landing (`/`), login (`/login`), and register (`/register`) routes check `getToken()`; if valid, they redirect to `/dashboard`.
3. User logs in via `POST /api/users/login`; access/refresh tokens and profile are saved to `localStorage`.
4. All `/dashboard/*` routes enforce authentication in `dashboard/+layout.ts`.
5. Admin-only pages (`/dashboard/users`, `/dashboard/settings`) enforce role checks on both client navigation and API request extractors.
6. The typed API client (`src/lib/api.ts`) transparently detects 401 Unauthorized responses, deduplicates token refresh requests against `POST /api/users/refresh`, and retries failed requests seamlessly.
7. On logout: `POST /api/users/logout` invalidates the session in Redis, clears `localStorage`, and redirects to `/login`.
