# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users

- **Primary**: Retail and swing traders active on the Indonesia Stock Exchange (IDX) looking to formulate, validate, and backtest multi-condition screening hypotheses against historical market data without writing code or Python scripts.
- **Secondary**: Active investors seeking quantitative validation of valuation/fundamental criteria (P/E, P/B, ROE, DER, Dividend Yield) alongside technical signals before committing capital.
- **Administrators**: Platform operators managing user access, system roles (`ADMIN` / `MEMBER`), credentials, and master AI intelligence toggles.

## Product Purpose

Trading Lab empowers Indonesian equity traders to test and refine trading strategies with rigorous historical simulation, deep risk analytics, and AI-assisted guidance. Success means traders can rapidly iterate on screening rules, understand their real risk-adjusted return (accounting for commission fees, holding constraints, and market volatility), and discover or share top-performing strategies with the community.

## Positioning

A purpose-built IDX equity screener and backtesting platform combining a visual multi-group rule engine with authentic Indonesian historical market data (via Sectors.app), realistic local broker commission fee modeling, Jakarta timezone equity curves, and Google Gemini AI insights—delivering institutional-grade quantitative backtesting in an accessible web application.

## Operating Context

- **Workflow**: Strategy ideation -> Visual condition rule building -> AI parameter refinement -> Historical backtest execution (asynchronous) -> Equity curve & metrics review -> Star & publish to community leaderboard.
- **Environment**: Desktop and laptop web browsers (Client-Side Rendered SvelteKit 5 SPA) used during pre-market prep, intraday analysis, or weekend research sessions.
- **Theme Support**: Dark mode (default navy-slate palette) and light mode with high-contrast data visualization.

## Capabilities and Constraints

- **Visual Condition Builder**: Dynamic multi-group rule builder supporting intra-group and inter-group `AND` / `OR` connectors across IDX valuation, profitability, solvency, dividend, and technical indicators.
- **Trade & Risk Management**: Configurable Take-Profit (%), Stop-Loss (%), live Risk-to-Reward ($R:R$) ratio calculation, and maximum holding period limits (days).
- **Asynchronous Backtest Engine**: Non-blocking simulation against historical IDX daily market data with customizable starting capital, simultaneous portfolio size, broker fees (buy/sell), and durations (1, 3, 6, 12 months).
- **Analytics & Visualizations**: Interactive baseline equity curve (TradingView Lightweight Charts) with Net vs Gross equity toggling, win/loss semicircular doughnut chart, sparklines, Sharpe ratio (2% risk-free rate), profit factor, volatility, top gainers/losers, most traded tickers, and complete trade execution history logs.
- **AI Intelligence**: Google Gemini structured suggestions for strategy refinement and qualitative 5-point executive summaries for completed backtests, backed by PostgreSQL/Redis caching and an admin master toggle.
- **Community & Social**: Public backtest leaderboard (ranked by Net Return % and Star Count), star/unstar interactions, and granular per-backtest privacy controls (`is_public`).
- **Access & Security**: Stateful JWT access/refresh token sessions backed by Redis with instant session revocation; RBAC for `ADMIN` and `MEMBER` users.

## Brand Commitments

- **Name**: Trading Lab
- **Slogan**: "Everyone built a stock screener, but no one ever backtested it!"
- **Tone & Voice**: Analytical, precise, objective, and professional. Clean financial terminology without hype or gambling rhetoric.
- **Primary Design Tokens**:
  - Primary Accent: Brand Cyan (`#30B4C9` / `var(--accent)`)
  - Dark Theme Background: Brand Navy Dark (`#2A344C` / `var(--bg)`)
  - Dark Theme Cards: Brand Navy Card (`#3C486A` / `var(--bg-card)`)
  - Light Theme Background: Light Slate (`#EEF0F6` / `var(--bg)`)
  - Typography: Montserrat (`font-sans`)

## Evidence on Hand

- Complete backend implementation in Rust (Actix-web, SQLx, Redis) with database schema migrations in `backend/migrations/`.
- Complete frontend implementation in SvelteKit 5 (CSR, Bun, Tailwind CSS v4, Lucide icons, Lightweight Charts) under `frontend/src/`.
- Architectural specifications, security invariants, and API contracts documented in `AGENTS.md` and `README.md`.
- Financial indicator catalog defined in `frontend/src/lib/constants.ts` and `backend/src/constants/`.

## Product Principles

1. **Empirical Rigor Over Guesswork**: Every strategy must be grounded in measurable data—factoring in real broker commissions, maximum holding duration decay, and exit reason telemetry.
2. **Clarity at a Glance**: Dense financial metrics (Sharpe ratio, max drawdown, win rate, equity curve) must be structured for rapid scanning and unambiguous interpretation.
3. **Frictionless Iteration**: Creating, duplicating, refining with AI, and backtesting variations of a strategy must take seconds, not hours.
4. **Data Integrity & Speed**: Asynchronous execution ensures zero UI blocking; caching guarantees instantaneous repeat lookups.
5. **Community Collaboration with Privacy by Default**: Users maintain full ownership and privacy over their proprietary strategies until explicitly choosing to publish.

## Accessibility & Inclusion

- Support for high-contrast data visualizations and responsive tabular navigation.
- Accessible dialogs, focus trapping, and clear semantic form validation with descriptive error messages.
- Full parity between dark and light themes for prolonged analytical reading.
