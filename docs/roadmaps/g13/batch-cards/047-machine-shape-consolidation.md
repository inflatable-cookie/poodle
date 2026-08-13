# 047 One Machine Shape Per Runtime, And The Unpinned Nine

Status: ready
Milestone: side-quest (headless layer, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-047-machine-shape-consolidation`
Depends on: none
Governing refs:
`docs/roadmaps/g11/002-headless-machine-spec-format-and-pilot-contracts.md`
(the documented shape: *classification line, Context, States, Events,
Transitions, Effects, Part Attribute Output, Machinery Dependencies*),
`docs/roadmaps/g11/006-rust-headless-mirror.md`,
`docs/contracts/template/component-contract-template.md` (§4 Behavior Machine),
`packages/core/src/machine.ts`

## Goal

The headless layer grew over three programmes and its machines are not the
same shape as each other. Some are canonical, some are older and simpler, and
some duplicated machines have nothing pinning the two implementations
together.

Two jobs, one card, because they overlap on four components.

**This is not cross-runtime consolidation.** TS and Rust machines stay as two
implementations — that is the intended design, and `g13.006` re-affirms it
(*"shared conformance vectors where runtime machines remain hand-written"*).
The goal is that machines look alike **within** a runtime, and that duplicated
pairs are pinned.

## Current State — Measured

### Shape

`packages/core/src/machine.ts` defines the canonical `TransitionResult<S, C,
Eff>`. **9 of 42** TS modules import it.

| Runtime | canonical | off-pattern | helpers only |
|---|---|---|---|
| TypeScript (40, excl. `index`/`machine`) | **7** | **10** | 23 |
| Rust headless (23, excl. `lib`) | **6** | **9** | 8 |

Canonical in TS: `edit`, `history_center`, `hover`, `menu`, `modal`, `popover`,
`tabs`.
Canonical in Rust: `audio`, `hover`, `menu`, `modal`, `popover`, `slider`.

**Canonical in both: `hover`, `menu`, `modal`, `popover`.** Those four are the
reference — the shape is read off them and `g11.002`, not invented here.

Note the disagreement: `tabs` is canonical in TS and off-pattern in Rust;
`slider` is the reverse. Even the good set does not agree across runtimes.

### Pinning

21 machines exist in both runtimes. **12 are pinned** by a shared conformance
vector; **9 are not**: `color`, `date`, `duration`, `nav`, `pagination`,
`single_select`, `switch`, `toggle_group`, `tree`.

### Where the two jobs meet

| Group | Components | Work |
|---|---|---|
| off-pattern **and** unpinned | `single_select`, `switch`, `toggle_group`, `tree` | both — do these first |
| unpinned, shape fine | `color`, `date`, `duration`, `nav`, `pagination` | vector only |
| off-pattern in both runtimes, pinned | `checkbox`, `disclosure` | shape only |
| off-pattern in one runtime | the remainder | shape only |

## Fixed By Ruling (do not re-decide)

### R1 — The scan is a starting list. Variance is sometimes correct.

The buckets above come from a structural grep: does the module export a
transition, a context/state, an effect type, and does TS import
`TransitionResult`. That detects *shape*, not *correctness*.

**A stateless machine legitimately has no `State` type, and a machine with no
side effects legitimately has no `Effect` type.** Six Rust modules are
"off-pattern" only because they lack `State` — for `checkbox`, `g11.002` calls
that the *trivial case* with "single state, value in context", so its shape may
already be right and the convention is what needs to say so.

Classify every candidate before changing it: *conforms already*, *should
conform and does not*, or *correctly different — the convention must
accommodate it*. Record all three, as `037` and `038` did. `037` found 22 of 34
were false positives; expect the same character here.

**Do not invent state to satisfy a pattern.** That is the failure mode this
ruling exists to prevent.

### R2 — The reference is the four, plus `g11.002`. Do not design a new shape.

`hover`, `menu`, `modal` and `popover` are canonical in both runtimes. The
convention is whatever those four and `g11.002`'s documented section list
already agree on. Write it down; do not improve it while writing it down.

If the four disagree with `g11.002`, say which and stop — that is a real
question about which authority wins, not a detail to settle silently.

### R3 — Interface invariance. Consumers are file-linked.

`g11` established that consumers link these packages directly. Renaming an
exported type or changing a transition's signature breaks them outside this
repo.

**Prefer additive conformance**: adopt `TransitionResult` where the shape
already matches, rather than renaming exports to fit a naming rule. Any change
to a public export is a **stop** — say which consumer would break.

### R4a — Presence is not coverage. `b045` proved it.

Amended after `b045`. `slider` counts as *pinned* in the table above because it
is a key in `machines.json`. Its vector has **3 cases**, and — measured — zero
mentions of thumb, pair, crossing, bipolar or origin. It pins single-thumb
transitions only, while the entire two-thumb surface goes unchecked.

So **the nine are the floor of the gap, not the whole of it.** For each of the
12 already-"pinned" machines, state in the log whether its vector actually
exercises the machine's real surface or just its happy path. That inventory is
worth more than the nine new vectors, because it says how much the mechanism
the roadmap depends on is actually carrying.

Do not fix thin vectors for the 12 in this card unless one is trivially thin —
scope it, record it, and let the inventory drive a follow-up. **`slider` is
excluded entirely**: `b046` is running against it with the vector as a fixed
target.

### R4 — Pin the nine. The vector is the deliverable, not the shape change.

Extend `packages/contracts/headless/vectors/` so the nine unpinned machines are
covered, and make **both** implementations run them — the TS side through
`packages/core/test/*conformance*`, the Rust side through its existing
harness.

A vector that only one side runs is not a pin. If a new vector immediately
fails, **that is the point** — it means the two implementations already
diverged, and the divergence is the finding. Report it; do not tune the vector
until it passes.

### R5 — Gate both halves.

- A duplicated machine (present in both `packages/core/src` and
  `packages/contracts/headless/src`) with no shared vector fails.
- A machine module that declares a transition but does not follow the
  convention fails.

Baseline anything R1 classified as *correctly different*, with its reason.
Wire as `effigy docs:machine-shape-drift` beside the other drift gates.

## Scope

### In scope

- `packages/core/src/*.ts` and `packages/contracts/headless/src/*.rs` —
  conformance to the documented shape only.
- `packages/contracts/headless/vectors/**` — new vectors for the nine.
- `packages/core/test/*conformance*` and the Rust vector harness.
- The new gate and its wiring.
- A short written statement of the convention, where `g11.002` left it implicit.

### Out of scope — stop conditions if reached

- **Cross-runtime consolidation.** Two implementations is the design.
- Porting any machine into `poodle-ir`. That is the IR lane's question and
  `g13.008`'s to answer.
- Behaviour changes of any kind. This is shape and coverage, not logic. If
  conforming a machine changes what it does, **stop**.
- `slider` — `b045` is running against it and its vector is a fixed target
  there.
- Any component file, CSS, or contract outside the machine layer.

## Required Tests

- Every existing conformance test passes unedited.
- Each new vector runs on **both** implementations.
- The gate fails on each half independently (plant an unpinned duplicate; plant
  an off-pattern transition), and passes clean.
- Every baseline entry has a reason.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- R1 before anything else. A card that canonicalises all 19 without classifying
  has failed even if every gate is green.
- A new vector that fails on first run is a **finding to report**, not a vector
  to adjust.
- Run `effigy test:core`, `ci:rust`, `ci:web`, `docs:lint`.
- Verify every governing-ref path resolves before relying on it.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-047-machine-shape-consolidation`. Do not
  merge.
- `PAPERCUTS.md` and `tasks/effigy.tasks.toml` are shared with a running
  worker: append only, and do not reflow neighbours.

## Writable Paths

- `packages/core/src/*.ts`
- `packages/core/test/**`
- `packages/contracts/headless/src/*.rs`
- `packages/contracts/headless/vectors/**`
- `packages/contracts/headless/tests/**`
- `packages/svelte/preview/scripts/**`
- `tasks/effigy.tasks.toml`
- `docs/roadmaps/g11/002-headless-machine-spec-format-and-pilot-contracts.md`
  (only to record the convention explicitly)
- `docs/logs/2026-08/<DD>-g13-047-machine-shape-consolidation.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy test:core`, `ci:rust`, `ci:web`, `docs:lint`,
   `git diff --check`. All green.
2. Read the four reference machines and `g11.002`. Write the convention down.
3. Classify all candidates into R1's three buckets. Record the evidence.
4. Fix the *should conform and does not* set, additively (R3).
5. Write vectors for the nine; run them on both sides. Report any that fail
   rather than tuning them.
6. Add the gate; baseline the *correctly different* set with reasons.
7. Prove the gate on both halves.
8. Validate:
   ```sh
   effigy test:core
   effigy test:components
   effigy ci:rust
   effigy ci:web
   effigy docs:lint
   effigy docs:machine-shape-drift
   git diff --check
   ```

## Acceptance Criteria

- [ ] The convention is written down, read off the four references and
  `g11.002` rather than invented.
- [ ] Every candidate classified into one of R1's three buckets.
- [ ] The nine are pinned, and each vector runs on both implementations.
- [ ] The depth inventory for the 12 already-pinned machines exists (R4a).
- [ ] Any vector that failed on first run is reported as a divergence finding.
- [ ] No public export renamed; no behaviour changed.
- [ ] The gate fails on each half independently and passes clean.
- [ ] All step-8 commands exit 0.

## Stop Conditions

- The four references disagree with `g11.002` about the shape.
- Conforming a machine would change its behaviour or rename a public export.
- A new vector fails because the two implementations genuinely diverge — report
  the divergence; fixing it is a separate card.

Stop with exact paths, commands, and the smallest unresolved question.
