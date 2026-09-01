# Nested Menu Pointer Intent Translation Memo

Status: delegate recommendation — ready for orchestrator intake
Created: 2026-09-01
Promotion authority: Poodle Northstar orchestrator
Implementation status: held pending the gates in this memo

This is the single planning packet requested by [`20260901-230402-menu-pointer-intent-translation.md`](../handoffs/20260901-230402-menu-pointer-intent-translation.md). It translates the completed nested-menu pointer-intent dossier into a reviewable recommendation. It is not a contract amendment, roadmap change, implementation authorization, or merge authority.

No operator response is requested in this delegate lane. The handoff has already settled the memo format and the architectural boundary. The items below are recommendations and promotion gates for the orchestrator to accept, revise, or reject during the normal review loop.

## Decision in one line

Compose the existing overlay geometry, dismissal, recursive ownership, and focus semantics. Extend the private menu-surface pointer path with a placement-aware corridor and a bounded close grace only after recursive Svelte, React, and GPUI paths have been admitted and proven equivalent. Do not add a pointer-history API, a second geometry registry, a public dwell/timing prop, a Menubar corridor, or a web-only parity waiver.

## Authority and current evidence

The recommendation is bounded by these sources:

- [`docs/research/value-tracks/nested-menu-pointer-intent.md`](../research/value-tracks/nested-menu-pointer-intent.md) — completed dossier, external evidence, runtime trace, proposed law, and promotion gates.
- [`docs/contracts/components/menu.md`](../contracts/components/menu.md) — recursive Menu behavior, geometry relay, placement, dismissal, focus, and GPUI notes.
- [`docs/contracts/components/context-menu.md`](../contracts/components/context-menu.md) — recursive ContextMenu behavior, virtual-point invocation, dismissal, and focus restoration.
- [`docs/contracts/components/menubar.md`](../contracts/components/menubar.md) — immediate top-level switching; cascading submenus beyond one level are out of scope.
- [`docs/contracts/002-anchored-overlays.md`](../contracts/002-anchored-overlays.md) — portal, collision, geometry, recursive ownership, and dismissal boundaries.
- [`docs/contracts/001-working-rules.md`](../contracts/001-working-rules.md) and [`docs/specs/003-accessibility-and-assistive-technology-baseline.md`](../specs/003-accessibility-and-assistive-technology-baseline.md) — cross-runtime contract and accessibility authority.
- [`docs/specs/004-overlay-focus-dismissal-and-layering-rules.md`](../specs/004-overlay-focus-dismissal-and-layering-rules.md) — keyboard, dismissal, layering, and focus-restoration requirements.
- [`docs/roadmaps/g16/parity-evidence-ledger.md`](../roadmaps/g16/parity-evidence-ledger.md) — current evidence posture and missing named mounted regressions.

The dossier's current implementation trace is material to scope:

- Svelte has recursive `MenuSurface` rendering and immediate child/sibling pointer handling. Its nested surface is in the parent item tree, while root surfaces use the existing anchored/portal path.
- React `MenuSurface`, `MenuItem`, and `ContextMenu` are currently flat. They do not yet provide the recursive child path needed for a pointer-intent implementation.
- Rust `MenuEntry`, `MenuSpec`, `ContextMenu`, and `poodle-render` are currently flat. GPUI has no generic pointer-coordinate/history stream; existing mouse-move paths are tied to specific interactions.
- The current GPUI preview is Poodle-rendered through the poodle-render/node backend. No current evidence establishes an OS-owned menu bridge or mounted native assistive-technology proof.
- The ledger has focused runtime/specimen evidence but no named mounted recursive Menu or ContextMenu regression across the active cohort.

## Recommended behavioral law

These are the exact defaults proposed for promotion. They are component-private behavior, not public API or design-token commitments.

### Eligibility and timing

| Case | Recommendation |
| --- | --- |
| Mouse pointer enters a submenu parent | Open the child immediately: `openGrace = 0 ms`. Preserve the existing immediate hover-to-open behavior. |
| Eligible mouse pointer leaves an active submenu parent toward its visible child | Keep the active branch only while the directional corridor remains valid and until one hard deadline: `closeGraceCap = 300 ms` from the parent-exit sample. |
| Pointer samples inside the corridor | Do not reset or extend the 300 ms deadline. A sample can preserve the branch, never make it sticky indefinitely. |
| Child entry | Switch to or focus the child immediately according to the existing component semantics. |
| Sibling, sibling submenu, opposite direction, unrelated item, outside ancestry, or outside the menu | Close or switch immediately. These paths bypass the grace deadline. |
| Keyboard, touch, pen, Escape, focus movement, and semantic dismissal | No pointer grace. Existing immediate keyboard, focus, and dismissal rules remain authoritative. |

The 300 ms cap is a safety bound, not a dwell delay. There is no second safety timer, consumer-configurable delay, motion timer, or `menuTransition` timing token. The cap is measured from the original parent-exit sample using a monotonic clock and is never extended by sampling, geometry updates, or re-entry into another branch.

### Corridor and direction rule

Use a private pure geometry helper with these fixed inputs and defaults:

- the latest parent-row rect;
- the latest visible child-surface rect;
- the latest resolved physical placement (`right`, `left`, `top`, or `bottom`), including collision flips and RTL resolution;
- the parent-exit point and latest pointer samples with monotonic timestamps;
- a fixed `0.5 CSS px` boundary buffer; and
- a fixed `0.1 CSS px/ms` forward-intent threshold.

The helper constructs the shortest placement-aware convex corridor from the parent exit edge toward the child's near edge. It must handle right-, left-, top-, and bottom-facing children, RTL-resolved placement, collision-flipped placement, offsets, gaps, and the actual current rectangles. It must not assume that a child is always to the right.

Retain the branch only when the current sample is inside the buffered corridor and its latest motion has sufficient forward projection toward the child's near edge (`>= 0.1 CSS px/ms`). A zero, reverse, opposite-direction, or out-of-corridor sample invalidates the hold and causes the normal close/switch path. The threshold is a direction-confidence floor, not permission to create a general speed-based dwell; the evidence plan must explicitly cover slow, stopped, turning, and reversing traces.

These values are clean-room behavioral defaults informed by the pinned Floating UI evidence in the dossier. Do not import its helper, source, 40 ms safety timeout, or API shape. Poodle has one semantic deadline: the 300 ms cap above.

### State boundaries

The grace may defer only the visual branch-close decision for an eligible active mouse submenu. It must not defer or suppress:

- outside dismissal or `layerContains` ancestry checks;
- Escape, ArrowLeft, focus exit, or other keyboard dismissal;
- activation, selection, disabled-item behavior, or checked/radio state;
- focus movement or focus restoration;
- ARIA state updates required by the current semantic transition; or
- touch/pen activation and any non-hover input.

The behavior is therefore a translation of pointer intent, not a new semantic state machine. The existing Menu/ContextMenu owner remains responsible for the machine, dismissal stack, focus, and restoration.

## Ownership and implementation boundary

### Web

After recursive React admission, add one private shared web geometry utility in the core package (candidate location: `packages/core/src/menu-pointer-intent.ts`, not a public barrel export). It should be pure numeric code: rectangle normalization, placement-aware corridor construction, sample projection, deadline comparison, and invalidation predicates. It must not read the DOM, own timers, focus nodes, dispatch pointer-intent events, or mutate menu state.

Svelte and React `MenuSurface` own the lifecycle around that helper:

- capture the parent-exit sample and deadline;
- request the current child/parent geometry;
- collect eligible mouse samples through the runtime's existing host path;
- call the helper and choose the existing close/switch operation;
- cancel on child entry, branch switch, dismissal, unmount, or invalidation; and
- preserve the existing active-item, ARIA, focus, and recursive ownership behavior.

`Menu.svelte`, `Menu.tsx`, `ContextMenu.tsx`, the dismissal layer, and `menuTransition` remain owners of their current semantic responsibilities. `surface-geometry.ts`, `observeOverlaySurfaceGeometry`, `anchored.ts`, `AnchoredSurface`, and the collision resolver remain the geometry/placement sources. The public web-only geometry callback remains an observation relay; it is not a pointer-history or helper-control channel.

### Menu, ContextMenu, and Menubar

- Menu owns the nested rule through its recursive `MenuSurface` tree.
- ContextMenu reuses the same nested rule. Its trigger/virtual-point anchor, invocation origin, consumer-owned triggerless focus target, overlay, and restoration behavior remain separate concerns.
- Menubar keeps immediate top-level hover-to-switch with `openGrace = 0 ms` and `closeGrace = 0 ms`. It receives no corridor. Deeper Menubar submenus require a separate contract decision and evidence lane.

### Native

Rust cannot execute the web helper. Once recursive native menu composition is admitted, `poodle-render` owns the semantic recursive tree and the GPUI host/native adapter owns event collection, coordinate conversion, hit-testing, and private sample delivery. Any Rust geometry helper must be an equivalent private implementation of the same numeric law, not a second public contract or a browser-like DOM abstraction.

The component owns the semantic result. The GPUI host owns the mechanism needed to observe real pointer coordinates and monotonic timestamps, including movement after the pointer leaves an individual row while the active branch is still eligible. No generic pointer-history surface is added to public `Interaction` or to the component contract merely to support this feature.

## Geometry freshness and collision invalidation

The corridor is valid only against the latest visible geometry for the active branch.

1. On parent exit, snapshot the current parent row, child surface, resolved physical placement, exit point, and the 300 ms deadline.
2. On any placement, rect, visibility, clipping, scroll, resize, collision flip, or relevant mount/unmount change, invalidate the old corridor.
3. If the same branch remains mounted and the newest parent/child geometry is visible, positive-area, finite, attached to the current surface, and has a valid near edge, recompute the corridor from fresh geometry without extending the original deadline.
4. If geometry is hidden, detached, zero-area, non-finite, clipped beyond the supported policy, removed, or lacks a valid near edge, fail closed: cancel the hold and run the existing close path. Never retain a stale corridor indefinitely.
5. A collision flip must use the newly resolved physical placement and current rect. A right-facing assumption must never survive a left/top/bottom flip.

This composes the existing anchored resolver and geometry observer. It does not create a second overlay registry, portal, dismissal root, geometry store, or consumer-facing query API. Nested flyouts continue to use their existing parent-tree ownership; the helper consumes their observed/current geometry without taking ownership of the surface.

## Admission prerequisites

Pointer-intent behavior is not ready for implementation or promotion until all of these prerequisites are true:

1. React adds the recursive `MenuItem.children` and recursive `MenuSurface` path for both Menu and ContextMenu, with the existing semantic roles, sibling switching, leaf behavior, focus, dismissal, and geometry relay preserved.
2. Rust `MenuEntry`, `MenuSpec`, ContextMenu composition, and `poodle-render` admit the same recursive semantic tree. The current flat native types and render path cannot be counted as equivalent evidence.
3. Svelte and React have the same named mounted recursive fixture, with depth 1/2/3, sibling and sibling-submenu branches, collision placement, scroll/resize, clipping, and unmount cases.
4. GPUI has a named host-owned sample/hit-test path and a production-path headless probe. A direct helper unit test is insufficient.
5. The existing Menu/ContextMenu focus-restoration contradiction is resolved in the owning contract before implementation. The current machine text says closing does not restore trigger focus, while the accessibility/overlay tables require restoration for Escape/submenu close. No implementation may silently select one.
6. The active cohort remains Svelte, React, and GPUI. Jetstream remains program-deferred and receives no per-component waiver or parity claim from this packet.

The current Svelte recursion is useful evidence, but it does not discharge the React/Rust admission or the named mounted cross-runtime proof. The current focused ledger rows and GPUI specimen evidence are prerequisites for the next probe, not proof of this behavior.

## Evidence and acceptance plan

The analytical traces in the dossier are directional evidence only. Promotion requires named mounted runtime evidence through the production paths.

### Browser fixture

Build one equivalent recursive Menu fixture and one equivalent ContextMenu fixture in Svelte and React after recursion is admitted. Exercise the actual surfaces, collision resolver, geometry observer, dismissal layer, focus path, and runtime host sampling. Record results separately for each runtime; do not collapse them into a helper-only result.

The fixture matrix must cover:

| Dimension | Required cases |
| --- | --- |
| Placement | right, left, top, bottom, RTL-resolved, collision-flipped |
| Motion | slow, medium, fast, stop, turn, reverse, direct child entry |
| Density/geometry | low and high density, short/tall rows, offsets, gaps |
| Viewport | interior, edge, clipped, scroll, resize, hidden, detached, unmount |
| Depth | one, two, and three nested levels |
| Branches | target child, sibling, sibling submenu, opposite direction, leave menu/window |
| Input | mouse, touch, pen, keyboard, Escape, ArrowLeft, ArrowRight, Enter, Space, focus restoration |
| Motion policy | full, reduced, frozen |

Capture at least these metrics by direction, speed, density, viewport, and depth:

- accidental close before intended child entry;
- sticky wrong branch after sibling, reverse, outside, or opposite-direction movement;
- time to child entry;
- false hold after stop/turn/reverse; and
- geometry invalidation or stale-branch events.

The canonical behavior target is zero accidental-close on target-directed paths and zero sticky-wrong outcomes on qualifying sibling/outside/opposite paths. Any failure is a promotion stop or a documented change to the proposed law; aggregate pass rates must not hide a failing direction, speed, placement, or depth.

Run the existing focused web and accessibility selectors that cover the mounted fixtures, including `effigy test:a11y` where the fixture is wired into the accessibility path. Add a named mounted regression for each recursive runtime before treating the evidence as durable.

### GPUI evidence

The GPUI probe must traverse the production host path:

- use the existing headless driver and `PlatformInput::MouseMove` path to deliver real coordinates and monotonic sample times;
- exercise host hit-testing across the recursive parent/child surfaces, including movement outside the row hitbox while the branch remains active;
- prove open, diagonal hold, child entry, sibling switch, reverse close, outside close, depth 1/2/3, collision/reposition invalidation, unmount cancellation, and no leaked timers/observers;
- prove ArrowRight, ArrowLeft, Enter, Space, Escape, active item, and focus restoration through the native path; and
- use the existing headless routes such as `effigy regressions:native` and `effigy probe:gpui-specimens` as the validation surface.

Do not run `*-windowed` selectors locally. A GPUI helper unit test, a flat `MenuEntry` render, or a visual screenshot is not host evidence. The current GPUI 0.2.2 capability record does not establish content assistive-technology proof; if native semantics or focus behavior cannot be proven for the active host, hold parity rather than granting a native exception.

### Promotion oracle

The orchestrator should promote only when all of the following are demonstrated:

- recursive Menu and ContextMenu semantic paths exist in Svelte, React, and GPUI;
- the exact `0 ms` open behavior, `300 ms` hard cap, `0.5 CSS px` buffer, and `0.1 CSS px/ms` forward-intent rule pass the mounted matrix, or an explicitly reviewed replacement is recorded;
- sibling, outside, opposite, reverse, and invalid-geometry paths close or switch without sticky state;
- fresh resolved placement and current rects govern every retained corridor, with no stale hold after collision, scroll, resize, clipping, hidden state, or unmount;
- keyboard, focus, dismissal, touch, pen, disabled, separator, check/radio, and leaf semantics remain unchanged and equivalent;
- full, reduced, and frozen motion policies produce the same semantic result and the pointer deadline is not tied to visual motion;
- named browser and GPUI mounted regressions exercise the real host paths;
- the Menu/ContextMenu focus-restoration contradiction is resolved in contract ownership;
- GPUI host evidence is sufficient for the semantic contract, with no unsupported AT claim; and
- the licensing record is complete and no unreviewed source, dependency, vendored asset, or license-file change is needed.

## Accessibility and modality matrix

| Input or state | Pointer-intent eligibility | Required result and proof |
| --- | --- | --- |
| Mouse diagonal parent → child | Yes, only for the active visible submenu | Immediate child open; corridor may defer branch close for at most 300 ms; child entry is immediate; named mounted proof. |
| Mouse to sibling or sibling submenu | No retained hold once the sibling is the target | Immediate close/switch; no sticky old branch; active-item and ARIA updates remain correct. |
| Mouse reverse, opposite direction, outside menu/window | No | Existing close/dismiss path runs immediately; outside ancestry remains authoritative. |
| Touch | No hover grace | Existing activation semantics remain immediate and usable; no invisible pointer corridor or hover-only state. |
| Pen | No hover grace | Same as touch unless the host's existing pen activation contract explicitly says otherwise; no hover delay. |
| Keyboard trigger/context invocation, ArrowRight, Enter, Space | No | Immediate open/activation, correct active item and focus, `aria-haspopup`/`aria-expanded`, and recursive child semantics. |
| ArrowLeft and Escape | No | Immediate close according to the resolved Menu/ContextMenu contract, with required focus restoration and no pointer timer interference. |
| Assistive technology / semantic state | No | Native/web equivalent menu and menuitem meaning, disabled/separator/check/radio/leaf semantics, no focus trap, and no delayed semantic update. Web axe/keyboard/AT evidence is required for each recursive runtime; current GPUI AT capability is a gate, not a waiver. |
| Outside pointer dismissal | No | Existing `layerContains`/dismiss-stack behavior remains unchanged; grace cannot keep an outside branch alive. |
| Full, reduced, frozen motion | No semantic difference | Same open/close/focus/ARIA result in all policies; frozen has no visual clock; no semantic timer is owned by motion. |
| Menubar top-level hover | Excluded | Immediate top-level switching remains `0/0`; deeper behavior is a separate contract lane. |

The implementation must not insert an invisible corridor element, block pointer events, create a focus trap, suppress outside ancestry, or use visual motion to hide semantic timing. The corridor exists only as private computation over host samples and current geometry.

## Licensing and attribution treatment

The dossier's external record was retrieved and pinned on 2026-09-01:

| Source | Treatment |
| --- | --- |
| [Rauno Web Interface Guidelines pinned README](https://raw.githubusercontent.com/raunofreiberg/interfaces/81f523a5b469ba1ea877fef262588f3b4b65d31f/README.md) | Reference/paraphrase only. The pinned repository does not provide an explicit license or package metadata in the reviewed surface; copy no text, assets, or code. |
| [WAI-ARIA APG menubar/menu-button](https://www.w3.org/WAI/ARIA/apg/patterns/menubar/) | Use as normative behavior guidance and paraphrase. Copied W3C text requires the W3C Document License 2023 notices. |
| [Floating UI `safePolygon`](https://raw.githubusercontent.com/floating-ui/floating-ui/27629b74ba36ab8ceb2a968051927b9b69511a3b/packages/react/src/safePolygon.ts) | MIT. Use the behavioral idea as clean-room input; do not import the helper, API, or source. If substantial code is ever adapted, preserve the required notice in a separately reviewed change. |
| [Radix menu implementation](https://raw.githubusercontent.com/radix-ui/primitives/f7ecd5ab16f5e1e820eb5786a1419a98a2d594ae5/packages/react/menu/src/menu.tsx) | MIT. Record as behavioral comparison; no source copy or dependency change in this lane. |
| [Ant Design Menu documentation](https://raw.githubusercontent.com/ant-design/ant-design/c046a8f10eb0610c125af6c00a34c021308b8bec/components/menu/index.en-US.md) | MIT. Record the delay-only comparator; no source copy, vendoring, or dependency change. |

Prefer a clean-room implementation from the Poodle contract and numeric law. Any future adaptation, copied documentation, dependency, vendored code, notice, or license-file change requires its own review. This packet makes no such change.

## Stop conditions and rejected paths

Stop implementation or promotion and return to the canonical planning/review path if any of these occur:

- React or Rust remains flat, or the proof uses a synthetic tree that the runtime cannot mount;
- GPUI evidence stops at a helper test, screenshot, or preview-only claim without host sampling and hit-testing;
- the current resolved placement or fresh rect is unavailable, stale, invalid, clipped, detached, or unmounting;
- the corridor needs a public pointer-history event, public dwell prop, global registry, second portal/dismissal channel, or consumer timing controller;
- the 300 ms deadline can be extended, leaked, or coupled to visual motion;
- keyboard, focus restoration, outside dismissal, ARIA, touch, pen, disabled, separator, check/radio, or leaf behavior changes;
- the Menu/ContextMenu focus-restoration contradiction remains unresolved;
- a web-only parity waiver, GPUI native exception, or unsupported AT claim is proposed;
- Menubar receives the recursive corridor without its own contract and evidence;
- the mounted matrix shows accidental close, sticky wrong-branch, direction, density, depth, invalidation, or timing failures hidden by aggregate metrics;
- a source license or attribution boundary is ambiguous; or
- the work expands into contract, roadmap, architecture, release, windowed-selector, or unrelated cleanup changes without a separate decision.

The following alternatives are rejected for this lane:

- immediate close alone: the dossier's model loses all target-directed traces;
- fixed open/close delays: the model retains accidental wrong-branch outcomes and makes unrelated sibling movement wait;
- verbatim Floating UI, Radix, or Ant implementation/import: wrong ownership and licensing boundary;
- global pointer history, a public pointer-intent event, or a consumer-configurable dwell token: expands the contract for one private behavior;
- a Menubar corridor: conflicts with its immediate top-level contract and current scope;
- an invisible overlay or pointer-events block: changes hit-testing and accessibility rather than translating intent;
- hover grace for touch or pen: modality-inappropriate; and
- native/OS menu behavior as an unproved parity exception: current GPUI evidence does not support it.

## Promotion route

Keep this packet as the intake/recommendation artifact. The orchestrator should make any contract or roadmap edits in a separate promotion batch after review; this PR does not update those authorities.

Recommended sequence:

1. Resolve the existing Menu/ContextMenu focus-restoration contract contradiction.
2. Admit recursive React and native semantic paths, including named mounted fixtures and the missing geometry/focus/dismissal proofs.
3. Add the GPUI host-owned sampling/hit-test path and its production-path headless regression.
4. Implement the private web helper and equivalent native adapter behavior against the exact law above.
5. Run the full bounded matrix, inspect per-dimension metrics and invalidation/lifecycle results, and review accessibility/licensing evidence.
6. Promote the observable rule into the owning component contract only after all gates pass. Keep Menubar and Jetstream on their current boundaries.

The implementation remains on hold until that promotion review accepts the prerequisites and gates. The next responsible action after this packet is orchestrator review and routing; no merge is part of this delegate lane.
