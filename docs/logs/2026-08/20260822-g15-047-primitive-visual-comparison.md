# g15.047 Primitive Visual Comparison

Date: 2026-08-22
Card: `../../roadmaps/g15/047-primitive-visual-comparison.md`
Parent: `../../roadmaps/g15/012-visual-conformance-lane.md`
Handoff: `../../handoffs/20260822-130425-g15-047-primitive-visual-comparison.md`
Worker branch: `t3code/g15-047-primitive-visual-comparison`
Worker worktree: `/Users/tom/.t3/worktrees/poodle/g15-047-primitive-visual-comparison`
(created via the handoff's manual fallback: the launcher supplied the `main`
planning checkout, the operator chose the fallback, and
`AGENTS_WORKTREE_CONTAINER_DIR` from `.agents.local.env` selected the
container; the worktree is registered, clean, and non-`main`)
Planning base: `41d3a4fdcfc9602b94eba6784af3ce45b5791f14`, an ancestor of the
branch point `88eca613c91c8b9194d97e38fcfddc17ece05f5f` (= `origin/main` at
dispatch, which contains the handoff itself)

## Outcome

The 18 accepted Button fixtures now render through all three real runtimes —
Svelte, React (private capture-only hosts driven by pinned headless Chromium),
and GPUI (`poodle-offscreen-capture --fixture`, Metal headless, no window) —
from the unchanged canonical inventory, each captured twice with byte-identical
repeats, every PNG verified against a typed `poodle.button-visual-capture.v1`
receipt before any comparison.

**Svelte ↔ React passes the exact policy on all 18 fixtures**: identical
dimensions, zero landmark edge delta, exactly equal role evidence, zero
differing pixels — the two web shells are pixel-identical on every case.

**Svelte ↔ GPUI passes the fixed renderer-aware policy on geometry, fill,
border, text, and pixels on all 18 fixtures, and stops red on two reported
categories** (below). Three real cross-runtime drifts were measured and
repaired under Bounded Repair Authority; the initial measurements are
preserved in `assets/g15-047/initial-mismatches.json`.

The comparator exits non-zero today, by design: the fixed policy is not met
(the named focus-ring blocker), and the shadow known-delta findings — though
contract-cited — remain blocking, because the card did not authorize a second
acceptance layer. Changing exit semantics is an orchestrator card decision.

## The named blocker: GPUI focus indicator is not the contract's focus ring

Web paints the contract §8 focus ring as a 2px (`border-width-focus`) outline
in `accent-focusRing` with a 2px offset. GPUI's focus treatment is the
existing 1px border recoloured to the same `accent-focusRing` — the
`StylePatch` focus channel carries no width and GPUI has no outline primitive.
Measured honestly on every focusable fixture (16 of 18; `state-disabled` and
`state-loading` declare no ring on either side, and pass):

```
roles | focus-ring width: web 2 vs gpui 1 (delta 1 > 0.5 logical px)
```

The focus-ring colour matches within tolerance everywhere. The width delta is
a real parity gap the contract's §12 known deltas do not cover, and repairing
it means growing a native focus-ring mechanism — a renderer-architecture /
contract decision the card reserves for the operator. Nothing is allowlisted:
the finding is reported per fixture in `summary.json` and the contact sheet,
and the run stays red until the operator rules.

## What was measured and repaired

Initial full-batch run (preserved): 78 blocking findings. Final run: 32
blocking findings — 16 the named focus-ring blocker, 16 the contract-cited
shadow absence (annotated, still blocking; see the review-changes section).

| # | Initial mismatch (measured) | Contract authority | Repair | Focused regression |
| --- | --- | --- | --- | --- |
| R1 | GPUI secondary default fill = raw `surface` token (0.067 vs web 0.171, ~26/255 off on 13 fixtures, both themes) | §8: `color-mix(surface 88%, text-primary)` elevation stacking | `packages/render/src/button.rs`: the **idle** fill now mixes 88% toward `text_primary`. (The first revision of this PR also moved hover/active to the contract's 80/84% stacking; the orchestrator review reverted that as unmeasured — see below.) | `secondary_default_idle_fill_is_elevation_stacked` |
| R2 | GPUI primary border darkened 86% (2.4–4.8/255 off); pressed non-primary border 86% | §8: `color-mix(accent-base 84%, black)`; pressed: 85% | same file: ratio corrections | `primary_border_is_the_darkened_fill`, `pressed_non_primary_toggle_takes_the_accent_treatment` |
| R3 | GPUI icon/spinner reserved only the 12px glyph box; web reserves the 16px wrapper, so labels sat 2px left (`content centre.x` delta 2.0) and icon boxes read 12 vs 16 | §8 icon wrapper: `size.icon.md` (16px) | same file: icons and spinner ride in a 16px `Container` with the 12px glyph centred (leading, trailing, spinner; glyph size unchanged) | `icons_and_spinner_ride_in_the_icon_md_wrapper_box` |
| — | xl Button captured 48px tall, not 52px | §8 size ladder | **harness scene bug, not a component defect**: the GPUI scene's column-flex shrank the button to the 48px content box; scene is now row-flex like the web host | covered by the `size-xl` geometry channel itself |
| — | capture receipt claimed GPUI revision `1ea16c1…` while the manifest pins the fork `87d9afb…` (g15.051 fallout) | handoff: receipts must record the exact current fork revision | `offscreen_capture.rs` + smoke script constant | the bin's `revision_constant_matches_manifest_pin` test now passes |

All three component repairs are inside `poodle-render`'s Button, the smallest
surface the contract already decides; every GPUI Button renders through
`poodle_render::button`, so the repair covers the whole native surface. The
one downstream consumer that pattern-matched Button children
(`agent_transcript`'s jump-control test) was updated to look through the
wrapper. `cargo test -p poodle-render`: 372/372.

Reported, not repaired (visible in every output, none per-fixture):

- **Shadow layer count** (16 fixtures): web secondary/primary paint the §8
  shadows; GPUI paints none — the contract §12 known delta
  `gpui-omits-box-shadow`. Per the orchestrator review, classification is
  annotation only: these findings carry their citation in the summary and
  contact sheet **and remain blocking** under the fixed policy (layer
  count/inset are exact). Excusing them needs an orchestrator card change with
  coherent aggregate semantics, not a runner edit.

Measurement-semantics decisions (comparator definitions, not component
changes), each verified against painted pixels:

- `content` is the label's **layout box** (advance width × font-size ×
  line-height, centred) on both runtimes — not GPUI's glyph ascent/descent
  box. Centres and widths then agree (width delta ≈0.4px, the known
  letter-spacing omission); the initial glyph-box definition produced a 2.7px
  height mismatch on every label fixture.
- `icon`/`spinner` measure the **rendered part** (web `svg` / `.poodle-spinner`
  host, GPUI glyph node), not the web-only 16px wrapper span; the wrapper's
  contract force is observable through root/content geometry.
- The dormant focus ring is what the runtime could paint **in the captured
  state**: an unfocusable (disabled/loading) control declares no ring on
  either side.

## The pixel cap separates antialiasing from drift

Pre-repair, real role drift produced 5.6–12.9% pixel differences (over the 3%
cap). Post-repair, the largest Svelte↔GPUI diff is **0.133%**
(`button/state-loading`) — residual rasterizer noise around glyphs and the
shadow absence, an order of magnitude under the cap. The fixed policy needs no
widening.

## Determinism

Every runtime/fixture captured twice per run, fresh navigation or fresh
process between: 54/54 repeat pairs byte-identical (`repeatMismatches: 0`),
in every run including the validation reruns. The loading spinner is frozen at
its declared initial frame on all runtimes (CSS animation freeze + Playwright
`animations: "disabled"`; GPUI `set_reduce_motion(true)`). Both sides
rasterise text from the same bundled Inter TTFs — web via `@font-face` in the
fixture hosts, GPUI via `text_system().add_fonts` — so glyph differences mean
text-stack differences, not missing fonts.

Two harness-level faults were found and fixed during integration (both would
have poisoned evidence silently):

- **Theme race (web hosts)**: theme attributes applied post-paint let the
  Button's unthemed→themed CSS transition run during measurement —
  `getComputedStyle` read mid-transition values while the settled pixels were
  correct (React receipt once reported fill 0.36–0.48 against a byte-identical
  PNG). Hosts now apply theme attributes before first paint (Svelte init
  block, React `useLayoutEffect`); verified stable across repeated runs.
- **Page degradation**: a single Playwright page stops settling after ~15–20
  SPA navigations. The harness now recycles pages on a fixed cadence and
  restarts a dead preview once per capture failure — infrastructure recovery,
  never frame picking; the byte-identity rule is untouched. Recorded in
  `PAPERCUTS.md`.

## Negative evidence

`compare.test.ts` (26 tests) plants the card's failure set through the
production compare/verify functions, in memory: missing capture (capture-set
completeness, named by fixture and runtime), two-logical-pixel root shift
(geometry fails while pixels pass — channel independence), missing icon/
spinner landmark (receipt verifier, exact fixture name), changed role colour,
changed shadow (layer count, and a non-empty layer-geometry delta that must
NOT classify as the known delta), PNG tamper (hash and dimension checks), a
pixel change beyond 3% (fails) beside a 1% change (passes), duplicate/extra
captures, repeat-capture divergence, and the recovery boundary (a repeat
mismatch or receipt failure can never enter the batch's retry branch; only a
pre-capture transport failure can). The GPUI bin carries 18 unit tests
(arg contract, publish safety, revision drift, id-stamped icon bounds); the
moved Rust inventory parser keeps all 15 planted-fault tests green; the
TypeScript inventory suite (43 tests) now names the loader's new home and its
one sanctioned non-test consumer.

## Source cost

| file | lines | note |
| --- | --- | --- |
| `test/visual/button-comparison/receipt.ts` | 412 | closed receipt schema + PNG/hash verification |
| `test/visual/button-comparison/compare.ts` | 427 | exact + renderer-aware comparator |
| `test/visual/button-comparison/policy.ts` | 121 | the card's fixed table + known-delta registry |
| `test/visual/button-comparison/capture-set.ts` | 91 | completeness + repeat rule + recovery boundary |
| `test/visual/button-comparison/capture-web.ts` | 482 | Playwright capture driver |
| `test/visual/button-comparison/capture-gpui.ts` | 109 | one-shot binary driver |
| `test/visual/button-comparison/contact-sheet.ts` | 123 | operator review surface |
| `test/visual/button-comparison/run.ts` | 312 | batch orchestration + summary |
| `test/visual/button-comparison/compare.test.ts` | 483 | 26 planted-failure tests |
| `test/visual/button-comparison/README.md` | 82 | boundary documentation |
| `packages/svelte/preview/src/fixture-host/FixtureHost.svelte` | 281 | private capture host (+5-line `main.ts` branch) |
| `packages/react/preview/src/fixture-host/FixtureHost.tsx` | 293 | mirror (+11-line `main.tsx` branch) |
| `packages/gpui/preview/src/bin/offscreen_capture.rs` | 750 (was 523) | fixture-mode dispatch, revision fix, 18 tests |
| `.../offscreen_capture/inventory.rs` | 801 | the g15.046 parser **moved** here + typed decode layer |
| `.../offscreen_capture/fixture_capture.rs` | 642 | fixture scene, landmarks, roles, receipt |
| `packages/gpui/preview/tests/visual_fixture_inventory.rs` | 495 (was 1,063) | parser included by path; planted faults unchanged |
| `packages/render/src/button.rs` | +124/−14 | the three bounded repairs + regressions |
| `packages/gpui/node-backend/src/lib.rs` | +8/−2 | id-stamped svg leaves record paint bounds |
| `tasks/effigy.tasks.toml` | +12 | `test:visual-button-comparison` selector |

The mechanism is Button-only: receipt and comparison types are closed to
unknown fields, the hosts map only the accepted fixture fields, and there is
no generic schema, registry, codegen, baseline, or refresh command.

## Duplicated registry count

g15.046's 12 duplicated lists + 1 rule stand unchanged — the Rust parser was
**moved**, not copied, so no third parser exists. The receipt schema is held
twice (TypeScript verifier ↔ Rust producer), the same posture as the
inventory:

1. the schema discriminator `poodle.button-visual-capture.v1`
2. the receipt root key set (10)
3. the runtime names (3)
4. the landmark-bounds key set (4)
5. the roles key set (5)
6. the shadow-layer key set (6)
7. the web environment key set (2)
8. the GPUI environment key set (4)
9. the fixture file-stem rule (`/` → `--`, also duplicated inside TypeScript
   between the two capture drivers)

**9 duplicated lists.** Held honest the same way as the inventory: the
verifier rejects unknown keys, and the GPUI receipts pass the TypeScript
verifier in every run — a schema drift on either side fails closed. The
cross-language check is real here (unlike the inventory selectors): the Rust
producer's output is parsed by the TypeScript verifier on every batch.

## Final metrics

| metric | value |
| --- | --- |
| fixtures | 18 (canonical order, unchanged inventory) |
| captures | 54 verified PNG/receipt pairs; 0 repeat mismatches |
| comparisons | 36 (18 exact + 18 renderer-aware) |
| Svelte↔React | all channels pass on all 18; zero differing pixels |
| Svelte↔GPUI geometry | pass on all 18 |
| Svelte↔GPUI roles | pass except: the named focus-ring blocker (16 fixtures) and the annotated-but-blocking shadow absence (16 fixtures) |
| Svelte↔GPUI pixels | pass on all 18; max diff 0.133% of viewport (cap 3%) |
| known-delta findings | 16, all `gpui-omits-box-shadow` (contract §12) — annotated, still blocking |
| blocking failures | 32 (16 focus-ring blocker + 16 shadow) |
| environment | chromium 151.0.7922.34; gpui `87d9afbe…` (macos/aarch64, metal-headless) |

## Environment and evidence

Committed review evidence under `docs/logs/2026-08/assets/g15-047/`:
`summary.json` (full machine-readable verdicts, per-channel metrics, hashes,
policy echo), `contact-sheet.html` (all 54 captures at native device scale in
canonical order, both diffs per fixture, per-channel verdicts with known-delta
citations), `captures/` (54 PNG + 54 receipts), `diffs/` (36), and
`initial-mismatches.json` (the pre-repair summary, preserved before any
repair). These are point-in-time review evidence; the comparator never reads
them, and no update/refresh command exists.

## Dependency and task impact

- **New dependencies:** none, in any language.
- **Package metadata / versions:** unchanged.
- **Cargo:** no target registration changes; the capture bin gained two
  sibling modules.
- **Effigy:** one new selector, `test:visual-button-comparison` (focused
  comparator tests + the full headless batch into a disposable directory). Not
  composed into `ci:*` or `qa` — gate composition remains the orchestrator's
  call, and the selector currently exits non-zero on the blocking findings by
  design.
- **GPUI revision receipts** now record the actual fork pin `87d9afbe…`; the
  pre-licence-fix upstream SHA is gone from the bin and the smoke script.
- **`PAPERCUTS.md`:** one entry (page-recycling/preview-restart knowledge for
  new capture harnesses).

## Review changes (PR #68, round 1)

The orchestrator review requested four changes; all landed before this
evidence set was generated:

1. **Subset mode removed from the batch runner.** `run.ts` accepted
   `--fixture` and ran the completeness check over the filtered list, so a
   one-fixture run could report a "complete" batch. The batch is now always
   the fixed 18 fixtures; unknown arguments are rejected. `--fixture` remains
   on the `poodle-offscreen-capture` one-shot target only.
2. **Capture recovery can no longer retry evidence failures.** The web batch
   loop previously caught every capture error — including a repeat mismatch —
   and retried once. Recovery is now gated on one exported predicate
   (`isRecoverableTransportError`) that admits only the new
   `PreviewTransportError` class (the pre-capture navigation/marker phase).
   Repeat mismatches and receipt failures throw `CaptureIntegrityError` and
   rethrow immediately. Four regression tests drive the production predicate
   with the exact error classes the capture paths throw.
3. **Known-delta classification no longer changes exit semantics.** The
   runner previously subtracted contract-cited findings from the blocking
   count. Classification is now annotation-only (citation in summary, contact
   sheet, console); every fixed-policy finding blocks.
4. **Unmeasured hover/active repair reverted.** The R1 repair now covers only
   the measured idle fill; secondary-default hover/active keep the old-tier
   formula (mix from the final fill toward `elevated`), which now computes
   from the repaired idle fill. The contract §8 text-primary stacking for
   hover/active joins the unmeasured-suspicion record.

## Validation

| check | result |
| --- | --- |
| `bun test test/visual/button-comparison/compare.test.ts` | 26 pass, 0 fail |
| `effigy test:visual-fixtures` | pass (43 bun + 15 cargo; updated loader path) |
| `effigy test:visual-button-comparison` ×2 | full batch ran end to end twice; byte-identical summary hashes (`1531b935…`), identical metrics (54 captures, 0 repeat mismatches, 32 blocking, 16 annotated known-delta); exit 1 by design on the blocking findings |
| `effigy smoke:gpui-offscreen-capture` | pass (legacy smoke unchanged, corrected revision) |
| `cargo test -p poodle-render` | 372 pass, 0 fail |
| `cargo test --bin poodle-offscreen-capture --features capture` | 18 pass, 0 fail |
| `cargo test --test headless_regressions` | 56 pass, 0 fail |
| `cargo test -p poodle-gpui-node-backend` | 24 pass, 0 fail |
| `effigy check:svelte` | 0 errors |
| `effigy react:build` | pass |
| `effigy check:gpui` | pass |
| `effigy docs:check` | pass |
| `git diff --check origin/main...HEAD` | clean |

No `*-windowed` selector, `test:native-visual`, GPUI preview window, Jetstream
selector, release mutation, tag, publication, or workflow edit ran.

## What this cannot prove

- Nothing about component completion. The mechanism is diagnostic; it cannot
  mark Button — or anything — done, and the named blocker stays open until
  the operator rules.
- Nothing about hover/active/focused frames. The batch captures rest,
  disabled, loading, and pressed only. Unmeasured contract suspicions were
  noted but deliberately not repaired (outside Bounded Repair Authority
  without a measurement): GPUI's secondary-default hover/active stacking, its
  primary hover/active fills mixing toward `elevated` rather than the
  contract's white/black mixes, and its primary hover border ratio.
- Nothing about a second component, other themes beyond eclipse/iceberg, or
  cross-machine native determinism (Core Text glyphs are per-host; receipts
  record the environment, same-run comparison only).
- Nothing about Jetstream, which is program-deferred.

## Continuation

The PR stops for review: the orchestrator reviews scope, adapters, receipt
integrity, determinism, planted failures, fixed thresholds, mismatch handling,
and repairs; the operator reviews all 54 captures and every native tolerance
through the contact sheet, and rules on the named focus-ring blocker. Merge
requires explicit operator authorization. No fixture-roster expansion, second
component, or Longhorn-lab work happened here.

## Operator verdict

_pending — awaiting operator visual review and a ruling on the named blocker._
