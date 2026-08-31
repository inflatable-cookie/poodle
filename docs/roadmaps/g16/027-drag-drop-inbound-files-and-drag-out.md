# g16.027 — Drag-And-Drop Inbound Files And Drag-Out

Status: ready — host bridge merged in PR #113
Depends on: `026-drag-drop-cross-window-bridge-and-dock-region.md`
Governing refs: architecture 011 and spec 069

## Goal

Add explicit host capabilities for inbound files and native file drag-out.
Poodle owns semantic source/target state and presentation. Hosts own file/path
resolution, materialization, native drag start, retention, and cleanup policy.

## Required Boundary

- Consumers supply opaque export subjects, never eager filesystem paths.
- Capabilities distinguish files, multiple files, promised files, and custom
  data types without user-agent or shell-name branching.
- Preparation is abortable and returns an opaque receipt. Unavailable,
  preparing, armed, dragging, cancelled, failed, and ended states are visible
  and accessible.
- Existing, eagerly materialized, promised, and custom forms remain distinct;
  file-backed export is the portable baseline.
- Browser `File`, Tauri path events, Electron `startDrag`, filesystem handles,
  and temp directories stay behind host adapters. Poodle imports none of them.
- Native drag end never authorizes Poodle to delete a temporary artifact.
- Inbound external data is validated for type, count, size, protocol, and
  host-issued identity before target eligibility.

## Acceptance Criteria

- [ ] TypeScript and Rust capability/receipt/lifecycle contracts are paired.
- [ ] Fake host adapters prove existing file, materialized file, unsupported,
      cancellation, failure, supersession, multiple-file, and retained-cleanup
      outcomes.
- [ ] Browser inbound-file targets use the common target/eligibility path.
- [ ] Svelte and React specimens show useful inbound and export states without
      exposing real paths; exhaustive cases stay in tests.
- [ ] Electron/Tauri integration points are documented interfaces or test
      stubs only; no shell dependency enters Poodle.
- [ ] OS/DAW acceptance remains manual downstream evidence and is not faked.
- [ ] No component evidence row moves.

## Writable Scope

- focused core/headless export and inbound-receipt types/machines;
- web native-data adapters and runtime projections;
- renderer-neutral Rust construction and GPUI presentation hooks where the
  active runtime can represent preparation state;
- focused fake-host/browser tests, contracts/guides, and curated specimens;
- this card, one log, g16 closeout, and `PAPERCUTS.md`.

Do not import or edit Electron, Tauri, Longhorn, Loophole, filesystem, shell,
application export policy, package versions, releases, workflows, or siblings.

## Validation

Run focused paired contract/machine/adapter tests, headless Chromium/WebKit
inbound probes, active native construction tests, docs and active-cohort boards,
unchanged ledger checks, one final headless `effigy qa`, and diff check. Never
run windowed/native visual, Jetstream, release, or sibling mutation commands.

## Stop Conditions

- A Poodle public prop must carry a path, native file object, shell type, or
  application record.
- Cleanup policy can only be expressed by deleting artifacts in Poodle.
- Tauri/Electron behavior must be guessed or pulled in as a dependency.
- The card expands into application materialization, DAW interoperability, or
  another component migration.

## Continuation

After merge, promote `g16.028` for remaining component migrations and programme
certification. Downstream shell adapters remain separately owned.
