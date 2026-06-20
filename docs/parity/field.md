<!-- parity consv=ok gpui=6 jetstream=6 specimen=gap -->
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

- [ ] Description renders as inline text below the label (`field.rs:166-173`), violating contract §4/§9 "description is **never rendered as an inline paragraph** — always inside a Popover triggered by an info icon". A `popover.rs` primitive exists (`packages/gpui/components/src/composites/popover.rs`) but is unused. **Build the info-icon + popover trigger in the label row; stop rendering `info_text()` inline.**
- [ ] No info-icon anatomy part at all — contract §2 requires `.field__info-icon` (1.25em wrapper, 0.75em SVG, pill radius, secondary-tinted bg) next to the label when `description`/`hint` set. Absent from the builder.
- [ ] Label color uses hardcoded opacity heuristic `Hsla { a: text_primary.a * 0.82, .. }` (`field.rs:118-121`) instead of the contract §8 formula `color-mix(text-primary 45%, text-secondary)`. **Resolve both `text-primary` and `text-secondary` and mix, or add a `label_color_token()` to FieldSpec; drop the raw `0.82`.**
- [ ] Optional marker rendered inside the label-row flow via `justify_between` on `label_row` but the optional text is added to `label_row` not a separate header end — verify it sits at header end per contract §2 (Optional Marker is a sibling of Label Row, not inside it). Current code (`field.rs:151-161`) places it as a second child of the same justify-between row, which is acceptable visually but does not mirror the Svelte header/label-row nesting.
- [ ] No size/density typography scaling on the optional/message vs label split is wrong-direction: optional + messages use `supporting_text_typography_token()` (`field.rs:158,171,187,199`) which is correct, but there is no info-popover so the per-size `em` icon-scaling requirement (contract §7, parity Tier-1) cannot be met until the icon exists.
- [ ] `span` / `grid_area` props exist on FieldSpec but are never consumed in the GPUI builder — contract §3 + §10 "GPUI must support equivalent layout" for grid-column span / grid-area. **Apply span/grid_area to the root element or document as accepted delta.**
- accepted: no ARIA (gpui has no accessibility API) — label-to-control and `aria-describedby` relationships are computed in FieldSpec (`described_by()`, `message_id()`) but not emitted into a native a11y tree.

## Jetstream gap (vs Svelte + contract)

- [ ] Description renders as inline text below the control (`field.rs:60-66`), violating contract §4/§9 (must be popover-only). A `popover.rs` primitive exists (`packages/jetstream/components/src/popover.rs`) but is unused. **Render `info_text()` through an info-icon popover trigger in the label row; remove the inline label.**
- [ ] No info-icon anatomy part — contract §2 `.field__info-icon` (1.25em/0.75em, pill radius, secondary-tinted bg) absent from `js_field`.
- [ ] Label color uses hardcoded opacity heuristic `tint(text_primary, 0.82)` (`field.rs:14`) instead of contract §8 `color-mix(text-primary 45%, text-secondary)`. **Mix text-primary with text-secondary (or add `FieldSpec::label_color_token()`); drop the raw `0.82`.**
- [ ] Optional marker uses `label_size` typography (`field.rs:43-46`) instead of `supporting_text_typography_token()` — contract §7 size table puts optional copy on the smaller supporting scale (e.g. md optional `0.75rem` vs label `0.8125rem`). **Use `spec.supporting_text_typography_token()` for the optional marker, matching GPUI (`gpui/field.rs:157`).**
- [ ] `span` / `grid_area` props on FieldSpec never consumed in `js_field` — contract §3 grid layout integration. **Apply or document as accepted delta.**
- [ ] Header layout omits `justify_between` / space-between — Svelte/contract §8 header is `justify-content: space-between` so the optional marker right-aligns; `js_field` label_row is a plain `flex_row` (`field.rs:20-23`) leaving the optional marker tight against the label. **Add space-between to the header row.**
- accepted: interaction (info-popover open/close on click) lives in the preview event loop / main.rs, not the component — Field fires no events of its own (contract §5), so this is inherent; popover trigger wiring still needs to exist in the component tree.
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
