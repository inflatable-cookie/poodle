<!-- parity consv=ok gpui=1 jetstream=1 specimen=gap -->
<!-- pass: label color now color-mix(text-primary 45%, text-secondary) both targets; info-icon pill part added (em-scaled, secondary-tinted, pill radius), description no longer inline; Jetstream optional marker on supporting size + header space-between. Remaining open: span/grid-area = accepted layout delta (no CSS-grid parent). Jetstream probe tests cover label+required+info-icon+error+optional. -->
# Parity: Field

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/field.md`
- Svelte (authoritative): `packages/svelte/components/src/Field.svelte`
- GPUI: `packages/gpui/components/src/primitives/field.rs`
- Jetstream: `packages/jetstream/components/src/field.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/FieldSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/field.rs` · jetstream `packages/jetstream/preview/src/specimens/field.rs`

## Contract ↔ Svelte

Props, anatomy, states, and ARIA all align — Svelte is a faithful implementation of the contract. No divergence found.

- Props: every contract §3 public prop is present in Svelte with matching type/default — `id`, `label`, `description=null`, `hint=null`, `error=null`, `pendingMessage=null`, `validationState="none"`, `required=false`, `optionalLabel=null`, `span=null`, `gridArea=null`, `size=null`, `sizeRole="control"`, `density=null` (`Field.svelte:36-53`). Plus the two composition snippets `control` / `children` (`Field.svelte:32-33`) matching contract §3 Composition.
- Description/hint merge: `infoText = description ?? hint` (`Field.svelte:59`) matches contract §3 "description takes precedence". Deprecated `hint` alias preserved.
- Anatomy: root `data-size`/`data-density`/`data-validation-state` (`Field.svelte:85-90`); header (`:92`), label-row (`:93`), `<label for={id}>` (`:94`), required marker `*` (`:97`), info popover via `Popover placement="top" offset={6}` with info-icon trigger (`:101-110`), optional marker (`:114`), control slot inside `UiPresentationProvider` (`:118-132`), sr-only description (`:135`), error/pending message (`:138-146`) — all contract §2 parts present.
- Description-never-inline rule (contract §4/§9): satisfied — `infoText` only renders in the Popover surface; the inline element at `:135` is the visually-hidden `.poodle-field__sr-description` for `aria-describedby`, not visible copy.
- States: invalid (`:138`), pending (`:142`), required (`:96`), valid (no message) — match contract §4.
- ARIA: `<label for>` association (`:94`); `aria-describedby` string = descriptionId + active messageId (`:72-74`); info icon `aria-label="More information"` (`:104`); popover `ariaLabel="Field description"` (`:101`); required marker `aria-hidden="true"` (`:97`); messages `aria-live="polite"` (`:139,:143`) — all contract §6 requirements met.
- Token usage: label color `color-mix(... text-primary 45%, text-secondary)` (`:199`); required/error use `--poodle-color-status-danger` (`:207,:228`); per-size + per-density CSS vars (`:231-267`); info-icon `em`-scaled `1.25em`/`0.75em` (`:278-291`) — match contract §7/§8.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] DONE Description no longer renders inline — the inline `info_text()` paragraph was removed (contract §4/§9). The info-icon part below now carries it.
- [x] DONE Info-icon anatomy part added — `.field__info-icon` pill (em-scaled `1.25em` wrapper / `0.75em` glyph, `radius.pill`, `text-secondary`@14% bg) renders next to the label when `description`/`hint` is set. Spec methods `info_icon_bg_token()`, `info_icon_color_token()`, `info_icon_radius_token()`, `INFO_ICON_EM`/`INFO_ICON_SVG_EM`/`INFO_ICON_BG_ALPHA` drive it.
- [x] DONE Label color resolves `color-mix(text-primary 45%, text-secondary)` via `theme_ext::color_mix` and the new `label_color_primary_token()`/`label_color_secondary_token()` + `LABEL_COLOR_PRIMARY_RATIO`. The `* 0.82` shortcut is gone.
- [x] DONE Optional marker stays on `supporting_text_typography_token()` at header end (sibling of the label row); the em icon-scaling requirement is now met because the icon exists.
- [ ] `span` / `grid_area` props are still not emitted as grid placement — **accepted layout delta**: GPUI has no CSS-grid parent context (contract §10/§12 grid integration is platform-owned). Documented inline in `field.rs`.
- accepted: info-popover open/close on hover is an interaction owned by the preview event loop, not this stateless `IntoElement` builder (same as the Popover trigger). The icon part + its content carry parity; contract §12 allows tooltip-vs-Popover freedom.
- accepted: no ARIA (gpui has no accessibility API) — label-to-control and `aria-describedby` relationships are computed in FieldSpec (`described_by()`, `message_id()`) but not emitted into a native a11y tree.

## Jetstream gap (vs Svelte + contract)

- [x] DONE Description no longer renders inline (contract §4/§9) — the inline `info_text()` label was removed; the info-icon part carries it.
- [x] DONE Info-icon anatomy part added — `.field__info-icon` pill (em-scaled `1.25em`/`0.75em`, `radius.pill`, `text-secondary`@14% bg via `tint`) in the label row when `description`/`hint` is set.
- [x] DONE Label color resolves `color-mix(text-primary 45%, text-secondary)` via `theme_ext::color_mix` and the new spec token methods; the `tint(text_primary, 0.82)` shortcut is gone.
- [x] DONE Optional marker uses `supporting_text_typography_token()` (smaller supporting scale), matching GPUI. Verified by probe test `field_optional_marker_uses_supporting_size_not_label_size`.
- [x] DONE Header now uses `justify_between` (space-between) with a dedicated label-row child so the optional marker right-aligns; the info icon sits inside the label row.
- [ ] `span` / `grid_area` props still not consumed — **accepted layout delta**: no Jetstream flex equivalent for CSS-grid placement (contract §10/§12). Documented inline in `field.rs`.
- accepted: info-popover open/close interaction lives in the preview event loop / main.rs, not the component — Field fires no events of its own (contract §5). The icon part + its content carry parity.
- accepted: no ARIA channel — relationships computed in FieldSpec but not surfaced.

## Specimen parity

- Svelte covers (`FieldSpecimen.svelte`): Default with description, Required, With error, Optional, plus `sizes` snippet (xs–xl) and `densities` snippet (compact/default/comfortable) driven by the preview harness.
- GPUI covers (`gpui/.../field.rs`): Default with description, Required, With error, Valid, Optional, With hint (progressive disclosure), Hint + description + required. — missing: **sizes** sweep, **densities** sweep, and **Pending** state. Has extra Valid/hint groups Svelte routes through harness snippets.
- Jetstream covers (`jetstream/.../field.rs`): With label + description, With error, Pending validation, Valid state. — missing: **Required** marker group, **Optional** marker group, **sizes** sweep, **densities** sweep.

## Notes

- `consv=ok`: contract and Svelte are fully aligned; no contract edits needed for this component.
- The dominant cross-target gap is identical in both Rust implementations: **description is shown as inline text rather than via the info-icon popover** (contract §4/§9). Both targets already ship a `popover.rs` primitive, so this is wiring work, not missing infrastructure. This single fix also unblocks the Tier-1 parity item "description renders in info popover, not inline" and "info icon scales with label font-size".
- Both Rust targets share the same label-color shortcut (`* 0.82` opacity vs `color-mix 45%`). Cleanest fix is a `FieldSpec::label_color_token()` (or a mix helper) so all three targets resolve identical label color from one source.
- No hardcoded px/color literals were found inside either component file — all dimensions resolve via `resolve_px`/token methods. The only raw literals are in specimen scaffolding (`gap(px(8.0))`, `w(300.0)`, group label `text_size(11.0)`), consistent with how `button.md` treats specimen chrome (not flagged as component token violations).
- FieldSpec exposes `described_by()`, `relationships()`, `message_id()`, `description_id()` etc. matching the Svelte slot props — the relationship logic is correct in the spec layer; the gap is purely that the GPUI/Jetstream builders don't emit it into an a11y tree (accepted runtime limit).
