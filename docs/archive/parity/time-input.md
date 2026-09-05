<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
# Parity: TimeInput

> Pass: Web adapters keep native `input[type=time]` and gate commits through the
> shared TimeInput machine. Incomplete native drafts (`value === ""` +
> `validity.badInput`) stay local and revert; a deliberate clear emits `null`.
> GPUI mounts a labelled group of Hour/Minute/conditional-Second spin-buttons
> on the same machine, with a persistent context cell on the reusable/
> specimen path. Jetstream renders the shared node snapshot (no preview-loop
> editor). The pre-1.0 `TimeFieldSpec` / `time_field` surface is gone.

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/time-input.md`
- Svelte: `packages/svelte/components/src/TimeInput.svelte`
- React: `packages/react/components/src/TimeInput.tsx`
- Shared model: `packages/core/src/time-input.ts` and
  `packages/contracts/headless/src/time_input.rs`
- GPUI: `packages/gpui/preview/src/specimens/time_input.rs` via
  `poodle_render::time_input_with_persistent_context`
- Jetstream: `packages/jetstream/preview/src/specimens/time_input.rs` via
  `poodle_render::time_input`
- Spec: `packages/contracts/components/src/time_input.rs` (`TimeInputSpec`)
- Specimens: svelte `packages/svelte/preview/src/specimens/TimeInputSpecimen.svelte`
  · gpui `packages/gpui/preview/src/specimens/time_input.rs`
  · jetstream `packages/jetstream/preview/src/specimens/time_input.rs`

Filename mapping: contract, Svelte, React, spec, and renderer all use
`time-input` / `TimeInput` / `TimeInputSpec`. No active `TimeFieldSpec` or
`time_field` module remains.

## Contract ↔ Svelte / React

Both web adapters render native `<input type="time">` and run `COMMIT_TEXT`
through the shared machine before `onValueChange`. Canonical values are
zero-padded 24-hour `HH:MM` / `HH:MM:SS` or `null`. Partial and invalid drafts
stay adapter-owned, mark `aria-invalid`, and revert on blur or Escape.

Native incomplete vs clear:

- incomplete: `value === ""` and `validity.badInput` — no callback, draft stays
- clear: `value === ""` without `badInput` — emit `null`

Stepping never emits an off-grid bound. Last on-grid in range is kept.

Divergences:

- [x] Shared commit boundary, overnight range, and step grid match the contract.
- [x] Native incomplete drafts are distinguished from a whole-control clear.
- [x] Rust public type is `TimeInputSpec`; renderer module is `time_input`.

## GPUI gap (vs Svelte + contract)

- [x] Segmented editor: labelled group + Hour / Minute / conditional Second
      spin-buttons, keys routed through the shared machine.
- [x] Reusable/specimen path owns a persistent `TimeInputContext` cell so a
      partial digit remounts with the draft and invalid border intact.
- [x] Mounted regression
      `time_input_segmented_editor_commits_drafts_and_bounds` drives that path.
- accepted: no native `input[type=time]`; custom 24-hour segments are the
  contract §12 delta.
- accepted: `describedBy` is not projected (GPUI has no described-by channel).

## Jetstream gap (vs Svelte + contract)

- accepted (preview-loop): Jetstream specimens compose `poodle_render::time_input`
  as a snapshot. Segment editing, arrow step, and `onValueChange` stay
  preview-loop; the shared renderer still paints the segmented control.
- accepted: no native `input[type=time]`; same contract §12 delta as GPUI.

## Specimen parity

- **Svelte** covers default, live value, min/max, seconds step, overnight,
  disabled, plus sizes and densities.
- **GPUI** covers the same groups through `TimeInput::from_spec` with a live
  context cell so incomplete edits remain visible.
- **Jetstream** covers value, placeholder, min/max, sizes, densities, disabled.
  Interaction stays preview-loop.

## Notes

- Public callbacks emit only constraint-valid canonical values or `null`.
- Step grid is anchored at `min` or midnight. Raw min/max endpoints are not
  committed when they sit off that grid.
- GPUI visual-comparison and described-by cells stay on the ledger as missing.
