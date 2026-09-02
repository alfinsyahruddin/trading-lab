---
name: Trading Lab
description: "Everyone built a stock screener, but no one ever backtested it!"
colors:
  primary: "#30b4c9"
  primary-hover: "#2a9fb2"
  primary-soft: "rgba(48, 180, 201, 0.12)"
  success: "#22c55e"
  danger: "#ef4444"
  warning: "#f59e0b"
  neutral-bg: "#f1f2f4"
  neutral-card: "#ffffff"
  neutral-fg: "#1e293b"
  neutral-muted: "#64748b"
  neutral-border: "#e2e8f0"
  dark-bg: "#2a344c"
  dark-card: "#303b5a"
  dark-card-elevated: "#38476c"
  dark-fg: "#f1f5f9"
  dark-muted: "#94a3b8"
  dark-border: "rgba(255, 255, 255, 0.1)"
typography:
  display:
    fontFamily: "Montserrat, system-ui, sans-serif"
    fontSize: "2.25rem"
    fontWeight: 700
    lineHeight: 1.15
    letterSpacing: "-0.02em"
  headline:
    fontFamily: "Montserrat, system-ui, sans-serif"
    fontSize: "1.5rem"
    fontWeight: 700
    lineHeight: 1.25
    letterSpacing: "-0.01em"
  title:
    fontFamily: "Montserrat, system-ui, sans-serif"
    fontSize: "1.125rem"
    fontWeight: 600
    lineHeight: 1.35
    letterSpacing: "normal"
  body:
    fontFamily: "Montserrat, system-ui, sans-serif"
    fontSize: "0.875rem"
    fontWeight: 400
    lineHeight: 1.6
    letterSpacing: "normal"
  label:
    fontFamily: "Montserrat, system-ui, sans-serif"
    fontSize: "0.75rem"
    fontWeight: 600
    lineHeight: 1.4
    letterSpacing: "0.025em"
rounded:
  sm: "6px"
  md: "8px"
  lg: "12px"
  xl: "16px"
  full: "9999px"
spacing:
  xs: "4px"
  sm: "8px"
  md: "16px"
  lg: "24px"
  xl: "32px"
  2xl: "48px"
components:
  button-primary:
    backgroundColor: "{colors.primary}"
    textColor: "#ffffff"
    rounded: "{rounded.md}"
    padding: "10px 18px"
  button-primary-hover:
    backgroundColor: "{colors.primary-hover}"
  card-standard:
    backgroundColor: "{colors.neutral-card}"
    rounded: "{rounded.lg}"
    padding: "24px"
  input-field:
    backgroundColor: "{colors.neutral-card}"
    textColor: "{colors.neutral-fg}"
    rounded: "{rounded.md}"
    padding: "10px 14px"
---

# Design System: Trading Lab

## Overview

**Creative North Star: "The Quantitative Cockpit"**

Trading Lab combines the focused precision of professional financial trading software with the clarity and elegance of modern web interfaces. The visual language is structured for high data density, rapid cognitive scanning, and unambiguous decision-making during fast-paced market analysis.

The UI is anchored by a dual-palette architecture: a deep, commanding Navy Dark theme (`#2A344C`) that reduces eye strain during extended analytical sessions, and a crisp, high-contrast Slate Light theme (`#F1F2F4`) for daylight productivity. An electric Brand Cyan (`#30B4C9`) acts as the single primary accent, guiding user focus to primary actions, active navigation states, and key interactive controls.

**Key Characteristics:**
- **High-Density Legibility:** Clean typography hierarchy powered by Montserrat with tabular numbers for financial figures and equity metrics.
- **Tonal Layering Over Heavy Shadows:** Visual depth achieved through surface tiering, subtle 1px border frames, and localized radial atmospheric orbs.
- **Tactile Micro-Interactions:** Custom iOS-inspired sliding segmented controls, smooth pill toggles, and sliding bottom navigation indicators.
- **AI Intelligence Distinction:** Chromatic multi-stop rotating conic gradient borders (`#FBAF33` -> `#FE426B` -> `#DE98FE` -> `#38C1FB`) reserve dynamic lighting strictly for AI-generated insights and recommendations.

## Colors

The palette is engineered for precision, pairing deep slate-navy backgrounds with high-visibility semantic signals for profit/loss, status tracking, and focal interactions.

### Primary
- **Electric Cyan** (`#30B4C9` / `var(--accent)`): The primary operational accent. Used for primary call-to-action buttons, active navigation indicators, focus rings, and highlighted strategy metrics.
- **Deep Cyan** (`#2A9FB2` / `var(--accent-hover)`): The interactive hover and pressed state for primary controls.
- **Cyan Tint** (`rgba(48, 180, 201, 0.12)` / `var(--accent-soft)`): Soft tinted background for active badges, selection chips, and subtle focus glows.

### Status & Semantic
- **Emerald Profit** (`#22C55E` / `var(--success)`): Positive trade returns, winning equity curves, take-profit triggers, and `DONE` execution states.
- **Coral Risk** (`#EF4444` / `var(--danger)`): Negative P/L, stop-loss triggers, `FAILED` backtest states, destructive actions, and field validation errors.
- **Amber Caution** (`#F59E0B` / `var(--warning)`): Community stars, pending review notices, `PROCESSING` state badges, and cautionary risk warnings.
- **Amethyst Metric** (`#8B5CF6`): Secondary analytical accents, backtest metric orbs, and duration indicators.

### Neutral (Theme Aware)
- **Deep Navy Base** (`#2A344C` / `var(--bg)` [Dark]): Main application canvas for dark theme.
- **Elevated Navy Card** (`#303B5A` / `var(--bg-card)` [Dark]): Primary surface container for dark mode cards, tables, and dialogs.
- **Nested Navy Card** (`#38476C` / `var(--bg-card-2)` [Dark]): Secondary nested surface inside cards and builder groups.
- **Light Slate Base** (`#F1F2F4` / `var(--bg)` [Light]): Main application canvas for light theme.
- **Pure White Card** (`#FFFFFF` / `var(--bg-card)` [Light]): Primary card and input background for light mode.
- **Slate Obsidian** (`#1E293B` / `var(--fg)` [Light]): High-contrast primary body text and titles in light mode.
- **Slate Starlight** (`#F1F5F9` / `var(--fg)` [Dark]): High-contrast primary body text and titles in dark mode.
- **Muted Slate** (`#64748B` [Light] / `#94A3B8` [Dark] / `var(--fg-muted)`): Secondary metadata, table headers, labels, and timestamps.
- **Subtle Border** (`#E2E8F0` [Light] / `rgba(255, 255, 255, 0.1)` [Dark] / `var(--border)`): 1px structural framing.

### Named Rules
**The 10% Accent Rarity Rule.** Electric Cyan is reserved strictly for interactive affordances and primary state indicators. It must never coat background containers or non-interactive decorations.
**The Strict Financial Polarity Rule.** Green (`#22C55E`) and Red (`#EF4444`) are exclusively reserved for financial outcomes (gain vs loss, TP vs SL) and execution status. Never use them for decorative or non-semantic UI elements.

## Typography

**Display & Body Font:** Montserrat (`font-sans`, fallback: `system-ui, sans-serif`)
**Tabular/Code Font:** Monospace (`ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace`)

**Character:** Modern geometric sans-serif with wide letterforms and crisp legibility at small sizes, delivering an authoritative fintech feel without visual clutter.

### Hierarchy
- **Display** (Bold 700/800, `2.25rem–2.5rem` / `36px–40px`, line-height `1.15`, letter-spacing `-0.02em`): KPI headline values, primary equity returns, and hero numbers. Always paired with `tabular-nums`.
- **Headline** (Bold 700, `1.5rem` / `24px`, line-height `1.25`, letter-spacing `-0.01em`): Page titles (e.g., "Dashboard", "Trading Strategies", "Backtest Results").
- **Title** (SemiBold 600 / Bold 700, `1.125rem–1.25rem` / `18px–20px`, line-height `1.35`): Section headings, card titles, modal headers, and table segment titles.
- **Body** (Regular 400 / Medium 500, `0.875rem` / `14px`, line-height `1.6`): Standard body copy, descriptions, input values, and data table cell contents.
- **Label / Caption** (SemiBold 600, `0.75rem` / `12px`, letter-spacing `0.025em`): Input field labels, badge text, table header columns, and chart axis labels.

### Named Rules
**The Tabular Metric Rule.** Every numeric figure representing prices, currency values, percentages, star counts, or dates must use tabular numbers (`tabular-nums`) to maintain alignment during live updates.

## Layout

The spatial model balances generous structural breathing room with dense data presentation inside cards and tables.

- **Shell Containers:**
  - Standard App Width: `max-w-7xl` (`1280px`) with responsive horizontal padding (`px-4 sm:px-6`).
  - Focused Dashboards & Detail Views: `max-w-5xl` (`1024px`).
  - Strategy & Backtest Builders: `max-w-4xl` (`896px`).
  - Modals & Auth Dialogs: `max-w-md` (`448px`) to `max-w-lg` (`512px`).
- **Rhythm & Grid:**
  - KPI Stat Grids: `grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4`.
  - Content Stack Gap: `space-y-6` to `space-y-8`.
  - Intra-Card Spacing: `p-5 sm:p-6`.
- **Responsive Adaptations:**
  - Navigation shifts from full horizontal desktop bar with sliding indicator to a sleek drawer overlay on screens `< 640px`.
  - Tables collapse horizontal padding and hide non-essential secondary columns on mobile screens.

## Elevation & Depth

Trading Lab uses a **flat-by-default, tonal layering** philosophy. Surfaces sit flat on the canvas and rely on distinct background color steps (`--bg` -> `--bg-card` -> `--bg-card-2`) and subtle border strokes (`1px solid var(--border)`).

### Shadow Vocabulary
- **Subtle Rest / Card** (`box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05)`): Light baseline elevation for cards and buttons.
- **Card Glow Orb** (`radial-gradient(ellipse 90% 80% at 100% 0%, [color] 0%, transparent 70%)`): Ambient color corner bloom on KPI statistic cards.
- **Modal Elevation** (`box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25)`): High-contrast backdrop-blurred modal dialog elevation.
- **Active Switch Glow** (`box-shadow: 0 0 12px rgba(48, 180, 201, 0.35)`): Ambient cyan emission on active toggle switches.
- **AI Summary Conic Beam** (`conic-gradient(...)` with `animation: spinBeam 6s linear infinite`): Dynamic moving light perimeter for AI intelligence cards.

### Named Rules
**The Structural Border Rule.** All cards, modals, dropdowns, and input containers must have an explicit 1px border (`var(--border)` or `var(--border-strong)`). Shadows alone must never define a container's edge.

## Shapes

Form language emphasizes smooth, approachable geometry through progressive corner radii:

- **Inputs & Standard Buttons:** `8px` (`rounded-lg` / `{rounded.md}`).
- **Cards & Data Tables:** `12px` to `16px` (`rounded-xl` / `rounded-2xl` / `{rounded.lg}` to `{rounded.xl}`).
- **Modals & Dialogs:** `16px` (`rounded-2xl` / `{rounded.xl}`).
- **Badges, Switch Controls & Floating Tabs:** `9999px` (`rounded-full`).

## Components

### Buttons
- **Primary:** Electric Cyan background (`#30B4C9`), white bold text, `rounded-lg` (`8px`), padding `10px 18px` (`px-4 py-2.5`). Interactive transform: `active:scale-97`.
- **Secondary / Neutral:** Transparent/card background, `1px solid var(--border)`, hover background `var(--bg-card-hover)`.
- **Destructive:** Coral Red fill (`#EF4444`) or red text on hover, reserved for deletions and session termination.

### Text & Select Fields
- **Container:** Background `var(--bg-input)`, border `var(--border-strong)`, `rounded-lg` (`8px`), padding `10px 14px`.
- **Focus State:** Border shifts to `var(--accent)` with a soft outer ring: `box-shadow: 0 0 0 3px rgba(48, 180, 201, 0.1)`.
- **Error State:** Border shifts to `var(--danger)` with error message caption below.

### Segmented Control
- **Structure:** iOS-style track (`rounded-[9px]`, padding `2px`) with smooth sliding pill thumb indicator (`transition: transform 0.28s cubic-bezier(0.32, 0.72, 0, 1)`).
- **Dividers:** Faded hairline vertical separators between unselected segments.

### Status Badges
- **Shape:** Pill container (`rounded-full`, padding `2px 8px`, font size `12px`, font weight `600`).
- **States:**
  - `PENDING`: Slate tint (`rgba(148, 163, 184, 0.15)`), static gray dot.
  - `PROCESSING`: Amber tint (`rgba(245, 158, 11, 0.15)`), bouncing warning dot, pulse animation.
  - `DONE`: Emerald tint (`rgba(16, 185, 129, 0.15)`), solid green dot.
  - `FAILED`: Red tint (`rgba(239, 68, 68, 0.15)`), solid red dot.

### AI Executive Summary Card
- **Signature Treatment:** Dual-layer container with a 1.5px border gap, an animated rotating conic beam (`#FBAF33` -> `#FE426B` -> `#DE98FE` -> `#38C1FB`), and a subtle four-corner radial color bleed inside the surface.

## Do's and Don'ts

### Do:
- **Do** format all financial figures, percentages, and metrics with `tabular-nums` for rock-solid tabular alignment.
- **Do** use `var(--accent)` for actionable highlights and active navigation states.
- **Do** maintain strict 1px border framing on cards and surfaces in both dark and light modes.
- **Do** provide smooth active micro-interactions (`active:scale-97`) on clickable buttons and interactive items.
- **Do** pair status badges with colored status dots for accessibility and instantaneous scanning.

### Don't:
- **Don't** use neon or saturated primary backgrounds for large content cards; keep backgrounds restful and navy/slate.
- **Don't** use red or green for non-financial decorative accents.
- **Don't** mix custom font families; stick to Montserrat for display/body and monospace for numbers/code.
- **Don't** apply heavy drop shadows without a 1px structural container border.
- **Don't** use raw unformatted numbers in charts or stat cards without appropriate precision and currency prefixes.

---

## Landing Page Design Identity

The public marketing landing page (`/`) operates under a **separate visual world** — the *Deep-Space Financial Observatory* — intentionally distinct from the dashboard's "Quantitative Cockpit" aesthetic. These systems coexist without conflict.

### World: Deep-Space Financial Observatory

**Concept:** IDX stocks as data constellations orbiting a central pulsar. Each screener pass is an astronomical observation; each backtest, a measured event track. The page establishes empirical credibility through scientific wonder.

**Design constants:**
- **Background ground:** `#070A12` (near-void cosmic black) — darker than the app's `#2A344C` to fully separate the marketing context.
- **Orbital arcs:** Brand Cyan (`#30B4C9`) at 1px, with `rgba(48, 180, 201, 0.25)` glow radials.
- **AI accent:** Faint gold `#F5C842` (replaces amber; reserved for AI features only on this surface).
- **Positive signal:** `#22C55E` (inherited from app system, used for return percentages in leaderboard).
- **Dark card surface:** `rgba(13, 21, 37, 0.72)` with 1px cyan-tinted border.
- **Theme:** Strict Dark Mode Only — cinematic observatory aesthetic locked to dark cosmic void.

### Typography (Landing-Exclusive)

| Role | Family | Weight | Notes |
|------|--------|--------|-------|
| Display / headlines | Barlow Condensed | 800 | Italic for key slogan phrase |
| Sub-headlines | Barlow Condensed | 700 | Section headings |
| Step numbers | Barlow Condensed | 800 | 3rem, 30% opacity accent |
| Body / descriptions | Figtree | 400–500 | Replaces Montserrat on this surface |
| Labels / eyebrows | Figtree | 600 | Tracked 0.2em uppercase |
| Stats / metrics | Barlow Condensed | 700 | Tabular nums |

Both faces are loaded from Google Fonts scoped to the landing page only — they do not affect the dashboard or any app route.

### Motion Principles

- **Parallax:** Three layers — nebula image (0.12x scroll factor), orbital arc image (0.06x), content pinned.
- **Starfield:** 2D canvas with ~250 micro-dots, sinusoidal twinkle at variable speeds. Cleared and redrawn at 60fps. No GPU shader dependency.
- **Entrance:** `opacity: 0 → 1` + `translateY(24px → 0)` via IntersectionObserver; `cubic-bezier(0.22, 1, 0.36, 1)` for hero; `ease` for sections.
- **CTA rings:** Three concentric SVG circles in slow independent rotation (40s / 25s / 15s).
- **Pulsar:** Sinusoidal `scale(1 → 1.5)` beat at 2.5s, with box-shadow bloom.
- **Reduced motion:** All animations disabled via `@media (prefers-reduced-motion: reduce)`.

### Static Assets

All assets reside in `frontend/static/` with `landing-` prefix:
| File | Description |
|------|-------------|
| `landing-hero.jpg` | IDX orbital arc constellation (AI-generated) |
| `landing-nebula.jpg` | Deep space nebula background (AI-generated) |
| `landing-ai.jpg` | Neural network constellation illustration (AI-generated) |
| `screenshot-dashboard.png` | App dashboard screenshot |
| `screenshot-backtest.png` | Backtest results screenshot |
| `screenshot-ai.png` | AI strategy suggestions screenshot |

### Isolation Rules

- The `.lp-root` CSS scope contains all landing-page tokens as custom properties. No global tokens from `app.css` are used directly (to prevent bleed).
- Fonts are loaded via `<svelte:head>` and scoped to `:global(body):has(.lp-root)` — they activate only when the landing page is mounted.
- The landing page does **not** use TailwindCSS utility classes — all styling is scoped component CSS, keeping the footprint isolated from the app's utility layer.
