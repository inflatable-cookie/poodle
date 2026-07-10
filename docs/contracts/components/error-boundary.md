# ErrorBoundary

Status: active
Updated: 2026-07-10

## 1. Purpose

- Component name: `ErrorBoundary`
- Layer: `composites`
- Summary: a Svelte error boundary that catches rendering errors in child
  content and replaces the failed subtree with an `EmptyState` showing the
  error message and a retry button
- In scope: error catching via `svelte:boundary`, error display with title
  and message, retry/reset action
- Out of scope: error reporting/logging, custom recovery flows, nested
  boundary coordination

## Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `children` | `Snippet` | -- | yes | Content to render inside the boundary |
| `title` | `string` | `"Something went wrong"` | no | Heading shown in the error state |
| `retryLabel` | `string` | `"Try again"` | no | Label for the retry button |

## States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| normal | no error | Children rendered normally |
| error | child rendering throws | `EmptyState` with error title, message, and retry button |

## Composition

- Composes: `EmptyState` from composites, `Button` from `@poodle/svelte`
- Uses `svelte:boundary` with `onerror` handler to catch child errors
- Retry button resets the error state and re-renders children

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 2. Accessibility

- error state uses `EmptyState` accessibility semantics
- retry button is keyboard-focusable with standard button semantics
- error message text is rendered as visible content inside the empty state
  so screen readers announce the failure context naturally
