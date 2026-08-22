# g15.052 Native Focus-Ring Parity

Date: 2026-08-22
Card: `../../roadmaps/g15/052-native-focus-ring-parity.md`
Handoff: `../../handoffs/20260822-215150-g15-052-native-focus-ring-parity.md`
Worker branch: `t3code/g15-052-native-focus-ring-parity`
Worker worktree:
`/Users/tom/.t3/worktrees/poodle/g15-052-native-focus-ring-parity` (manual
fallback per the handoff: the launcher supplied the `main` planning checkout
and no `g15-052` worktree existed, so the worktree was created under the
operator-selected `AGENTS_WORKTREE_CONTAINER_DIR` from `.agents.local.env`;
registered, clean at start, non-`main`)
Planning base: `62b4da1337c356aac8918a5b3977e29a542bff54`, an ancestor of the
branch point `2ed4421932e28ed4b3db76529de16f876930bc55` (= `origin/main` at
dispatch, containing the handoff itself)

## Outcome

One reusable focus-ring capability joined the shared node vocabulary, GPUI
projects it faithfully, and both measured native gaps closed with it:

- **Button:** the 18-fixture comparator loses all 16 focus-ring findings
  under the unchanged fixed policy. The batch now reports exactly the 16
  annotated `gpui-omits-box-shadow` findings — still blocking, still cited —
  and nothing else. Policy, thresholds, fixtures, receipt identity, and
  known-delta classification are untouched.
- **Stepper:** the trigger, rerun, and summary controls carry stable tracked
  focus identities and declared sequential focus stops, so keyboard entry
  reaches them with no prior pointer press, the contracted ring paints while
  focused (inset on the summary), and `Enter`/`Space` activates the focused
  action. Selection, rerun, collapse, and pointer behavior from `g15.042`
  are unchanged (its two mounted regressions pass unmodified).

## The channel

`poodle-node` gains one typed value on `NodeStyle`, default absent:

```rust
pub struct FocusRing { pub color: ColorValue, pub width: f32, pub offset: f32 }
```

It is deliberately not a `StylePatch`: a ring is an out-of-flow affordance
that must preserve the resting border, must work on a borderless control, and
must not move layout. `StylePatch::focus` remains for other focused visual
changes; the two compose. `offset` is signed — positive draws outside the
border box (Button, Stepper trigger/rerun: +2px), negative insets (Stepper
summary: −2px, contract §8). Jetstream receives the field through normal
compilation and does not project it; it remains program-deferred.

## The GPUI paint mechanism

A declared ring is sufficient for focus tracking (`tracks_focus`), which is
what creates the node's real, retrievable `FocusHandle` — previously a
borderless `focusable` control got none, so keyboard entry depended on a
pointer press. The tracked handle now also carries the node's declared
`a11y.tab_index` behavior: gpui default-creates handles with `tab_stop` off,
and once `track_focus` attaches, the handle's flags — not the element
refinement's — decide traversal. Without this the tracked controls were never
tab stops at all.

Painting is a canvas child of the focused element, not a style refinement:

- The canvas is absolutely positioned and anchored at the element's top-left
  inset, so its bounds ARE the padding box (an unanchored absolute child
  would sit at the justify-static position instead — the g15.047 capture
  code documents that shift). The border box is one border-width outward per
  side; per-side border widths compose as they do in `apply_paint`.
- While — and only while — the real handle reports `is_focused` in the paint
  pass, the canvas paints one `PaintQuad` (`window.paint_quad`): bounds =
  border box outset by `offset + width`, transparent fill, border width =
  ring width, border color = ring color. gpui draws quad borders inside the
  quad's bounds, so the painted band covers exactly `[offset, offset+width]`
  from the border-box edge — CSS `outline` + `outline-offset`, including the
  negative-offset inset case.
- Corner radii are concentric: each element radius grows by the same
  `offset + width` expansion, so the ring's inner edge parallels the border
  box instead of rounding harder or going square.

Why this preserves everything else: the ring never touches the element's
style refinement, so the resting border, shadow stack, radius, and fill are
structurally undisturbed (a unit test pins the shadow refinement intact with
a ring declared); being a child quad painted after the element, it cannot be
overwritten by gpui's hover-after-focus refinement order (a mounted test
hovers a focused node and the ring survives); being absolutely positioned,
layout never sees it (the comparator's geometry channel passes on all 18
fixtures). Focus and blur are observed from the real handle both ways — the
paint pass also records what it painted per element id
(`painted_ring_for`, same observation posture as `bounds_for`), which is what
the mounted proofs and the evidence receipts read.

## Button closure

`packages/render/src/button.rs` declares the ring on enabled buttons —
`accent.focusRing`, `border.width.focus` (resolves 2px), offset
`rem_to_px(0.125)` = 2px — replacing the focus-time border recolour. No
idle/hover/active/pressed/disabled/loading/layout output changed; disabled
and loading buttons declare no ring, matching the web's dormant-ring absence.
The receipt observation in `fixture_capture.rs` learned the new field
(`color`/`width` read from `style.focus_ring`, `status: "dormant"` unchanged);
the closed receipt schema, its key sets, and the TypeScript verifier are
untouched.

Comparator verdict, unchanged fixed policy, disposable output
(`test/visual/button-comparison/out`):

| | pre-repair (g15.047 final) | post-repair |
| --- | --- | --- |
| captures | 54 verified, 0 repeat mismatches | 54 verified, 0 repeat mismatches |
| Svelte↔React | all channels pass, 18/18 | all channels pass, 18/18 |
| Svelte↔GPUI geometry | pass 18/18 | pass 18/18 |
| Svelte↔GPUI pixels | pass 18/18 (max diff 0.133%) | pass 18/18 |
| focus-ring findings | **16 blocking** (web 2 vs gpui 1) | **0** |
| shadow findings | 16 annotated, blocking | 16 annotated, blocking — unchanged |
| blocking total | 32 | 16 (shadow only), exit 1 by design |

The remaining red is exactly the operator-accepted `gpui-omits-box-shadow`
limitation for v0.2.0 — visible, cited, unabsorbed.

## Stepper closure

`packages/render/src/stepper.rs` applies the channel to the three contracted
controls only, each with a stable id keyed by step value
(`poodle-stepper:trigger:<value>`, `poodle-stepper:rerun:<value>`, the
summary's existing `poodle-stepper-summary`), `a11y.tab_index = Some(0)` (the
web buttons' implicit sequential stop, contract §6), and the ring: +2px
offset on trigger and rerun, −2px inset on the summary. Two radius notes:
the trigger carries `radius-control` so its ring corners inside the track per
contract §8 (invisible at rest — the trigger background is transparent), and
the summary now carries its contracted resting `radius-control` (contract §8
states it; the native node omitted it), which also rounds the summary's hover
fill as the contract intends. Disabled steps declare neither ring nor tab
stop. No resting border was invented.

Mounted proofs in `packages/gpui/preview/tests/headless_regressions.rs`
(`effigy regressions:native`), all through the real backend, no pointer:

- `stepper_keyboard_entry_focuses_and_activates_without_a_pointer_press` —
  tab-stop traversal reaches the trigger, the ring paints (width 2, offset
  2), `Enter` selects; traversal moves to the rerun in contract order, the
  trigger's ring clears, `Space` re-runs without selecting; traversal
  continues to the next step; blur clears the ring.
- `stepper_summary_takes_keyboard_entry_and_paints_the_inset_ring` — the
  summary is reachable by traversal, paints the inset ring (offset −2), and
  `Enter` toggles collapse.
- `a_declared_ring_paints_outside_a_bordered_node_only_while_focused` /
  `a_borderless_node_paints_the_declared_ring_without_a_resting_border` —
  the channel's proof nodes: exact painted ring geometry against a known
  border box, focus/blur, hover composition, resting-border preservation.
- Backend unit tests pin the new tracking rule (a declared ring is
  sufficient; bare `focusable` stays untracked) and shadow composition.

## Evidence

`docs/logs/2026-08/assets/g15-052/` — point-in-time review evidence, never a
baseline. A new closed third mode of the capture bin
(`--focus-evidence <button|stepper-trigger|stepper-summary>`) renders the
scene headless (Metal, no window), moves real focus through the backend
registry, refuses to capture unless the backend reports focus held AND the
ring painted, and writes PNG + typed receipt
(`poodle.gpui-focus-evidence.v1`, standalone — not part of the comparator's
verified schema). `contact-sheet.html` frames the three captures:

- `button.png` — resting vs focused bordered Button (sha256 `df21a076…`)
- `stepper-trigger.png` — borderless trigger, outset ring (`dc952fd7…`)
- `stepper-summary.png` — summary, inset ring (`e0e6a83a…`)

Every scene captured twice: byte-identical repeats (hashes above match the
committed receipts). The g15.047 assets are untouched.

## Validation

| check | result |
| --- | --- |
| `cargo test -p poodle-node` | 4 pass, 0 fail |
| `cargo test -p poodle-render` | 373 pass, 0 fail |
| `cargo test -p poodle-gpui-node-backend` | 26 pass, 0 fail |
| `effigy regressions:native` (mounted headless) | 60 pass, 0 fail |
| `cargo test --bin poodle-preview specimen_probe` | 8 pass, 0 fail |
| `effigy test:visual-button-comparison` | comparator tests 26 pass; batch: 54 captures, 0 repeat mismatches, **0 focus-ring findings**, 16 blocking shadow-only; exit 1 by design |
| `effigy smoke:gpui-offscreen-capture` | pass (legacy smoke unchanged) |
| `effigy check:gpui` | pass |
| `effigy docs:check` | pass |
| `git diff --check origin/main...HEAD` | clean |

No `*-windowed` selector, `test:native-visual`, GPUI preview window,
Jetstream selector, release mutation, tag, publication, or workflow edit ran.

## Boundaries and what this cannot prove

- Svelte/React components, CSS, public props, tokens, specimen pages, the
  fixture inventory, comparator policy/thresholds/known-delta rules, and the
  shadow findings are all unchanged. The GPUI backend holds no
  component-specific ring logic — it paints any declared ring; only
  `poodle-render` decides who declares one.
- The focused-state evidence is same-host Metal rasterisation; cross-machine
  determinism is not claimed (same posture as g15.047).
- Native arrow/`Home`/`End` stepper movement remains the separately recorded
  web-only delta (contract §10); sequential Tab entry is what this card
  closed. Native Button tab traversal beyond the ring is untouched — the
  card scoped Button to the ring channel only.
- Nothing about Jetstream, which compiles the new field but does not project
  it (program-deferred).

## Continuation

The PR stops for review: the orchestrator reviews the new vocabulary, the
GPUI projection, real focus tracking, the comparator counts, Stepper keyboard
entry, scope, and assets; the operator reviews the focused-state evidence in
`assets/g15-052/contact-sheet.html`. After merge the orchestrator closes the
two native focus rows in the release-gap register and advances to `g15.043`.
