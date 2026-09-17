# Frontend Architecture & Component Guidelines

This document provides a comprehensive reference for the frontend application of Trading Lab, built with [SvelteKit 2](https://kit.svelte.dev/), [Svelte 5 Runes](https://svelte.dev/docs/svelte/v5-migration-guide), [Bun](https://bun.sh/), and [Tailwind CSS v4](https://tailwindcss.com/).

---

## 1. Tech Stack & Execution Mode

- **Framework**: [SvelteKit 2](https://kit.svelte.dev/) with **Svelte 5 Runes**
- **Mode**: Pure Client-Side Rendering (CSR / SPA mode: `export const ssr = false;` in [`src/routes/+layout.ts`](../frontend/src/routes/+layout.ts))
- **Runtime & Package Manager**: [Bun](https://bun.sh/)
- **Styling**: [Tailwind CSS v4](https://tailwindcss.com/) with CSS design token variables in [`src/app.css`](../frontend/src/app.css)
- **Charts**: [TradingView Lightweight Charts](https://tradingview.github.io/lightweight-charts/) (`PortfolioChart.svelte`) and native SVG charts (`HalfDoughnutChart.svelte`, `BacktestResultPreview.svelte`)
- **Icons**: Lucide icons via [`@iconify/svelte`](https://iconify.design/)
- **Testing**: [Vitest](https://vitest.dev/) for unit/component tests and [Playwright](https://playwright.dev/) for end-to-end integration tests

---

## 2. Svelte 5 Runes & State Conventions

All components strictly use **Svelte 5 Runes**:
- `$props()`: Strongly-typed component property definitions.
- `$state()`: Reactive variables and object state.
- `$derived()` / `$derived.by()`: Pure computed values and derived state.
- `$effect()`: Side-effects, DOM manipulations, and external event subscriptions.
- **Helper Modules**: Any reactive state declared outside `.svelte` components must reside in `.svelte.ts` files (e.g. [`src/lib/helpers/toast.svelte.ts`](../frontend/src/lib/helpers/toast.svelte.ts)).
- **Keyed Each Loops**: Always use explicit unique keys for `#each` blocks: `{#each items as item (item.id)}`.

---

## 3. Client-Side Routing & Feature Gating

Because the application runs entirely as a Client-Side Single Page Application (CSR), authentication checks and feature flags execute in client-side guards:

1. **Authentication Guards**:
   - [`src/routes/dashboard/+layout.ts`](../frontend/src/routes/dashboard/+layout.ts): Reads `getToken()`. If null, redirects visitors to `/login`.
   - [`src/routes/login/+page.ts`](../frontend/src/routes/login/+page.ts) & [`register/+page.ts`](../frontend/src/routes/register/+page.ts): If the user is already authenticated, redirects directly to `/dashboard`.
2. **Coming Soon Mode (`PUBLIC_IS_COMING_SOON`)**:
   - Feature flag controlled by `PUBLIC_IS_COMING_SOON` in [`frontend/.env.example`](../frontend/.env.example) and evaluated via [`isComingSoon()`](../frontend/src/lib/helpers/config.ts).
   - When enabled, registration submissions, quick logins, and public CTA buttons display [`ComingSoonModal.svelte`](../frontend/src/lib/components/ComingSoonModal.svelte) instead of submitting or navigating.

### Client-Side Routes Breakdown:
- [`src/routes/+page.svelte`](../frontend/src/routes/+page.svelte): Public landing page with live interactive equity chart widget, feature catalog, workflow walkthrough, and testimonials.
- [`src/routes/login/+page.svelte`](../frontend/src/routes/login/+page.svelte): User login with remembered accounts quick login.
- [`src/routes/register/+page.svelte`](../frontend/src/routes/register/+page.svelte): Member registration with SVG captcha challenge verification.
- [`src/routes/dashboard/+page.svelte`](../frontend/src/routes/dashboard/+page.svelte): Dashboard home featuring user KPI stats cards and community backtest leaderboards.
- [`src/routes/dashboard/strategies/+page.svelte`](../frontend/src/routes/dashboard/strategies/+page.svelte): Strategy listing with duplicate, edit, and delete actions.
- [`src/routes/dashboard/strategies/new/+page.svelte`](../frontend/src/routes/dashboard/strategies/new/+page.svelte): Visual strategy builder with AI suggestion interception.
- [`src/routes/dashboard/strategies/[id]/+page.svelte`](../frontend/src/routes/dashboard/strategies/[id]/+page.svelte): Strategy detail view and linked backtests.
- [`src/routes/dashboard/strategies/[id]/edit/+page.svelte`](../frontend/src/routes/dashboard/strategies/[id]/edit/+page.svelte): Strategy update form.
- [`src/routes/dashboard/backtests/+page.svelte`](../frontend/src/routes/dashboard/backtests/+page.svelte): Backtest job cards with sparkline previews and status polling.
- [`src/routes/dashboard/backtests/new/+page.svelte`](../frontend/src/routes/dashboard/backtests/new/+page.svelte): Simulation configuration (year, duration, cash, max holdings, max screener candidates, fees, visibility).
- [`src/routes/dashboard/backtests/[id]/+page.svelte`](../frontend/src/routes/dashboard/backtests/[id]/+page.svelte): Comprehensive backtest analytics, TradingView chart, win/loss ratio, trade logs, and screener indicator inspection.
- [`src/routes/dashboard/settings/+page.svelte`](../frontend/src/routes/dashboard/settings/+page.svelte): Platform settings (Master AI toggle - Admin only).
- [`src/routes/dashboard/users/+page.svelte`](../frontend/src/routes/dashboard/users/+page.svelte): User management console (Admin only).

---

## 4. UI Component Catalog

### A. Common & Shared UI ([`src/lib/components/`](../frontend/src/lib/components))
- [`TopBar.svelte`](../frontend/src/lib/components/TopBar.svelte): Global app navigation, user menu, profile edit modal, password reset modal, theme toggle.
- [`EditProfileModal.svelte`](../frontend/src/lib/components/EditProfileModal.svelte): Dialog for changing name and email.
- [`ChangePasswordModal.svelte`](../frontend/src/lib/components/ChangePasswordModal.svelte): Dialog for updating password with Argon2id rehash.
- [`ComingSoonModal.svelte`](../frontend/src/lib/components/ComingSoonModal.svelte): Gating modal informing visitors that public onboarding is currently limited.
- [`Modal.svelte`](../frontend/src/lib/components/Modal.svelte) & [`ConfirmModal.svelte`](../frontend/src/lib/components/ConfirmModal.svelte): Accessible dialog containers with backdrop blur.
- [`DataTable.svelte`](../frontend/src/lib/components/DataTable.svelte): Responsive tabular layout for data collections.
- [`SegmentedControl.svelte`](../frontend/src/lib/components/SegmentedControl.svelte): Pill-style toggle control.
- [`SelectField.svelte`](../frontend/src/lib/components/SelectField.svelte) & [`TextField.svelte`](../frontend/src/lib/components/TextField.svelte): Standard input components with validation feedback and label bindings.
- [`RoleBadge.svelte`](../frontend/src/lib/components/RoleBadge.svelte): Stylized tag badge for `ADMIN` and `MEMBER`.
- [`IosSwitch.svelte`](../frontend/src/lib/components/IosSwitch.svelte): Smooth iOS-style toggle.
- [`ThemeToggle.svelte`](../frontend/src/lib/components/ThemeToggle.svelte): Dark / light theme toggle synced to `data-theme` attribute and `localStorage`.
- [`ToastViewport.svelte`](../frontend/src/lib/components/ToastViewport.svelte): Toast notification container.
- [`EmptyState.svelte`](../frontend/src/lib/components/EmptyState.svelte): Fallback state illustration when lists are empty.

### B. Strategy Components ([`src/lib/components/strategy/`](../frontend/src/lib/components/strategy))
- [`StrategyForm.svelte`](../frontend/src/lib/components/strategy/StrategyForm.svelte): Core strategy configuration (name, description, TP%, SL%, max holding days, live R:R calculation).
- [`StrategyAiSuggestionsCard.svelte`](../frontend/src/lib/components/strategy/StrategyAiSuggestionsCard.svelte): Suggestion panel offering one-click "Accept" or "Ignore" recommendations for parameters and rules.
- [`WhereConditionsBuilder.svelte`](../frontend/src/lib/components/strategy/WhereConditionsBuilder.svelte): Multi-group condition editor with intra-group and inter-group `AND`/`OR` connectors.
- [`VariablePickerModal.svelte`](../frontend/src/lib/components/strategy/VariablePickerModal.svelte): Searchable, categorized financial indicator catalog.

### C. Backtest Components ([`src/lib/components/backtest/`](../frontend/src/lib/components/backtest))
- [`BacktestAiSummary.svelte`](../frontend/src/lib/components/backtest/BacktestAiSummary.svelte): 5-dimension qualitative summary card with glowing animated border.
- [`StatusBadge.svelte`](../frontend/src/lib/components/backtest/StatusBadge.svelte): Visual status tag for `PENDING`, `PROCESSING`, `DONE`, `FAILED`.
- [`HalfDoughnutChart.svelte`](../frontend/src/lib/components/backtest/HalfDoughnutChart.svelte): Semicircular SVG win vs loss ratio chart.
- [`PortfolioChart.svelte`](../frontend/src/lib/components/backtest/PortfolioChart.svelte): Baseline interactive equity curve powered by `lightweight-charts`.
- [`BacktestResultPreview.svelte`](../frontend/src/lib/components/backtest/BacktestResultPreview.svelte): Compact SVG sparkline area chart.
- [`TradeInfoModal.svelte`](../frontend/src/lib/components/backtest/TradeInfoModal.svelte): Modal revealing the exact screener values and query metrics (`query_values`) captured when a trade entry was triggered.

### D. Dashboard Components ([`src/lib/components/dashboard/`](../frontend/src/lib/components/dashboard))
- [`StatsCard.svelte`](../frontend/src/lib/components/dashboard/StatsCard.svelte): Glassmorphism KPI statistic card with glowing accent orbs.
- [`LeaderboardCard.svelte`](../frontend/src/lib/components/dashboard/LeaderboardCard.svelte): Public backtest entry card with return %, win rate, mini sparkline, and star button.

### E. Landing Page Components ([`src/lib/components/landing/`](../frontend/src/lib/components/landing))
- [`HeroSection.svelte`](../frontend/src/lib/components/landing/HeroSection.svelte): Landing hero banner with call to action.
- [`HeroChartWidget.svelte`](../frontend/src/lib/components/landing/HeroChartWidget.svelte): Interactive lightweight-chart widget displayed in the hero section.
- [`FeaturesSection.svelte`](../frontend/src/lib/components/landing/FeaturesSection.svelte) & [`FeatureCard.svelte`](../frontend/src/lib/components/landing/FeatureCard.svelte): Categorized feature breakdown.
- [`HowItWorksSection.svelte`](../frontend/src/lib/components/landing/HowItWorksSection.svelte) & [`WorkflowStep.svelte`](../frontend/src/lib/components/landing/WorkflowStep.svelte): Step-by-step workflow guide.
- [`ScreenshotsSection.svelte`](../frontend/src/lib/components/landing/ScreenshotsSection.svelte) & [`ScreenshotCard.svelte`](../frontend/src/lib/components/landing/ScreenshotCard.svelte): Product screenshot showcases.
- [`CtaSection.svelte`](../frontend/src/lib/components/landing/CtaSection.svelte) & [`LandingCtaButtons.svelte`](../frontend/src/lib/components/landing/LandingCtaButtons.svelte): Conversion triggers with Coming Soon modal gating.
- [`LandingFooter.svelte`](../frontend/src/lib/components/landing/LandingFooter.svelte), [`LandingLogo.svelte`](../frontend/src/lib/components/landing/LandingLogo.svelte), [`SectionHeader.svelte`](../frontend/src/lib/components/landing/SectionHeader.svelte): Landing layout components.

---

## 5. Helpers & API Client

- **API Client** ([`src/lib/api.ts`](../frontend/src/lib/api.ts)):
  - Attaches `Authorization: Bearer <token>` to requests.
  - Automatically intercepts 401s to refresh access tokens via `/api/users/refresh` with single-flight request deduplication.
  - Unwraps standard `BaseResponse<T>` payloads and raises typed [`ApiError`](../frontend/src/lib/api.ts) on non-2xx responses.
- **Session Helper** ([`src/lib/helpers/session.ts`](../frontend/src/lib/helpers/session.ts)):
  - Manages access/refresh tokens in `localStorage`.
  - Maintains remembered accounts for quick login functionality.
- **Config Helper** ([`src/lib/helpers/config.ts`](../frontend/src/lib/helpers/config.ts)):
  - Resolves `PUBLIC_API_BASE_URL` and `PUBLIC_IS_COMING_SOON`.
- **Date Helper** ([`src/lib/helpers/date.ts`](../frontend/src/lib/helpers/date.ts)):
  - Jakarta timezone calculations, backtest date range generation (`calculateBacktestDateRange`).
- **Reveal Action** ([`src/lib/helpers/reveal.ts`](../frontend/src/lib/helpers/reveal.ts)):
  - IntersectionObserver Svelte action (`use:reveal`) for scroll animations.
- **Toast Store** ([`src/lib/helpers/toast.svelte.ts`](../frontend/src/lib/helpers/toast.svelte.ts)):
  - Reactive toast notification manager (`toast.success()`, `toast.error()`, `toast.info()`).

---

## 6. Styling & Design Tokens

Defined in [`src/app.css`](../frontend/src/app.css) using Tailwind CSS v4 `@theme` variables:
- Primary Accent: `--color-brand-cyan` (`#30B4C9`)
- Dark Background: `--color-brand-navy-dark` (`#2A344C`)
- Dark Card Background: `--color-brand-navy-card` (`#3C486A`)
- Light Background: `--color-brand-light-bg` (`#EEF0F6`)
- Light Card Background: `--color-brand-light-card` (`#FFFFFF`)
- Font Family: `Montserrat`
- **Rule**: Always use semantic CSS tokens (`var(--accent)`, `var(--bg-card)`, `var(--fg)`, `var(--border)`) rather than hardcoded colors.
