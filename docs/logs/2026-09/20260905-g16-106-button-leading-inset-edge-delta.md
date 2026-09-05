# g16.106 — Button Leading-Inset Edge Delta Diagnosis

Status: complete — awaiting orchestrator review
Date: 2026-09-05
Card: `docs/roadmaps/g16/106-button-leading-inset-edge-delta.md`
Handoff: `docs/handoffs/20260905-085227-g16-106-button-leading-inset-edge-delta.md`
Governing refs: `packages/render/src/button.rs`,
`packages/render/src/presentation.rs`,
`test/visual/button-comparison/policy.ts`,
`docs/contracts/components/button.md` §12
Branch: `worker/g16.106-button-leading-inset-edge-delta`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-106-button-leading-inset-edge-delta`
Planning base: `9481cc95dbd65c1dff8c73a6b74b9504cf19b077` (`origin/main`)
Worker PR: https://github.com/inflatable-cookie/poodle/pull/211

## Outcome

The half-pixel leading edge is a GPUI rasterisation delta, not a Poodle
rounding defect. `poodle-render` emits the CSS inset exactly. The comparator
now classifies the two lab fixtures as `gpui-snaps-subpixel-edge`. Render
padding is unchanged. GPUI's paint path is unchanged.

## Diagnosis

Node inventory for the two lab fixtures (md/default, 16px rem base), printed
beside the Svelte CSS `calc(var(--poodle-space-control-x) - 0.125rem)`:

```
button/content-leading-icon: css pad_x=12 inset=2 pad_left=10 pad_right=12 | native token_pad_x=12 inset=2 emitted pad_left=10 pad_right=12
button/state-loading: css pad_x=12 inset=2 pad_left=10 pad_right=12 | native token_pad_x=12 inset=2 emitted pad_left=10 pad_right=12
button/rest-secondary: css pad_left=12 pad_right=12 | native emitted pad_left=12 pad_right=12
```

`rem_to_px` is `rem * 16.0` with no rounding. `LayoutEdges` is `f32`. The
md icon inset `0.125rem` is exactly 2px; pad_left is exactly 10px. The node
vocabulary does not drop the fraction because there is no fraction to drop.

## What landed

- Headless proof `leading_inset_fixtures_emit_the_css_padding` and the
  icon-inset rem ladder / fractional `rem_to_px` pins in `presentation.rs`.
- Policy finding `gpui-snaps-subpixel-edge` with fixtures
  `button/content-leading-icon` and `button/state-loading`. Classification
  is annotation-only; the geometry channel still fails at `rootEdge: 0.5`.
- Contract §12 row (ledger known-delta generator input). The generated
  Button known-delta cell stays `present`.
- `compare.test.ts` pins the g15.047 geometry/roles/pixels numbers and
  proves classification on the two fixtures only.

## Falsification

| Row | Plant | Result |
| --- | --- | --- |
| Diagnosis is evidenced | claim rasteriser without printed insets | test log prints CSS and native pad_left/inset side by side |
| Repair is exact | no render padding change | rest-secondary still emits 12/12; leading fixtures 10/12 |
| Contract is honest | role finding while emitted values differ | emitted values match; finding added |
| No tolerance creep | change `GEOMETRY.rootEdge` | `g15.047 policy numbers stay the g15.047 table` fails |

## Validation

- `cargo test -p poodle-render --manifest-path packages/render/Cargo.toml leading_inset_fixtures_emit_the_css_padding -- --nocapture` — pass; printed inventory above
- `cargo test -p poodle-render --manifest-path packages/render/Cargo.toml` — 641 passed
- `effigy test:visual-comparator` / `bun test test/visual/button-comparison/compare.test.ts` — 38 pass
- `effigy regressions:native` — 203 passed
- Nucleus cohort mechanical repin at `8623e926dbf32dc73c3a59fba494c3f7b55d7434`
  (g16.100/g16.105 precedent): 29 receipts and the manifest
  `source_commit` advanced; payloads differ only in that SHA. The generated
  ledger text was unchanged (Button known-delta cell already `present`).
- `effigy docs:check` — after the receipt commit
- `git diff --check origin/main...HEAD`

No `*-windowed` selector was run.

## Closeout

Reserved for the coordinator at merge: `docs/roadmaps/g16/README.md`,
`docs/roadmaps/generation-index.md`, and `docs/roadmaps/dispatch.md`.
