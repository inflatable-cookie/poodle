# g17.003 — Lab adoption request: background-safe non-activation proof

Status: request — adopt the merged `g17.003` proof in PR #230 at
`583aa173935dd66ea0d8bd17196115f5b211a01b`; resolves the poodle-lab `g01.006`
GPUI-leg foreground blocker
Date: 2026-09-08
Poodle card: `docs/roadmaps/g17/003-background-safe-nonactivation-proof.md`
Poodle implementation commit: `93266dad973ca6fea2d2e4045bbbf0e21bdf3c0e`
(request written before the PR opened; the PR head adds documentation only
and has the same runtime tree over the capture surface)
Poodle base for Lab pin: merge commit
`583aa173935dd66ea0d8bd17196115f5b211a01b`, which carries `93266dad9` — Lab
repins to that exact commit before its next cohort capture, per the existing
pin rule.

## Why Lab must adopt

Poodle's foreground proof previously rejected ANY frontmost-application
change during a capture run. Ordinary operator work (switching from the
lab runner to another application, or a notification app fronting)
therefore invalidated long in-memory capture batches even though the
capture window stayed unfocused, non-key, and inactive — the `g01.006`
blocker. Poodle now proves the narrower, causal claim: **the capture
process never became frontmost**. Unrelated foreground transitions are
admissible and remain recorded on the receipt.

## Receipt contract changes (all `poodle-window-capture` modes)

Every receipt's `foreground` block changed shape and every schema advanced
one revision:

| Mode | Old schema | New schema |
| --- | --- | --- |
| smoke | `poodle.gpui-window-capture.v1` | `poodle.gpui-window-capture.v2` |
| button fixture (GPUI leg) | `poodle.button-visual-capture.v2` | `poodle.button-visual-capture.v3` |
| focus evidence | `poodle.gpui-focus-evidence.v2` | `poodle.gpui-focus-evidence.v3` |
| inset shadow evidence | `poodle.gpui-inset-shadow-evidence.v1` | `poodle.gpui-inset-shadow-evidence.v2` |
| cohort | `poodle.cohort-visual-capture.v1` | `poodle.cohort-visual-capture.v2` |
| icon geometry | `poodle.icon-geometry-visual-capture.v1` | `poodle.icon-geometry-visual-capture.v2` |

`foreground` fields (snake_case, closed key sets):

- `capturer_pid` (integer) — NEW. The pid of the process under test; the
  proof is about this process.
- `baseline` (object `{ identity: string, pid: integer }`, nullable) —
  CHANGED from a bare identity string. The frontmost process before any
  window existed.
- `observed` (array of `{ identity: string, pid: integer }`) — CHANGED from
  an array of identity strings. Every distinct frontmost reading of the run,
  baseline included; unrelated operator transitions appear here and are
  admissible.
- `samples` (integer) — unchanged meaning; successful readings.
- `failed_reads` (integer) — NEW. Ticks the monitor could not read; any
  failure makes the run unprovable.
- `verdict` — CHANGED vocabulary: `"proved"` | `"selffrontmost"` |
  `"unprovable"`. The old `"changed"` verdict is removed: it named a
  non-causal finding (any other application frontmost). `"selffrontmost"`
  is the typed failure (capture pid was the baseline or appeared later);
  only `"proved"` is ever published.

## What Lab must change before the next full cohort capture

1. Update Lab's external `ForegroundWatch` to the same non-activation
   semantics: reject only the Lab controller/capture process itself
   becoming frontmost (its own pid), and allow unrelated operator
   transitions while retaining them in evidence. Do not weaken the proof to
   bundle-name guessing: compare pids.
2. Repin Poodle to the merge commit carrying `93266dad9` (or later main
   containing it) and re-run Lab's cohort capture with the updated watch.
3. If Lab's receipt parser validates the `foreground` block, migrate it to
   the new closed shape above and the new schema ids — old-shape receipts
   no longer validate and are not accepted evidence.

Poodle's own TypeScript verifier
(`test/visual/button-comparison/receipt.ts`) is the reference for the
re-derived claim: positive `capturer_pid`, pid-bearing samples, no observed
pid equal to `capturer_pid`, `failed_reads === 0`, at least 8 samples, and
`verdict: "proved"`.

## Acceptance after adoption

One separately authorized cohort capture with the operator actively using
other applications throughout: every receipt publishes `verdict: "proved"`
with the operator's own applications recorded in `observed` and no
self-frontmost sample. After adoption, resume the queue-managed `g01.006`
thread per the card's continuation note.
