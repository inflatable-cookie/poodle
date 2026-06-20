<!-- parity consv=gap gpui=4 jetstream=5 specimen=gap -->
# Parity: Toolbar

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/toolbar.md`
- Svelte (authoritative): `packages/svelte/components/src/Toolbar.svelte`
- GPUI: `packages/gpui/components/src/primitives/toolbar.rs`
- Jetstream: `packages/jetstream/components/src/toolbar.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/ToolbarSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/toolbar.rs` · jetstream `packages/jetstream/preview/src/specimens/toolbar.rs`

## Contract ↔ Svelte

Contract §8 token values and §6/§9 ARIA notes drift from the authoritative Svelte source. Svelte is authoritative — update the contract.

- `gap`: contract §8 says `0.25rem`; Svelte base is `0.375rem` (`Toolbar.svelte:86`). **Fix: contract §8 + Tier-2 checklist to `0.375rem`.**
- `padding`: contract §8 says `0.25rem` (uniform); Svelte base is `0.25rem 0.375rem` (`Toolbar.svelte:87`). **Fix: contract §8 to `0.25rem 0.375rem` (block 0.25, inline 0.375).**
- Size variants: Svelte defines per-size padding/gap for xs/sm/lg/xl (`Toolbar.svelte:94-97`) — contract §8 only documents the base. **Fix: add the size table to contract §8 (`size` prop is in §3 but its visual effect is undocumented).**
- Density variants: Svelte overrides only `padding-inline` + `gap` for compact/comfortable (`Toolbar.svelte:100-101`) — density never touches block padding (matches density contract). Contract §8 omits the density table. **Fix: document compact/comfortable inline-padding + gap in contract §8.**
- `aria-orientation`: contract §6 says it is NOT set (only `data-orientation`); contract §9 contradicts and claims it IS set from `orientation`. Svelte sets neither `aria-orientation` nor `aria-label` is unconditional — it sets `data-orientation` only (`Toolbar.svelte:54`), no `aria-orientation` anywhere. **Fix: reconcile §6/§9 — `aria-orientation` is unimplemented in Svelte; pick one statement.**
- `alignment` prop: contract §3 has no `alignment` prop, but all three Rust specs expose `ToolbarSpec.alignment` (`Start|Center|End|Stretch`). Svelte has no alignment prop either. **Fix: either add `alignment` to contract §3 or drop it from `ToolbarSpec` — currently a Rust-only invention.**

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] No roving focus / arrow-key navigation — contract §6 Tier-1 requires ArrowLeft/Right (horizontal), ArrowUp/Down (vertical), wrapping, Tab-as-single-stop. GPUI `Toolbar` is render-only; no focus management, no `tabindex` roving.
- [ ] Token violation: size/density padding+gap are a hardcoded inline rem table (`toolbar.rs:98-109`), not resolved from `ToolbarSpec::padding_token()` / `gap_token()`. Resolve from tokens; drop the literal `(0.25, 0.375, 0.375)` rows.
- [ ] `has_separator` is a Rust-only prop with no contract basis — when set it adds `border_b_2()` (`toolbar.rs:136-139`), an emphasis the contract never specifies. Remove or land it in the contract.
- [ ] `alignment` branch (`toolbar.rs:125-134`) renders `justify_*` for a prop the contract does not define (see Contract↔Svelte). Reconcile with contract first.
- accepted: no ARIA (gpui has no accessibility API) — `role="toolbar"` / `aria-orientation` / `aria-label` not emitted.
- note: color-mix replicated correctly via alpha-scaling (`toolbar.rs:91-93`, `panel*0.94`, `border*0.78`); vertical orientation → `flex_col` + (implicit) handled.

## Jetstream gap (vs Svelte + contract)

- [ ] No background fill — contract §8 + Svelte require `color-mix(panel 94%, transparent)`; `js_toolbar` never calls `.bg(...)`. Add panel-mix fill.
- [ ] No container border — contract §8 + Svelte require a full `0.0625rem` border (border-subtle 78%); Jetstream only draws `border_b_1` and only when `has_separator` (`toolbar.rs:30-33`). Add the full border.
- [ ] No border-radius — contract §8 requires `radius.surface`; `js_toolbar` never calls `.rounded(...)`. Add radius.surface.
- [ ] Vertical padding/height violation: uses `min_h(control_height)` + `pl`/`pr` only (`toolbar.rs:16-21`) — no block padding, height is forced to control-height instead of auto-sizing to content like Svelte `inline-flex`. Use `py(padding)` block padding, drop `min_h`.
- [ ] No roving focus / arrow-key navigation and no `orientation` handling — `js_toolbar` is always `flex_row` (`toolbar.rs:17`); vertical orientation (contract §4) ignored. Interaction not present in preview `main.rs` event loop either (grep: no toolbar focus wiring).
- accepted: no ARIA channel (`role`/`aria-label` not emitted).
- note: `gap`/`padding` correctly resolved via `resolve_px(spec.gap_token())` / `padding_token()`.

## Specimen parity

- Svelte covers: Formatting toolbar (ghost icon buttons + separator), Actions toolbar (secondary + separator + primary), Sizes snippet, Densities snippet (`ToolbarSpecimen.svelte`).
- GPUI covers: Horizontal (ghost B/I/U + separator + align arrows), With primary action (Discard/Save draft + separator + Publish), Sizes, Densities, click-action feedback. — missing: nothing material; uses glyph labels (`B`/`I`/`U`/arrows) instead of real icon buttons (icon-registry gap, not a contract gap).
- Jetstream covers: Start aligned (default), End aligned. — missing: **Sizes** group, **Densities** group, **separator** usage, **primary-action** layout, icon-button items. Specimen exercises the Rust-only `alignment` prop rather than the contract's Formatting/Actions specimens (§13).

## Notes

- Biggest `consv=gap` driver: contract §8 `gap`/`padding` literals are stale vs Svelte (`0.375rem` gap, `0.25rem 0.375rem` padding), and the size/density variant tables are undocumented.
- `alignment` is a three-target Rust invention with no Svelte/contract counterpart — decide whether to promote it to the contract or remove it; it currently shapes the Jetstream specimen.
- Roving focus is the dominant cross-target behavioral gap: neither Rust target implements the arrow-key/Tab roving pattern that is Tier-1 strict parity in the contract.
