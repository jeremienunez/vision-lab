# Graphic Tokens Spec

## Objective

The design system must give PerceptionLab a clear product identity while staying implementation-friendly. The frontend should feel like a technical ML lab: precise, dark-first, dense enough for data, but still readable during a live demo.

The visual system is token-driven. Components must not hard-code colors, spacing, radii, shadows, or status colors.

## Token Layers

PerceptionLab uses three token layers:

```text
primitive tokens
  -> raw color, spacing, radius, typography, shadow, timing values

semantic tokens
  -> background, surface, border, text, success, warning, danger, info

component tokens
  -> button, card, badge, chart, detection overlay, camera HUD
```

Primitive tokens are allowed only in token files. Components should consume semantic or component tokens.

## Visual Direction

Keywords:

```text
technical
precise
low-noise
real-time
computer vision
ML infrastructure
```

The default theme is dark. A light theme can exist later, but the portfolio demo should be optimized for dark UI, because overlays, charts, and camera previews are easier to read with controlled contrast.

## Color Primitives

CSS custom properties:

```css
:root {
  --pl-gray-950: #09090b;
  --pl-gray-900: #111114;
  --pl-gray-850: #17171c;
  --pl-gray-800: #1f1f26;
  --pl-gray-700: #2d2d36;
  --pl-gray-600: #444451;
  --pl-gray-500: #686879;
  --pl-gray-400: #9a9aaa;
  --pl-gray-300: #c8c8d2;
  --pl-gray-200: #e4e4ea;
  --pl-gray-100: #f4f4f7;

  --pl-blue-500: #3b82f6;
  --pl-cyan-500: #06b6d4;
  --pl-violet-500: #8b5cf6;
  --pl-green-500: #22c55e;
  --pl-amber-500: #f59e0b;
  --pl-red-500: #ef4444;
}
```

The palette is intentionally small. New colors must be added only when a semantic role cannot be expressed with the existing primitives.

## Semantic Colors

```css
:root {
  --pl-bg-app: var(--pl-gray-950);
  --pl-bg-shell: var(--pl-gray-900);
  --pl-bg-surface: var(--pl-gray-850);
  --pl-bg-surface-raised: var(--pl-gray-800);

  --pl-border-subtle: rgba(255, 255, 255, 0.08);
  --pl-border-strong: rgba(255, 255, 255, 0.16);

  --pl-text-primary: var(--pl-gray-100);
  --pl-text-secondary: var(--pl-gray-300);
  --pl-text-muted: var(--pl-gray-400);
  --pl-text-disabled: var(--pl-gray-500);

  --pl-accent-primary: var(--pl-blue-500);
  --pl-accent-secondary: var(--pl-cyan-500);
  --pl-accent-experiment: var(--pl-violet-500);

  --pl-status-success: var(--pl-green-500);
  --pl-status-warning: var(--pl-amber-500);
  --pl-status-danger: var(--pl-red-500);
  --pl-status-info: var(--pl-blue-500);
}
```

## Status Tokens

Training, export, model, and camera statuses must share the same semantic grammar.

```css
:root {
  --pl-status-queued-bg: rgba(59, 130, 246, 0.12);
  --pl-status-queued-fg: #93c5fd;

  --pl-status-running-bg: rgba(6, 182, 212, 0.12);
  --pl-status-running-fg: #67e8f9;

  --pl-status-succeeded-bg: rgba(34, 197, 94, 0.12);
  --pl-status-succeeded-fg: #86efac;

  --pl-status-failed-bg: rgba(239, 68, 68, 0.12);
  --pl-status-failed-fg: #fca5a5;

  --pl-status-cancelled-bg: rgba(154, 154, 170, 0.12);
  --pl-status-cancelled-fg: #c8c8d2;
}
```

Status badges should use semantic status tokens, never custom per-component colors.

## Detection Tokens

Object detection overlays need their own component tokens because they appear over camera and image content.

```css
:root {
  --pl-detection-box-stroke: #60a5fa;
  --pl-detection-box-stroke-strong: #22d3ee;
  --pl-detection-label-bg: rgba(9, 9, 11, 0.84);
  --pl-detection-label-fg: #f4f4f7;
  --pl-detection-label-border: rgba(255, 255, 255, 0.18);
  --pl-detection-low-confidence: #f59e0b;
  --pl-detection-high-confidence: #22c55e;
  --pl-detection-depth: #8b5cf6;
}
```

Rules:

- bbox stroke color should not encode class by default
- confidence and depth can add label accents
- low-confidence detections may use warning styling
- stale detections should fade rather than disappear abruptly in live mode

## Typography Tokens

```css
:root {
  --pl-font-sans: Inter, ui-sans-serif, system-ui, sans-serif;
  --pl-font-mono: JetBrains Mono, ui-monospace, SFMono-Regular, monospace;

  --pl-text-xs: 0.75rem;
  --pl-text-sm: 0.875rem;
  --pl-text-md: 1rem;
  --pl-text-lg: 1.125rem;
  --pl-text-xl: 1.25rem;
  --pl-text-2xl: 1.5rem;

  --pl-line-tight: 1.15;
  --pl-line-normal: 1.5;
  --pl-line-relaxed: 1.7;
}
```

Usage:

- product copy uses sans
- IDs, metrics, JSON snippets, and latency use mono
- metric cards can use mono numerals for stability

## Spacing Tokens

```css
:root {
  --pl-space-1: 0.25rem;
  --pl-space-2: 0.5rem;
  --pl-space-3: 0.75rem;
  --pl-space-4: 1rem;
  --pl-space-5: 1.25rem;
  --pl-space-6: 1.5rem;
  --pl-space-8: 2rem;
  --pl-space-10: 2.5rem;
  --pl-space-12: 3rem;
}
```

Rules:

- dense tables use `space-2` and `space-3`
- cards use `space-4` or `space-5`
- page sections use `space-8` or `space-10`

## Radius Tokens

```css
:root {
  --pl-radius-sm: 0.375rem;
  --pl-radius-md: 0.625rem;
  --pl-radius-lg: 0.875rem;
  --pl-radius-xl: 1.25rem;
  --pl-radius-pill: 999px;
}
```

Usage:

- badge: pill
- button: md
- card: lg
- modal/panel: xl
- detection label: sm

## Shadow And Depth Tokens

```css
:root {
  --pl-shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.24);
  --pl-shadow-md: 0 12px 32px rgba(0, 0, 0, 0.28);
  --pl-shadow-glow-blue: 0 0 0 1px rgba(59, 130, 246, 0.24), 0 0 32px rgba(59, 130, 246, 0.12);
}
```

Use glow sparingly. It should highlight active live camera or inference states, not decorate static cards.

## Motion Tokens

```css
:root {
  --pl-duration-fast: 120ms;
  --pl-duration-normal: 180ms;
  --pl-duration-slow: 280ms;
  --pl-ease-standard: cubic-bezier(0.2, 0, 0, 1);
}
```

Rules:

- no slow decorative animation in live camera mode
- loading shimmer should not appear over camera preview
- live overlay fade must stay under `duration-slow`

## Z-Index Tokens

```css
:root {
  --pl-z-base: 0;
  --pl-z-overlay: 10;
  --pl-z-topbar: 20;
  --pl-z-popover: 30;
  --pl-z-modal: 40;
  --pl-z-toast: 50;
}
```

Detection canvas uses `z-overlay` inside the camera panel, not global z-index hacks.

## Component Token Examples

```css
:root {
  --pl-card-bg: var(--pl-bg-surface);
  --pl-card-border: var(--pl-border-subtle);
  --pl-card-radius: var(--pl-radius-lg);
  --pl-card-padding: var(--pl-space-5);

  --pl-button-primary-bg: var(--pl-accent-primary);
  --pl-button-primary-fg: white;
  --pl-button-secondary-bg: var(--pl-bg-surface-raised);
  --pl-button-secondary-fg: var(--pl-text-primary);

  --pl-camera-frame-bg: #000000;
  --pl-camera-frame-border: var(--pl-border-strong);
  --pl-camera-hud-bg: rgba(9, 9, 11, 0.72);
  --pl-camera-hud-fg: var(--pl-text-primary);
}
```

## TypeScript Token Mirror

The frontend should also expose a typed mirror for logic that needs token names.

```ts
export const statusTokenByJobStatus = {
  queued: "queued",
  running: "running",
  succeeded: "succeeded",
  failed: "failed",
  cancelled: "cancelled",
} as const;

export type StatusTokenName = keyof typeof statusTokenByJobStatus;
```

Do not duplicate raw CSS values in TypeScript. TypeScript should name tokens and states; CSS should own visual values.

## Chart Tokens

```css
:root {
  --pl-chart-loss: var(--pl-amber-500);
  --pl-chart-map50: var(--pl-green-500);
  --pl-chart-precision: var(--pl-blue-500);
  --pl-chart-recall: var(--pl-cyan-500);
  --pl-chart-baseline: var(--pl-gray-500);
  --pl-chart-experiment: var(--pl-violet-500);
}
```

Rules:

- same metric always gets the same token
- baseline and challenger models must be visually distinct
- avoid rainbow palettes for class metrics in the first dashboard

## Accessibility Rules

- text and icons must not rely on color alone
- status badges include text labels
- camera controls must be keyboard accessible
- destructive actions require explicit labels
- live camera must expose a clear stop button
- confidence and latency must be text-readable, not only visually encoded

## Forbidden Patterns

Forbidden:

```text
inline hex colors in React components
inline spacing magic numbers
component-local status colors
hard-coded detection label styles
unbounded animation loops
canvas overlay without scaling tests
raw CSS files per feature with duplicated variables
```

Allowed exceptions:

```text
canvas drawing can receive resolved CSS token values at runtime
third-party chart libraries may need token values passed as props
```

## Current Token Implementation

The current dashboard implementation keeps Tailwind v4 theme tokens in:

```text
web/src/index.css
web/src/dashboard/components/tone.js
```

A future extracted frontend workspace can split these into CSS and TypeScript token modules, but current work should extend the `web/` token surface instead of creating a second app root.

## Definition Of Done

A UI component is accepted only if:

- all colors come from tokens
- spacing uses tokens or agreed layout primitives
- status visuals map to semantic status tokens
- detection overlays use detection tokens
- both loading and empty states are styled with the same system
- screenshots are stable enough for README/demo use
