# g07.011 — Cross-Runtime Parity Report and Delta Register

Status: Completed
Updated: 2026-03-14

> Historical note: the parity matrix below reflects the pre-`g10.004`
> split-crate layout. The live component spec surface now ships from
> `poodle-specs`.

## Objective

Document parity between Svelte and GPUI implementations across the full
118-component surface. Identify intentional deltas and verify that both
renderers consume the same shared contract layer.

## Parity Summary

### Coverage Matrix

| Layer | Shared Specs | Svelte Components | GPUI RenderComponent | Parity |
|-------|-------------|-------------------|---------------------|--------|
| Primitives | 64 | 64 | 64 | Full |
| Composites | 41 | 41 | 41 | Full |
| Workstation | 13 | 13 | 13 | Full |
| **Total** | **118** | **118** | **118** | **Full** |

### Shared Infrastructure

Both renderers consume the same contract crates from g06:

Current equivalent after `g10.004`: `poodle-primitives` and
`poodle-composites` were merged into `poodle-specs`.

| Crate | Purpose | Svelte | GPUI |
|-------|---------|--------|------|
| poodle-tokens | Token constants + typed resolution | via JSON artifacts | via Rust typed module |
| poodle-primitives | 64 primitive specs | ✓ | ✓ |
| poodle-composites | 41 composite specs | ✓ | ✓ |
| poodle-workstation | 13 workstation specs | ✓ | ✓ |
| poodle-layout | Layout intent types | N/A (CSS native) | ✓ maps to GpuiStyle |
| poodle-events | Semantic event types | N/A (DOM native) | ✓ (defined, not yet wired) |
| poodle-style | Style descriptor IR | N/A (CSS native) | ✓ maps to GpuiStyle |
| poodle-adapter | Renderer adapter traits | N/A (Svelte is reference) | ✓ implements traits |

### Parity Tiers

**Tier 1 — Strict parity (structural + behavioral):**
All 118 components have matching spec coverage. Both renderers accept the same
spec structs (or equivalent JSON). Token resolution produces visually equivalent
output.

**Tier 2 — Visual parity:**
Svelte components render via CSS with full browser rendering. GPUI adapter
currently produces `GpuiElementHandle` stubs that verify spec-type routing
and style mapping but defer actual GPUI element construction to runtime
integration. Visual parity will be verified during demo-app implementation
(g07.012).

## Intentional Deltas

### GPUI-specific behaviors

| Area | Delta | Reason |
|------|-------|--------|
| Event model | GPUI uses `SemanticEvent` enum; Svelte uses DOM events | Platform difference — both map to same logical events |
| Layout | GPUI uses `LayoutIntent` → flexbox mapping; Svelte uses CSS directly | GPUI lacks CSS engine — layout intent bridges the gap |
| Style resolution | GPUI resolves via `ThemeProvider` trait; Svelte resolves via CSS custom properties | Different runtime models — same token values |
| Typography | GPUI uses `GpuiTypography` intermediate; Svelte uses CSS font properties | GPUI's text layout is element-based, not CSS-based |
| Accessibility | GPUI will use platform accessibility APIs; Svelte uses ARIA attributes | Both target the same a11y semantics |

### Components with known native divergence

None. GPUI supports the full desktop interaction model (mouse, keyboard,
drag-and-drop, hover, focus, scroll). No components are intentionally
unsupported.

## Test Evidence

```
GPUI adapter: 137 tests passing
  - Theme resolution: 8 tests
  - Style mapping: 9 tests
  - Structural primitives: 8 tests
  - Action primitives: 12 tests
  - Selection primitives: 14 tests
  - Overlay primitives: 13 tests
  - Informational primitives: 16 tests
  - Form composites: 5 tests
  - Data composites: 12 tests
  - Editing composites: 24 tests
  - Workstation: 13 tests
  - Core adapter: 3 tests
```

## Remaining Work

- Demo-app parity verification (g07.012) will provide side-by-side visual proof
- Event wiring will be completed during GPUI runtime integration
- Accessibility API mapping will be verified with platform tools
