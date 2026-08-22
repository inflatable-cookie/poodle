# g15.047 — Primitive Visual Comparison

Status: **ready — exact Button comparison and operator-review envelope
compiled on 2026-08-22**
Parent: `012-visual-conformance-lane.md`
Depends on: `g15.045`, accepted `g15.046`
Unblocks: `g15.012` closeout, then release certification
Governing refs: `../../roadmaps/g14/conformance-estate.md`,
`../../contracts/001-working-rules.md`,
`../../contracts/components/button.md`,
`012-visual-conformance-lane.md`,
`046-primitive-visual-fixture-inventory.md`

## Goal

Render the exact 18 Button fixture identities through Svelte, React, and GPUI.
Compare same-run geometry, resolved Button visual roles, and pixels with one
explicit renderer-aware policy. Produce evidence the operator can inspect.

This is a diagnostic mechanism proof. It is not component authority, a
completion gate, a baseline updater, or permission to add another component.

## Fixed Decisions

### Denominator and scene

- Consume the accepted
  `test/visual/fixtures/button-visual-inventory.json` unchanged: 18 fixtures,
  54 retained captures, no additions or exclusions.
- Each runtime renders its real Button implementation from the fixture's fully
  resolved values. Runtime adapters are hand-written and Button-only.
- Capture one 240×80 logical viewport at 2×. The whole viewport uses the
  fixture theme's canvas background. Place the Button at logical `(16, 16)`.
- Capture the final declared state directly: rest, disabled, loading, or
  pressed. Do not replay input to reach it.
- Freeze animation and time. Loading-spinner output must be deterministic;
  every runtime captures each fixture twice and the pair must be byte-identical.
- Web capture uses one pinned headless Chromium build at device scale 2. GPUI
  uses the adopted Metal `HeadlessAppContext` path. No desktop window, focus,
  screen-capture permission, or windowed fallback.

### Runtime adapters

- Add private Button fixture hosts to the Svelte and React previews. They are
  capture-only routes or entry points, separate from catalogue specimens.
- Extend `poodle-offscreen-capture` to accept `--fixture <exact-name>` and load
  the same canonical inventory through a reusable internal Rust loader. Move
  the accepted test-only parser rather than creating a third parser.
- The adapters may map only the accepted Button fields: theme, size, density,
  viewport, scale, variant, tone, content, and visual state.
- Do not touch specimen pages. Do not add a generic fixture registry, generated
  adapter, scene tree, prop schema, action language, or reusable component
  observation model.

### Capture receipt

Every successful capture has a PNG and a typed Button-only receipt. Missing,
stale, aliased, or hash-mismatched pairs fail closed. The receipt records:

- schema `poodle.button-visual-capture.v1`;
- exact fixture name and runtime (`svelte`, `react`, `gpui`);
- logical viewport, scale, device dimensions, PNG SHA-256;
- renderer environment: Chromium version for web; OS, architecture, and exact
  GPUI revision for native; no hostname, username, or absolute path;
- actual logical bounds for the fixture's declared landmarks;
- fixed Button visual-role evidence for `fill`, `border`, `text`, `shadow`,
  and dormant `focus-ring`, including normalized sRGB values and numeric
  widths/layers where present.

The receipt is evidence for this Button batch only. It may not accept a
component name, arbitrary landmark, arbitrary role, or free-form property bag.

### Comparison policy

All comparisons are same-run. Committed images are review evidence, never the
expected side of a future gate. There is no refresh/update-baseline command.

Svelte ↔ React is exact because both run in the same browser and share CSS:

- identical device dimensions;
- zero logical-edge delta for every landmark;
- exact normalized role evidence;
- zero differing pixels.

Svelte is the web reference for the renderer-aware GPUI comparison. React must
first match Svelte exactly; GPUI cannot borrow React's pass.

Web ↔ GPUI limits are fixed for the first batch:

| Channel | Maximum delta |
| --- | --- |
| root landmark, each edge | 0.5 logical px |
| icon/spinner centre and size, each axis | 1 logical px |
| content centre, each axis | 1 logical px |
| content width/height | 2 logical px |
| fill/border/text/focus-ring colour | 1 in any 8-bit sRGB channel |
| border/focus-ring width | 0.5 logical px |
| shadow layer count/inset | exact |
| shadow offset/blur/spread | 0.5 logical px |
| pixels | `pixelmatch` threshold `0.1`, `includeAA: false`, at most 3% of the full viewport |

One policy covers all 18 fixtures. No per-fixture allowlist. Do not raise the
geometry or role limits. Do not raise the pixel cap above 3% in the worker
thread; stop for an operator decision if real antialiasing cannot fit it while
geometry and roles are correct.

### Evidence and review

- One command captures and compares all 18 fixtures into an explicit output
  directory. Default validation output uses a disposable directory.
- Produce a machine-readable summary, all 54 PNG/receipt pairs, 18
  Svelte↔React diffs, 18 Svelte↔GPUI diffs, and contact sheets grouped in the
  accepted fixture order.
- Commit the reviewed report, contact sheets, receipts, and captures under one
  August `g15.047` log asset directory. These are point-in-time evidence, not
  baselines read by the comparator.
- The worker records every initial mismatch and any bounded Button repair in
  the execution log. Do not hide a mismatch by naming it expected.
- The PR stops for operator review. Every one of the 54 captures and every
  accepted native tolerance needs explicit sign-off before merge.

## Bounded Repair Authority

The worker may repair a directly measured Button mismatch when the current
Button contract already dictates the result. Keep it to the smallest relevant
shared CSS, Svelte/React shell, `poodle-render` Button, or GPUI backend
projection change. Add a focused regression for each repair and preserve the
before/after finding in the log.

Stop instead when a mismatch needs a public API, contract, token, typography,
renderer architecture, or platform-delta decision. This card measures that
blocker; it does not decide it.

## Writable Scope

- `test/visual/fixtures/` for the smallest reuse of the accepted inventory
  loader and focused tests
- one Button-only comparator/capture directory under `test/visual/`
- private Button fixture hosts under `packages/{svelte,react}/preview/src/`
- `packages/gpui/preview/src/bin/offscreen_capture.rs`, its smallest internal
  fixture-loader/receipt modules, focused tests, and `Cargo.toml` only if target
  registration changes
- `packages/gpui/node-backend/` only for capture observation or a directly
  measured contract-backed Button projection repair
- `packages/core/src/styles/button.css`, Button shell files, and
  `packages/render/src/button.rs` only under Bounded Repair Authority
- `tasks/effigy.tasks.toml` for one headless comparison selector
- `docs/logs/2026-08/20260822-g15-047-*.md` and
  `docs/logs/2026-08/assets/g15-047/`
- `PAPERCUTS.md` for newly found execution friction

Do not edit other components, specimens, component contracts, tokens,
generated files, public package exports, package versions, release notes,
workflows, Jetstream, the Longhorn lab, or release/tag/publication surfaces.

## Acceptance Envelope

- [ ] The exact 18 accepted fixtures produce 54 verified capture/receipt pairs
      and 36 pairwise diffs. Missing, extra, duplicated, stale, or hash-invalid
      output fails.
- [ ] Both repeat captures for every runtime/fixture are byte-identical.
- [ ] Svelte and React pass the exact policy. Web and GPUI pass the fixed
      renderer-aware policy or the card stops with a named blocker; nothing is
      silently allowlisted.
- [ ] Geometry, role, and pixel verdicts remain separate in the report. A
      pixel pass cannot hide a geometry or role failure.
- [ ] In-memory planted failures prove missing capture, two-logical-pixel root
      shift, missing icon/spinner landmark, changed role colour/shadow, PNG
      tamper, and a pixel change exceeding 3% each fail through the production
      comparator path.
- [ ] Contact sheets and the full machine-readable report preserve all 54
      captures in canonical fixture order. The operator reviews them before
      merge.
- [ ] No committed capture is consumed as a future expected baseline. No
      update/refresh command exists.
- [ ] The execution log records source cost, duplicated registry count,
      initial mismatches, repairs, final metrics, environment, and operator
      verdict.
- [ ] The mechanism remains Button-only and diagnostic. It cannot mark Button
      or another component complete.

## Validation

- `effigy test:visual-fixtures`
- focused comparator unit/negative tests
- the new headless 18-fixture comparison selector, twice, with matching report
  metrics and hashes
- `effigy smoke:gpui-offscreen-capture`
- focused Button Svelte/React parity tests and Rust/GPUI tests touched by any
  repair
- `effigy check:svelte`
- `effigy react:build`
- `effigy check:gpui`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Do not run `*-windowed`, `test:native-visual`, a GPUI preview, any Jetstream
selector, a release mutation, tag, publication, or workflow edit.

## Stop Conditions

- The comparator needs a generic component schema, generated adapters, a
  normalized cross-component observation plane, or a shared behaviour corpus.
- Any accepted fixture cannot be rendered from its existing public Button
  inputs in one active runtime.
- A runtime borrows another runtime's capture, receipt, or completion status.
- A capture opens or focuses a desktop window, invokes desktop capture, or
  needs screen-recording/accessibility permission.
- Repeat captures differ under fixed input.
- The fixed tolerance policy cannot separate antialiasing from a planted
  structural, role, or pixel drift.
- A required repair crosses the Bounded Repair Authority.
- The worker needs to add a second component, Longhorn orchestration, a
  workflow, or release mutation.

## Continuation

After the worker PR is ready, the orchestrator and operator review the contact
sheets and report. Merge needs explicit operator authorization. An accepted
merge completes parent `g15.012`; it does not expand the fixture roster.

The Longhorn-backed conformance lab remains a separate post-comparator design
decision. `g15.050` remains blocked on `g15.043` and other open release gaps.
