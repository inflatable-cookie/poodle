<!-- parity consv=gap gpui=5 jetstream=7 specimen=gap -->
# Parity: Popover

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/popover.md`
- Svelte (authoritative): `packages/svelte/components/src/Popover.svelte`
- GPUI: `packages/gpui/components/src/primitives/popover.rs`
- Jetstream: `packages/jetstream/components/src/popover.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/PopoverSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/popover.rs` · jetstream `packages/jetstream/preview/src/specimens/popover.rs`

## Contract ↔ Svelte

Svelte exposes props the contract omits, and the contract's surface token table (border/background/shadow) disagrees with what Svelte ships. Svelte is authoritative — update the contract.

- Svelte adds `disabled?: boolean` (default `false`, lines 39, 91–93, 143–145) → `data-disabled`, `aria-disabled`, `tabindex=-1`, blocks `setOpen`, `cursor: not-allowed`. Not in contract §3 props or §6 ARIA. **Fix: add `disabled` prop + disabled trigger semantics to contract.**
- Svelte adds `surfaceWidth?: "content" | "trigger"` (default `"content"`, lines 38, 165, 234–238) → `data-surface-width`; `"trigger"` makes surface `width:100%`/`min-width:100%`. Contract §7 only mentions "anchored width may optionally match the trigger" prose. **Fix: promote to a documented `surfaceWidth` prop.**
- Svelte adds `surfaceMinWidth?: string | null` and `surfaceMaxWidth?: string | null` (lines 40–41, 170–172, 218–219) → override `--poodle-popover-surface-min-width`/`-max-width`. Not in contract. **Fix: add both props.**
- Contract §8 surface **border** says `0.0625rem solid color-mix(border-default 72%, transparent)`. Svelte uses `border-subtle` at **74%** (line 222–224), falling back from a `--poodle-treatment-surface-elevated-border` token. Contract names wrong token + wrong %. **Fix: contract → border-subtle 74% (treatment-elevated override).**
- Contract §8 **background** says `color-mix(elevated 98%, panel)`. Svelte uses plain `var(--poodle-color-background-elevated)` (line 226). Contract names a mix Svelte does not do. **Fix: contract → plain background-elevated.**
- Contract §8 **box-shadow** says `var(--poodle-elevation-overlay)`. Svelte hardcodes a 3-layer shadow (inset highlight + two drops, lines 228–231). **Fix: reconcile — either Svelte should consume `--poodle-elevation-overlay` or contract should document the literal 3-layer stack.**
- Contract §8 **radius** says `var(--poodle-radius-surface)`. Svelte uses `--poodle-treatment-surface-elevated-radius` falling back to `radius-surface` (line 225). Minor — note the treatment override.
- `block` prop: present in both (contract §3, Svelte line 38) — OK.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded shadow color/blur literals: `hsla(0.0,0.0,0.0,0.10)` + `px(16.0)` and `hsla(0.0,0.0,0.0,0.06)` + `px(4.0)` at `popover.rs:181–191`. Resolve the elevation stack from an elevation token, not raw HSLA/px. (Also diverges from Svelte's inset-highlight + two-drop stack.)
- [ ] Wrong border token: uses `color.border.subtle` × 0.74 (`popover.rs:132,142–145`) which matches *Svelte* but the surface bg uses plain `elevated` while Svelte's contract reconciliation is still pending — verify against final contract once §8 border/background fixed. Currently consistent with Svelte, flag to re-check after contract update.
- [ ] No `surfaceWidth`/`surfaceMinWidth`/`surfaceMaxWidth` support — builder (`popover.rs:64–98`) has no width props; `min_w`/`max_w` are fixed from `size.menu.minWidth`/`size.popover.maxWidth` tokens (`popover.rs:135–136,196–197`). Trigger-width anchoring impossible.
- [ ] No `disabled` support — no builder method; trigger always clickable (`popover.rs:151–165`). Svelte `disabled` blocks open + sets not-allowed cursor.
- [ ] `initialFocus` not honored — spec stores it but `into_element` never reads `initial_focus`; surface is `.focusable()` only (`popover.rs:175`), no first-focusable vs content branch. (Documented partial delta in header, but `first-focusable` vs `content` distinction is lost.)
- accepted: no ARIA — `role="dialog"`, `aria-expanded`, `aria-controls` not expressible on GPUI native elements (documented in file header lines 19–24).
- accepted: `dismiss_on_outside_interact` — no window-level outside-click interceptor; Escape-to-close wired instead (header note). Spec field stored, unused.

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded panel padding: `rem_to_px(0.75)` / `rem_to_px(0.5)` at `popover.rs:19–20`. Contract §8 padding = `var(--poodle-space-panel-y) var(--poodle-space-panel-x)`; resolve from `space.panel.x`/`space.panel.y` tokens (as GPUI does at `popover.rs:133–134`), not raw rem literals.
- [ ] Wrong border token: uses `color.border.default` (`popover.rs:15`) at full opacity; Svelte uses `border-subtle` at 74%. Resolve subtle + apply 0.74 alpha.
- [ ] No `min-width`/`max-width` constraint — `js_popover` (`popover.rs:22–28`) sets no `min_w`/`max_w`; contract §7 requires min 14rem / max min(24rem,90vw). Surface sizes only to content.
- [ ] Generic `shadow_md()` (`popover.rs:28`) instead of the contract elevation-overlay stack — no token resolution, no match to Svelte's 3-layer shadow.
- [ ] No `placement` handling — `js_popover` ignores `spec.placement`; positioning is whatever `overlay()` does. Contract §8 placement offset rules (top/bottom/left/right + end) unimplemented. Trigger anchoring + `--poodle-popover-offset` absent.
- [ ] No trigger rendering or open/close state — `js_popover` renders the surface only; there is no trigger, no `aria`/role, no `disabled`, no `surfaceWidth`. Open/close + dismiss/Escape must live in preview `main.rs` event loop (verify it exists there; not in component).
- [ ] No `disabled`/`onOpenChange`/`initialFocus` surface at all.
- accepted: no ARIA channel (`role="dialog"`, `aria-expanded`, `aria-controls`) — Jetstream has no accessibility API.
- accepted: interaction (toggle, outside-dismiss, Escape) lives in the preview event loop, not the component.

## Specimen parity

- Svelte covers: Default (bottom-start) with Button trigger + heading/paragraph content, Top placement with Button trigger + paragraph — both interactive (open/dismiss/Escape) (`PopoverSpecimen.svelte`). Matches contract §13 exactly.
- GPUI covers: Default (bottom-start) + Top placement, real Button triggers, `on_open_change` wired through `overlay_state`, open/close interactive (`gpui/.../popover.rs`). — matches Svelte; closest parity of the three.
- Jetstream covers: "With text content", "With rich content" (divider + options), "Empty (no content)" — **surface-only, no trigger, no placement, not interactive** (`jetstream/.../popover.rs`). — missing: **Button trigger**, **bottom-start vs top placement** groups, **open/dismiss interaction**. Demonstrates content shells rather than the contract's anchored-overlay behavior. Largest specimen gap.

## Notes

- `consv=gap` drivers: undocumented `disabled`/`surfaceWidth`/`surfaceMinWidth`/`surfaceMaxWidth` props, plus contract §8 surface tokens (border token+%, background mix, shadow) that disagree with shipped Svelte. All belong in the contract per "Svelte is parity authority".
- GPUI is the only target with a working trigger→surface→dismiss flow. Jetstream `js_popover` is a passive surface shell; the anchored-overlay contract (placement, offset, outside-dismiss) is effectively unimplemented there.
- Both Rust shadow treatments (GPUI hardcoded HSLA stack, Jetstream `shadow_md()`) bypass token resolution — once an `elevation-overlay`/`elevation-popover` token exists, both should consume it.
