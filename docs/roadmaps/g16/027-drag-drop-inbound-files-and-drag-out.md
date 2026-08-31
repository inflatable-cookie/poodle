# g16.027 — Drag-And-Drop Inbound Files And Drag-Out

Status: complete — delivered in PR #115, under review
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

- [x] TypeScript and Rust capability/receipt/lifecycle contracts are paired —
      `packages/core/src/external-file-drag.ts` and
      `packages/contracts/headless/src/external_file_drag.rs`, with the shared
      abort channel moved to the kernel module as `DragHostAbort`.
- [x] Fake host adapters prove existing file, materialized file, unsupported,
      cancellation, failure, supersession, multiple-file, and retained-cleanup
      outcomes — 27 web cases in
      `test/headless-dom/inbound-files-and-drag-out.test.ts` plus six mounted
      GPUI host-stub regressions.
- [x] Browser inbound-file targets use the common target/eligibility path —
      one subject kind, the ordinary hit test, arbitration, `canDrop`,
      revalidation, and `onDrop`; no second file-drop callback exists.
- [x] Svelte and React specimens show useful inbound and export states without
      exposing real paths; exhaustive cases stay in tests — `ExternalFileSurface`
      in both frameworks, whose tests assert the host's path and receipt id
      never reach the rendered surface.
- [x] Electron/Tauri integration points are documented interfaces or test
      stubs only; no shell dependency enters Poodle — the mapping is documented
      in the core README and spec 069; no package imports either.
- [x] OS/DAW acceptance remains manual downstream evidence and is not faked —
      there is no committed export terminal, and the probe says what it does
      not prove.
- [x] No component evidence row moves.

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

## Delivered

- Paired contracts, validation, and bounds in both languages, with the shared
  `DragHostAbort` / `DragHostCleanup` moved onto the kernel module.
- Web controller: export preparation on the pre-drag gesture, the host-owned
  native start, the export state projection and `data-poodle-drag-export`
  attribute, the inbound session, and validation before eligibility.
- `createInboundFileDataTransferBridge`: the browser's own file drag as a
  bridge, holding the `File` objects behind a consumer-authored projection.
- Renderer-neutral construction (`inbound_file_target`, `file_export_source`)
  and the same two seams wired through the GPUI controller, with six mounted
  host-stub regressions.
- Both providers pass an inbound bridge; both frameworks carry the curated
  specimen.
- A new external-file leg in the headless Chromium/WebKit probe: nine checks
  per engine over the engines' own `DataTransfer`, `DataTransferItem`, and
  `File`.

Wiring found one real defect: the GPUI end-of-frame sweep cancelled an inbound
session on its first frame, because a batch from outside the application has
no local source — the same shape the cross-window projection was already
exempted for.

## Continuation

After merge, promote `g16.028` for remaining component migrations and programme
certification. Downstream shell adapters remain separately owned.
