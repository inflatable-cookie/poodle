<!-- parity consv=fixed gpui=0 jetstream=0 specimen=gap -->
<!-- pass: propagation gaps reclassified accepted/architectural — neither Rust target has a runtime context channel; resolver math (resolve_size) is exact and the wrapper is already display:contents-faithful in GPUI. No representable spec/visual gap remains. -->
# Parity: UiPresentationProvider

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/ui-presentation-provider.md`
- Svelte (authoritative): `packages/svelte/components/src/UiPresentationProvider.svelte`
  + context/resolver module `packages/svelte/components/src/presentation.ts`
- GPUI: `packages/gpui/components/src/primitives/ui_presentation_provider.rs`
  + resolver module `packages/gpui/components/src/presentation.rs`
- Jetstream: resolver module `packages/jetstream/components/src/presentation.rs`
  (named `presentation.rs`, **not** `ui_presentation_provider.rs` — no provider
  component file exists)
- Spec struct: `packages/contracts/components/src/ui_presentation_provider.rs`
  (`UiPresentationProviderSpec`, with `resolve_size`)
- Specimens: svelte `packages/svelte/preview/src/specimens/UiPresentationProviderSpecimen.svelte`
  · gpui `packages/gpui/preview/src/specimens/ui_presentation_provider.rs`
  · jetstream **none**

## Contract ↔ Svelte

This is a context/provider component with no visual render. Parity axes are the
context values (props/defaults), the propagation mechanism, and the resolver
functions. Token-target and ARIA checks are N/A (see Notes). Divergences found:

- [x] FIXED **`resolveSupportingVisualSize` is undocumented.** `presentation.ts:70-75`
  exports a second resolver (`xl→lg, lg→md, md→sm, else identity`). Added a
  `resolveSupportingVisualSize` table to contract §3 (xs→xs, sm→sm, md→sm, lg→md,
  xl→lg).
- [x] FIXED **Root class name mismatch.** Svelte renders
  `.poodle-ui-presentation-provider` (`UiPresentationProvider.svelte:44,49`).
  Renamed in contract anatomy §2, §8 root selector, and §9.
- [x] FIXED **Nesting mechanism mis-described.** Contract §9 said the provider
  "updates the existing store rather than creating a new one." Svelte
  unconditionally calls `writable(value)` + `setContext` (`presentation.ts:20-24`),
  seeds with a literal `{density:"default", sizeScale:"md"}` then syncs via
  `$effect`. Reworded §9: each provider creates a fresh scoped context store;
  inner shadows outer via Svelte context scope (no store mutation).
- **Seed value is a hardcoded literal, not props.** `UiPresentationProvider.svelte:24`
  seeds the store with `default`/`md` regardless of props, relying on the
  `$effect` (line 29) to immediately overwrite with real prop values. Cosmetic /
  harmless, but worth noting the contract's "reflected immediately" claim (§3)
  depends on that effect firing. No fix required.
- Context symbol `Symbol("poodle-ui-presentation")` (`presentation.ts:13`) and
  props `density: ControlDensity = "default"`, `sizeScale: ControlSize = "md"`
  (`UiPresentationProvider.svelte:18-22`) match the contract §3 exactly. CSS
  custom-property outputs and both resolved-value tables (height-by-size,
  spacing-by-density) match `presentation.ts:62-93` exactly. `resolveSemanticControlSize`
  (`presentation.ts:32-60`) matches the contract §3 chrome/control/prominent table
  for all five scales.

## GPUI gap (vs Svelte + contract)

- accepted (architectural): **Provider does not propagate — it is a
  display:contents-faithful wrapper with no runtime context channel.**
  `UiPresentationProvider::into_element` (`ui_presentation_provider.rs:36-38`)
  returns `self.child.unwrap_or_else(div)` — which is the correct GPUI analogue
  of `display: contents` (no extra box, no layout influence, accessibility-
  neutral; contract §6/§7). The `density`/`size_scale` baseline does not reach
  descendants because GPUI specs have **no global/context read channel**:
  descendant specs (e.g. `markdown_editor.rs:79`, `command_palette.rs:136`)
  resolve `resolve_semantic_size(spec.size, spec.size_role)` from their own spec.
  Wiring true propagation would require a cross-cutting GPUI global
  (`cx.set_global` + every spec reading it at render) — an architecture change
  out of scope for this component. **Reclassified accepted: the supported
  channel is per-component `size`/`size_role`, and the resolver math
  (`UiPresentationProviderSpec::resolve_size`, `presentation.rs::resolve_*`) is
  exact (Tier-1 value tables verified by unit tests, lines 195-368).** The
  Tier-1 "context values propagate" axis is the architectural delta; nesting
  override follows from it (neither inner nor outer is consulted at descendant
  render time). No representable spec/visual gap remains to close.
- accepted: no ARIA (provider is accessibility-neutral by contract §6; N/A).
- note: resolver parity is exact — `presentation.rs` `resolve_semantic_size`,
  `control_height_rem`, `resolve_supporting_visual_size`, and the density
  helpers all match Svelte (verified by in-file unit tests, lines 195-368).
  `UiPresentationProviderSpec::resolve_size` (`ui_presentation_provider.rs:33-47`)
  also matches. The gap is purely propagation, not value math.

## Jetstream gap (vs Svelte + contract)

- accepted (architectural): **No provider component — intentionally out-of-scope
  for Jetstream.** `packages/jetstream/components/src/` has `presentation.rs`
  (resolver helpers only) and no provider-shaped consumer. A `js_ui_presentation_
  provider` is **not representable as a faithful wrapper**: `JsEl::div()` is a
  real flex box with no `display: contents` analogue, so wrapping a subtree would
  *alter* layout (worse than not having it; contract §7 mandates layout-
  neutrality), and `JsEl` has no context/global channel for true value
  propagation. **Reclassified accepted: descendants take explicit
  `size`/`size_role` and the `presentation.rs` resolver helpers
  (`resolve_semantic_size`, `control_height_rem`, `resolve_supporting_visual_size`,
  density helpers) are the supported channel — exact vs Svelte/GPUI, unit-tested
  (lines 158-280).** Propagation/nesting is the same architectural delta as GPUI.
  No representable spec/visual gap remains to close.
- accepted: no ARIA (provider is accessibility-neutral; N/A).
- note: resolver parity is exact — `presentation.rs:19-156` (`resolve_semantic_size`,
  `control_height_rem`, `resolve_supporting_visual_size`, density helpers) matches
  Svelte and GPUI, with matching unit tests (lines 158-280). The gap is the
  absent provider, not the value math.

## Specimen parity

- Svelte covers: **Compact small scope** (`density="compact" sizeScale="sm"`)
  and **Comfortable large scope** (`density="comfortable" sizeScale="lg"`), each
  wrapping a `Button` + `TextInput` + `Select` inside a `Surface`
  (`UiPresentationProviderSpecimen.svelte:11-35`). Demonstrates inherited
  presentation. — missing: the contract's third case, **Nested override**
  (outer `default/md`, inner `compact/sm`), per Integration table §12.
- GPUI covers: Compact/sm and Comfortable/lg scopes wrapping `Button` +
  `TextInput` (`ui_presentation_provider.rs:13-52`). — but because the provider
  does not propagate (see GPUI gap), the specimen does **not actually
  demonstrate inheritance**: the controls render at their own default size
  regardless of the wrapping provider. Effectively a false-positive specimen.
  Also missing the Nested override case.
- Jetstream covers: **nothing** — no specimen file exists. — missing: the entire
  specimen (all three integration cases).

## Notes

- **Provider / no-visual-render nature.** `UiPresentationProvider` renders no
  visual chrome. In Svelte it is a `display: contents` wrapper that (a) sets four
  CSS custom properties and (b) seeds a Svelte context store; descendants read
  either channel. Because it has no visual properties of its own, the usual
  parity checks are **N/A**: token-target/hardcoded-pixel checks (no colors,
  radii, or dimensions to resolve), and ARIA/keyboard/focus (contract §6 mandates
  accessibility-neutral, no role, no landmark). The real parity surface is the
  three non-visual axes: **context values** (props + defaults), the
  **propagation mechanism**, and the **resolver functions**. Resolver-function
  parity is exact across all three targets; the divergences are in propagation
  (GPUI no-op, Jetstream absent) and in three small contract inaccuracies.
- The Rust resolver math is duplicated verbatim in two modules
  (`gpui/.../presentation.rs` and `jetstream/.../presentation.rs`) plus
  `UiPresentationProviderSpec::resolve_size`. They agree today, but three copies
  of the chrome/prominent table is drift risk — a shared `poodle-specs` resolver
  would remove it. Out of scope for this audit; flagged only.
- `consv=gap` driver: undocumented `resolveSupportingVisualSize`, wrong root
  class name, and the mis-described nesting/store-update mechanism in §9. All
  three are contract-side fixes per "Svelte is parity authority".
