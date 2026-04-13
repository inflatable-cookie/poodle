# g10.010 GPUI Behavior, Adapter Realization, And Form Remediation Parity

Status: complete
Owner: Poodle core
Depends on: g10.009
Updated: 2026-04-13

## Context

Two different “GPUI implementation” surfaces exist:

1. **`poodle_gpui_components`** — full interactive widgets used by the GPUI
   preview.
2. **`packages/gpui/adapter` `RenderComponent`** — mostly returns
   `GpuiElementHandle` with partial style sampling; **not** mounting the full
   component trees.

**Pagination** (`pagination.rs`): full variant **“Go to page”** and **limit
selector** were non-interactive stubs vs Svelte behavior.

**Form / validation family**: contracts and specs exist (`form-shell`,
`validation-summary`, `remediation-banner`, `inline-remediation`), but
`render_form_composites.rs` stubs **FormShell**, **ValidationSummary**,
**RemediationBanner**, **InlineRemediation**, and historically used a generic
handle for **ConfirmAction** — while `composites/confirm_action.rs` is a real
widget. Adapter and preview were **out of sync** for confirm metadata.

**Cross-cutting**: `ui-presentation-provider.md` and `surface-elevation.md`
describe behavior Svelte implements with CSS variables and context; GPUI uses
Rust presentation helpers and the theme adapter. Parity is **behavioral** rather
than a named provider component file.

## Governing Refs

- `packages/gpui/adapter/src/render_form_composites.rs`
- `docs/contracts/components/pagination.md`
- `docs/contracts/components/form-shell.md`
- `docs/contracts/components/validation-summary.md`
- `docs/contracts/components/inline-remediation.md`
- `docs/contracts/components/remediation-banner.md`
- `docs/contracts/components/ui-presentation-provider.md`
- `docs/contracts/components/surface-elevation.md`
- `docs/roadmaps/g10/012-gpui-runtime-truth-and-deferred-work-closure.md` (adapter mount / form family backlog)
- `docs/roadmaps/g07/007-gpui-form-validation-and-remediation-composites.md`
  (historical context)

## Goals

- **Pagination**: implement real interaction for full variant (page jump input,
  limit selector via `Select` + parent callbacks) aligned with `PaginationSpec`
  and preview specimens.
- **Adapter**: wire `RenderComponent` for composites blocked by crate cycles
  using **spec-derived handles** where full trees cannot mount (at minimum
  **ConfirmAction** fingerprint).
- **Form stack**: implement GPUI **FormShell**, **ValidationSummary**,
  **InlineRemediation**, **RemediationBanner** as real components **or**
  document deferral — **deferred** (D-109); no fake widgets added.
- **Presentation / elevation**: record how GPUI satisfies presentation and
  elevation **without** a 1:1 Svelte provider component (see Outcome).

## Non-Goals

- replacing the entire adapter architecture in one milestone
- Underlay-specific integration (stays behind adapters)
- adding an adapter → components dependency (forbidden by crate cycle)

## Execution Plan

### Batch 10.1 — Pagination

- [x] Replace stubs with `TextInput` + `Select` when parent supplies callbacks;
      keep static affordances when unwired (demo compatibility).
- [x] Align with `PaginationSpec` / preview specimen using `SpecimenState`.

### Batch 10.2 — Adapter realization (incremental)

- [x] Document cycle constraint and encode **ConfirmAction** handle from spec
      (`open`, `title`).
- [ ] Map every `GpuiElementHandle::new` call site to “real tree”, “style
      probe”, or “deferred” — partial; only confirm lane closed here.

### Batch 10.3 — Documentation

- [x] Delta register: D-108 (adapter vs components), D-109 (form stubs); D-107
      note pagination interaction follow-up.
- [x] UiPresentation / surface elevation: equivalence called out in Outcome
      below (no new architecture file).

## Validation

- `cargo test -p poodle-gpui --manifest-path packages/gpui/adapter/Cargo.toml render_form`
- `cargo check --manifest-path packages/gpui/preview/Cargo.toml`
- `git diff --check`

## Outcome

- **Pagination** (`packages/gpui/components/.../pagination.rs`): Full variant
  uses `TextInput` for “go to” when `on_goto_input_change` and `on_page_change`
  are set; limit selector uses `Select` when `on_limit_open_change` and
  `on_page_size_change` are set. Limit row is ordered before nav controls to
  match contract anatomy. `on_page_change` / `on_page_size_change` use
  `&usize` for `cx.listener` compatibility.
- **Preview** (`pagination` specimen): interactive block drives
  `pagination-full-*` keys in `SpecimenState`.
- **Adapter**: `ConfirmActionSpec` → `GpuiElementHandle` with
  `element_id = confirm-action|open=…|title=…` (unit test asserts prefix and
  title segment).
- **Deferred**: FormShell / validation / remediation adapter paths remain
  generic handles until dedicated GPUI composites exist or a host crate breaks
  the dependency cycle (D-109).
- **UiPresentationProvider / SurfaceElevation**: Svelte resolves inherited
  size/density/elevation through CSS custom properties and provider context.
  GPUI resolves the same semantics in `poodle-gpui-components` via
  `crate::presentation` helpers (`resolve_semantic_size`, rem tables) plus
  `GpuiThemeProvider` token resolution — no separate provider widget is required
  for equivalent **inputs** to components; elevation shadows use GPUI
  `BoxShadow` presets where CSS `box-shadow` differs (existing D-004).
