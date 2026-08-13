# 046 RangeSlider Stateful Proof — GPUI And Jetstream

Status: ready
Milestone: `g13.006` (part 2 of 2 — **this card closes `g13.006`**)
Owner: Poodle core
Branch: `thread/g13-046-range-slider-slice-native-runtimes`
Depends on: `g13-b045` (`db4e587a`), merged
Governing refs: `docs/roadmaps/g13/006-range-slider-stateful-control-proof.md`,
`docs/logs/2026-08/13-g13-045-range-slider-slice-rust-authoring-and-web.md`
(**read this first — its findings are the point of the card**),
`docs/roadmaps/g13/batch-cards/042-button-slice-native-runtimes.md`
(the generated-Rust route this follows)

## Goal

`b045` put the two web runtimes on a Rust-authored RangeSlider definition and
produced two negative findings. This card does the natives and closes
`g13.006`.

It also has a second job `042` did not: **test whether `b045`'s two findings
hold on the native side, or are web-specific.** That distinction matters to
`g13.008` and nobody has checked it.

## What `b045` Found

1. **`PartKind::Repeated` cannot express the two thumbs.** It requires a `List`
   prop and yields identical instances with no per-item identity; the thumbs
   are a fixed pair with per-position semantics. The web renderer hard-codes
   "two". `parts.rs:72` names *"the two RangeSlider thumbs"* as `Repeated`'s
   own motivating example — the IR's example does not work for the case it
   names.
2. **The `slider` conformance vector is thin.** It passes unedited, but its 3
   cases pin single-thumb `sliderTransition` only — no thumb, pair, crossing,
   bipolar or origin coverage. The two-thumb surface is unpinned in both
   runtimes.

## Fixed By Ruling (do not re-decide)

### R1 — Follow `042`'s route exactly.

Self-contained generated Rust, no `use` of any Poodle crate, emitted into
`packages/render/src/generated/` beside `button.rs`. **Do not add `poodle-ir`
or `poodle-codegen` to `packages/render/Cargo.toml`** — `b003 R2` still bars
it, and `042` proved the route works.

Add a sibling target. Do not change `button-rust`, `button-ts` or `shell-rust`
output; their tests byte-compare it. Sharing a helper is fine with proof the
bytes did not move — `b036`, `b041` and `b042` each did exactly that.

### R2 — Re-test `b045`'s findings natively. Do not assume they carry.

For each of the two findings, answer in the log:

- **Repeated anatomy.** `poodle-render` builds a node tree rather than DOM. Does
  the two-thumb limitation reproduce there, or does the node path express it
  where the web path could not? If `range_slider.rs` also ends up hard-coding
  "two", say so — that makes the finding structural rather than web-specific,
  which is a stronger result for `g13.008`.
- **Vector thinness.** The natives consume the same `slider` vector. Confirm
  whether the Rust machine's two-thumb path is exercised by anything at all.

A finding that turns out to be web-only is just as useful as one that
generalises. Report which.

### R3 — The machine stays hand-written, the vector stays fixed.

Same as `b045` R1/R5. Do not port `packages/contracts/headless/src/slider.rs`
into the IR. `packages/contracts/headless/vectors/machines.json` must pass
**unedited** — a change requiring an edit is a behaviour change wearing a
refactor's clothes.

**Do not widen the vector here either.** Its thinness is a recorded finding,
and card `047` owns vector coverage. Fixing it inside this card would blur the
evidence `g13.008` needs.

### R4 — Pixels and public API unchanged.

`RangeSliderSpec` keeps its fields; `poodle-render` keeps its signatures; the
natives render what they render today. Geometry is the thing most likely to
shift — bipolar fill grows from the origin and detents land where they land.

**A moving native visual baseline is a stop condition, not a refresh.**
Classify the delta and report it. `b042` correctly classified a GPUI delta as
*stale, not moved* and left it alone; do the same or stop.

### R5 — Look at both natives, and look at the hard states.

GPUI takes `--screenshot`, Jetstream has a headless `snap`. Capture RangeSlider
in both — and specifically the states Button could not have: **two thumbs at
rest, thumbs crossed or clamped, and bipolar fill with a negative value.**

`b042` found the Jetstream snap's fixed 640px viewport clips a component's
lower rows; check RangeSlider's states are actually inside the frame before
concluding anything from a snap.

**Environment note:** `jetstream-poodle` is a sibling-repo path dep that does
not resolve from a worktree. Build through
`/Users/tom/Dev/projects/poodle-wt/poodle`, which points at the main repo. In
`PAPERCUTS.md`.

### R6 — The exception inventory is the headline output.

Extend `b045`'s inventory to GPUI and Jetstream: what came from the definition,
what stayed hand-written, why.

This is the number `g13.008` turns on. Button's remainder was large — DOM
element, event wiring, every derived value, all per-attribute derivation. If
RangeSlider's native remainder is comparably large, that is the trend the
verdict needs, and stating it plainly is worth more than a card that reads
well.

## Scope

### In scope

- The new target and the artifact under `packages/render/src/generated/`.
- `packages/render/src/range_slider.rs` consuming it.
- `ir:build` / `ir:check` coverage; tests.
- `docs/roadmaps/g13/006-range-slider-stateful-control-proof.md` — status to
  complete.

### Out of scope — stop conditions if reached

- `poodle-ir` schema changes. If two thumbs need a field that does not exist,
  **stop** — that is `g13.006`'s finding, restated natively, not a licence to
  add one.
- The machines and the vectors (R3). Vector coverage is `047`'s.
- `button-rust` / `button-ts` / `shell-rust` output.
- Any component other than RangeSlider.
- Refreshing a baseline (R4).

## Required Tests

- `ir:build` / `ir:check` exit 0; `ir:check` fails on drift in the new artifact
  (plant, watch, restore).
- One definition change reaches **all four** previews, as `042` proved for
  Button.
- `packages/render/Cargo.toml` gained no `poodle-ir`/`poodle-codegen` — assert
  it.
- The `slider` vector passes unedited against the Rust machine.
- Existing `poodle-render` RangeSlider tests pass unedited.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- **Read `b045`'s log before starting.** R2 is a question about its findings;
  you cannot answer it without them.
- A negative result is a result. If the node path also cannot express two
  thumbs, that is the finding — write it down rather than working around it.
- Run `effigy ci:web` (includes `test:web-pack-install`) and `ci:rust`.
- Verify every governing-ref path resolves before relying on it.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-046-range-slider-slice-native-runtimes`. Do
  not merge.

## Writable Paths

- `packages/codegen/src/**`
- `packages/codegen/tests/**`
- `packages/render/src/range_slider.rs`
- `packages/render/src/generated/**`
- `packages/render/src/lib.rs` (module declaration only)
- `tasks/effigy.tasks.toml`
- `docs/roadmaps/g13/006-range-slider-stateful-control-proof.md`
- `docs/logs/2026-08/<DD>-g13-046-range-slider-slice-native-runtimes.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy ir:build`, `ir:check`, `ci:rust`, `ci:web`,
   `git diff --check`. All green.
2. Read `b045`'s log.
3. Add the target; emit into `packages/render/src/generated/`.
4. Wire `range_slider.rs` to consume it.
5. Answer R2's two questions explicitly.
6. Prove the four-runtime propagation; record each.
7. Screenshot both natives, including the hard states (R5).
8. Write the R6 inventory for both natives.
9. Validate:
   ```sh
   effigy ir:build
   effigy ir:check
   effigy ci:rust
   effigy ci:web
   effigy test:parity
   effigy check:svelte
   effigy docs:lint
   git diff --check
   ```

## Acceptance Criteria

- [ ] One definition change visible in all four previews.
- [ ] R2's two questions answered — does each `b045` finding generalise?
- [ ] `poodle-render` depends on neither `poodle-ir` nor `poodle-codegen`.
- [ ] The `slider` vector passes unedited and was not widened.
- [ ] Both natives screenshotted including two-thumb and bipolar states.
- [ ] Exception inventory covers GPUI and Jetstream.
- [ ] `g13.006` marked complete.
- [ ] All step-9 commands exit 0; no baseline refreshed.

## Stop Conditions

- Two thumbs need a `poodle-ir` field that does not exist.
- Fill geometry cannot be declared without a runtime-specific value path.
- A native visual baseline moves.
- The `slider` vector would need editing.

Each is a **finding for `g13.008`**. Stop with exact paths, commands, and the
smallest unresolved question.
