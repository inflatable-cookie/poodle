<!-- parity consv=fixed gpui=3 jetstream=2 specimen=gap -->
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

- [ ] **Provider does not propagate — it is a no-op wrapper.**
  `UiPresentationProvider::into_element` (`ui_presentation_provider.rs:36-38`)
  discards `self.spec` and returns `self.child.unwrap_or_else(div)`. The
  `density`/`size_scale` baseline never reaches descendants; there is no GPUI
  context/global push (`cx.set_global` etc.). In the specimen, the child
  `Button`/`TextInput` specs are built with `::new()` and resolve their own
  default size independently — so wrapping changes nothing. **Implement context
  propagation (push provider spec onto a GPUI context/global that descendant
  specs read), or document this as an accepted runtime limit.** This is the
  Tier-1 strict-parity requirement ("context values propagate to descendants").
- [ ] **Nesting override unimplemented** (follows from the above): inner provider
  cannot override outer because neither is consulted at descendant render time.
- [ ] **Seed/default reconciliation:** descendant components consume
  `resolve_semantic_size(spec.size, spec.size_role)` directly from each
  component's own spec (e.g. `markdown_editor.rs:79`, `command_palette.rs:136`),
  bypassing any provider scope. Decide whether provider-driven `size_scale`
  should feed `spec.size` (the propagation channel) and wire it.
- accepted: no ARIA (provider is accessibility-neutral by contract §6; N/A).
- note: resolver parity is exact — `presentation.rs` `resolve_semantic_size`,
  `control_height_rem`, `resolve_supporting_visual_size`, and the density
  helpers all match Svelte (verified by in-file unit tests, lines 195-368).
  `UiPresentationProviderSpec::resolve_size` (`ui_presentation_provider.rs:33-47`)
  also matches. The gap is purely propagation, not value math.

## Jetstream gap (vs Svelte + contract)

- [ ] **No provider component exists.** `packages/jetstream/components/src/`
  has `presentation.rs` (resolver helpers only) but no `UiPresentationProvider`,
  no `js_ui_presentation_provider`, and no provider-shaped spec consumer. The
  provider concept (a scope that sets density/size_scale baselines for a subtree)
  is entirely absent. **Add a `js_ui_presentation_provider` (or equivalent
  context push) so descendants can inherit `density`/`size_scale`** — or
  document the provider as intentionally out-of-scope for Jetstream and rely on
  per-component `size`/`size_role`.
- [ ] **No propagation/nesting channel.** Jetstream components import
  `presentation::{rem_to_px, size_font_rem, ...}` per-component (e.g. every
  preview specimen, `form_dialog.rs:8` etc.) and pass size explicitly; there is
  no inherited-scope mechanism and therefore no inner-overrides-outer behavior.
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
