# g15.022 — Audio and Music Curation (August batch log)

Date: 2026-08-19
Card: `docs/roadmaps/g15/022-curate-audio-music.md`
Parent: `docs/roadmaps/g15/018-overloaded-examples-curation.md`
Handoff: `docs/handoffs/20260819-183903-g15-022-audio-music-curation.md`
Worktree: `/Users/tom/.t3/worktrees/poodle/g15-022-audio-music-curation`
Branch: `t3code/g15-022-audio-music-curation`
Worker base: `6ba53a7b872a69f280ae09cf4378cefda841345c` (`origin/main` at
dispatch; handoff planning base `15069e7e579a2af67e987cb5a4dd16ba2e866cd0`
confirmed as an ancestor)

## Summary

All eleven audio and music pages re-measured before editing. The `g15.016`
idiom convergence and `g15.017` axis placement had already removed the
in-body axis matrices and hand-rolled captions the audit graded, so eight of
eleven pages were already inside the section budget. Three pages changed:
AudioMeter 10 → 9 captions, Knob 10 → 9, GainReductionMeter 9 → 8. No
component, contract, prop, or token changes.

## Change class

- **Change class:** specimen curation, plus one orchestrator-authorized,
  docs-only contract wording correction (`knob.md` § 13, requested in PR
  review)
- **Packages touched:** `poodle-svelte` preview, `poodle-react` preview,
  `poodle-render` (shared native specimen definitions)
- **Public entry points:** none
- **Downstream re-check:** none — no public surface changed
- **app_state.rs:** unused

## Baseline recount at the worker base

The audit graded these pages before `g15.016`/`g15.017` landed. Its React
counts included the in-body axis matrices and its Svelte B-grade rows
carried no count; the worker-base re-measurement below supersedes them.
Before/after finding recorded per the card: React lost two matrix captions
per page (`AudioMeter` 11 → 10, `AudioSwitch` 8 → 6, `EnvelopeEditor` 9 → 7,
`Fader` 10 → 8, `GainReductionMeter` 11 → 9, `Keyboard` 8 → 6, `Knob` 12 →
10, `ModMatrixGrid` 7 → 5, `ValueReadout` 11 → 9, `WaveformDisplay` 8 → 6,
`XYPad` 10 → 8); Svelte counts matched the audit where the audit recorded
one. The card's page set is unchanged — all eleven pages still belong to
this family.

| Page | Svelte | React | GPUI | Ruling |
| --- | ---: | ---: | ---: | --- |
| AudioMeter | 10 | 10 | 9 | curate — over the nine-caption ceiling |
| AudioSwitch | 6 | 6 | 6 | keep — inside the 3–6 target |
| EnvelopeEditor | 7 | 7 | 9 | keep — contract-named set, reason below |
| Fader | 8 | 8 | 8 | keep — contract-named set, reason below |
| GainReductionMeter | 9 | 9 | 9 | curate — duplicate zero-state group |
| Keyboard | 6 | 6 | 6 | keep — inside the 3–6 target |
| Knob | 10 | 10 | 10 | curate — over the nine-caption ceiling |
| ModMatrixGrid | 5 | 5 | 5 | keep — inside the 3–6 target |
| ValueReadout | 9 | 9 | 9 | keep — contract-named set, reason below |
| WaveformDisplay | 6 | 6 | 6 | keep — inside the 3–6 target |
| XYPad | 8 | 8 | 8 | keep — contract-named set, reason below |

Counts are captioned examples in the `Examples` pane (Svelte/React
`SpecimenGroup` labels, GPUI `page()` group titles), excluding the size and
density axis panes.

## After

| Page | Svelte | React | GPUI |
| --- | ---: | ---: | ---: |
| AudioMeter | 9 | 9 | 9 |
| AudioSwitch | 6 | 6 | 6 |
| EnvelopeEditor | 7 | 7 | 9 |
| Fader | 8 | 8 | 8 |
| GainReductionMeter | 8 | 8 | 8 |
| Keyboard | 6 | 6 | 6 |
| Knob | 9 | 9 | 9 |
| ModMatrixGrid | 5 | 5 | 5 |
| ValueReadout | 9 | 9 | 9 |
| WaveformDisplay | 6 | 6 | 6 |
| XYPad | 8 | 8 | 8 |

Svelte and React captions are verbatim identical on every page. GPUI
teaches the same evidence set; EnvelopeEditor's native page spells out the
two pairs the web pages fold (add/remove beside selection, curve nudges
beside keyboard nudges), which the outline permits.

## Final ordered captions (changed pages)

**AudioMeter** — VU — 300 ms integration; PPM; Sample peak; RMS window;
Bar and segment styles; Mono and stereo; Vertical and horizontal; Peak
hold; Clip latch and manual reset. The MeterSurface pointer now sits above
the groups as an uncaptioned intro paragraph, copy unchanged.

**Knob** — Linear / default reset; Logarithmic frequency; Bipolar center;
Stepped values; Fine drag (Shift); Circular mode; Automation state;
Type-in and keyboard bounds; Disabled.

**GainReductionMeter** — No reduction / reset; Attack; Release; Maximum
reduction; Bar and segment styles; Vertical and horizontal; Invalid-frame
rejection; Disabled.

## Named removals

- **AudioMeter — "Batched rendering" group.** A navigation note, not an
  example; it held no component. Moved to an uncaptioned intro paragraph
  with identical copy. No contract evidence removed — the batched tier is
  governed by spec 068 and taught on the MeterSurface page.
- **GainReductionMeter — "Reset" group.** Merged into "No reduction /
  reset". A statically rendered post-reset context is pixel-identical to a
  fresh zero context, so the standalone caption duplicated the default
  state. Reset behaviour evidence stays in the focused core transition
  tests (`packages/core/test/audio-gain-reduction-meter.test.ts`); the
  contract's named reset evidence stays visible in the merged caption.
- **Knob — "Type-in (Enter)" and "Keyboard bounds (Home / End)" groups.**
  Merged into "Type-in and keyboard bounds": one row of three knobs
  (type-in, minimum, maximum). Both contract-named behaviours remain
  visible; the page was one caption over the ceiling. Contract § 13
  previously required *standalone* groups for the two behaviours; PR review
  authorized a docs-only wording correction so § 13 now describes the
  required evidence set and explicitly allows type-in and keyboard bounds
  to share one group.

## Contract coverage

Preserved. Every behaviour each contract's § 13 names remains visible on
its page; three captions now name two behaviours each. One contract text
changed: `knob.md` § 13, an orchestrator-authorized docs-only wording
correction requested in PR review — it now describes the required evidence
set and explicitly permits type-in and keyboard bounds to share a group,
matching what the page teaches. No component implementation, public prop,
or token changed. No behaviour lost its only visible evidence.

## Pages intentionally left unchanged

- **AudioSwitch (6)** — inside the 3–6 target; the six groups are exactly
  contract § 13's list.
- **EnvelopeEditor (7 web / 9 GPUI)** — contract § 13 names nine items;
  the web pages fold add/remove into the selection group and curve nudges
  into the keyboard group, GPUI spells all nine. Pre-existing audit
  finding, not addressed here: the page overflows its pane by 32px at a
  768px viewport (a layout matter, outside this card's writable scope).
- **Fader (8)** — contract § 13's eight groups; each teaches a distinct
  behaviour (orientation, laws, detents, drag, automation, type-in,
  keyboard bounds, disabled).
- **Keyboard (6)** — inside the 3–6 target.
- **ModMatrixGrid (5)** — inside the 3–6 target.
- **ValueReadout (9)** — one group per contract-named format vocabulary
  (eight formats) plus the negative/boundary/disabled states group.
- **WaveformDisplay (6)** — inside the 3–6 target.
- **XYPad (8)** — contract § 13's eight groups.

## Validation

- Focused specimen evidence: `bunx vitest run --project parity --project
  svelte-preview --project react-preview` — 13 files, 403 tests passed,
  including `specimen-caption-parity` (Svelte and React captions verbatim
  equal on all eleven routes) and `specimen-axis-census` (twelve audio pages
  paired with populated axis tabs).
- `effigy catalogue:check` — pass.
- `effigy check:svelte` — 0 errors; the 8 warnings in 5 files are
  pre-existing (identical count on unmodified `main`).
- `effigy react:build` — pass.
- `effigy check:gpui` — pass.
- `effigy docs:check` — pass.
- `cargo test --manifest-path packages/render/Cargo.toml audio_specimens` —
  4 passed (axis panes carry no matrix; one representative per requested
  step; requested size/density reach the control).
- `git diff --check origin/main...HEAD` — clean.

No new failure caused by this branch; the pre-existing `effigy doctor` scan
findings named in the handoff were not part of the card and were not
touched.

## Operator review checkpoint

The changed pages (AudioMeter, Knob, GainReductionMeter in the Svelte and
React previews) require the operator's live review before this card can
close. Status: **pending**.
