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

- **The existing abort channel is reused, not renamed.** Both host bridges
  abandon requests for the same reasons, so the export bridge takes
  `CrossWindowAbort` / `CrossWindowCleanup` exactly as `g16.026` landed them.
  The name reads oddly for a file materialization; a rename would have been a
  breaking change to a merged public API that this card does not authorize,
  and a second channel would be a second place for the idempotence rule to
  drift. See review round 1.

- **Every observed batch reaches exactly one release.** Ownership is the whole
  inbound contract: a batch this window ignored — because a local gesture had
  the controller, because another batch was already in flight, because its
  bridge had been replaced, because the surface was gone — would leave a host
  holding material for a gesture nobody will ever finish. Refusing is still an
  answer. Symmetrically, a repeat of an id already owned is one observation
  rather than two, and news for a released batch can neither commit nor
  cancel.

- **A release ends an id, not one observation of it.** Live-only duplicate
  protection is not enough: a host that re-publishes `entered` for a batch
  that already committed or was refused looks exactly like a new drag, so it
  would open a second session and release the same id twice. Each installation
  keeps a bounded tombstone of the ids it has answered. Scoped to the
  installation, because an id is one host's own name for something — a
  replacement bridge, or a reconnect, may legitimately reuse the same opaque
  text. See review round 2.

- **Inbound batches carry a protocol version, checked first.** An adapter
  ships separately from Poodle and can be pinned to an older release. A batch
  whose shape this build cannot fully understand is refused before any other
  field is read, because none of them is trustworthy yet — the same strictness
  the cross-window receipt already has.

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
| A finished id cannot resurrect | a re-published `entered` for a committed *and* for a refused id opens nothing and releases nothing, in web and GPUI |
| A replacement host may reuse an id | the tombstone is the installation's: a new bridge, or a reconnect, takes the same opaque text as a new batch |
| Every observed batch is answered exactly once | busy window, second batch, repeated id, post-disconnect news, and stale news after a bridge swap — web and GPUI |
| Replacement ends the outgoing session | asserted at the moment of replacement, before any frame, so no end-of-frame sweep can stand in for it |
| A consumer projection that throws ends the drag | the exception does not escape the drop listener and the controller returns to idle |
| A foreign protocol version is refused first | pure TS and Rust cases, plus mounted web and GPUI: the consumer resolver is never called |
| Installation survives a synchronous or throwing `start` | the subscription is closed rather than stored on a dead transaction; a start exception stays visibly failed |
| Every export terminal has its own accessible wording | mounted GPUI: ended, cancelled, failed, and declined, each proved against the exact string |
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

Review round 1 added eleven more, checked the same way:

9. the busy-window refusal, by ignoring the batch again — the host is left
   holding it;
10. the same-id guard, by answering a repeat — one batch, two releases;
11. the post-disconnect answer, by returning silently;
12. the throwing projection, by letting the exception escape the drop
    listener — the controller stays dragging forever;
13. the synchronous-terminal install, by storing the returned cleanup
    unconditionally — the subscription is never closed;
14. the failure-preserving release, by letting it overwrite `failed` with
    `cancelled`;
15. the protocol check, removed from the TypeScript validator;
16. the protocol check, removed from the Rust validator — the pure paired
    cases and the mounted GPUI case both fail;
17. the GPUI export announcement, by passing `None` for the export state —
    "Cancelled moving Alpha." comes back instead of "Finished exporting
    Alpha.";
18. the GPUI busy refusal, by ignoring the batch again;
19. the GPUI stale-news attribution, by dropping the message instead of
    answering through its publishing bridge; and
20. the GPUI replacement terminal, by releasing without ending the session.

One proof was vacuous on the way and was fixed: the replacement case first
asserted the phase *after* the next frame, where the end-of-frame sweep closes
the stranded session anyway and the assertion passed with or without the
repair. It now asserts the phase at the moment of replacement, which no sweep
can reach — and the web controller, which has no sweep at all, is the reason
that distinction matters.

Review round 2 added five more:

21. the web replay guard, by taking a re-published `entered` for an id this
    installation already answered — a second session opens over one batch;
22. the web answer-once guard, by releasing whenever asked — the
    post-disconnect repeat answers three times for one batch;
23. the web tombstone's scope, by never clearing it at connect — a reconnected
    surface can no longer take an id the previous installation used;
24. the GPUI replay guard, the same way as 21; and
25. the GPUI generation key, by making the tombstone generation-blind — a
    replacement host can then never reuse an id the outgoing one finished,
    which is the opposite error and just as wrong.

Round 2's finding was a real hole, not a documentation gap: the round 1
duplicate protection only covered an id while it was *live*, so a host that
re-published `entered` after a commit or a refusal started a fresh session and
released the same id a second time. The round 1 tests named "cannot
resurrect" only replayed `dropped` and `cancelled`, which the live check
already handled — they could not have caught it.

## Review round 1

The orchestrator required changes on `de4358a7d`. Five blocking findings, all
closed here. The `PreparedFileExport.form` / `fileCount` extension was
accepted as landed.

1. **Inbound batches did not have exact host terminal ownership.** Four holes,
   all the same shape — a batch this window observed and never answered.
   Busy-window `entered` was silently dropped; a GPUI bridge replacement
   released the receipt without ending the semantic session it was under;
   stale queued news carried only a generation, so it could not be answered
   through the host that published it; and a consumer projection that threw
   inside the DOM adapter's `drop` left the controller dragging a batch that
   could never be dropped. Now: every observed batch reaches exactly one
   release, a repeated id is one observation, replacement ends the outgoing
   session, stale news carries its publishing bridge, and a throwing
   projection ends the drag cleanly.

2. **The native runtime did not project the export lifecycle.** GPUI's
   announcement event had no export state, so an ordinary drag-out ending was
   announced as "Cancelled moving" — the exact lie the web wording exists to
   avoid, told only to assistive technology. The state now travels on the
   native announcement seam, and ended, declined, cancelled, and failed each
   have their own wording, proved by mounted evidence. The web gained the
   matching `Cancelled exporting` case for parity.

3. **Web export start was neither reentrancy- nor failure-safe.** A host that
   answered inside `start` had its cleanup stored on a transaction nobody
   would release again — the subscription stayed installed forever. A `start`
   that threw set `failed` and was then overwritten with `cancelled` by the
   release that followed, so the surface showed the wrong thing. Both fixed,
   with exact stop/cancel accounting.

4. **The required inbound protocol check was absent.** The card and spec both
   require protocol validation before eligibility, and neither language had a
   version to check. `INBOUND_FILE_PROTOCOL_VERSION` is now paired, stamped by
   the DataTransfer adapter, and refused first in web, GPUI, and the pure
   tests.

5. **An unapproved breaking Rust migration.** This card renamed
   `CrossWindowAbort` / `CrossWindowCleanup`, landed publicly by `g16.026`,
   without an operator decision. Reverted: both names and their module are
   exactly as merged, the export bridge reuses that channel, and no alias
   exists.

## Review round 2

The orchestrator closed four of the five and found one bounded hole left in
the first: **a released inbound id could still resurrect**. Duplicate
protection covered an id only while it was live, so a host that re-published
`entered` after that batch committed or was refused opened a fresh session and
released the same id twice. The round 1 tests labelled "cannot resurrect"
replayed only `dropped` and `cancelled` — news the live check already
handled — so they could not have caught it.

Each installation now keeps a bounded tombstone of the ids it has answered,
consulted before an `entered` is taken and written by the single path every
terminal and refusal goes through. It is scoped to the installation — the
subscription on web, the generation on GPUI — so a replacement host may
legitimately use the same opaque text. Both errors are proved: a replay that
must do nothing, and a replacement that must work.

## Evidence

- Contracts: `packages/core/src/external-file-drag.ts`,
  `packages/contracts/headless/src/external_file_drag.rs` (9 Rust cases),
  `packages/core/test/external-file-drag.test.ts` (12 cases).
- Web adapter: `packages/core/src/dom/inbound-file-data-transfer.ts`.
- Web controller: `packages/core/src/dom/drag-drop-controller.ts`; 36
  adversarial cases in `test/headless-dom/inbound-files-and-drag-out.test.ts`.
- Shared Rust: `packages/render/src/drag_drop.rs`
  (`inbound_file_target`, `file_export_source`),
  `packages/contracts/node/src/drag.rs`.
- GPUI: `packages/gpui/node-backend/src/drag.rs`; twelve mounted regressions
  in `packages/gpui/preview/tests/headless_regressions.rs`.
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
