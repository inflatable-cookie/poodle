# g15.007 — Licence Family Native Completion (August batch log)

Date: 2026-08-17
Card: `docs/roadmaps/g15/007-licence-family-native-completion.md`
Worktree: `t3code/complete-licence-native-support` (worker lane, parallel with
`g15.014`)
Handoff: `docs/handoffs/20260817-070944-g15-007-licence-native-completion.md`

## Summary

Completed the native surface for the Licence family — LicenceActivation,
LicenceSeats, LicenceStatus — across hand-written Rust declarations,
`poodle-render` composition, and GPUI, including the two native prerequisites
the carry-forward card hid: explicit CodeInput grouping/separator/completion
result, and a generic GPUI FileUpload single-file selection/read seam.

## Batches

- **Batch A — native prerequisites.** CodeInput lost its inferred 3+3 split:
  `CodeInputSpec` gains `groups` (one complete positive-integer partition of
  `length`), a presentation-only `separator`, and `completion_result` bound
  to the exact value it was checked against. The renderer applies group-end
  margins at explicit boundaries only, renders separators at valid boundaries,
  and shows the tick/cross only while the value is the checked one. FileUpload
  gains the generic browse intent (`FileUploadHandlers.on_browse`);
  `poodle_gpui_node_backend::file_capability` provides the GPUI seam — the
  live OS path prompt plus read, and the same `finish_file_pick` pipeline
  (accept/size rules then bare base64) for headless-injected fixtures.
  Accept/size/base64 mirror `packages/core/src/file-upload.ts` in
  `poodle_headless::file_upload`.
- **Batch B — declarations.** `poodle_headless::licence` ports the approved
  `licence.ts` semantics (status view derivation, submit resolver emitting
  exact structural credentials, seat-row derivation, absolute-time formatter).
  `LicenceActivationSpec`, `LicenceSeatsSpec`, `LicenceStatusSpec` are
  cloneable data; web-native props stay out.
- **Batch C — render.** `licence_status`, `licence_seats`, and
  `licence_activation` in `poodle-render`. No component-specific behaviour in
  generic runners.
- **Batch D — GPUI and evidence.** Specimens for all three plus mounted-window
  regressions.

## Evidence per runtime

- **Rust spec tests** (`cargo test -p poodle-specs`): grouping never inferred,
  completion result belongs to its value, LicenceActivationSpec route/label/
  freeze derivations (260 passed).
- **Headless logic tests** (`cargo test -p poodle-headless`): file accept/size
  rules and base64 vectors, licence status view copy/tone rules, submit
  resolution, seat rows, absolute time (96 passed).
- **Render tests** (`cargo test -p poodle-render`): CodeInput group margins
  only at explicit boundaries, separators never entering the value, completion
  indicator bound to the checked value; FileUpload browse wiring; licence
  status/seats/activation composition incl. no machine ids in rendered text,
  rename blank-to-null, release confirm, submit freeze (222 passed).
- **GPUI node backend** (`cargo test -p poodle-gpui-node-backend`): the
  generic file seam — injected fixture through the same pipeline as the OS
  prompt, accept rejection honesty, size rule default (16 passed).
- **Mounted headless regressions** (`effigy regressions:native`): 13 passed —
  the six retained plus grouped CodeInput typing/completion, stale completion
  cannot render, dropzone browse through the injected seam (selection and
  rejection), LicenceActivation segmented key path (type → tick → submit
  emits the exact key), LicenceSeats release through ConfirmAction, and
  LicenceStatus display with authority reads as data state.
- **GPUI preview** (`cargo check -p poodle-gpui-preview`): the three new
  specimens compile into the catalogue (`licence-activation`,
  `licence-seats`, `licence-status`).

One runtime does not borrow another's pass; each surface's evidence is named
above.

## Intentional binding differences

- **LicenceStatus quiet detail timezone.** The web formats the `inGrace`
  absolute line in the user's local timezone; the pure Rust mirror has no
  timezone database, so `poodle_headless::licence::format_display_time_date`
  formats the same instant in UTC. Honest (accurate instant, correct modern
  date) and recorded in the licence-status contract §11. Not a local-time
  claim.
- **GPUI file accept filter.** GPUI 0.2.2's `PathPromptOptions` has no accept
  field, so the configured `fileAccept` (and the web default 10 MB size rule)
  is enforced after selection and a rejection is reported honestly — never
  claimed as OS-filtered. Recorded in the file-upload contract §15.
- **Host-driven open/edit state on Rust targets.** Confirm-dialog open and
  inline-edit state are controlled spec fields on the Rust targets (the
  established vocabulary pattern), rather than component-internal reactive
  state as on the web.

## Validation run

- `cargo test -p poodle-render` — 222 passed
- `cargo test -p poodle-specs` — 260 passed
- `cargo test -p poodle-headless` — 96 passed
- `cargo test -p poodle-gpui-node-backend` — 16 passed
- `cargo check -p poodle-gpui-preview` — clean
- `cargo test --test headless_regressions` — 13 passed
- `cargo test --test catalogue` — 7 passed

Baseline (unchanged by this lane, confirmed identical on `origin/main`):
the preview-lib `contract_usage_docs` unit tests fail parsing
`button`/`sidebar-nav`/`media-preview` usage data; they are not part of
`check:gpui` or `regressions:native`.

## Closeout

- `release-baseline-roster.md`: the three Licence rows moved from `missing` to
  named spec/render/specimen files; summary counts updated (Rust declaration
  166/8, Rust render 164/10, GPUI specimen 148/26).
- `release-gap-register.md`: the Licence family row recorded closed; headless
  regression note updated.
- Component contracts: Rust binding notes added to licence-activation,
  licence-seats, licence-status, code-input, and file-upload; the stale
  CodeInput 3+3 delta rows in code-input and licence-activation marked closed.
- PAPERCUTS additions for the headless-driver element-id limitation.

## Notes / papercuts

- The headless driver's test platform renders a view several times per draw,
  so an interactive node without a declared `id` does not keep a stable
  element across a click (the production preview renders once per frame and is
  unaffected). Mounted regressions therefore assign explicit ids to the
  interactive nodes they drive — the pattern every retained regression in
  `tests/headless_regressions.rs` already used. Recorded in `PAPERCUTS.md`.
