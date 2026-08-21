# g15.012 — Primitive-First Visual Conformance Lane

Status: **non-dispatchable parent** — exact children `g15.044`–`g15.047`;
`g15.044` is complete and `g15.045` is ready
Depends on: `g15.001` (measured roster), `g15.011` (before fixture freeze),
`g15.038` (SegmentedControl native option parity)
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
human review, not a component-completion gate. By operator ruling, the card
still completes before the generation's final release-certification card.

## Scope

- a bounded inventory of primitive variants/styles (foundation display and
  shell primitives first), recorded per component as named fixtures
- real harnesses in each runtime rendering the same named fixtures
- renderer-aware comparison of geometry, tokens, and pixels with
  antialiasing tolerance
- web snapshot tooling under `test/visual/`, plus the GPUI offscreen seam that
  `g15.044` proved and `g15.045` must adopt; the retained windowed native
  capture is evidence to replace, not a release fallback

## Exact Children

1. [`g15.044`](044-gpui-offscreen-capture-feasibility.md) — prove or reject a
   GPUI-native offscreen pixel path. **Complete in PR #61; reviewed verdict
   `go`.** It built no fixtures and did not change the production GPUI pin.
2. [`g15.045`](045-gpui-offscreen-capture-adoption.md) — adopt the exact proved
   GPUI pin/seam after an operator-reviewed `go` verdict. **Ready.**
3. [`g15.046`](046-primitive-visual-fixture-inventory.md) — freeze the small
   named primitive batch after the human-centred catalogue audit completes.
4. [`g15.047`](047-primitive-visual-comparison.md) — capture and compare the
   first batch across Svelte, React, and GPUI with human-reviewed tolerances.

### g15.044 Evidence (2026-08-21)

The native half of this lane is unblocked. A real Poodle Button
renders offscreen to a deterministic RGBA PNG at
`zed-industries/zed@1ea16c1ab9dd6d36649e002dc60995634da04daf` with no
`NSWindow`, no `screencapture`, no focus theft, and no permission prompt —
proved by construction, not by opening a window. Ten captures of identical
input are byte-identical, and the whole result is re-runnable from
[`reproduce.sh`](../../logs/2026-08/assets/g15-044/reproduce.sh). Migration cost is 17 mechanical compile errors across
9 files plus one added dependency, with `headless_regressions` passing 56/56 on
the migrated copy.

Two measured constraints propagate into `g15.046`/`g15.047` fixture design:

- Captures are **2× only** at that revision (`TestWindow::scale_factor` is
  hardcoded `2.0`). Viewport is freely controllable; scale is not.
- Glyphs come from the host Core Text stack, so byte-identical output is proved
  **within one machine**, not across machines. Native baselines need
  renderer-aware tolerance and a recorded capture environment — which is
  already this lane's stated posture, and this evidence does not soften it.

Full evidence: [`../../research/gpui-offscreen-capture-feasibility.md`](../../research/gpui-offscreen-capture-feasibility.md).

This parent never dispatches directly. A Longhorn-backed Tauri lab is optional
control tooling, not one of these children and not a prerequisite for the
comparator. Longhorn can already control and compose Tauri webviews without
focus theft; its current contract cannot see genuinely native GPUI pixels.

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
- [ ] Headless capture evidence exists for the first primitive batch in every
      active runtime with renderer-aware tolerances.
- [ ] No semantic/behaviour authority or completion gate is introduced; the
      bounded renderer-aware cross-runtime comparator remains diagnostic.

## Stop Conditions

- The fixture inventory grows a universal scene/component language or
  becomes an executable behaviour authority.
- A capture replaces focused functional evidence.
- The windowed harness is run to compensate for a missing headless path.
- Work expands beyond the named batch without a new card.

## Closeout

This parent completes only when `g15.044`–`g15.047` have landed, the first
primitive batch has operator-reviewed evidence for every active runtime, and
the resulting mechanism still satisfies the forbidden-authority boundaries
above.

## Validation

- web snapshot tooling for the batch
- `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector or `test:native-visual` on a local desktop
without explicit operator approval; never run `qa:jetstream` or any Jetstream
selector.
