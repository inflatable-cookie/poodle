<!-- parity consv=fixed gpui=1 jetstream=1 specimen=ok -->
<!-- pass 85: closed the columns gap — added CardToggleGroupSpec.columns (u32, default 2) +
     with_columns + column_count() (clamp 1..4). Both targets now lay options out in rows of
     column_count() (flex_col of flex_rows, last row padded with flex spacers) instead of a
     plain flex-wrap. Contract §3 columns done. Remaining 1/1: count pill (no spec field).
     1 probe test (4 opts @ columns=2 render all + clamp); specs 116, jet 739, gpui clean. -->
<!-- specimen pass: GPUI rebuilt (single + multi selection, disabled option, disabled group, sizes xs–xl, densities) and Jetstream card_toggle_group.rs created + registered (pub mod + dispatch arm) with the same coverage via real js_card_toggle_group/Card. Both previews build clean. `count` pill + `columns` not exercised: neither field exists on CardToggleGroupSpec/CardToggleOption (shared-spec gap, same omission as GPUI component). -->
<!-- pass 46: Jetstream js_card_toggle_group rebuilt to match GPUI — each option composes
     js_card(interactive/selected when in values) with title (weight 600, text-primary at
     title_font_rem) + optional description (text-secondary at description_font_rem); multi-
     select model kept (values Vec<String>); flex-wrap grid + density-driven gap
     (control_space_x_rem); per-item + group disabled via state.opacity.disabled. All token/
     contract-rem resolved, zero hardcoded hsla/px. Probe tests: titles+description, multi-
     select (a+c), disabled group. gpui+jet build/test clean. Remaining jet (2): count-pill +
     columns header (not in shared spec — same omission as GPUI; needs human spec call).
     Selection/keyboard = preview-loop (accepted). -->
# Parity: CardToggleGroup

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/card-toggle-group.md`
- Svelte (authoritative): `packages/svelte/components/src/CardToggleGroup.svelte`
- GPUI: `packages/gpui/components/src/composites/card_toggle_group.rs`
- Jetstream: `packages/jetstream/components/src/card_toggle_group.rs` **(ABSENT — not in `lib.rs`)**
- Specimens: svelte `packages/svelte/preview/src/specimens/CardToggleGroupSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/card_toggle_group_specimen.rs` · jetstream **ABSENT**

## Contract ↔ Svelte

Contract anatomy invents an indicator Svelte never renders; one class-prefix gap. Svelte authoritative.

- [x] FIXED Anatomy: removed the phantom `[Indicator]` part from contract §2; the header now lists only `Title` + optional `Count`, matching Svelte (selection shown via the `Card` selected state).
- [x] FIXED Class prefix: contract §2 anatomy and the new §7 token tables use the `poodle-` prefix and target `:global(.poodle-card)`.
- [x] FIXED Token/sizing tables: added §6 Layout (auto-fit grid model + min-width) and §7 Token Usage with per-part token tables, a full Size Adjustments matrix (min-width, card gap/padding, title/description/count font + count padding), a Density Adjustments table (gap only), and a data-attributes table — all mirroring Svelte (`:164-371`). The trailing "Use With CardRadioGroup" section is renumbered to §8.
- Props otherwise match: `value`/`defaultValue`/`allowDeactivation`/`columns`/`onValueChange` all present in Svelte exactly as contract §3 documents. OK.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] **Spec uses the wrong selection model.** `CardToggleGroupSpec` has `values: Vec<String>` + `is_selected()` (multi-select) (`packages/contracts/components/src/card_toggle_group.rs:34,73`); contract/Svelte are single-value (`value`/`defaultValue`/`allowDeactivation`). **Replace `values: Vec<String>` with single `value`/`default_value` + `allow_deactivation`.**
- [ ] Wrong field name: `CardToggleOption` uses `title` (`card_toggle_group.rs spec:6`); contract/Svelte item type uses `label`. GPUI reads `option.title` (`card_toggle_group.rs:39`). Rename to `label` for parity with `CardToggleItem`.
- [ ] No `count` support — `CardToggleOption` has no count field and the header never renders the count pill (contract §2 `[Count]`, Svelte `:143-147`).
- [ ] No `columns` / grid — renders a vertical `flex_col` (`card_toggle_group.rs:26`) instead of the auto-fit N-column grid; `columns` prop absent entirely.
- [ ] Hardcoded gaps `.gap(px(8.0))` (`card_toggle_group.rs:26`) and `.gap(px(4.0))` (`:36`) — raw literals; resolve root gap + card body gap from density/size tokens.
- [ ] No `disabled` handling — neither group `disabled` nor per-item `disabled` dims or blocks the card (`is_disabled`/opacity never applied).
- [ ] No `allowDeactivation` behavior, no `onValueChange` callback, no selection interaction — render-only.
- [ ] No keyboard navigation (arrow roving tabindex + Space/Enter toggle, contract §5).
- [ ] No size/density variant resolution — `size`/`size_role`/`density` spec fields exist but are unused; title font, padding, count sizing all fixed.
- accepted: no ARIA (gpui has no accessibility API) — `role="group"`/`role="button"`/`aria-pressed` not expressible.

## Jetstream gap (vs Svelte + contract)

**TOP PRIORITY: no Jetstream implementation exists at all.** `packages/jetstream/components/src/card_toggle_group.rs` is absent and there is no `pub mod card_toggle_group;` in `packages/jetstream/components/src/lib.rs` (radio is at line 133, toggle is missing). No specimen either. All work below is greenfield.

- [x] DONE: `js_card_toggle_group(spec, theme)` created + registered, composing the `Card` primitive (interactive/selected) per option with title + optional description Text. Mirrors GPUI's scope. Probe-tested.
- [ ] **Spec/contract question (shared with GPUI, unresolved):** `CardToggleGroupSpec` is multi-select (`values: Vec<String>`) — which is correct for a *toggle* group, contradicting the "single-value" analysis claim. It also lacks `label`/`count` header fields and a `columns` field. Needs a human call: is this single- or multi-select, and does it have a label/count header? GPUI also omits these, so it's a shared-spec gap, not a Jetstream-only one. Render left model-agnostic (renders whatever `is_selected` reports).
- [ ] Render header = `label` + optional `count` pill; resolve count border/padding/font from the per-size token table.
- [ ] Implement `columns` as an auto-fit grid with the size-driven `min-width` (Svelte `:167-212`).
- [ ] Resolve all dims from tokens — title font, card gap/padding, description font, count sizing per the size matrix; root gap per density.
- [ ] Disabled group + per-item disabled → `resolve_opacity(theme, "state.opacity.disabled")`, block selection.
- [ ] `allowDeactivation` toggle-clear semantics + selection state (interaction wired through preview `main.rs` event loop).
- [ ] Add specimen `packages/jetstream/preview/src/specimens/card_toggle_group.rs` mirroring Svelte (query variants, deactivation, disabled, sizes, densities).
- [ ] Register the specimen in the jetstream preview `specimens/mod.rs` + component registry.
- accepted: no ARIA channel; interaction in preview event loop, not the component.

## Specimen parity

- Svelte covers: Query variants (4col, counts), Deactivation allowed (3col, `allowDeactivation`), Disabled group (3col), `sizes` + `densities` snippets, live "Selected:" readout (`CardToggleGroupSpecimen.svelte`).
- GPUI covers: a single static group (grid/list/audit view) with two pre-selected values (`card_toggle_group_specimen.rs`). — missing: **counts**, **deactivation**, **disabled group**, **columns**, **sizes**, **densities**, live readout; and it demonstrates multi-select which contradicts the contract.
- Jetstream covers: **nothing — no specimen file**. — missing: entire specimen set.

## Notes

- The GPUI `values: Vec<String>` multi-select model is the deepest defect: it makes the GPUI component a different control than the contract describes. The spec lives in `packages/contracts/components/src/card_toggle_group.rs` and is shared with Jetstream, so fixing it unblocks both Rust targets.
- `consv=fixed`: phantom Indicator part removed, `poodle-` class prefix applied, and the missing token/sizing table added (§6 Layout + §7 Token Usage). All were contract-doc fixes; Svelte unchanged. Remaining gpui/jetstream work (incl. greenfield jetstream + shared-spec single-value fix) is code-side.
- Jetstream count (12) reflects full greenfield build incl. spec fix, module wiring, component, and specimen + registry — this is the single biggest gap across both assigned components.
