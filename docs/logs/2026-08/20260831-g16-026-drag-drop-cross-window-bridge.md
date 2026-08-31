# g16.026 — Drag-And-Drop Cross-Window Bridge, Tabs, And DockRegion

Status: complete — merged in PR #113 after Northstar review rounds 1-2
Date: 2026-08-31
PR: https://github.com/inflatable-cookie/poodle/pull/113
Card: `docs/roadmaps/g16/026-drag-drop-cross-window-bridge-and-dock-region.md`
Handoff: `docs/handoffs/20260831-145018-g16-026-cross-window-drag-bridge.md`
Governing refs: `docs/architecture/011-drag-and-drop-substrate.md`,
`docs/specs/069-dependable-drag-and-drop-substrate.md`,
`docs/contracts/components/tabs.md`,
`docs/contracts/components/dock-region.md`
Branch: `codex/g16-026-cross-window-drag-bridge`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-026-cross-window-drag-bridge`

## Outcome

One semantic drag transaction now spans local web, same-application
cross-window, and GPUI. The split source/window bridge, the bounded opaque
receipt, the Tabs and DockRegion migrations, the Tabs subject-family seam, and
the window-owned GPUI provider census all landed.

Deleted with no alias, wrapper, or fallback: `ReorderState`,
`createReorderState`, `handleDragStart`, `handleDragOver`, `handleDrop`, the
DOM-shaped `tabs-reorder.ts` module and both framework re-export files, every
`DockExternalDrag*` / `DockExternalDrop*` type,
`createDockExternalDragController`, `dockPanelDragSession`, and the
`application/x-poodle-panel-drag` wire. `PanelDragData.sourceZone` is required
and its older-build fallback is gone.

## Decisions worth keeping

- **`HOST_TERMINAL` is the one kernel addition.** A local drop reaches its
  result through `DROP_REQUESTED` because the adapter holds the live target and
  can revalidate it. A host transaction has no local target: the drop happened
  in another window, and the ordinary path can only degrade a host refusal from
  `dragging` into a cancellation. That would lose the refusal *and* its reason,
  which the oracle forbids. One event, no new phase, no new effect, the same
  session-id and phase inertia as everything else, mirrored in Rust and pinned
  by six shared vectors.

- **`draggable` is the advertisement, so it is gated on the armed receipt.**
  Chromium takes a native drag the instant the attribute appears. Advertising
  from the registration meant a slow host lost the gesture entirely — the
  browser started and we refused, and the user had to release and try again.
  The headless probe found this; the unit tests could not have.

- **The web's only real cross-window transport is the browser's own drag.** A
  page cannot observe a pointer in another window, so mouse and pen hand over
  at `dragstart` with the receipt in a bounded envelope. `window-capture` is
  left for touch, where a host advertising the capability is claiming an
  out-of-window observation the page itself does not have.

- **A native end is not a result.** `dragend` closes the transport and nothing
  else; the host's terminal subscription is the sole authority. This is why
  `HOST_TERMINAL` exists at all.

- **A decline kills the transfer, not the drag.** The gesture is held across
  the kernel's cancellation — hold timer included, so a declined touch source
  does not also lose the hold it was in the middle of — and re-enters the
  ordinary local lifecycle with a fresh session that bypasses the bridge.

- **A window's provider census is a field on the host, not a static.** The
  g16.025 attempt used a thread-global sweep mark, so rendering window A swept
  controllers owned by window B. A census that belongs to one host can only
  name that host's own controllers, so A's frame has nothing of B's to look at.

- **Semantic idle is not enough on GPUI.** A terminal reached with no provider
  left has no controller host to drain `pending_stop_active_drag`, so the sweep
  runs from a paint canvas that reaches a `Window` and calls `stop_active_drag`
  before forgetting the controller.

- **Tabs preserves its reorder *result*, not just its reorder.** A dropped tab
  has always landed *at* the tab it was dropped on, whichever half the pointer
  was over. The whole tab is one band, and which side it resolves to depends on
  where the dragged tab started. Splitting it into halves would have changed
  the order for the same gesture.

- **A disabled tab cannot be picked up and is still a place to put one.** It
  occupies an index; a reorder that could not pass through it would be a
  different result. Hence the source is gated on `disabled` and the target is
  not.

- **Semantic family and registration namespace are different things.**
  `dragSubjectKind` says who may consider a row; `sourceId` / `targetId` say
  which registration it is. Two strips under one controller may legitimately
  hold the same values, and duplicate live ids are an error rather than
  last-writer-wins.

- **A shared family must refuse a foreign subject during *eligibility*.**
  `resolveDropTarget` discards rejected candidates, so refusing there lets an
  eligible ancestor composite win. Claiming the drop and rejecting at commit
  would swallow it.

- **DockRegion's panel identity travels in `DragSubject.id`.** HTML5 hides the
  `DataTransfer` body during `dragover`, which is the only reason a module
  global was ever needed to answer `canAcceptPanel` at hover. On the substrate
  the subject is part of the session. The encoding is percent-encoded fields,
  not JSON, because it becomes part of generated DOM ids — and every public
  boundary decodes it back.

- **`crossWindowDropTarget` throws on a region that joined a provider.** The
  provider is the window; a window bridge belongs there. A silent no-op would
  be exactly the kind of quiet fallback the working rules forbid.

## Review oracle

| Adversarial case | Proof |
| --- | --- |
| One receipt, one session | `cross-window-drag-bridge.test.ts` — a superseded preparation is cancelled once, its late receipt is handed back, and it cannot arm the successor |
| Native end is not host commit | same file, plus the headless probe on both engines: `dragend` reports `move`, no terminal runs, the host's `rejected` is the result |
| Drop revalidates live authority | target disabled between projection and drop — `commit` is never called |
| Wire is opaque and bounded | `cross-window-data-transfer.test.ts` (malformed, oversized, future, extra keys) plus the probe's foreign-envelope drop |
| Projection follows host geometry | host moves its target with no local pointer input; the stale position cannot commit |
| Window loss is terminal | both directions in the probe: receiving context closed, and sending window hidden with a live lease |
| Keyboard is the same transaction | picker returns a stale target; ordinary revalidation refuses it |
| Touch is capability-bound | a `touch: false` host never starts, and internal touch still arms the local session |
| Local reorder stays independent | host declines; pointer and Alt+Arrow still reorder with no native payload |
| Tabs composition does not capture a foreign subject | `TabsSubjectComposition` in both frameworks, plus the shared-Rust case |
| DockRegion has no hidden local bus | `DockRegionZoneDrop` in both frameworks — shared-provider pair cross-drops, self-provided pair does not |
| Two GPUI windows are isolated | `one_window_frame_cannot_cancel_another_windows_live_drag` |
| Provider unmount stops the real GPUI drag | `unmounting_a_provider_mid_drag_cancels_it_and_stops_the_native_drag`, asserting `App::has_active_drag` before and after |
| Clean migration is complete | absence search over nine deleted names; only prose references remain |

### Falsified

Sixteen claims were checked by planting the pre-fix behaviour back and confirming
the case fails:

1. the two-window isolation, by reintroducing a thread-global census — it
   reproduced the original defect exactly (`end:cancelled:Explicit` in the
   background window);
2. the GPUI provider-unmount cancellation, by disabling the sweep;
3. the native-drag stop *separately*, by removing only `drain_pending_stop` —
   so the two halves are independently real;
4. the foreign-subject rejection, by removing the ownership check;
5. the registration-id scoping, by minting ids from the value alone
   (`Duplicate drag source id "alpha"`);
6. the DockRegion sibling isolation, by flipping the pair onto a shared
   provider;
7. the armed-receipt gate on GPUI, by activating without waiting;
8. the host-owned terminal, by letting a release commit locally;
9. the late-receipt return, by dropping it;
10. the picker's receipt binding, by trusting any projection;
11. the abandoned-commit signal, by releasing the transaction without telling
    the host;
12. the allocating-bridge binding, by returning the late lease through the
    current source;
13. the wake path, by queueing without waking;
14. the absence of an installation probe, by planting it back;
15. the replacement teardown, by swapping the bridge without ending the
    outgoing transaction;
16. the installation-generation gate, by letting stale target news through.

One test was found to be vacuous on the way and fixed: the composition case
originally dropped onto the receiving strip's *self-rejecting* tab, so it
passed with or without the rule.

## Evidence

- Kernel: `packages/core/src/drag-drop.ts`,
  `packages/contracts/headless/src/drag_drop.rs`, six new `dragDrop` vectors.
- Bridge contracts: `packages/core/src/cross-window-drag.ts`,
  `packages/contracts/headless/src/cross_window_drag.rs`.
- Bounded codec: `packages/core/src/dom/cross-window-data-transfer.ts` and its
  test.
- Web controller: `packages/core/src/dom/drag-drop-controller.ts`;
  17 adversarial cases in `test/headless-dom/cross-window-drag-bridge.test.ts`.
- Headless multi-context: `test/drag-drop/cross-window.{html,ts}` and the new
  leg in `test/drag-drop/probe.ts` — two isolated Playwright contexts with the
  host in neither, green on Chromium and WebKit.
- Tabs: both frameworks plus `tabs-parts/`; `TabsSubjectComposition` tests.
- DockRegion: both frameworks plus `dock-region-parts/`; `DockRegionZoneDrop`
  tests and the `DockRegionZoneDropHarness`.
- Shared Rust: `packages/render/src/drag_drop.rs` (family builders, dock panel
  subject codec), `packages/render/src/tabs.rs`,
  `packages/render/src/dock_region.rs`,
  `packages/contracts/components/src/tabs.rs`.
- GPUI: `DragDropWindowHost` in `packages/gpui/node-backend/src/drag.rs`, wired
  in the preview root and the headless driver, documented in
  `packages/gpui/adapter/README.md` and
  `docs/guides/gpui-developer-guide.md`.

## Boards

`effigy ci:web`, `effigy ci:rust`, `effigy ci:native`, `effigy docs:check`,
`effigy test:drag-drop-browser` (both engines), and one final `effigy qa`. No
`*-windowed`, native visual, or Jetstream selector was run.

## Ledger

Unchanged. No cell moved. Two evidence paths were repaired because the tests
they named were deleted by this migration (`DockRegionDragOverGate` in both
frameworks); the ledger's own generator now derives the replacements.

`src/tabs/` is named `src/tabs-parts/` in both frameworks: on a
case-insensitive filesystem the directory collided with `Tabs.tsx` in the
ledger's extension-less path resolver and was reported as drift.

## Review round 1

The orchestrator requested changes on PR #113. Two blockers, both closed here,
plus an addendum that changed the shape of the first repair.

1. **The Rust cross-window bridge was declaration-only.** Nothing outside the
   trait definitions consumed it: the GPUI controller had no bridge input,
   `DockRegionHandlers` had no seam, and native could register and locally move
   panels but could not prepare, project, revalidate, commit, or take a host
   terminal. That failed ordered work 2 and 6 and the Svelte/React/GPUI
   lifecycle acceptance row. Now wired end to end, with four host-stub
   regressions.

   The addendum then ruled that the traits could not be wired as written: my
   first Rust draft had dropped `AbortSignal` from `prepare` and `commit`, and
   dropped the receipt from `pick_target`. A completion callback may replace a
   promise; abortability and receipt identity may not disappear.
   `CrossWindowAbort` is the renderer-neutral signal — idempotent, first reason
   wins, listeners run once — and `pick_target` is bound to the exact receipt
   it is picking for.

   Wiring found three real defects, none of which the declaration could have:

   - a release over a local target **committed locally** while the host owned
     the terminal, which is precisely the inference the boundary forbids;
   - the frame sweep cancelled every incoming projection on its first frame,
     because a projected session has no local source by construction;
   - a projected drop re-hit-tested local bounds, overruling the host with
     geometry it never used.

   Two of the four new proofs were **vacuous on the first pass**: a second line
   of defence in the controller hid the rule under test, so removing the rule
   left them green. The mismatched-pick case now asserts the live transaction
   survives untouched, and the abandoned-commit case asserts the host was
   actually told to stop rather than merely ignored. Both bite now.

2. **DockRegion authority contradicted the landed API.** The contract still
   declared `sourceZone?`, still described deleted `onDragPrepare` /
   `onDragStart` / `onDragEnd`, and its delta table still named deleted
   `externalDragSource` / `externalDropTarget` and claimed the natives run no
   panel gesture — false after repair 1. All four are corrected, along with the
   same stale claim in the `poodle-render` module doc. Absence search now
   covers authoritative docs as well as active source.

## Review round 2

Three blocking lifecycle defects in the round 1 repair, plus one adjacent case
the operator folded in. All four are the same class: **authority must be
exact**.

1. **A late preparation receipt could be cancelled through the wrong bridge.**
   `CrossWindowMessage::Prepared` carried only a session id, and the helper
   that returned a stale lease read the *currently active* source instead. If A
   was superseded by B with a different host, A's lease went back through B —
   leaking A's and issuing a command B never made. The message now carries the
   allocating bridge. The round 1 test used one stub for both attempts and
   could not see this; the new one uses two.

2. **Asynchronous host answers did not wake GPUI.** Every callback appended to
   the inbox and nothing asked for a frame, so an idle window could sit in
   `Preparing` indefinitely — and the contract explicitly permits a host to
   answer whenever its lease resolves. A foreground pump now drains on the main
   thread: the `Send + Sync` half is an unbounded sender a host callback holds,
   while the receiving task holds a weak controller handle and upgrades it per
   wake through an `AsyncApp`. One `post` helper does queue-and-wake, so a
   future host answer cannot be added that queues without waking.

3. **Installing a target bridge called `pick_target` with a fake receipt.**
   That is an observable host request outside any transaction, absent from the
   TypeScript contract, and it forced implementations to special-case a token
   naming nothing. Installation now asks the host for nothing; the declared
   capability is enforced on a real keyboard pick bound to a live receipt.

4. **Replacing the window bridge stranded the outgoing transaction.**
   (Operator addition.) A was unsubscribed and B stored, but A's transaction
   stayed live — and the commit path read the controller's *current* bridge, so
   A's receipt would have gone to B. The transaction now owns the bridge that
   published it, replacement ends the outgoing transaction first, and release
   clears the stored bridge.

Two further defects were caught by operator inspection of the in-progress
round 2 diff, both in code I had just written:

5. **The wake pump was a reference cycle.** The controller owned the sender and
   the detached task owned a strong controller clone; the stream only ends when
   every sender drops, so neither could ever be released. The pump now holds
   `Rc::downgrade` and upgrades per wake, so an ordinary drop takes the sender
   with it and the task exits. This one is structural and **not** covered by a
   regression: observing task lifetime needs the window and the provider
   closure to release their own controller clones first, which the headless
   harness cannot arrange cheaply. Recorded rather than papered over with a
   test that would not actually measure it.

6. **Target news lost its publishing installation.** A projection queued by
   host A could drain after B was installed, and `apply_projection` would bind
   A's receipt to B — a B-owned transaction over a lease B never issued. Each
   installation now has a generation, the message carries it, and news from a
   replaced or unsubscribed installation is discarded whole. Release bumps the
   generation too, so nothing outstanding can be applied afterwards.

One of the four new proofs was vacuous until the stub was fixed: it still
carried an empty-token special case written for the old probe, which hid
exactly the behaviour under test. Removing that special case is also the point
of finding 3 — a host should not need it.

## Accepted boundaries

- The GPUI host bridge drains through a foreground pump installed the first
  time the controller sees an `App` — on its first frame, or when a source
  begins preparing. A controller that has never rendered has no session, so
  there is nothing to wake.
- Cross-region transfer between two DockRegions now requires one common
  `DragDropProvider`. That is the operator's decision recorded in spec 069 and
  the card, and the specimen was updated to wrap both docks; consumers with two
  provider-less regions must add one.
- Jetstream stayed at compile-only renderer-neutral maintenance, per the card.
