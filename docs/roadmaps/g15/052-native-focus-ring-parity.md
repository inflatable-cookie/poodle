# g15.052 — Native Focus-Ring Parity

Status: **ready — reusable node capability and exact Button/Stepper closure
compiled 2026-08-22**
Found by: `g15.042`, measured by `g15.047`
Depends on: `g15.042`, `g15.047`
Unblocks: `g15.050`, then `g15.013`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/button.md`,
`../../contracts/components/stepper.md`,
`../../logs/2026-08/20260822-g15-047-primitive-visual-comparison.md`,
`release-gap-register.md`

## Goal

Add one reusable shared-Rust focus-ring capability, project it faithfully in
GPUI, and use it to close the two measured native gaps:

- Button currently recolours its resting 1px border instead of drawing the
  contracted 2px ring;
- Stepper trigger, rerun, and summary controls have no native ring and cannot
  receive keyboard entry until pointer focus has already reached them.

This is a renderer-substrate repair, not a component sweep and not a new
cross-runtime representation.

## Fixed Architecture Decision

Add a dedicated focus-ring value to the existing `poodle-node` style
vocabulary. It carries resolved `color`, `width`, and `offset` and belongs on
`NodeStyle`, separate from `StylePatch`.

The separation is deliberate:

- a ring is an out-of-flow focus affordance, not a replacement for the
  resting border;
- it must work on controls with no resting border;
- it must not alter layout, component size, radius, fill, or resting border;
- generic focus-state changes may still use `StylePatch::focus`; the ring and
  patch compose when both exist.

The shared render decides whether a component declares a ring and supplies
resolved values. The GPUI backend owns focus observation and drawing. Do not
put GPUI objects in `poodle-node`, duplicate component logic in the backend,
or infer a ring for every focusable node.

Jetstream receives the renderer-neutral field through normal compilation but
remains program-deferred. This card does not add Jetstream projection,
preview work, or QA.

## Exact Scope

### Shared node and GPUI projection

- Introduce one small typed focus-ring value with resolved color, width, and
  offset; default is absent.
- Make a declared ring sufficient for the backend to track a focus handle.
- Paint it only while that real handle is focused. Preserve any resting border
  and existing shadow stack. Hover must not overwrite it.
- Project the declared width and offset without changing layout bounds. Use
  the smallest GPUI-native paint mechanism that reproduces the ring. Do not
  encode the ring as a wider replacement border or silently drop the offset.
- Add focused and unfocused backend regressions, including a borderless node
  and a node with an existing border and shadow.

### Button

- Replace Button's focus-time border recolour with the dedicated ring using
  `border.width.focus`, `color.accent.focusRing`, and the contract's 2px
  offset.
- Keep all idle, hover, active, pressed, disabled, loading, size, and layout
  output unchanged.
- Rerun the accepted 18-fixture comparator into a disposable directory. The
  16 focus-ring role failures must disappear without changing policy,
  tolerance, fixture identity, or known-delta classification. The 16 existing
  shadow findings may remain annotated and blocking.

### Stepper

- Give the trigger, rerun action, and summary control stable native focus
  tracking and the contracted visible ring where the Stepper contract requires
  it. Do not invent a resting border.
- Prove keyboard entry reaches the controls in contract order without a prior
  pointer press, and `Enter`/`Space` activates the focused action.
- Preserve selection, rerun, collapse, and pointer behavior landed by
  `g15.042`.

## Evidence

- Record the pre/post Button role verdict and the unchanged fixed comparison
  policy in one August `g15.052` execution log.
- Retain a small operator-readable focused-state image or contact sheet for a
  bordered Button and borderless Stepper control from the headless GPUI path.
  It is review evidence, not a future baseline.
- Record the exact GPUI paint mechanism and why it preserves ring width,
  offset, radius, resting border, shadows, and layout.

## Writable Scope

- `packages/contracts/node/` for the focus-ring value and focused tests
- `packages/render/src/button.rs` and `packages/render/src/stepper.rs`
- `packages/gpui/node-backend/` for focus tracking/projection and regressions
- the smallest existing GPUI headless/specimen tests needed for real Button
  and Stepper focus evidence
- `test/visual/button-comparison/` only if receipt observation must learn the
  new node field; do not change policy or fixtures
- `docs/contracts/components/{button,stepper}.md` only for native runtime notes
  that clarify the already-contracted behavior
- `docs/logs/2026-08/20260822-g15-052-*.md` and
  `docs/logs/2026-08/assets/g15-052/`
- `release-gap-register.md`, this card, and `PAPERCUTS.md` when evidence changes

Do not edit Svelte/React components or CSS, public component props, tokens,
specimen catalogue pages, the 18-fixture inventory, comparator thresholds,
known-delta rules, package versions, workflows, Jetstream, Longhorn, release
notes, tag/publication surfaces, or `g15.043`.

## Acceptance

- [ ] `poodle-node` has one reusable, renderer-neutral focus-ring channel with
      resolved color, width, and offset; absent remains the default.
- [ ] GPUI paints the ring outside layout without replacing the resting border
      or dropping its offset; bordered and borderless proofs pass.
- [ ] Button's comparator reports zero focus-ring findings across all 18
      fixtures under the unchanged fixed policy.
- [ ] Button has no unrelated visual or interaction change; the existing
      shadow limitation stays explicit rather than being repaired incidentally.
- [ ] Stepper accepts keyboard focus without a preceding pointer press, paints
      the ring on the contracted controls, and activates the focused action.
- [ ] Real mounted/headless evidence covers focus entry, movement, activation,
      blur, and painted output without opening or focusing a desktop window.
- [ ] No component-specific focus-ring drawing appears in the GPUI backend and
      no universal scene/component authority is introduced.

## Validation

- focused `poodle-node`, `poodle-render`, and `poodle-gpui-node-backend` tests
- focused Stepper mounted headless regressions and specimen probe
- `effigy test:visual-button-comparison` with an output directory outside the
  committed `g15.047` evidence; expected aggregate remains red only for the
  existing shadow findings
- `effigy smoke:gpui-offscreen-capture`
- `effigy check:gpui`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Never run `*-windowed`, `test:native-visual`, a GPUI preview window, any
Jetstream selector, a release mutation, tag, publication, or workflow edit.

## Stop Conditions

- GPUI cannot paint the declared width and offset without changing layout,
  replacing the resting border, or erasing existing shadows.
- Closing Stepper keyboard entry needs a general focus-order system beyond the
  three named controls.
- The repair requires a public component API, token, GPUI fork, or contract
  semantics decision not already fixed here.
- The comparator only passes after a tolerance, fixture, role, or known-delta
  change.
- Work expands into a repository-wide migration of existing focus treatments.

## Continuation

After operator review and merge, close both native focus rows in the release
gap register. `g15.043` remains the final implementation card before
`g15.050`; do not start the release candidate while either is open.
