# g15.046 — Primitive Visual Fixture Inventory

Status: **ready** — exact Button-only batch compiled; `g15.011` and `g15.045`
are complete
Parent: `012-visual-conformance-lane.md`
Depends on: completed human-centred catalogue audit and headless capture in
every active runtime
Unblocks: `g15.047`
Governing refs: `release-baseline-roster.md`,
`../../contracts/001-working-rules.md`, `specimen-plan-outline.md`,
`012-visual-conformance-lane.md`

## Goal

Freeze a small first-batch inventory of renderer-neutral **fixture identities**
for primitive visual comparison. The inventory names observable inputs; every
runtime adapter still renders its real component.

## First Batch

The first batch is **Button only**. This is an intentional mechanism proof,
not the final primitive inventory and not a representative claim over the
library. It uses one fixed environment unless the fixture says otherwise:
Eclipse theme, `md`, default density, 240×80 logical viewport, 2× capture
scale, and the label `Run`.

Freeze these 18 identities, each with fully resolved values rather than
inheritance or implied defaults:

| Group | Fixture identities |
| --- | --- |
| Resting variants | `button/rest-secondary`, `button/variant-primary`, `button/variant-ghost` |
| Secondary status tones | `button/tone-danger`, `button/tone-success`, `button/tone-warning` |
| Size ladder beyond the `md` base | `button/size-xs`, `button/size-sm`, `button/size-lg`, `button/size-xl` |
| Density ladder beyond the default base | `button/density-compact`, `button/density-comfortable` |
| Visual states | `button/state-disabled`, `button/state-loading`, `button/state-pressed` |
| Content shapes | `button/content-leading-icon`, `button/content-icon-only` |
| Reference light theme | `button/theme-iceberg` |

The first batch deliberately samples status tones on the secondary variant;
it does not claim the full variant × tone cross-product. Expansion waits until
`g15.047` proves the comparator and the operator reviews every first-batch
result.

## Scope Envelope

- Start and stop with the exact Button batch above.
- Name only contract-backed values: variant/appearance, state, exact public
  size and density domains, theme, content, and viewport.
- Keep interactions in focused tests. A fixture may name a resulting visual
  state, but it does not encode an action script or behavior machine.
- Define stable crop/geometry landmarks and token roles to report beside pixels.
- Record per-renderer exclusions only where the contract permits a real
  platform difference.

## Deliverables

- One versioned, Button-specific canonical data file under
  `test/visual/fixtures/`. It contains identities and observable inputs only;
  it is not a component API schema, scene, render tree, or action language.
- Small TypeScript and Rust loaders/validators that consume that same file and
  fail on duplicate/unknown IDs, missing required fields, unknown enum values,
  unresolved defaults, invalid viewport/scale, or an inventory other than the
  exact 18 names above. No code generation.
- Renderer-neutral landmark names for later geometry receipts: `root`,
  `content`, and conditional `icon` or `spinner`. Record semantic report roles
  `fill`, `border`, `text`, `shadow`, and `focus-ring`; do not record expected
  renderer output or resolved pixel values.
- One short inventory document under `test/visual/` explaining the boundary,
  exact denominator, fixed environment, and how `g15.047` will consume it.
- One August execution log with source cost, duplicated registry count, and
  validation.

## Acceptance Envelope

- [ ] The canonical inventory contains exactly the 18 Button identities above;
      TypeScript and Rust read the same file and reject a planted missing,
      extra, duplicate, or invalid fixture by exact name.
- [ ] Every identity carries explicit theme, size, density, viewport, scale,
      content, variant, tone, and visual-state values; no runtime supplies a
      silent default.
- [ ] The batch is small enough for a human to review all 54 future captures
      (18 identities × Svelte, React, GPUI).
- [ ] No fixture contains framework code, node trees, generated props, event
      scripts, or a universal component schema.
- [ ] The loaders prove every value maps to the existing Button contract
      domains in both languages without changing a component contract or
      public package surface.
- [ ] The inventory is diagnostic and cannot mark a component complete.

## Writable Scope

- `test/visual/fixtures/` and the smallest adjacent validator/test/doc surfaces
- a focused GPUI preview test module needed to parse and validate the same data
- this card's August execution log
- package/dev metadata only if a loader needs an already-present dependency
  made explicit; stop before adding a new production dependency
- `PAPERCUTS.md` for newly found execution friction

Do not add captures, baselines, pixel thresholds, comparison output, specimen
changes, component changes, contracts, generated component adapters, workflow
edits, release mutations, Jetstream work, or a second component.

## Stop Conditions

- The inventory starts describing component APIs generally instead of naming
  bounded visual cases.
- A fixture needs executable behavior or normalized renderer output.
- The batch expands beyond the named primitives before the first comparison is
  reviewed.

## Validation

- focused TypeScript inventory tests
- focused Rust inventory tests
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Never run a `*-windowed`, `test:native-visual`, GPUI preview, or Jetstream
selector.
