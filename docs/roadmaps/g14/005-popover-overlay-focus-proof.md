# g14.005 — Popover Overlay And Focus Proof

Status: complete — accepted in PR #18
Depends on: `g14.023` — accepted in PR #17
Governing refs: `../../architecture/009-cross-runtime-component-conformance.md`,
`../../specs/066-executable-component-conformance.md`,
`../../contracts/001-working-rules.md`,
`../../contracts/components/popover.md`, `conformance-estate.md`
Input evidence: `023-headless-gpui-conformance-execution.md`,
`../../logs/2026-08/14-g14-023-headless-gpui-conformance-execution.md`,
`../../../PAPERCUTS.md`

## Outcome

Prove the first complete overlay profile through the landed conformance
kernel:

```text
one Popover interface + one typed case corpus
  -> real Svelte and React overlays
  -> shared poodle-render composition
  -> real headless GPUI layer, dismissal, placement, and focus paths
  -> normalized component-observation.v1 results
  -> strict active-cohort completion
```

Runtime layer mechanisms may differ. Observable state, relationships,
dismissal, focus transfer, focus restore, event order, placement result, and
token use may not drift.

This card certifies Popover and the smallest generic overlay vocabulary it
needs. It is not a general overlay rewrite, a Modal/Menu rollout, or a reason
to expose exhaustive cases in the catalogue.

Jetstream remains program-deferred. Do not install, link, build, execute, or
write Jetstream code for this card.

## Frozen Contract

Treat `docs/contracts/components/popover.md` as shipped meaning and Svelte as
the reference when implementations disagree. Freeze these claims before
changing code:

- controlled `open` / `onOpenChange` and uncontrolled `defaultOpen`
- `trigger` and `children` composition regions
- all 12 `OverlayPlacement` values and numeric `offset`
- `dismissOnOutsideInteract`
- `initialFocus`: `first-focusable`, `content`, and `none`
- `ariaLabel`, `disabled`, `block`, `surfaceWidth`, and surface width bounds
- trigger/surface relationship, open state, dialog role/name, and token roles
- close through trigger, Escape, outside interaction, and controlled host
- focus entry after mount and trigger-focus restoration after every close path

`triggerIsInteractive` and `onSurfaceGeometryChange` are documented web-only
extensions. Keep them beside the web adapters. They do not enter the portable
interface or active-cohort parity claims.

Do not add `requestClose`, focus trapping, modal semantics, pinned panels, or
menu item behaviour. They are not in the Popover contract.

## Current Baseline

Inventory and record the exact before-state in the batch log before editing:

- `packages/core/src/popover.ts` and generated TS machine types own the
  portable transition/part helpers, but Popover has no g14 component interface
  or executable corpus.
- `packages/contracts/components/src/popover.rs` is a hand-written
  `PopoverSpec`; the accepted interface/codegen path must replace its portable
  declaration rather than sit beside it.
- `packages/contracts/headless/vectors/machines.json` has only four Popover
  machine vectors. It proves transition intent, not mounted dismissal, focus,
  placement, semantics, or layer behaviour.
- Svelte and React use the shared machine, dismissable-layer stack, anchored
  surface machinery, real DOM focus, and shared CSS. Their runtime adapters
  still require mounted conformance evidence.
- `poodle-render::popover` renders only the surface. The GPUI preview wrapper
  owns trigger composition and fixed absolute positioning, toggles only by
  click, and currently has no equivalent Escape, outside-dismiss, initial
  focus, or restore-focus path.
- `overlay.intent` is certified as deferred in the primitive capability
  roster. This card must turn the required rows into executed evidence.
- `component-observation.v1` and the governing spec name logical bounds,
  parent/layer relationships, and layer order, but the landed observers expose
  only a small geometry subset and part-local fields. Add only the generic
  observation/action vocabulary the Popover cases actually require.
- Curated Popover specimens exist in Svelte, React, and GPUI. They are teaching
  surfaces, not conformance fixtures, and must remain curated.

Recheck this baseline against current main. Record contradictions before
implementation; do not preserve a stale claim because this card says it.

## Goals

- [ ] Make one serializable `popoverInterface` the portable declaration
      authority and generate the Rust portable declaration from it.
- [ ] Author one typed Popover case corpus that executes unchanged in Svelte,
      React, and GPUI.
- [ ] Extend generic actions and observations only where the overlay profile
      cannot be expressed by the landed vocabulary.
- [ ] Execute real layer, dismissal, placement, and focus paths through each
      runtime; direct callback or state mutation is not runtime evidence.
- [ ] Reconcile the old machine vectors and hand-written declaration without
      leaving duplicate passing authority.
- [ ] Preserve the three curated catalogue specimens and keep exhaustive
      corpus projection out of their `Examples` view.

## Required Case Corpus

Freeze the case IDs and assertion shape before writing runtime adapters. The
corpus must cover the following claims with the smallest non-overlapping set
of cases:

| Claim | Required proof |
| --- | --- |
| closed default | trigger present and focusable; surface absent; open state and relationship output correct |
| uncontrolled open | pointer and keyboard trigger paths mount the named dialog, emit `openChange(true)`, and expose the surface above normal content |
| controlled ownership | the consumer host owns `open`; component action emits the request once and the host update produces the visible state without double emission |
| disabled | pointer, Enter, Space, and programmatic open-direction paths stay inert; trigger semantics match the contract |
| focus: first | opening focuses the first focusable descendant after the surface mounts |
| focus: content | opening focuses the surface and exposes the required focusability |
| focus: none | opening does not steal focus |
| explicit close | trigger close emits once, unmounts the surface, then restores trigger focus |
| Escape | the real runtime Escape route closes the top Popover, prevents the handled key where the platform exposes that result, and restores trigger focus |
| outside interaction | a real outside pointer route closes and restores focus; an inside interaction does not close |
| outside guard | `dismissOnOutsideInteract=false` keeps the Popover open without disabling Escape |
| nested layer | Escape dismisses the innermost open layer first; an interaction inside an outer layer but outside the inner layer follows the shared dismiss-stack contract |
| placement | representative top, bottom, left, and right cases prove relative surface/trigger geometry; start/end resolution and all enum values remain declaration-covered |
| offset and width | offset and trigger-width mode change the asserted logical bounds without runtime-specific constants |
| semantics and tokens | trigger/surface relationship, dialog role/name, state, surface token roles, and overlay/layer evidence agree |

Do not multiply display cases merely to enumerate every placement. Enum
closure belongs to the interface; mounted cases prove each placement family
and the start/end rule with a bounded set.

Event assertions remain exact and order-sensitive. Observation sequence must
distinguish open-change emission, host update, focus entry, close emission,
surface removal, and focus restoration. Do not invent a portable
`requestClose` event to make this easier.

## Execution Plan

### Batch A — Authority and vocabulary

- [ ] Measure before LOC and inventory the contract, TS machine/parts helper,
      Rust machine mirror, hand-written `PopoverSpec`, web implementations,
      shared Rust surface, GPUI wrapper/backend, primitive capability row,
      vectors, focused tests, and curated specimens.
- [ ] Add `packages/core/src/conformance/popover.ts` and derive portable prop,
      event, part, state, region, axis, and capability types from one
      `defineComponentInterface` value.
- [ ] Generate the portable Rust Popover declaration into the established
      `poodle-specs` generated path. Keep token recipes and genuinely
      Rust-only helpers in the extension module; delete the replaced
      hand-written declaration.
- [ ] Bind Svelte and React portable props/events to inferred interface types.
      Keep framework carriers and the two named web extensions local.
- [ ] Add the minimum generic vocabulary needed for `dismiss`, outside/inside
      pointer intent, focused-part progression, parent/layer relationship,
      layer order/overlay state, and relative logical bounds. Names and data
      must work for later overlay components without mentioning Popover.
- [ ] Update `overlay.intent` and any split capability rows so ownership and
      required evidence are finite. A capability may pass only after executed
      web, render-neutral, and GPUI evidence exists.

Stop before runtime work if the interface needs DOM nodes, portals, GPUI
handles, selectors, arbitrary callbacks, or behaviour expressions.

### Batch B — Corpus and web execution

- [ ] Author the typed case corpus from the required claims above. Use compact
      string regions for trigger/content fixtures; do not introduce a shared
      scene or arbitrary render tree.
- [ ] Add thin Svelte and React fixture hosts. Controlled hosts update state
      the same way a consumer does; runtime adapters do not restate fixture
      data or expected results.
- [ ] Dispatch real DOM pointer, keyboard, focus, Escape, and outside
      interactions. Exercise the shared dismissable-layer stack and anchored
      surface implementation, including the portalled surface.
- [ ] Observe real DOM/computed geometry, focus, relationships, layer result,
      states, token channels, and exact event trace through the generic web
      observer.
- [ ] Repair reference/runtime defects exposed by the cases. Change the
      contract only if the recorded shipped meaning is internally impossible;
      stop and report that contradiction before doing so.

### Batch C — Shared Rust and headless GPUI execution

- [ ] Move portable Popover composition behind the generated spec and shared
      `poodle-render` path. The renderer-neutral tree must carry stable part
      identity, accessibility metadata, focus/dismiss/layer intent, and
      placement inputs required by the generic backend path.
- [ ] Reuse or extract the existing `floating_overlay` composition. Remove the
      equivalent Popover-only GPUI positioning copy when the shared path owns
      the claim; do not leave two active placement implementations.
- [ ] Implement the smallest renderer-neutral layer/dismiss/focus vocabulary
      needed for parity. Do not leak GPUI types into `poodle-node` and do not
      add a general overlay manager without cases that require it.
- [ ] Drive all actions through GPUI 0.2.2's in-memory test platform and real
      node-backend event tree. Escape, outside interaction, focus entry, focus
      restore, and nested dismissal must not call host handlers directly.
- [ ] Observe the rendered node/backend result through the generic normalized
      observer. Placement assertions use relative trigger/surface bounds with
      assertion-local named tolerances; no blanket GPUI tolerance.
- [ ] Keep every selector headless. Opening or activating an OS window is a
      card failure.

### Batch D — Failure proof, consolidation, and cost

- [ ] Plant and revert at least these representative defects: inert Escape,
      inert outside dismissal, wrong initial-focus target, missing focus
      restore, reversed nested-layer dismissal, absent overlay/layer evidence,
      and wrong placement offset. Each must fail the expected
      runtime/case/step/field.
- [ ] Reconcile Popover machine vectors. Adapt unique pure-transition claims
      into the corpus or retain them as a named lower-level unit seam; delete
      duplicated fixture claims and task wiring. Do not count curated
      specimens as replacement savings.
- [ ] Re-run Button, RangeSlider, and Tabs unchanged through the same kernel.
      A generic overlay addition may not regress earlier profiles.
- [ ] Emit the deterministic corpus/interface artifacts, completion report,
      primitive capability report, and cost report.
- [ ] Update `conformance-estate.md` and one August batch log with before/after
      LOC, generated bytes, replaced surfaces, defects found, planted-failure
      evidence, residual debt, and ongoing Popover authoring cost.

## Acceptance Criteria

- [ ] One interface owns every portable Popover prop, event, region, part,
      state, token role, and capability; generated Rust replaces the equivalent
      hand-written declaration.
- [ ] Every required case executes and passes in Svelte, React, and GPUI through
      real runtime interactions. Jetstream is reported once as
      program-deferred, never passing or not-applicable.
- [ ] Controlled ownership, open/close event count, focus entry, focus restore,
      Escape, outside interaction, and nested dismissal agree exactly.
- [ ] Trigger/surface relationships, dialog role/name, surface presence, state,
      token roles, and overlay/layer observations agree.
- [ ] Placement family, start/end rule, offset, and trigger-width results use
      authored relative-geometry assertions with named bounds only.
- [ ] The generic action, observer, comparison, and headless-driver code has no
      Popover identifier, selector, part list, fixture, expected value, or
      tolerance branch.
- [ ] `overlay.intent` and any newly split overlay capability rows have executed
      evidence in every required active layer. Missing backend evidence fails
      completion.
- [ ] An inert handler or missing focus/layer/placement implementation cannot
      pass through machine-vector or pre-backend node evidence.
- [ ] Equivalent machine/vector/declaration surfaces are removed or retain one
      explicit non-duplicated lower-level claim and owner.
- [ ] Curated Svelte, React, and GPUI Popover specimens remain useful and are
      not replaced by the exhaustive corpus.
- [ ] Earlier Button, RangeSlider, and Tabs cases remain green. All local
      validation is headless.

## Stop Conditions

- A shared case needs a portal, DOM selector, GPUI handle, backend node type,
  or runtime branch.
- Generic code needs a Popover identifier, anatomy branch, fixture, expected
  value, or runtime-specific tolerance.
- Placement parity requires pixel-image equality or a blanket tolerance rather
  than a relative geometric claim.
- Focus or dismissal is proved by direct callback invocation, state mutation,
  machine vectors alone, or a node before the backend executes it.
- The active runtime cannot expose enough evidence to distinguish an inert
  layer, dismissal route, focus route, or placement implementation.
- The work grows a universal overlay manager, shared render tree, behaviour
  compiler, or second fixture model.
- The contract must gain modal, menu, pinned, or request-close semantics.
- The work touches TextInput, HistoryCenter, Modal/Menu rollout, the audio-meter
  lane, catalogue navigation, or specimen-audit implementation.
- Any validation path creates an OS window, activates an application, or takes
  operator focus.
- Jetstream becomes a build or implementation dependency.

Stop with the exact failed claim, evidence, smallest options, and cost. Do not
hide the gap behind a waiver or widen the model around it.

## Writable Scope

- `packages/core/src/conformance/`, Popover machine/parts helpers, and focused
  core tests
- conformance codegen, deterministic artifacts, and generated Popover Rust
  declaration
- `packages/contracts/components/src/popover.rs` and the smallest required
  renderer-neutral node interaction/layer vocabulary
- `packages/render/src/popover.rs`, shared floating-overlay composition,
  generic conformance observation/support, and focused tests
- `packages/svelte/components/src/Popover.svelte` and focused conformance host
- `packages/react/components/src/Popover.tsx` and focused conformance host
- generic web conformance runner/observer/action support under
  `test/conformance/`
- GPUI Popover conformance adapter, shared headless driver support,
  `packages/gpui/node-backend/`, and focused headless tests
- Popover machine interfaces/vectors only where this card records their
  disposition
- `tasks/effigy.tasks.toml` only for existing conformance selector wiring
- `docs/roadmaps/g14/conformance-estate.md`
- one August batch log and append-only `PAPERCUTS.md`

Do not edit curated specimen files except to repair a direct regression caused
by a portable implementation change; record any such edit separately. Do not
add a catalogue `Conformance` tab. Do not edit other component contracts,
other roadmap files, g14 status/index, release workflows, external
repositories, Jetstream, or the independent `g14.024`/`g14.026` lanes.

Workers do not change this card's status or the g14 runway status. Return one
PR for orchestrator review with the card still marked `ready`.

## Validation

Use `effigy test --plan` before choosing focused selectors. Run one meaningful
validation board after each completed batch, not after each small edit.

Required final board, entirely headless:

- generated Popover interface/corpus drift checks
- focused core, Svelte, React, renderer-neutral Rust, GPUI backend, and GPUI
  headless Popover tests
- Popover-only active-cohort conformance execution
- primitive capability report/check
- `effigy conformance:check`
- `effigy conformance:complete`
- `effigy conformance:cost`
- `effigy ci:web`
- `effigy ci:rust`
- `effigy ci:native`
- `effigy docs:check`
- `git diff --check`

Never run `test:native-visual`, a deleted/legacy `*-windowed` selector, or any
foreground preview as validation for this card.
