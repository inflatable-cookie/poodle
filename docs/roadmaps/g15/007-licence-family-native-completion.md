# g15.007 — Licence Family Native Completion

Status: **complete** — accepted and merged in PR #32 (`a6017abb`)
Depends on: `g15.001` (measured gaps); carries `g14.017` requirements with
approved web references (`g14.015`/`g14.016`) intact
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../roadmaps/g14/017-licence-active-runtime-completion.md`,
`../../roadmaps/g14/conformance-estate.md`, `../../contracts/001-working-rules.md`

## Outcome

Complete the native surface for the Licence family — LicenceActivation,
LicenceSeats, LicenceStatus — which the inventory measured as web-complete but
missing Rust declaration, Rust render, and GPUI specimen. This recompiles the
approved `g14.017` requirements against the post-reject system: hand-written
`<Name>Spec` declarations, `poodle-render` composition, and focused
owner-local evidence. It does not revive the portable-interface or shared-corpus
pipeline.

## Scope

- LicenceActivation, LicenceSeats, LicenceStatus
- the two native prerequisites required by LicenceActivation: explicit
  CodeInput groups/separators/completion result, and generic GPUI FileUpload
  single-file selection/read plumbing
- new Rust declarations in `poodle_specs`
- new `poodle-render` implementations
- new GPUI specimens and focused headless tests

## Native Binding Boundary

The Rust declaration stays cloneable data. Runtime callbacks, parsers, async
account work, and host-owned account content do not enter `<Name>Spec`.

- `LicenceActivationHandlers` owns the synchronous key parser/typo classifier,
  exact activation callback for key/file credentials, account-activation
  request, view switch, and controlled machine-label edits.
- Account mode composes optional host-owned `poodle-node` content beside the
  spec and emits an account-activation request. The host owns async token
  acquisition and resulting command; tokens never enter render state.
- Key/file activation emits the exact structural credential plus the trimmed
  optional label. Parser input remains raw; file contents are base64 without a
  data-URL prefix and are never rendered or logged.
- File selection is a reusable FileUpload/GPUI capability, not
  LicenceActivation-specific backend logic. GPUI 0.2.2's path prompt has no
  accept-filter field, so enforce the configured accept rule after selection
  and report rejection honestly rather than claiming the OS dialog filtered it.
- Headless evidence injects a selected fixture path/bytes through the same
  generic capability seam. It never opens an OS window or substitutes a static
  rendered filename for the selection/read path.
- Add the Rust binding note to the component contracts before implementation;
  stop if the existing spec/handler split cannot preserve these semantics.

## Execution Plan

- [x] **Batch A — native prerequisites:** port CodeInput's explicit `groups`,
      optional `separator`, and full-value success/failure indication through
      Rust spec/render/GPUI; add generic GPUI FileUpload single-file
      selection/read plumbing with a headless injected-result path.
- [x] **Batch B — declarations:** hand-written `LicenceActivationSpec`,
      `LicenceSeatsSpec`, `LicenceStatusSpec` in `poodle_specs` matching the
      contract props tables and the native binding boundary above (web-native
      props stay out of the portable spec).
- [x] **Batch C — render:** `poodle-render` implementations for all three; no
      component-specific behaviour in generic runners.
- [x] **Batch D — GPUI and evidence:** GPUI specimens plus focused tests
      (activation input path, inline seat rename, release, status display);
      headless regression cases via `effigy regressions:native` where a
      mounted window is required, otherwise focused `#[test]` cases in the
      render crate.

## Goals

- [x] Hand-written `LicenceActivationSpec`, `LicenceSeatsSpec`,
      `LicenceStatusSpec` matching the contract props tables (web-native props
      stay out of the portable spec).
- [x] `poodle-render` implementations for all three; no component-specific
      behaviour in generic runners.
- [x] GPUI specimens and focused tests: activation input path, inline seat
      rename, release, status display; never render machine IDs; label-only
      visible identity.
- [x] Headless regression cases via `effigy regressions:native` where a
      mounted window is required; otherwise focused `#[test]` cases in the
      render crate.
- [x] Preserve approved curated specimens unchanged.
- [x] CodeInput no longer infers a 3+3 split; explicit valid partitions,
      separators, and completion results work in native rendering.
- [x] GPUI file activation uses a real path-prompt/read/base64 route in the
      live adapter and the same generic seam with injected fixture bytes in
      headless evidence.

## Acceptance

- [x] Every active-cohort surface (spec, render, GPUI) has evidence named in
      the card log; one runtime does not borrow another's pass.
- [x] `cargo test -p poodle-render`, `effigy check:gpui`, and
      `effigy regressions:native` pass.
- [x] Jetstream reported as program-deferred, not as an accepted absence.
- [x] Longhorn's adapter assertions stay Longhorn-owned; no Poodle → Longhorn
      edge added.

## Stop Conditions

- A portable interface, shared corpus, or comparator reappears under a new
  name.
- File selection/base64 is silently substituted by a static mock instead of
  being recorded as a runtime-owned capability delta.
- Generic CodeInput or FileUpload work expands beyond the exact prerequisite
  behavior above.
- `LicenceCentre` is revived by symmetry.

## Writable Scope

- Rust declarations, render modules, poodle-node/GPUI backend capability
  wiring, GPUI specimens, focused tests
- component-contract Rust binding notes for the three Licence components and
  the exact CodeInput/FileUpload prerequisite deltas
- bounded contract-first fixes to scoped defects the new evidence exposes
- `release-baseline-roster.md` and `release-gap-register.md` (native rows only,
  no status lines)
- one August batch log under `docs/logs/2026-08/`
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `cargo test -p poodle-render`
- `effigy check:gpui`
- `effigy regressions:native`
- `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
