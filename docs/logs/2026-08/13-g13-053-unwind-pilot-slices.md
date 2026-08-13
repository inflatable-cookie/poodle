# 13 — g13.053 Unwind The Three Pilot Slices (batch log)

Branch: `thread/g13-053-unwind-pilot-slices` (pushed with
`git push -u origin thread/g13-053-unwind-pilot-slices`)
Date: 2026-08-13
Card: `docs/roadmaps/g13/batch-cards/053-unwind-the-three-pilot-slices.md`
Executes: the `g13.020` verdict (retire the vocabulary IR, unwind the three
pilot slices, keep the scene). No planning or status authority — this log
does not flip `g13.020` or edit `docs/roadmaps/g14/README.md`.

The three component definitions described source they did not own, at ~9×
the line count, without catching drift the gates already catch. This card
removes the definitions and generated artifacts, restores the nine
consumers to hand-written form, and rehomes the capability declarations so
the gate survives.

Pre-pilot baseline is `0dd58b80` (“Compile g13-b041”). Not a revert:
post-slice semantics that are not IR wiring stay.

## 1. Baseline (step 1)

Worktree on `thread/g13-053-unwind-pilot-slices` at `ad14838e` (same as
`t3code/unwind-three-pilot-slices` / then-main). Fresh worktree needed
`bun install` (`marked` missing; `test:core` failed once, then green). One-off,
not a papercut.

| Command | Exit | Notes |
|---|---|---|
| `git diff --check` | 0 | |
| `effigy test:core` | 0 | after `bun install` |
| `effigy test:components` | 0 | 1142 tests |
| `effigy ci:rust` | 0 | |
| `effigy docs:lint` | 0 | |
| `effigy ci:web` | 0 | |
| `effigy docs:capability-drift` | 0 | **36 rows** |

`ci:native` is step 7 only (and hits the known worktree jetstream layout;
§8).

## 2. R3 — b052 thread branch

Deleted `thread/g13-052-vocabulary-coverage-first-tranche` locally and on
the remote. The five definitions stay unmerged; their numbers live in the
`g13.020` reassessment. Confirmed gone at close: `git branch -a` lists no
`g13-052` ref.

## 3. R2 — capability declarations rehome

New `packages/contracts/headless/capabilities/capabilities.json`. Same
serialized shape the gate already reads:

```
{ components: [{ id, capabilities: [{ capability, runtimes: [{ runtime, provision, reason }] }] }] }
```

Every declared row from `button-model.json`, `range-slider-model.json`, and
`text-input-model.json` carried across. Dropped none.

| Component | Capabilities | Declared runtime rows |
|---|---|---|
| button | focus (empty `runtimes` — pre-g13.018 shape, skipped by the gate) | 0 |
| range-slider | pointer-capture, focus, scrub-fraction | 12 |
| text-input | focus, text-editing, ime, clipboard, measurement, timers | 24 |
| **total gated** | | **36** |

Gate reader (`packages/svelte/preview/scripts/capability-drift.ts`) switched
from `FIXTURES_DIR` (`packages/codegen/fixtures/`) to `CAPABILITIES_PATH`.
`PROBES` table untouched.

### Plant proof

Svelte `text-editing` provision flipped to `absent` → gate failed direction
A (runtime still traces it; 1 trace). Restored. Clean run:

```
capability-drift: 36 declared capability rows verified against runtime traces.
```

## 4. Consumer restore (step 3)

Against `0dd58b80`, IR wiring only removed. Two post-slice changes kept —
neither is artifact consumption:

- **Button.svelte** — `forwardNative` / DOM-spelled `onclick`/`onfocus`/
  `onblur` composition from `734d5e26` (SettingsShell dead-button fix).
  +24 vs baseline. The only kept Button delta.
- **TextInput.svelte / TextInput.tsx** — Card 048 IME composition gating
  (`composing` + `compositionBuffer`; commit once on `compositionend`).
  First restore to `0dd58b80` dropped the traces; `docs:capability-drift`
  then failed direction B (`ime @ svelte` / `ime @ react` declared
  delegated, no trace). Re-applied the IME handlers. +32 / +34 vs
  baseline.

RangeSlider web + all three native files: exact `0dd58b80`. b051's
definition-derived thumb count was IR wiring; observable semantics remain
two thumbs.

Component tests after restore: Button 5/5, TextInput tests pass unedited.
RangeSlider had only generated-artifact tests (deleted with the artifacts).
`poodle-render` lib tests: 179 passed.

Generated dirs, models, targets, fixtures, and `*.generated.test.*` for
all three components deleted. `ir:build` / `ir:check` now:
`--author-shell`, synthetic fixture, four shell-scene/shell-rust emits
only. `poodle-codegen` CLI: `--author-shell` only. `packages/render/src/lib.rs`:
`pub mod generated` gone.

Empty `packages/{svelte,react}/components/src/generated/` and
`packages/render/src/generated/` removed. Preview shell artifacts remain
(R1):

- `packages/svelte/preview/src/generated/preview-shell.ts`
- `packages/react/preview/src/generated/preview-shell.ts`
- `packages/gpui/preview/src/generated/preview-shell.rs`
- `packages/jetstream/preview/src/generated/preview-shell.rs`

## 5. Shell identity (step 4)

`effigy ir:build` then `effigy ir:check`: shell artifacts byte-identical
to the committed R1-keep files. `ir:check` exit 0 at close (shell-scene ×2,
shell-rust ×2, synthetic targets). Nothing in this card touches the shell
scene emit; `shell_rust.rs` comment-only (dropped "shared with button-rust"
now that the sibling is gone).

## 6. Grep (step 5)

No `generated/` import in the nine consumers. No `button-ts` /
`author-button` / `button-model.json` (or range-slider / text-input
equivalents) in code.

Remaining `poodle-ir` / `poodle-codegen` / `generated/` / `ir:build` refs
are R1-kept: codegen crate, ir crate, shell artifacts, token
`src/generated`, release manifest, synthetic `packages/codegen/generated/`.

## 7. LOC vs the verdict ledger (step 6)

Nine consumers:

| | `0dd58b80` | pre-053 (`ad14838e`) | now |
|---|---|---|---|
| Total | 3,672 | 4,683 | 3,762 |

- vs pre-053: **−921**. Ledger said +965 at verdict time; b051 later grew
  `range_slider.rs` 665→711, so the unwind is larger than the ledger on
  the IR side and smaller on the consumer total because IME and
  `forwardNative` stay.
- vs `0dd58b80`: **+90** — Button.svelte `forwardNative` (+24), TextInput
  IME svelte (+32) and react (+34). Every other consumer matches baseline
  line count.

Repo diff vs HEAD (capabilities.json untracked at measurement): 50 files,
+287 / −21,998.

## 8. Validation (step 7)

| Command | Exit | Notes |
|---|---|---|
| `effigy test:core` | 0 | |
| `effigy test:components` | 0 | 1112 tests (was 1142; 30 generated-artifact tests deleted) |
| `effigy ci:rust` | 0 | |
| `effigy ci:web` | 0 | includes core, components, `docs:lint`, `docs:capability-drift` (36 rows) |
| `effigy ci:native` | 1 | `test:jetstream-a11y` (and `drift:roles`) cannot resolve sibling `jetstream-input` at `/Users/tom/.t3/worktrees/poodle/jetstream/crates/jetstream-input`. Known worktree layout papercut (`PAPERCUTS.md` 2026-08-11; g13-013 hit the same wall). GPUI check/test, `poodle-render` tests, and `test:jetstream-adapter` passed before the a11y bin. Not a new papercut; not caused by this unwind. |
| `effigy docs:lint` | 0 | |
| `effigy docs:capability-drift` | 0 | 36 rows |
| `git diff --check` | 0 | |
| `effigy ir:check` | 0 | shell-only |
| `cargo test --manifest-path packages/codegen/Cargo.toml` | 0 | remaining tests after model/target deletion |

## 9. g14 closeout checklist (not edited)

`docs/roadmaps/g14/README.md` names these as g13 closeout items but is
**not** in Writable Paths. Closed on this card, recorded here only:

- 053 executed and validated (not merged — card forbids merge).
- b052 thread branch deleted (R3); numbers preserved in `g13.020`.
- `docs:capability-drift` declaration home rehomed to
  `contracts/headless/capabilities/` (R2); the gate survives.

Watcher-guard / React-preview / Jetstream-snap / HistoryCentre /
batch-cards-README items on that list are not this card.

## 10. Acceptance criteria

- [x] The nine files consume no generated artifact; public surfaces
  unchanged (component tests pass unedited).
- [x] Generated dirs, models, targets, and fixtures for the three
  components are gone.
- [x] R2 rehomed to `contracts/headless/capabilities/` with no rows
  dropped; the capability gate passes clean and fails on a plant.
- [x] Shell artifacts byte-identical after `ir:build`; component targets
  gone, shell targets intact.
- [x] LOC delta reported against the pilot ledger.
- [x] Step-7 commands exit 0 except `ci:native`, which fails on the
  pre-existing worktree jetstream layout (same as g13-013).
