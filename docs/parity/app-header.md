<!-- parity consv=fixed gpui=2 jetstream=2 specimen=gap -->
<!-- pass 40: app-header rebuilt on BOTH targets to the contract 3-region anatomy. GPUI: flat
     flex-justify-between → 3-col grid (identity 1fr / actions auto / utility auto), title→title-
     size ladder + subtitle→subtitle-size (were body), panel@94% bg + border.subtle, composed
     actions/utility. Jetstream: was title-only → js_app_header_with_slots (identity + actions
     [js_button] + utility [js_icon_button]), size/density ladders, 3 probe tests (title+subtitle+
     action, not title-only). Additive AppHeaderSpec: size/size_role/density + rem ladders +
     token helpers (existing count fields kept for workstation, non-breaking). specs 61, jet 140,
     gpui clean. Remaining: action clicks = preview-loop; size/density rems no dedicated token. -->
# Parity: AppHeader

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/app-header.md`
- Svelte (authoritative): `packages/svelte/components/src/AppHeader.svelte`
- GPUI: `packages/gpui/components/src/composites/app_header.rs`
- Jetstream: `packages/jetstream/components/src/app_header.rs`
- Spec: `packages/contracts/components/src/app_header.rs` (`AppHeaderSpec`)
- Specimens: svelte `packages/svelte/preview/src/specimens/AppHeaderSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/app_header.rs` · jetstream `packages/jetstream/preview/src/specimens/app_header.rs`

## Contract ↔ Svelte

Props in contract §3 (`title`, `subtitle`, `dragRegion`, `ariaLabel`, `size`, `sizeRole`, `density`) and snippets §4 (`identity`, `actions`, `utility`) all match Svelte exactly. The §8/§9 token tables, three-column grid, size ladder, and density overrides also match the authoritative Svelte CSS verbatim. **No contract↔Svelte divergence — the contract is already in sync.** `consv=fixed`.

The remaining gap is the shared `AppHeaderSpec` (Rust), a code-side change outside the contract-reconciliation scope. Recorded as a Rust-side todo (Notes):

- (Rust spec todo, not a contract edit) `AppHeaderSpec` lacks `size`/`sizeRole`/`density` (`app_header.rs:4-13`); add them so Rust targets honor the ladder.
- (Rust spec todo, not a contract edit) `AppHeaderSpec` invents `primary_action_count`/`utility_item_count`/`is_utility_heavy()` (`app_header.rs:11-12,53-65`) with no contract/Svelte counterpart; drop or document as a Rust-only slot-presence hint.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded inline padding literal `px(rem_to_px(1.0))` at `app_header.rs:103` — resolve from `space.panel.x` token, not a raw `1.0` rem; also drops density inset (compact `-0.125rem`, comfortable `+0.125rem`, Svelte lines 159,166).
- [ ] Title uses `typography.body.size` at `app_header.rs:106,141` — contract §9 title is `--poodle-app-header-title-size` (0.9375rem md), distinct from body size; add a title-size token and resolve it.
- [ ] Subtitle reuses `body_size` at `app_header.rs:152-156` — contract subtitle is `--poodle-app-header-subtitle-size` (0.75rem md); resolve a separate subtitle-size token.
- [ ] Layout is `flex justify_between` (`app_header.rs:113-117`), not the contract `grid minmax(0,1fr) auto auto` three-column shell (§8) — identity/actions/utility collapse into left+right; rebuild as three regions.
- [ ] No `size`/`density` handling — header height fixed to `size.panel.header` (`app_header.rs:106`); size ladder (xs..xl min-height/title/subtitle, Svelte 125-153) and density spacing (155-167) unimplemented.
- [ ] Background uses `spec.background_token()` flat fill (`app_header.rs:108,119`) — contract §9 is `color-mix(... background-panel 94%, transparent)`; apply the 94% panel mix.
- [ ] Border color resolves `color.border.default` at `app_header.rs:110` — contract §8 border is `border-subtle`; use `color.border.subtle`.
- [ ] No subtitle/title baseline grouping or `min-width:0` truncation+ellipsis on subtitle (contract §9 subtitle block); subtitle just sits inline.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored on spec but not emitted as a header landmark label.
- accepted: drag-region maps to `WindowControlArea::Drag` (`app_header.rs:126-128`) — correct native equivalent of `data-drag-region`.

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded header height literal `rem_to_px(3.0)` at `app_header.rs:16` — contract min-height is `size.panel.header` / size-ladder driven; resolve from token, not `3.0`.
- [ ] Hardcoded title/subtitle gap `rem_to_px(0.5)` at `app_header.rs:24` — contract region gap is `--poodle-app-header-region-gap` (`space.inline.sm`); resolve from token.
- [ ] Subtitle font is ad-hoc `size_font_rem(Md) * 0.875` at `app_header.rs:22` — contract subtitle-size is its own token (0.75rem md); resolve it, drop the `* 0.875` heuristic.
- [ ] Title font pinned to `size_font_rem(Md)` at `app_header.rs:20` — should be the title-size token and follow the `size` ladder, not hardcoded Md.
- [ ] Padding pinned to `panel_space_x_rem(Default)` at `app_header.rs:18` — ignores `spec.density`; honor compact/comfortable inset.
- [ ] No actions or utility regions rendered — `js_app_header` only emits the title row (`app_header.rs:26-58`); contract §2 actions + utility regions and the three-column grid are absent.
- [ ] No `identity` slot path — when `title` is None nothing renders; Svelte falls back to identity snippet (Svelte 48-57).
- [ ] No `size`/`density`/`drag_region` posture applied (spec lacks the fields; see Contract↔Svelte).
- accepted: no ARIA channel for `aria_label` (no accessibility API).
- accepted: interaction for child actions lives in the preview event loop, not the component.

## Specimen parity

- Svelte covers: Full app window header (title + inline Menubar + 3 utility IconButtons), Title+actions+utility, Title only, Custom identity slot, Density ladder (compact/default/comfortable), Size ladder (xs..xl). (`AppHeaderSpecimen.svelte`)
- GPUI covers: Full app window header (title + ghost Buttons as menubar + 3 utility IconButtons), Title+actions+utility, Title only, Custom identity (via `with_leading`). — missing: **Density ladder**, **Size ladder** groups; menubar is faked with ghost Buttons rather than a real Menubar (acceptable until JS Menubar parity).
- Jetstream covers: With title, Without title only (`app_header.rs:12-18`). — missing: **actions**, **utility**, **custom identity**, **density ladder**, **size ladder**, **full app-window** groups. Largest specimen gap of the three.

## Notes

- `consv=gap` is driven by `AppHeaderSpec` drift (missing `size`/`size_role`/`density`; spurious `*_count` fields), not by any Svelte-vs-contract prop mismatch. Fix the spec, leave the contract.
- GPUI `with_leading` is a Rust-only slot name for the contract `identity()` snippet; rename or document to avoid confusion (`identity` is the contract term).
- Specimen GPUI/Jetstream use literal `px(...)`/`gap(...)` values inside specimen layout scaffolding (e.g. `app_header.rs:14,28` gpui); those are preview-harness chrome, not component tokens — out of scope for component token audit but worth a pass.
