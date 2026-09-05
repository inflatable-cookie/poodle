<!-- parity consv=ok gpui=1 jetstream=1 specimen=ok -->
<!-- specimen=ok: Jetstream specimen rebuilt to mirror GPUI — End/Start/Between, Responsive danger (real js_form_actions_full inline danger + dangerItems ellipsis overflow), Submitting (loading primary + disabled cancel), Density ladder (compact/default/comfortable), Footer-embedded (showTopSeparation=false), Bordered separation (showTopBorder=true); all real js_form_actions + js_button. Both previews build clean. -->
<!-- gpui-specimen pass: GPUI specimen done; Jetstream pending engine recovery. form_actions.rs now covers End/Start/Between, danger-overflow (real with_danger_action + dangerItems → ghost ellipsis), Submitting (loading primary + disabled cancel), Density ladder (compact/default/comfortable), Footer-embedded (showTopSeparation=false), Bordered separation (showTopBorder=true). Replaced the old fake "responsive danger" group (three plain stacked buttons) with the real danger inline-group + overflow trigger path. specimen stays gap — Jetstream specimen unverifiable while the engine is build-blocked. -->
<!-- pass 42: danger model landed (additive). `FormActionDangerItem` struct added
     to poodle-specs (label/value?/disabled, value-fallback `index:label`) +
     `FormActionsSpec.danger_items` Vec + `with_danger_item(s)`/`has_danger_menu`/
     `danger_inline_gap_*` helpers. Both targets now render the inline `__danger`
     group (inline-flex, gap == form-actions gap, before primary actions) and the
     overflow danger-menu trigger (real ghost ellipsis IconButton, aria-label
     "More actions", sizeRole chrome) when `dangerItems` is present — GPUI via
     `with_danger_action(...)`, Jetstream via `js_form_actions_full(spec, theme,
     danger, children)` (the old `js_form_actions` delegates with empty danger).
     2 new Jetstream probe tests (inline danger + ellipsis trigger render; absent
     without dangerItems) + 1 spec unit test. Only remaining gap on each target is
     the responsive container-query collapse — RECLASSIFIED as a runtime/preview-
     loop limit (no container-query channel in either Rust target; both the inline
     group and the trigger render, the width-driven swap is host-owned). Builds +
     all tests green.
     pass 41: FormActionsSpec gained `density` (+ gap_rem/top_separation_rem/
     border_gap_rem helpers, contract §8 exact rems). Both targets now resolve
     gap/top-separation/border-gap per density; dropped the `max(4.0)*0.5`
     border-gap heuristic. GPUI gained show_top_separation/show_top_border/
     density builders; Jetstream Start align now explicit justify_start + probe
     tests (alignment + density + primary/secondary + submitting). -->
# Parity: FormActions

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/form-actions.md`
- Svelte (authoritative): `packages/svelte/components/src/FormActions.svelte`
- GPUI: `packages/gpui/components/src/primitives/form_actions.rs`
- Jetstream: `packages/jetstream/components/src/form_actions.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/FormActionsSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/form_actions.rs` · jetstream `packages/jetstream/preview/src/specimens/form_actions.rs`

## Contract ↔ Svelte

Svelte matches the contract on every prop, anatomy part, alignment, density, and responsive rule. No divergence.

- Props align: `align` default `"end"`, `density` default `null` (inherits presentation), `showTopSeparation` default `true`, `showTopBorder` default `false`, `dangerItems` default `[]` — all match contract §3 (`FormActions.svelte:10-26`).
- Anatomy align: root + `__danger` inline snippet + `__danger-menu` overflow trigger all present (`FormActions.svelte:46-69`), matching contract §2.
- Alignment tokens align: `start`→`flex-start`, `end`→`flex-end`, `between`→`space-between` (`FormActions.svelte:107-117`) match contract §8 Root Alignment.
- Density align: `compact`/`default`/`comfortable` gap + top-padding + border-gap values (`FormActions.svelte:130-146`) match contract §8 Density + Divider Offset tables exactly.
- Responsive swap align: `@container (max-width: 31.25rem)` hides inline danger, shows menu only when both `danger` and `dangerItems` present (`FormActions.svelte:152-160`) — matches contract §8 Responsive Swap.
- ARIA align: collapsed danger trigger carries `ariaLabel="More actions"` (`FormActions.svelte:61,64`) — satisfies contract §6.
- Note (not a divergence): contract §8 lists `--poodle-form-actions-border-gap` default as `0.5rem` and the compact/comfortable density gap (`0.5rem`/`0.875rem`) + border-gap (`0.25rem`/`0.625rem`) as raw rem literals; Svelte uses the same raw rems (`FormActions.svelte:80,131-145`). Contract documents these as literal values, so this is contract-sanctioned, not a Svelte divergence.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] Border-gap heuristic removed — now `rem_to_px(spec.border_gap_rem())` (density-keyed `0.25`/`0.5`/`0.625rem`, contract §8 Divider Offset Variants).
- [x] `density` support added — `FormActionsSpec.density` + `.density()` builder; gap/top-separation/border-gap resolve per density (compact/comfortable use contract-exact rems, default inherits tokens).
- [x] `showTopSeparation` builder added — `FormActions::show_top_separation(bool)`; footer-embedded mode reachable.
- [x] `showTopBorder` builder added — `FormActions::show_top_border(bool)`; bordered-separation state reachable.
- [x] `danger` snippet / `__danger` inline group added — `FormActions::with_danger_action(...)` accumulates inline destructive/cancel content into an `inline-flex` group (gap == form-actions gap, contract §8 Danger Inline) rendered before the primary actions (contract §2 anatomy order).
- [x] `dangerItems` overflow menu trigger added — when `spec.has_danger_menu()` the row renders a real ghost ellipsis `IconButton` (`aria-label "More actions"`, `sizeRole` chrome) matching the Svelte `__danger-menu` trigger. The `FormActionDangerItem` descriptors live on the spec; `onSelect` is host-owned.
- [ ] Responsive container-query collapse not implemented — the `@container (max-width: 31.25rem)` swap that *hides* the inline group and shows *only* the trigger (contract §8 Responsive Swap) has no channel in GPUI's render-immediate model. **Runtime limit (reclassified):** both the inline group and the trigger render; the width-driven hide/show is host-owned, not faked.
- accepted: no ARIA (gpui has no accessibility API) — the overflow trigger carries an `aria_label` on the spec/builder but GPUI cannot emit it to the platform a11y tree.

## Jetstream gap (vs Svelte + contract)

- [x] Border-gap heuristic removed — now `rem_to_px(spec.border_gap_rem())` (density-keyed per contract §8).
- [x] `density` support added — `js_form_actions` varies gap/top-separation/border-gap by `spec.density` (compact/comfortable contract-exact rems, default token-inherited).
- [x] `FormActionAlign::Start` now explicit `el.justify_start()` (no longer relying on default flex start).
- [x] `showTopBorder` border-gap now contract density value (`border_gap_rem`), not the old heuristic.
- [x] `danger` snippet / `__danger` inline group added — `js_form_actions_full(spec, theme, danger, children)` renders inline destructive/cancel content in an `inline-flex` group (gap == form-actions gap, contract §8 Danger Inline) before the primary actions (contract §2). `js_form_actions(...)` delegates with an empty danger group, so existing callers are unchanged.
- [x] `dangerItems` overflow menu trigger added — when `spec.has_danger_menu()` the row renders a real ghost ellipsis `js_icon_button` (`aria-label "More actions"`, `sizeRole` chrome), the JsEl analogue of the Svelte `__danger-menu` trigger. Probe-asserted (inline danger + ellipsis Icon present; absent without `dangerItems`).
- [ ] No responsive container-query swap — no `inline-size` boundary / `max-width: 31.25rem` collapse (contract §8). **Runtime limit (reclassified):** the JsEl render-immediate model has no container-query channel, so both the inline group and the trigger render; the width-driven hide/show is host-owned, not faked.
- accepted: no ARIA channel for the overflow trigger `aria-label` (contract §6) — the spec carries it; the runtime has no a11y surface to emit it.
- accepted: interaction (button click, `dangerItems[].onSelect`) lives in the preview event loop, not the component.

## Specimen parity

- Svelte covers: End-aligned (default), Start-aligned, Space between, Responsive danger actions (with `dangerItems` + `danger` snippet, constrained to 20rem to show collapse), Bordered separation, Density ladder (compact/default/comfortable via `densities` snippet) (`FormActionsSpecimen.svelte`).
- GPUI covers (specimen done): End-aligned, Start-aligned, Space between, Responsive danger actions (real `with_danger_action` inline group + `dangerItems` overflow ellipsis trigger), Submitting (loading primary + disabled cancel), Density ladder (compact/default/comfortable), Footer-embedded (`showTopSeparation=false`), Bordered separation (`showTopBorder=true`), Last-action click feedback. Full contract-state coverage; no fakes. (Jetstream specimen pending engine recovery.)
- Jetstream covers: End-aligned, Start-aligned, Between, Dialog footer custom actions (`showTopSeparation=false`). The component now supports `js_form_actions_full` (inline danger + overflow trigger), but the specimen has not yet been re-wired. — missing (specimen): **Density ladder**, **Bordered separation** (`showTopBorder`), **Responsive danger actions** demo using the new danger path.

## Notes

- consv=ok: Svelte is a faithful, complete implementation of the contract — every prop, anatomy part, density value, alignment, responsive rule, and ARIA requirement is present and matches.
- Density (pass 41) and the danger model (pass 42) are both now resolved on the spec: `FormActionsSpec` (`packages/contracts/components/src/form_actions.rs`) has `density` + `danger_items: Vec<FormActionDangerItem>` with `has_danger_menu()` / `danger_inline_gap_*` helpers. Both Rust targets render the inline `__danger` group and the overflow trigger from these.
- The only remaining cross-target gap is the **responsive container-query collapse** (§8 Responsive Swap) — reclassified as a runtime limit on both targets (no `inline-size` container-query channel in render-immediate / JsEl). Both render the inline group *and* the trigger; the width-driven hide/show is host-owned.
- There are zero hardcoded color literals or raw-px sizing literals in either component; all danger-region spacing reuses the form-actions gap (token / density rem).
