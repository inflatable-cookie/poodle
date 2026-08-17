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

## Review round (orchestrator verdict on PR #32)

The review's six findings are addressed in the landed branch:

1. **Live OS picker is completion-driven.** The preview awaits each dialog's
   oneshot receiver in a GPUI task (`AppState::start_file_picks`) instead of
   polling it — a dialog result schedules the render that consumes it, and
   the old `try_recv` path (which treated a dropped sender as pending) is
   gone, replaced by `resolve_os_selection`. New regression
   `a_file_pick_result_lands_after_the_receiver_completes` proves the result
   arrives after the first frame through the executor.
2. **Route change invalidates the file read.** `NodeSpecimenEvent::FileInvalidate`
   clears completed bytes and bumps `file_generation`; every pick task
   captures the generation at spawn and drops a stale outcome. New bin-unit
   tests `a_route_change_invalidates_file_state_and_stales_late_outcomes` and
   `a_pending_pick_is_dropped_by_route_change` cover the web-caught
   route-away regression.
3. **Machine-name edit completes cleanly.** `LicenceActivationHandlers` now
   has distinct `on_machine_label_change` / `_commit` / `_cancel`; the
   specimen closes edit state on commit/cancel, never on a keystroke. Render
   test `machine_label_change_commit_and_cancel_are_distinct`.
4. **Embedded account/offline Activate is wired.** The specimen's embedded
   instance fires a host-owned acquisition request on the account route and
   runs the shared resolver on the offline route. Render test
   `submit_fires_in_account_and_offline_routes` plus mounted regression
   `licence_activation_account_submit_fires_through_the_real_tree`.
5. **LicenceStatus quiet line is local time.** The renderer resolves the
   authority instant through the platform's `localtime` (Unix) and formats
   with the pure `format_time_date_parts`; the UTC exception is removed from
   the contract and this log.
6. **FileUpload validation copy mirrors core.** `format_file_size` matches
   core's `toFixed(1)`+`parseFloat` spelling (`10 MB`, `2 KB`, `512 B`), with
   cross-implementation error-copy vectors in
   `error_copy_vectors_match_the_web_exactly`.

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
  rules and base64 vectors incl. the exact core error-copy vectors, licence
  status view copy/tone rules, submit resolution, seat rows, time-date parts
  (97 passed).
- **Render tests** (`cargo test -p poodle-render`): CodeInput group margins
  only at explicit boundaries, separators never entering the value, completion
  indicator bound to the checked value; FileUpload browse wiring; licence
  status/seats/activation composition incl. no machine ids in rendered text,
  rename blank-to-null, release confirm, submit freeze, account/offline
  submit firing, and distinct machine-label change/commit/cancel (224 passed).
- **GPUI node backend** (`cargo test -p poodle-gpui-node-backend`): the
  generic file seam — injected fixture through the same pipeline as the OS
  prompt, accept rejection honesty, size rule default, and
  `resolve_os_selection` reading a real temporary file, plus correct
  cancellation/failure handling (19 passed).
- **Mounted headless regressions** (`effigy regressions:native`): 15 passed —
  the six retained plus grouped CodeInput typing/completion, stale completion
  cannot render, dropzone browse through the injected seam (selection and
  rejection), LicenceActivation segmented key path (type → tick → submit
  emits the exact key), LicenceSeats release through ConfirmAction,
  LicenceStatus display with authority reads as data state, the account-view
  submit firing through the real tree, and the OS pick result landing after
  the receiver completes (async completion-driven delivery).
- **Preview bin unit tests** (`cargo test --bin poodle-preview app_state`):
  the route-change invalidation and stale-outcome generation guard (2 passed).
- **GPUI preview** (`cargo check -p poodle-gpui-preview`): the three new
  specimens compile into the catalogue (`licence-activation`,
  `licence-seats`, `licence-status`).

One runtime does not borrow another's pass; each surface's evidence is named
above.

## Intentional binding differences

- **GPUI file accept filter.** GPUI 0.2.2's `PathPromptOptions` has no accept
  field, so the configured `fileAccept` (and the web default 10 MB size rule)
  is enforced after selection and a rejection is reported honestly — never
  claimed as OS-filtered. Recorded in the file-upload contract §15.
- **Host-driven open/edit state on Rust targets.** Confirm-dialog open and
  inline-edit state are controlled spec fields on the Rust targets (the
  established vocabulary pattern), rather than component-internal reactive
  state as on the web.
- **Live OS pick completion.** The preview awaits each dialog's oneshot
  receiver in a GPUI task (completion-driven, never a poll), lands the
  outcome through entity state, and notifies — and a route change after the
  dialog opened makes the result stale by generation so it cannot land.

## Validation run

- `cargo test -p poodle-render` — 224 passed
- `cargo test -p poodle-specs` — 260 passed
- `cargo test -p poodle-headless` — 97 passed
- `cargo test -p poodle-gpui-node-backend` — 19 passed
- `cargo check -p poodle-gpui-preview` — clean
- `cargo test --test headless_regressions` — 15 passed
- `cargo test --bin poodle-preview app_state` — 2 passed (staleness/invalidation)
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
