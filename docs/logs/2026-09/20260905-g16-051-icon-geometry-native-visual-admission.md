# g16.051 — Icon Geometry Native Visual Admission

Status: complete — IG-06 admitted all six candidate pairs in both directions;
awaiting fresh independent exact-head review
Date: 2026-09-05
Card: `docs/roadmaps/g16/051-icon-geometry-native-visual-admission.md`
Launch handoff: `docs/handoffs/20260905-g16-051-icon-geometry-native-visual-admission.md`
Closeout handoff: `docs/handoffs/20260905-g16-051-icon-geometry-native-visual-admission-closeout.md`
Branch: `worker/g16-051-icon-geometry-native-visual-admission`
Planning base: `3dbabac3990fb5f3856305b7c8f971039b0a81be` (`origin/main`)
Lab bundle: `poodle-lab/docs/logs/2026-09/05-111446-g01-002-icon-geometry-batch-bundle`
Lab run: `2026-09-05T11-14-46` (`g01.002`)
Worker PR: https://github.com/inflatable-cookie/poodle/pull/217
Revision review: BLOCK comment https://github.com/inflatable-cookie/poodle/pull/217#issuecomment-5551704145

## Outcome

IG-06 is admitted for all six Poodle candidate pairs, in both directions.
The evidence is native visual evidence for the internal fixture cohort only.
The six registry entries remain `status: "candidate"`; no runtime eligibility,
public contract, visual ledger, production source, or `IconMorph` surface was
changed. IG-07 remains a separate planning decision.

The lab bundle is closed and complete: 84 fixtures (6 pairs × 2 directions ×
7 states), 252 admitted captures (three runtimes × two repeats), and 168
Svelte-anchored comparison channels. The initial mechanical result was 156/168
channels passed with 18 findings. The findings are adjudicated below; none
blocks this internal admission.

## Evidence and provenance

- Poodle capture source: `85609d941a208ff2f854e9f7c0e457089cc77d0e`
  (`g16.105`, PR #210). That commit is an ancestor of the worker base, and
  the relevant morph-pair, private shell, render, GPUI capture, and font paths
  are unchanged at `3dbabac39`.
- The whole comparison-policy module is not unchanged: `test/visual/button-comparison/policy.ts`
  changed at `94febafad53aa4e3feff5a28c82fcf20ccc0b6da` (`g16.106`) between
  the lab pin and the Poodle base. That change adds the Button-only
  `gpui-snaps-subpixel-edge` known-delta entry, its two-fixture registry, and
  classifier context. The icon comparator does not invoke that Button
  known-delta/classifier path.
- The icon comparison basis remains the unchanged numeric table consumed by
  the lab's icon comparator: Svelte↔React is exact; Svelte↔GPUI geometry uses
  `rootEdge: 0.5`, `contentCentre: 1`, and `contentSize: 1` logical px;
  roles use `colorChannel8Bit: 1` and `lineWidth: 0.5`; pixels use
  `threshold: 0.1`, `includeAA: false`, and `maxDiffRatio: 0.03`. The
  `g16.106` module change does not alter those values. This record claims
  numeric-policy continuity for the icon run, not whole-file policy identity.
- Lab bundle directory digest: `f3404acd3fd6fd69208e36371f01c8afe5e7cf8c746b456be43c3d266bfa1ed6`.
- Longhorn: `168ecc72be5d8643afe6e1246f080cc1c07701ab`.
- Bundle schema: `poodle-lab.icon-geometry-run.v1`; `closedBatch: true`;
  admission reported by the lab: `none`.
- Capture law: 128×128 logical, 2× scale / 256×256 device, frozen motion,
  no crop, no frame picking, no averaging, two exact agreeing repeats.
- Every capture was process-bounded, unfocused/non-activating, and foreground
  proved with `sh.paseo.desktop`; the lab recorded no failed foreground reads.
  Teardown receipts recorded `tornDown: true`, zero live clocks, and no icon
  landmark. The web teardown hash was `64e0b3264b41…`; GPUI teardown was
  `bb58af9d03d1…`.
- Svelte↔React was exact on all 84 fixtures. Svelte↔GPUI dimensions, geometry,
  and pixels passed on all 84; pixel ratios stayed below the 3% policy limit.

## Pair and direction review

The Poodle-owned review used the generated canonical contours, recorded
correspondence, midpoint frame, reverse midpoint, interruption, and the
native/web evidence. Each row below represents seven fixtures, 42 admitted
runtime captures, and 14 comparison channels (two runtime pairs per state).

| Pair | Direction | Intermediate geometry review | Seven-state result | Verdict |
| --- | --- | --- | --- | --- |
| `arrow-down-to-arrow-up` | forward | Two open contours preserve the stem and arrowhead roles; the heading changes without contour crossing or invention. | 13/14 mechanical channels passed; teardown role finding adjudicated as capture metadata. | admitted |
| `arrow-down-to-arrow-up` | reverse | Same correspondence traversed toward the opposite endpoint; reverse midpoint is distinct and remains a two-contour arrow. | 13/14 mechanical channels passed; teardown role finding and reverse/frozen oracle finding adjudicated below. | admitted |
| `arrow-left-to-arrow-right` | forward | Two open contours preserve the horizontal stem and head; the turn is bounded and does not twist or duplicate contours. | 13/14 mechanical channels passed; teardown role finding adjudicated as capture metadata. | admitted |
| `arrow-left-to-arrow-right` | reverse | Reverse traversal preserves the same two-contour topology and target-facing arrowhead. | 13/14 mechanical channels passed; teardown role finding and reverse/frozen oracle finding adjudicated below. | admitted |
| `chevron-left-to-chevron-right` | forward | One open two-segment contour passes through a straight vertical midpoint, with no extra contour or self-crossing. | 13/14 mechanical channels passed; teardown role finding adjudicated as capture metadata. | admitted |
| `chevron-left-to-chevron-right` | reverse | Reverse traversal is the same bounded one-contour path in the opposite direction; reverse midpoint differs from midpoint. | 13/14 mechanical channels passed; teardown role finding and reverse/frozen oracle finding adjudicated below. | admitted |
| `circle-to-dot` | forward | One closed contour shrinks concentrically; closure is preserved and no off-centre travel or collapse artefact is visible. | 13/14 mechanical channels passed; teardown role finding adjudicated as capture metadata. | admitted |
| `circle-to-dot` | reverse | Reverse expansion remains one closed concentric contour and returns to the latest target on freeze. | 13/14 mechanical channels passed; teardown role finding and reverse/frozen oracle finding adjudicated below. | admitted |
| `ellipsis-to-ellipsis-vertical` | forward | Three independent closed dot contours relocate through a bounded diagonal midpoint; no merge, split, or invented contour. | 13/14 mechanical channels passed; teardown role finding adjudicated as capture metadata. | admitted |
| `ellipsis-to-ellipsis-vertical` | reverse | Reverse relocation preserves three closed contours and reverses the same correspondence without crossing. | 13/14 mechanical channels passed; teardown role finding and reverse/frozen oracle finding adjudicated below. | admitted |
| `plus-to-x` | forward | Two open line contours rotate around the shared centre into diagonals; no contour merge, split, or unintended global translation. | 13/14 mechanical channels passed; teardown role finding adjudicated as capture metadata. | admitted |
| `plus-to-x` | reverse | Reverse rotation returns along the same two-line correspondence and freezes at the reverse target. | 13/14 mechanical channels passed; teardown role finding and reverse/frozen oracle finding adjudicated below. | admitted |

The midpoint review also retained these generated-plan anchors: arrow pairs
`542181` cost micros, chevron `27446`, circle/dot `2867359`, ellipsis
`1748569`, and plus/x `583397`. Numeric cost was used only as a traceable
plan anchor; the admission decision is based on the explicit geometry review
and the complete native evidence.

## Finding adjudication

### 12 teardown stroke-width findings

The findings cover exactly the six pairs × two directions at
`svelte-gpui/.../teardown`. Every other teardown channel passed: Svelte↔React
was exact, dimensions/geometry/pixels passed, and both repeats agreed. The
teardown receipts proved no live handle, no icon landmark, and an empty scene.

The 2 logical-pixel difference is a teardown role-measurement artefact. The
web host measures no stroke after the icon is removed, while the GPUI adapter's
sanitized receipt retains the fixed icon stroke role (`width: 2`) even though
its capture scene is the empty container. It is not a GPUI paint-width defect:
the GPUI teardown path clears the resolved node to a container, and the pixel
channel remains within policy. It is not a contracted visual delta: the
contracted `ROLES.lineWidth` limit stays `0.5`, and no known-delta or visual
ledger entry is added. The lab files are read-only; this Poodle record retains
the evidence and classification.

### 6 reverse/frozen oracle findings

The findings cover exactly the six `svelte` reverse/frozen fixtures. The lab
oracle expected `endpoint-to`, but its own retained result says forward frozen
matched `endpoint-to` and reverse frozen matched `endpoint-from`. In a reverse
fixture, `endpoint-from` is the latest semantic target after the reversal.
Architecture 012 requires frozen motion to snap to the latest semantic
endpoint, not to a direction-blind field name. Therefore all six findings are
oracle expectation errors, not runtime defects. The Poodle adjudication is to
correct the expectation for reverse fixtures to `endpoint-from` in the next
lab manifest revision; no lab file is edited in this PR.

## Admission decision

Admit all six candidate pairs in both directions to the IG-06 internal visual
evidence cohort. The result is complete because every fixture/state/runtime
was captured twice, repeats agreed, provenance and foreground were proved,
the pair geometry was reviewed, and both finding classes have evidence-backed
dispositions. Keep the authored registry and private runtime fixture-only;
public IconMorph admission is not part of this card.

## Validation

- `bun test packages/core/test/icon-geometry.test.ts packages/core/test/icon-geometry-registry.test.ts packages/core/test/icon-geometry-runtime.test.ts` — pass (33 tests, 1,248 expectations)
- `cargo test --manifest-path packages/contracts/components/Cargo.toml --test icon_geometry` — pass (5 tests)
- `cargo test --manifest-path packages/render/Cargo.toml --lib icon_geometry` — pass (5 tests)
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test icon_geometry_headless -- --test-threads=1` — pass (6 tests)
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --bin poodle-window-capture --features window-capture icon_geometry` — pass (5 tests)
- `effigy audit:icons` — pass (108 default icon names verified)
- `effigy docs:lint` — pass
- `effigy docs:check` — pass
- `git diff --check origin/main...HEAD` — pass after final commit

No local `*-windowed`, native-visual, capture, release, workflow, or consumer
selector was run.

## Closeout

The next step is independent exact-head review of the pushed PR. The reviewer
must verify this log against the card, the complete diff, the retained bundle
digest, each pair/direction row, and both finding adjudications. Do not merge
from this lane.
