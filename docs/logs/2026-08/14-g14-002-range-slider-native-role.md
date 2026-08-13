# 14 — g14.002 RangeSlider Native Slider-role Projection (batch log)

Branch: `thread/g14-002-range-slider-native-role`
Date: 2026-08-13
Card: `docs/roadmaps/g14/batch-cards/002-range-slider-native-slider-role.md`
Milestone: `g14.008` lane, pulled early — main's `ci:native` was red

## 1. Baseline (step 1) — the gate is red

Fresh branch from `origin/main` (`847a8652`, after the g13-b053 unwind —
`range_slider.rs` is hand-written again, no generated artifact). The census
confirms the card's finding:

```
"range-slider": ["GenericContainer", "Label", "Window"]
```

`docs:contract-role-drift` exits 1: `1 contract role(s) never projected —
range-slider slider`.

## 2. The census environment (the card's known constraint)

The role gate and the a11y audit run the Jetstream preview, which path-deps
the sibling jetstream repo — unresolvable from a worktree, as the card
records (ci:native runs at review in the main checkout). The documented
g13-036 symlink workaround (`poodle-wt/poodle`-style layout) **does not
work** under current cargo: a symlinked `poodle` next to a real worktree
produces a lockfile collision (two textual paths to the same crates). The
working recipe, used for the live proofs below:

- `git clone -b <branch>` of the poodle repo at
  `/Users/tom/.t3/worktrees/poodle/poodle` (one textual path for every
  poodle crate);
- sources-only copy of the jetstream repo at
  `/Users/tom/.t3/worktrees/poodle/jetstream` (no `target/`);
- `CARGO_TARGET_DIR=/Users/tom/Dev/projects/jetstream/target` reuses the
  main checkout's warm build cache (mtimes preserved by `rsync -a`, so the
  fingerprints hit).

Both dirs sit outside the repo and are environment-only. Recorded in
`PAPERCUTS.md` so the next jetstream-touching card does not rediscover the
collision.

## 3. The fix (step 3) — `packages/render/src/range_slider.rs`

The contract (§6) requires the slider role on the native path; the audit
fails on unnamed Slider nodes; the ruling says the role must land on a node
that already carries an accessible name. Both land on the existing control
node (`el`, which is focusable and already carries `aria_label`), following
the `audio.rs`/`color_picker.rs` precedent:

```rust
if let Some(label) = spec.aria_label.as_deref() {
    if !label.is_empty() {
        el.a11y.label = Some(label.to_string());
        el.a11y.role = Some(NodeRole::Slider);
    }
}
```

**The role is gated on the name, deliberately.** An unconditional role
turned the 24 unlabeled specimen sliders (positions/sizes/densities rows)
into unnamed Slider findings and failed `test:jetstream-a11y` — the audit's
own rationale is that a nameless "slider" announcement is worse than a
nameless container. The ruling's "already carries an accessible name" clause
resolves it without touching the adapter layer (the second stop condition
is not reached): named control → slider; unnamed control → stays a
roleless container, exactly as before.

**The dual-thumb semantics question, recorded per the first stop
condition.** The contract's two-input model is a per-thumb pattern (two
labels, "…minimum"/"…maximum", with fallbacks). The native tree has one
focusable control and one name, and the contract explicitly allows the
"single custom control" pattern with the same a11y result. The ruling
chooses the single labeled slider; per-thumb native stops would need the
embedded-variant focus-stop shape, which no card has asked for. The gate is
set-based, so one projected `Slider` satisfies it. This is the ruling's
choice, applied — not a silent one.

The fix sits in the shared `poodle-render` path, so GPUI and Jetstream both
inherit the projection.

## 4. Render tests (step 3)

Two tests, both in `range_slider.rs`:

- `the_control_node_exposes_the_slider_role` — with `aria_label`, exactly
  one node carries `NodeRole::Slider`, it carries the same label, and it is
  focusable (precedent: `audio.rs` asserts `knob.a11y.role ==
  Some(NodeRole::Slider)`).
- `an_unnamed_control_stays_roleless` — no `aria_label`, no slider role;
  this is what keeps the a11y audit clean.

## 5. Live proofs (steps 4–5)

All three run against the branch sources in the clone:

| Proof | Before | After |
|---|---|---|
| Census (`--json`) | `GenericContainer, Label, Window` | `GenericContainer, Label, **Slider**, Window` |
| `docs:contract-role-drift` | exit 1 (`range-slider slider`) | exit 0 — "every ARIA role a contract names is projected" |
| `test:jetstream-a11y` audit | n/a (no slider existed) | exit 0 — "every role that needs an accessible name has one" |

Worktree validation:

| Command | Exit |
|---|---|
| `effigy ci:rust` | 0 (render: 181 tests, incl. the two new) |
| `effigy test:core` | 0 (659 tests) |
| `effigy test:components` | 0 (79 files, 1112 tests) |
| `git diff --check` | 0 |

`ci:native` itself runs at review in the main checkout, per the card — the
two jetstream-dependent gates in it (`drift:roles`, `test:jetstream-a11y`)
are proven green live above.

## 6. Acceptance

- [x] `effigy docs:contract-role-drift` exits 0 (proven live, clone).
- [x] `effigy ci:native` — at review in the main checkout; both
  jetstream-dependent members proven green here.
- [x] No web file, contract, or adapter changed — the diff is one Rust file
  (`packages/render/src/range_slider.rs`, +41/−2).
- [x] Explainable in a11y terms: the node a screen reader describes as the
  slider is the focusable control node with the accessible name — role and
  name on the same node, never a role without a name.

## 7. Papercuts

One entry appended: the symlink-layout collision and the clone +
`CARGO_TARGET_DIR` recipe. No stop condition reached — the ruling's two
named questions (dual-thumb single-role, unnamed node) are recorded in §3,
both answered by the ruling's own clauses.
