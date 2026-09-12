# g18.024 — Slider-family layout and vertical repair

Status: complete — merged as `c73db47d0de36dd0ce99ba697424dedb7c7b82da` (PR #256) on 2026-09-12 after exact-head independent review (PR comment `5645742372`, `ready_to_merge`) at `8f05b316c39e77121934c84208877fed543bcc85` with green rust/web checks
Date: 2026-09-12
Branch: `ns-0d7f161b-615a-4570-be16-0bc1cab8c2ef`
Card: `docs/roadmaps/g18/024-slider-family-layout-and-vertical-repair.md`
Handoff: `docs/handoffs/20260912-g18-024-slider-family-layout-and-vertical-repair.md`
Governing refs: `docs/contracts/components/slider.md`,
`docs/contracts/components/range-slider.md`,
`docs/architecture/012-feedback-motion-and-state-change.md`
Base: `origin/main` at `4255c62ee16b0f35447189509142fc5b7e6e50ae`

## Review rounds

- Round 1 at `ec9e97821…` (PR comment `5645560034`): **changes required** —
  the web half verified green (hit probe 248/0, inline probe 192/0 across
  Chromium + WebKit; ladder, density, hit-out-of-layout, and the step-aware
  serializer correct), but the native vertical half was not repaired and two
  native regressions were measured on a temporary mounted GPUI probe:
  `fraction_anchor_vertical` applied the centring offset to `top` (handle off
  the rail axis and shifted 10px up), and `block_grab_with_axis` kept fill
  sizing alongside the negative cross insets, so filled sizing won and the
  scrub overlay stayed capsule-sized, leaving the 44×44 overflow band dead at
  xs/sm/md. Two pre-existing in-scope defects were also measured: native
  vertical block fills grew sideways (width percentage, value-independent on
  the block axis), and vertical anchor direction was inverted against the
  bottom-referenced scrub axis (range lower thumb above upper). The review
  also flagged the missing mounted vertical coverage and noted the exported
  core helper signature change as non-blocking.
- Round 2 at `ec094159d…` (PR comment `5645682270`): **changes required** —
  all four round-1 findings verified fixed by independent mounted probes, but
  two blocking native defects remained: the visible thumb painted at the hit's
  top-left (an unaligned in-flow child — the card's "handles float outside
  the rail" counterexample), and at `xl` the clamped hit inset left the 44×44
  target flush with the surface start edge (4px off the rail centre).
- Round-3 fix at `ed48044bd…` → `8f05b316c…`: `block_hit` centres the visible
  thumb (matching web's grid place-items:center) and stamps thumb ids; the
  anchor layers use the signed `(cross - hit) / 2` offset so the hit centres
  at every size (negative below `lg`, positive at `lg`/`xl`); mounted
  regressions assert the thumb node and the `xl` centring on both
  orientations. Focus intent confirmed and documented: the ring lives on the
  focusable 44×44 hit because the node vocabulary resolves ring painting
  through the ringed node's own focus handle; web styles the thumb as a
  descendant of its focused control — both stay visible and unclipped.
- Round 3 at `8f05b316c…` (PR comment `5645742372`): **ready to merge** —
  both round-2 blockers independently re-measured fixed (thumb centre = hit
  centre = rail centre at every shared size, both families, both
  orientations; `xl` hit centred on the 52px capsule), all round-1/2 findings
  still hold, web half unchanged and green. Evidence repinned to the
  thumb-centring head (`source_commit` → `ed48044bd`, run id
  `2026-09-12-g18-024-thumb-centring-expected`); receipts differ only in
  `source_commit`/run id, no observation or capture change.

## Outcome

The block Slider family now consumes the shared control-size axis and renders
as a complete, aligned control at every size and orientation:

- The block capsule consumes the shared control-height ladder (24/28/36/44/52px
  for `xs`–`xl`) in CSS and in shared Rust composition
  (`slider_block::capsule_height_rem`); the private 28/30/32/36/40px ladder is
  gone from both.
- The root's measured layout box equals the visible capsule. The ≥44×44 hit
  rectangle stays measurable, interactive, and out of layout: it lives in
  absolute overflow layers (web) and absolute anchor layers with centred
  overflow (native), and the native scrub overlay extends across the hit
  overflow so the envelope remains draggable.
- Density no longer participates in block layout: the density padding rules
  are scoped to the embedded variant in both stylesheets, and density stays
  independent of size everywhere.
- One shared default display serializer binds both families: the snapped value
  rounds to the decimal precision implied by `min` and a finite positive
  `step` (`sliderDisplayPrecision` / `slider_display_precision`), trims
  insignificant zeroes, normalizes negative zero, and never shows binary
  tails. Without a finite positive step the value keeps its shortest exact
  form. The web `formatVisibleValue` callback and native
  `visible_*_text`/`resolved_visible_text` explicit strings remain the
  authoritative overrides. Value math, callbacks, accessible values, and the
  formatter props are unchanged.
- Vertical block is complete native-axis geometry: the rail cross-size is the
  shared capsule size (no more `max(size, 44px)` width), text keeps a fixed
  value-independent block inset, and the optional RangeSlider label centers on
  the exact rail middle as an absolute overlay (web and native), so the upper
  and lower endpoints stay pinned to the physical top and bottom and the three
  upright rows never clip, shift, or collapse each other. This also retires
  the g18.022 caveat that RangeSlider label centering was exact only for equal
  endpoint advances.
- Specimens put each family beside same-size rows and show representative
  fractional vertical values; the paired probes assert the new law.

## Execution

- Measured the failures first by rewriting `test/block-slider-hit-probe` to
  the new law (five shared heights beside same-size Button/TextInput in both
  frameworks, density independence, capsule-equals-root, 44×44 hit overflow
  and dispatch) and extending `test/block-slider-inline-probe` with
  fractional-serializer and vertical-range-rail oracles; both failed against
  merged main exactly as the card's evidence predicted.
- Fixed the shared law first (`packages/core/src/slider.ts`,
  `packages/contracts/headless/src/slider.rs`), then mirrored layout changes
  across CSS, Svelte/React wrappers, shared Rust composition
  (`packages/render/src/slider_block.rs`, `slider.rs`, `range_slider.rs`),
  and the GPUI host (`block_slider_host.rs`).
- g18.023 was not touched; no release, version, changelog, workflow, Desktop,
  or unrelated-component files changed.

## Validation

- `bun test packages/core` — core suite green, including new serializer
  vectors (fractional steps, fractional minima, integer steps, trailing-zero
  trimming, negative zero, no-step passthrough, custom-formatter precedence).
- `cargo test -p poodle-headless` — 224+ green including the mirrored Rust
  serializer vectors; `cargo test -p poodle-specs` — 330 green;
  `cargo test -p poodle-jetstream` — 163 green.
- `cargo test -p poodle-render` — 645 passed with only the two pre-existing
  unrelated failures (context a11y-wrapper, segmented-control icon-only)
  verified identical on the base commit.
- `effigy regressions:native` — `headless_regressions` 238/0 plus the other
  three gpui-preview test targets (catalogue 7, icon geometry 6, visual
  fixture inventory 15) and `poodle-gpui-node-backend` 52/0.
- Native: `effigy regressions:native` — `headless_regressions` 242/0 at the
  round-3 head (241 plus the xl anchor-centring regression), and the other
  three gpui-preview test targets (catalogue 7, icon geometry 6, visual
  fixture inventory 15) and `poodle-gpui-node-backend` 52/0.
- Block-slider hit probe 124/0 and inline probe 96/0 per engine on headless
  Chromium and WebKit at the round-1 head; the web half is unchanged in round
  2 and was independently verified at 248/0 and 192/0 across both engines.
- Full vitest board — 415 files / 4134 tests green (svelte-components,
  react-components, previews, parity, a11y, headless-dom).
- `svelte-check` components — 0 errors; preview back at its pre-existing
  232-error baseline with zero new errors; react `tsc` shows no slider errors
  (pre-existing unrelated errors unchanged, one fewer than base).
- Package builds green: core, svelte components, react components, svelte
  preview, react preview.
- Visual smoke board: slider pairs match; the only failure (`pill` at
  0.665%) reproduces identically on the base commit and is environmental.
- Census/evidence hygiene: source changes move the receipts' SOURCE_PATHS, so
  the Nucleus/GPUI census evidence was repinned — first to the round-1 source
  head, then to the native-vertical head (`00d90ba64`), then to the
  thumb-centring head (`ed48044bd`, run id
  `2026-09-12-g18-024-thumb-centring-expected`) following the g18.022
  precedent (receipts differ only in `source_commit`/run id;
  `effigy regressions:native` re-run at each head; no observation or capture
  change; admitted capabilities identical: 73 admitted rows, 65 mounted
  receipts).
- `git diff --check` — clean.

## Explicitly not done

- g18.023, release state, versions, changelog, workflows, Desktop, and
  unrelated components untouched.
- TextInput draws its own 1px border outside the shared ladder (its total box
  is 2px taller at every size); the probe asserts alignment within that
  TextInput-owned border. Fixing TextInput's box model is outside this task.
- The `pill` visual-smoke delta is pre-existing/environmental (reproduces on
  the base commit) and stays out of scope.
- The exported core helper signature change
  (`resolveSliderVisibleValue`/`resolveRangeVisibleValue`/
  `defaultVisibleValueText` and the Rust `resolved_visible_text` now carry
  `min`/`step`) is deliberate: the precision law needs them, the web
  `formatVisibleValue` prop and native explicit-string channels are
  unchanged, and pre-v1 rules forbid compatibility shims. The reviewer
  classified it non-blocking.

## Review and merge

Independent round-3 review at `8f05b316c39e77121934c84208877fed543bcc85` (PR
comment `5645742372`): **ready to merge**. The reviewer re-measured both
round-2 blockers on a mounted `HeadlessDriver` tree (thumb centre = hit centre
= rail centre at every shared size, both families, both orientations; `xl`
hit centred on the 52px capsule through the signed offset) and confirmed all
round-1/2 findings still hold (bottom-referenced vertical scrub axis,
RangeSlider vertical ordering, window fill, horizontal geometry, unchanged web
half).

Validation at the reviewed head: `effigy regressions:native` —
`headless_regressions` 242/0 (241 plus the new `xl` anchor-centring
regression); `cargo test -p poodle-render` 645 pass with the same 2
pre-existing unrelated failures (`context.rs:589`,
`segmented_control.rs:1006`); `effigy test:block-slider-hit` 248/0 and
`effigy test:block-slider-inline` 192/0 across Chromium + WebKit;
`effigy test:core` 1378/0; Svelte + React slider vitest 77/0;
`effigy check:gpui-census` and `effigy check:parity-evidence-ledger` pass; the
`docs/evidence` repin is mechanical only (`source_commit` → `ed48044bd`, run
id `2026-09-12-g18-024-thumb-centring-expected`).

Two non-blocking notes, each deferred rather than repaired here:

- The round-3 review flagged this execution log as stale (old `Status:` line,
  doubled "Round 2" bullets, census hygiene stopping at the native-vertical
  repin). Fixed here at closeout; behaviour unaffected.
- Focus intent is accepted and documented (the ring sits on the focusable
  44×44 hit; web styles the thumb as a descendant of its focused control).
  Optional follow-up only: one native note in
  `docs/contracts/components/{slider,range-slider}.md` next to the focus-ring
  row so both realizations are explicit. Out of scope for this task.

Merge gate: PR #256 merged as `c73db47d0de36dd0ce99ba697424dedb7c7b82da`
with green rust/web checks.

## Continuation

g18.024 is merged. After the repaired specimens are accepted, the retained
g18.006 release-candidate task resumes with both repairs in the `0.4.0`
source identity; g18.009 stays serial behind it. Further planning direction
needs the operator.
