# 049 TextInput Environment Boundary — GPUI And Jetstream

Status: ready
Milestone: `g13.007` (part 2 of 2 — **this card closes `g13.007`**)
Owner: Poodle core
Branch: `thread/g13-049-text-input-slice-native-runtimes`
Depends on: `g13-b048` (`83e8cbfe`), merged
Governing refs: `docs/roadmaps/g13/007-text-input-environment-boundary-proof.md`,
`docs/logs/2026-08/13-g13-048-text-input-slice-rust-authoring-and-web.md`
(**read this first — its capability table is the thing this card tests**),
`docs/roadmaps/g13/pilot-expressiveness-corpus.md`
(the authority for every `TXT-NN` id; the contract itself carries no ids),
`docs/roadmaps/g13/batch-cards/046-range-slider-slice-native-runtimes.md`
(the generated-Rust route this follows)

## Goal

`b048` put the two web runtimes on a Rust-authored TextInput definition with a
**typed capability boundary**, and returned the largest hand-written remainder
of the three slices. This card does the natives and closes `g13.007`.

It is also the best test the whole pilot has of the boundary idea, for a reason
no previous card could reproduce.

## Current State — Measured

### One capability, three completely different runtimes

| Runtime | Text editing | Measured evidence |
|---|---|---|
| Svelte / React | **Delegates entirely** — the browser *is* the machine | zero selection references in either component; `b048` reports zero props executed by the definition |
| GPUI | **Implements it** | `packages/gpui/node-backend/src/ime.rs` (218 lines), `input_text.rs` (574 lines); `on_edit_key`, `on_select_range`, `on_edit_insert`, `SelectGranularity` all present |
| Jetstream | **Does not have it at all** | **zero** occurrences of `on_edit_key`, `on_select_range`, `on_edit_insert`; no `ime.rs`; the specimen is display-only (`with_value("Hello world")`) |

No previous pilot component spanned this range. Button and RangeSlider were
implemented everywhere. TextInput is delegated in one runtime, implemented in
another, and **absent in a third** — which makes it the only real test of
`g13.007`'s acceptance line: *"Capability gaps are visible and typed; none are
silently ignored."*

### Sizes

| | |
|---|---|
| `packages/render/src/text_input.rs` | 625 lines |
| `packages/contracts/components/src/text_input.rs` | 459 lines |
| `packages/contracts/headless/src/text_input.rs` | 837 lines |
| GPUI specimen / Jetstream specimen | 459 / 267 lines |
| corpus `TXT-NN` rows | 32, of which **13** name Rust/backend/native ownership |

### Two precision notes, measured, so this card does not repeat a wrong reading

1. **`shape_line`, `x_for_index` and `closest_index_for_x` do not exist as
   functions.** They appear only in doc comments, the IR capability
   descriptions, and corpus row `TXT-21`, where they name a *glyph-measurement
   obligation on the backend*. Do not go looking for them; do not treat their
   absence as a gap to fill.
2. **The `TXT-NN` ids are the corpus's, not the contract's.**
   `docs/contracts/components/text-input.md` contains no requirement ids at
   all. `b048` cited 29 of them correctly against the corpus. Cite the corpus.

Everything the corpus names for `TXT-21`–`TXT-24` **does** exist and was
verified: `TextInputHandlers`, `selection_range`, `on_edit_key`,
`on_select_range`, `on_focus_change`, `on_edit_insert`, `SelectGranularity`.

## Fixed By Ruling (do not re-decide)

### R1 — Follow `046`'s route exactly.

Self-contained generated Rust, no `use` of any Poodle crate, emitted into
`packages/render/src/generated/` beside `button.rs` and `range-slider/`, pulled
in by `#[path]`. **Do not add `poodle-ir` or `poodle-codegen` to
`packages/render/Cargo.toml`** — `b003 R2` still bars it; assert it in a test.

Add a sibling target. Do not change `button-ts`, `button-rust`,
`range-slider-ts`, `range-slider-rust`, `text-input-ts` or `shell-*` output —
their tests byte-compare it. Sharing a helper is fine with proof the bytes did
not move.

### R2 — The three-way split is the headline. Answer it explicitly.

`b048`'s capability table assigns a web owner and a Rust owner per capability.
Test that table against two runtimes that disagree with each other. In the log,
per capability:

1. Does the **GPUI** owner named in `b048`'s table match what GPUI actually
   does?
2. What does the declaration mean for **Jetstream**, which does not implement
   the capability at all?
3. Can **one** declaration serve delegate / implement / absent — or does the
   boundary need per-runtime expression? `b048` already found ownership is
   prose, not a typed field. Say whether that is merely untidy or actually
   load-bearing once a third runtime disagrees.

### R3 — Jetstream's gap is the finding. Do not close it.

**Do not implement text editing, IME, selection or clipboard in Jetstream.**
That is a large feature, it is not what `g13.007` asked for, and building it
would destroy the evidence this card exists to gather.

The question is whether the definition makes the gap **visible and typed**, or
whether Jetstream silently renders a text field that cannot be typed into and
nothing in the model says so. Answer it. If the honest answer is "the IR cannot
express that a runtime lacks a declared capability", that is the single most
useful sentence this card can produce for `g13.008`.

### R4 — Public API and pixels unchanged.

`TextInputSpec` keeps its fields; `poodle-render` keeps its signatures; both
natives render what they render today.

**A moving native visual baseline is a stop condition, not a refresh.**
Classify the delta and report it. `b042` correctly classified a GPUI delta as
*stale, not moved*; `b046` refreshed nothing. Do the same or stop.

### R5 — The editing model and the vectors stay fixed.

`packages/contracts/headless/src/text_input.rs` is not going into the IR.
`packages/contracts/headless/vectors/**` must pass **unedited** — `b047` pinned
all 21 machines and they are a fixed target.

`b048` recorded that `machines.json` has **no `text` key**, so the editing
model is unit-test-pinned only. **Record it again if it still holds; do not fix
it here.** That vector gap is a follow-up card, and closing it inside this one
would blur the evidence.

### R6 — Look at both natives, and at the states only natives have.

GPUI takes `--screenshot`, Jetstream has a headless `snap`. Capture TextInput
in both, including: focused with a caret, a selection range, and a validation
state.

Two recorded environment hazards, both in `PAPERCUTS.md`:
- The Jetstream snap's fixed 640px viewport clips lower rows — check the states
  are inside the frame before concluding anything from a snap.
- The Jetstream `snap -- specimens` bin **overwrites its output in place with
  no warning**, so copy each capture before re-rendering or a before/after
  proof silently destroys its own baseline.
- `jetstream-poodle` is a sibling-repo path dep that does not resolve from a
  worktree. Build through `/Users/tom/Dev/projects/poodle-wt/poodle`, which
  points at the main repo.

### R7 — The exception inventory closes the trend. This is the last input.

Extend `b048`'s inventory to GPUI and Jetstream, and then put all three slices
in **one table**: Button, RangeSlider, TextInput — what the definition carried,
what stayed hand-written, per runtime.

`g13.008` is next and decides adopt / revise / reject. The measured trend so
far is that the IR carries vocabulary and not behaviour, and that the
hand-written remainder grew with each slice. If the natives confirm it, say so
plainly. **A clear negative trend is a successful pilot outcome**, not a
failure to write around.

## Scope

### In scope

- The new target and the artifact under `packages/render/src/generated/`.
- `packages/render/src/text_input.rs` consuming it.
- `ir:build` / `ir:check` coverage; tests.
- `docs/roadmaps/g13/007-text-input-environment-boundary-proof.md` — status to
  complete.

### Out of scope — stop conditions if reached

- Implementing any text capability in Jetstream (R3).
- `poodle-ir` schema changes. If typing a runtime-level capability gap needs a
  field that does not exist, **stop** — that is `g13.007`'s finding.
- The machines and the vectors (R5); the missing `text` vector key.
- `packages/contracts/headless/src/text_input.rs`.
- Sibling target outputs; the shell scene; any component other than TextInput.
- Refreshing a baseline (R4).
- The web components — `b048` did those and they are merged.

## Required Tests

- `ir:build` / `ir:check` exit 0; `ir:check` fails on drift in the new artifact
  (plant, watch, restore).
- One definition change reaches **all four** runtimes, as `042` and `046`
  proved.
- `packages/render/Cargo.toml` gained no `poodle-ir`/`poodle-codegen` — assert
  it.
- The `slider`-style vector suite passes unedited; `machines.json` untouched.
- Existing `poodle-render` TextInput tests pass unedited.
- GPUI's edit handlers still work: `on_edit_key`, `on_select_range` and
  `on_edit_insert` behave as they do today.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- **Read `b048`'s log before starting.** R2 tests its capability table; you
  cannot do that without it.
- Cite `TXT-NN` ids against the **corpus**, not the contract (Current State).
- A negative result is a result. Jetstream's missing capability is data.
- Run `effigy ci:web` (includes `test:web-pack-install`) and `ci:rust`.
- Verify every governing-ref path resolves before relying on it.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-049-text-input-slice-native-runtimes`. Do not
  merge.
- `PAPERCUTS.md` is append-only and shared: do not reflow neighbours.

## Writable Paths

- `packages/codegen/src/**`
- `packages/codegen/tests/**`
- `packages/render/src/text_input.rs`
- `packages/render/src/generated/**`
- `packages/render/src/lib.rs` (module declaration only)
- `tasks/effigy.tasks.toml`
- `docs/roadmaps/g13/007-text-input-environment-boundary-proof.md`
- `docs/logs/2026-08/<DD>-g13-049-text-input-slice-native-runtimes.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy ir:build`, `ir:check`, `ci:rust`, `ci:web`,
   `git diff --check`. All green.
2. Read `b048`'s log and its capability table.
3. Add the target; emit into `packages/render/src/generated/`.
4. Wire `text_input.rs` to consume it.
5. Answer R2's three questions per capability.
6. Answer R3: is Jetstream's gap visible and typed, or silent?
7. Prove the four-runtime propagation; record each.
8. Screenshot both natives, including caret, selection and validation states.
9. Write the R7 inventory, and the three-slice comparison table.
10. Validate:
    ```sh
    effigy ir:build
    effigy ir:check
    effigy ci:rust
    effigy ci:web
    effigy test:core
    effigy test:components
    effigy test:parity
    effigy check:svelte
    effigy docs:lint
    effigy docs:machine-shape-drift
    git diff --check
    ```

## Acceptance Criteria

- [ ] R2's three questions answered per capability, against GPUI's real
  behaviour.
- [ ] R3 answered: whether the model makes Jetstream's absent capability
  visible and typed, or silently ignores it.
- [ ] Jetstream gained no text-editing implementation.
- [ ] `poodle-render` depends on neither `poodle-ir` nor `poodle-codegen`.
- [ ] One definition change visible in all four runtimes.
- [ ] Both natives screenshotted including caret and selection states.
- [ ] The three-slice comparison table exists (R7).
- [ ] `g13.007` marked complete.
- [ ] All step-10 commands exit 0; no baseline refreshed.

## Stop Conditions

- Typing a runtime-level capability gap needs a `poodle-ir` field that does not
  exist.
- Honouring the definition would change GPUI's editing, selection or IME
  behaviour.
- A native visual baseline moves.
- `machines.json` would need editing.

Each is a **finding for `g13.008`**. Stop with exact paths, commands, and the
smallest unresolved question.
