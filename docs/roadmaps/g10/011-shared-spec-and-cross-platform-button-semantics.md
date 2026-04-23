# g10.011 Shared Spec And Cross-Platform Button Semantics

Status: complete
Owner: Poodle core
Depends on: g10.010
Updated: 2026-04-13

## Context

`docs/contracts/components/button.md` includes **web-specific** props (`type`,
`form`, `formaction`, …) and **`ariaExpanded`** for disclosure triggers.

**Svelte** implements those on the native `<button>`.

**Rust `ButtonSpec`** previously modeled toggle state (`pressed`, `default_pressed`)
but did not carry `aria_expanded` or HTML form attributes.

**GPUI** `button.rs` follows `ButtonSpec`; GPUI still cannot emit ARIA attributes on
elements (D-002).

## Governing Refs

- `docs/contracts/components/button.md`
- `packages/contracts/components/src/button.rs`
- `packages/svelte/components/src/Button.svelte`
- `packages/gpui/components/src/primitives/button.rs`
- `docs/roadmaps/g10/012-gpui-runtime-truth-and-deferred-work-closure.md` (assistive / ARIA limits)

## Goals

- decide per prop class:
  - **HTML-only** — document in contract as web implementation; no Rust field
  - **Cross-platform ARIA** — add to `ButtonSpec` where semantics are portable
- add **`aria_expanded`** to `ButtonSpec` aligned with `ariaExpanded`
- surface the field in adapter fingerprints and preview where helpful
- defer HTML form props on `ButtonSpec` (non-goals unchanged)

## Non-Goals

- full GPUI form submission model (HTML forms)
- changing Svelte behavior without contract alignment (Svelte already had the prop)
- Jetstream / GPUI platform accessibility emission without runtime API support

## Execution Plan

- [x] contract edit: portable vs web-only props table
- [x] `ButtonSpec`: `aria_expanded: Option<bool>` + builders
- [x] GPUI: document D-002 gap; `Button::aria_expanded` mutates spec; preview specimen
- [x] Adapter: encode `aria_expanded` in `ButtonSpec` element id when set
- [x] Jetstream: document spec parity note on `js_button`
- [x] Svelte: unchanged (primitives already implement `ariaExpanded`)

## Validation

- `git diff --check`
- `cargo check` for specs, GPUI components, GPUI preview, Jetstream components,
  GPUI adapter; `cargo test` adapter `render_button_aria_expanded_in_element_id`

## Outcome

- **Contract** (`button.md`): table maps `ariaExpanded` ↔ `aria_expanded`; HTML
  form props explicitly web-only and absent from `ButtonSpec`.
- **Spec**: `aria_expanded`, `with_aria_expanded`, `without_aria_expanded`.
- **GPUI**: module docs + `Button::aria_expanded`; preview specimen toggles
  `ButtonSpec.aria_expanded` (native tree still blocked on D-002).
- **Adapter**: `button-…|aria_expanded=true|false` when the field is `Some`.
- **Jetstream**: crate-level note on spec vs `JsEl` accessibility mapping.
