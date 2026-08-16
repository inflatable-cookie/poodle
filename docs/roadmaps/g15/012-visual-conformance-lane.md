# g15.012 — Primitive-First Visual Conformance Lane

Status: **blocked** — orchestration hold; `g15.003` is the active card
Depends on: `g15.001` (measured roster)
Governing refs: `release-baseline-roster.md`,
`../../roadmaps/g14/022-generation-closeout.md` (operator ruling on future
visual conformance), `../../roadmaps/g14/conformance-estate.md`,
`../../contracts/001-working-rules.md`

## Outcome

Build the operator-approved visual-conformance lane, per the decision
recorded in `g14.022`: inventory bounded component variants, capture the
**same named fixtures per runtime**, and compare geometry, tokens, and pixels
within **renderer-aware tolerances** (including antialiasing). It starts with
primitives and reuses the retained headless/native capture foundation.

What remains forbidden is making those fixtures a component API/behaviour
authority or a universal runtime representation — the rejected g13/g14
mechanisms do not return under new names. The lane is a diagnostic aid for
human review, not a completion gate and not a release prerequisite.

## Scope

- a bounded inventory of primitive variants/styles (foundation display and
  shell primitives first), recorded per component as named fixtures
- real harnesses in each runtime rendering the same named fixtures
- renderer-aware comparison of geometry, tokens, and pixels with
  antialiasing tolerance
- web snapshot tooling under `test/visual/` and the retained native capture
  (`effigy test:native-visual` with `--control-size`) as the seam

## Execution Plan

- [ ] **Batch 1 — capture-platform decision:** confirm a headless capture path
      that cannot steal desktop focus exists for every active runtime. If
      GPUI cannot provide one, this batch stops with that finding recorded —
      the windowed harness is not run and the lane's native half is blocked
      until a headless path exists.
- [ ] **Batch 2 — fixture inventory:** name the bounded variant/style set for
      the first primitive batch (foundation display & shell) as shared,
      renderer-neutral fixture identities (component, variant, size/density
      axis values, tokens under test).
- [ ] **Batch 3 — paired captures and comparison:** implement the real
      per-runtime harnesses, capture the same named fixtures, and compare
      geometry/tokens/pixels within renderer-aware tolerances; land baselines
      for the first batch as diagnostic evidence.

## Goals

- [ ] The lane is primitive-first and bounded to named components per batch.
- [ ] Captures are headless and cannot steal desktop focus; if that is
      impossible for a runtime, the finding is recorded and that runtime's
      half stops rather than falling back to a windowed harness.
- [ ] Green baselines are diagnostic only: they never count as component
      completion, and every capture is reviewed by a human.

## Acceptance

- [ ] The fixture inventory is renderer-neutral and is not a universal scene
      or component representation.
- [ ] Fixtures are never an API/behaviour authority; contracts remain the
      semantic authority and focused functional evidence stays the
      completion surface.
- [ ] Headless capture evidence exists for the first primitive batch with
      renderer-aware tolerances; native visual compare remains local-only and
      operator-owned where it cannot be headless.
- [ ] No semantic/behaviour authority or completion gate is introduced; the
      bounded renderer-aware cross-runtime comparator remains diagnostic.

## Stop Conditions

- The fixture inventory grows a universal scene/component language or
  becomes an executable behaviour authority.
- A capture replaces focused functional evidence.
- The windowed harness is run to compensate for a missing headless path.
- Work expands beyond the named batch without a new card.

## Writable Scope

- fixture identities, capture harnesses, baselines, and batch cards
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- web snapshot tooling for the batch
- `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector or `test:native-visual` on a local desktop
without explicit operator approval; never run `qa:jetstream` or any Jetstream
selector.
