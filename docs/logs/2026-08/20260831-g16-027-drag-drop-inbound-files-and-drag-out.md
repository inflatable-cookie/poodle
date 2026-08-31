# g16.027 — Drag-And-Drop Inbound Files And Drag-Out

Status: delivered — under review
Date: 2026-08-31
PR: https://github.com/inflatable-cookie/poodle/pull/115
Card: `docs/roadmaps/g16/027-drag-drop-inbound-files-and-drag-out.md`
Handoff: `docs/handoffs/20260831-180228-g16-027-inbound-files-drag-out.md`
Governing refs: `docs/architecture/011-drag-and-drop-substrate.md`,
`docs/specs/069-dependable-drag-and-drop-substrate.md`
Branch: `codex/g16-027-inbound-files-drag-out`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-027-inbound-files-drag-out`

## Outcome

The substrate now has both external-file boundaries, paired in TypeScript and
Rust and wired through the web controller, the renderer-neutral registrations,
and the GPUI controller.

Going out: a source declares one `fileExportBridge`, its host prepares an
opaque receipt on the pre-drag gesture, and the host runs the operating
system's drag while Poodle refuses the browser's own. Coming in: one
window-owned bridge publishes batches that become ordinary sessions under one
subject kind, validated before any target is asked and again at the drop.

No path, descriptor, temporary directory, or `File` crosses a public seam in
either direction, and nothing in Poodle deletes anything.

## Decisions worth keeping

- **There is no committed export terminal.** A native drag ending does not
  prove a destination consumed a file, and no browser or shell API reports
  that it did. So the export's honest qualities are `ended`, `cancelled`, and
  `failed`. The kernel records what it can check — nothing *local* committed,
  which is a cancellation — while the export's own state carries what the host
  reported. This is the one place the two disagree, deliberately.

- **The export state is a second projection, not a second lifecycle.** The
  card requires `unavailable`, `preparing`, `armed`, `dragging`, `cancelled`,
  `failed`, and `ended` to be visible and accessible. Two of those exist
  outside any session: `unavailable` before a gesture, and the terminal states
  after one. So the projection outlives the transaction, and the default
  announcement reads it — otherwise a screen reader would be told "cancelled"
  for a file the user successfully dragged onto their desktop.

- **The kernel was not touched.** Adding an intent-free terminal outcome would
  have rippled through both machines, the shared vectors, and every adapter,
  to express something the export projection already says honestly.

- **The host runs the native drag, so the browser's must not.** On `dragstart`
  an armed export calls `preventDefault()` and hands off — the documented
  Electron pattern. The cross-window bridge does the opposite for the same
  reason it exists: there the browser's drag *is* the transport. Two live
  drags for one gesture would also give the page a `dragend` that looks like
  an outcome.

- **One source leaves one way.** A registration carrying both a cross-window
  bridge and an export bridge throws. Any precedence rule would be a product
  choice made silently on the consumer's behalf.

- **A receipt beyond its adapter's capabilities is refused *and returned*.**
  Three files from an adapter that advertised one is a drag that fails
  somewhere far away. Refusing alone would abandon the temporary file the host
  already wrote, so the receipt goes back with `preparation-failed` — which is
  not a delete order either.

- **A display name that is a path is refused, not trimmed.** `displayName` is
  the one field with a plausible excuse to carry a location. Quietly
  presenting `secret.wav` for `/Users/tom/private/secret.wav` would hide the
  leak instead of stopping it.

- **Hover cannot see names or sizes, so the receipt says `null`.** A browser
  discloses only item kinds and declared types during `dragover`. Inventing
  `"file"` and `0` would produce a hover answer the drop contradicts; `null`
  defers exactly the undecidable rules, and the disclosed batch is validated
  again before it can commit. That is also the sharpest inbound case in the
  matrix: a file that is only too large once disclosed is refused at the drop.

- **Inbound validation runs before eligibility.** Count, size, declared type,
  name shape, transport, and host-issued identity are untrusted external
  input. A target should be answering "do I want this", not "is this even
  real", and its resolver is never called for a batch the boundary refused.

- **The inbound transport claim is exclusive and checked loudly.** A
  `data-transfer` bridge that cannot observe the document, and a `host` bridge
  that also binds document drag events, are both errors. Tauri's native
  capture and a webview's own drag events can be live at once; a window that
  listened to both would take one gesture as two drops.

- **The DOM adapter claims the drop for the whole document while a batch is
  live.** An unclaimed file drop navigates the window to the file and destroys
  the surface the user was dragging onto. Refusal is presented by Poodle's own
  target posture, not by the OS cursor.

- **`File` objects stay behind the adapter.** The default projection is the
  receipt; a consumer that needs the real thing supplies `project` and gets
  exactly what it asked for. The projection is live for the commit and
  released with the terminal.

- **The shared abort channel moved to the kernel module.** Both host bridges
  abandon requests for the same reasons, and `CrossWindowAbort` was the wrong
  name for the channel a file materialization watches. It is `DragHostAbort` /
  `DragHostCleanup` in `drag_drop.rs` now, with the same idempotence rules.

## Review oracle

| Adversarial case | Proof |
| --- | --- |
| A receipt cannot exceed its own adapter's capabilities | `inbound-files-and-drag-out.test.ts` — a three-file receipt from a single-file host is refused and returned; the GPUI mirror asserts the artifact survives |
| An unarmed export cannot start a native drag | same file, plus both probe engines: `dragstart` is refused and no host `start` runs |
| A native end is not a result | `dragend` reporting nothing, then the host's own terminal; the session is still dragging in between |
| An ending deletes nothing | after `ended`: zero cancels, one stop, the host's artifact still present, in web, GPUI, and both engines |
| Late preparation cannot resurrect superseded work | a receipt answered after supersession is handed back and cannot arm the successor — web and GPUI, the GPUI case with a real abort assertion |
| A path never crosses the seam | `name-is-a-path` refusal in both languages; the mounted specimens assert the host's path and receipt id are absent from the rendered surface; the probe asserts the same against real markup |
| Unsupported capabilities stay inert | a bridge that can export nothing never prepares, never advertises, and leaves an ordinary local drag; keyboard pickup never arms an export |
| A decline kills the transfer, not the drag | the local fallback commits an ordinary drop, and the declined bridge is not asked again |
| Inbound validation precedes eligibility | `canDrop` is never called for an over-limit batch, in web and in GPUI |
| Drop-time revalidation | a size hover could not see refuses the drop; hover acceptance does not carry it |
| Terminal accounting is exact | one `release` per batch, with the outcome the session actually reached; a repeat from the host cannot produce a second |
| A local gesture always wins | a batch arriving mid-drag is ignored and not released |
| Transport exclusivity | both mismatched bridge shapes throw at connect |
| Engine-level disclosure | Chromium and WebKit: `types` reports `Files`, `items[i].kind` reports `file`, hover names are undisclosed, drop names and sizes are real |
| Per-element leaves are not the drag leaving | the depth counter survives an inner leave and ends on the last one, in both engines |

### Falsified

Each of these was checked by planting the pre-fix behaviour back and
confirming the case fails:

1. the GPUI first-frame sweep — found this way rather than reasoned about: the
   inbound session died on its first frame until the sweep learned that a
   batch from outside the application has no local source, exactly like an
   incoming projection;
2. validation-before-eligibility, by moving the check after `canDrop` — the
   over-limit case then passes on the consumer's answer;
3. drop-time revalidation, by keeping the hover batch — the oversized file
   commits;
4. the returned over-capability receipt, by refusing without cancelling — the
   host's artifact is stranded with no command;
5. the superseded return, by dropping the late receipt;
6. the `preventDefault` on an armed export's `dragstart`, by letting the
   browser's drag proceed — the probe then sees a live web drag beside the
   host's;
7. the path-shaped display name, by trimming to a basename instead of
   refusing;
8. the exclusive transport claim, by accepting a bridge that both declares
   `host` and binds the document — the document then produces a second drop
   for one gesture.

## Evidence

- Contracts: `packages/core/src/external-file-drag.ts`,
  `packages/contracts/headless/src/external_file_drag.rs` (8 Rust cases),
  `packages/core/test/external-file-drag.test.ts` (11 cases).
- Web adapter: `packages/core/src/dom/inbound-file-data-transfer.ts`.
- Web controller: `packages/core/src/dom/drag-drop-controller.ts`; 27
  adversarial cases in `test/headless-dom/inbound-files-and-drag-out.test.ts`.
- Shared Rust: `packages/render/src/drag_drop.rs`
  (`inbound_file_target`, `file_export_source`),
  `packages/contracts/node/src/drag.rs`.
- GPUI: `packages/gpui/node-backend/src/drag.rs`; six mounted regressions in
  `packages/gpui/preview/tests/headless_regressions.rs`.
- Frameworks: `DragDropProvider` in both, `ExternalFileSurface` specimens and
  their four mounted tests each.
- Headless engines: `test/drag-drop/files.{html,ts}` and the new leg in
  `test/drag-drop/probe.ts` — nine checks per engine, green on Chromium and
  WebKit.
- Docs: spec 069 (drag-out and inbound sections rewritten to the delivered
  contract), core/Svelte/React READMEs, GPUI adapter README, GPUI developer
  guide.

## Boards

`effigy ci:web`, `effigy ci:rust`, `effigy ci:native`, `effigy docs:check`,
`effigy test:drag-drop-browser` (both engines), and one final `effigy qa`. No
`*-windowed`, native visual, or Jetstream selector was run.

## Accepted limits

- **OS and DAW acceptance is not automated.** Playwright cannot originate an
  operating-system file drag into a page, and no API reports what a desktop
  did with an exported one. The probe states both limits where it runs, and
  neither is simulated by a component callback.
- **A declined export ends the gesture on GPUI and falls back on the web.**
  The web pointer sensor still owns a gesture whose host declined, so it
  re-enters an ordinary local session; the GPUI press handler has already
  returned by then. This is the same asymmetry `g16.026` landed for the
  cross-window bridge, kept rather than forked.

## Ledger

Unchanged. No cell moved, and no component gained or lost evidence.
