# g17.003 — Background-safe non-activation proof

Status: complete — merged in PR #230 at `583aa173935dd66ea0d8bd17196115f5b211a01b` on 2026-09-08
Date: 2026-09-08
Card: `docs/roadmaps/g17/003-background-safe-nonactivation-proof.md`
Base: `origin/main` at `47d42ba0e43943ba78d0e909973f060e7c29bc15`
Implementation commit: `93266dad973ca6fea2d2e4045bbbf0e21bdf3c0e`
Branch: `ns-006ef628-17ed-4e0a-8a57-5d081ff1af6a` (queue-owned; the earlier
`worker/g17-003-background-safe-nonactivation-proof` ref carried the same
commits and was folded into it during queue identity repair)
Review: accepted independent exact-head `ready_to_merge` verdict in [review
comment #5584289808](https://github.com/inflatable-cookie/poodle/pull/230#issuecomment-5584289808)
at head `62be0575a51919697f78dbeaf9789339d83ae2f6`; merge commit:
`583aa173935dd66ea0d8bd17196115f5b211a01b`

## Outcome

The native foreground proof no longer requires the operator's foreground to
stay frozen. The shared transport samples the frontmost process identity AND
pid and compares the pid against the capture process's own pid. Unrelated
operator transitions (editor → browser → editor) are admissible, remain
recorded on the receipt, and never fail a run; a run fails only when the
capture process itself was frontmost — as the baseline or in any later
sample — or when the evidence cannot support the claim.

- `ForegroundSample { identity, pid }` rides on every reading; the receipt
  now names `capturer_pid`, the process the proof is about, so the verdict
  is re-derivable from the receipt fields.
- Verdicts are `proved` | `selffrontmost` | `unprovable` (wire form). The
  old `changed` verdict (any other application frontmost) is gone:
  `selffrontmost` is the typed failure, and unrelated transitions are
  evidence, not failures.
- Fail-closed additions: ticks that cannot be read are counted as
  `failed_reads`, and any failed read makes the run unprovable; an unreadable
  baseline and a watch shorter than `MIN_FOREGROUND_SAMPLES` stay unprovable.
  Self-frontmost readings outrank every unprovable cause (short watch,
  failed read, absent baseline).
- Every mode inherits through the shared transport — smoke, fixture
  (batch), cohort, icon-geometry, focus evidence, inset evidence. No mode
  gained a local bypass. `forbidden.rs` (no activation API anywhere) is
  unchanged and still pins the structural boundary.
- Receipt shape changed, so every receipt schema advanced one revision and
  was migrated directly — no alias, shim, or fallback:
  - smoke `poodle.gpui-window-capture.v1` → `.v2`
  - fixture `poodle.button-visual-capture.v2` → `.v3`
  - focus evidence `poodle.gpui-focus-evidence.v2` → `.v3`
  - inset evidence `poodle.gpui-inset-shadow-evidence.v1` → `.v2`
  - cohort `poodle.cohort-visual-capture.v1` → `.v2`
  - icon geometry `poodle.icon-geometry-visual-capture.v1` → `.v2`
- TypeScript verifier (`test/visual/button-comparison/receipt.ts`) now
  re-derives the claim: positive integer `capturer_pid`, pid-bearing sample
  objects for `baseline`/`observed`, no observed pid equal to `capturer_pid`,
  `failed_reads === 0`, ≥ `MIN_FOREGROUND_SAMPLES`, `verdict: "proved"`.
  The diagnostic and inset-shadow scripts apply the same rules to live
  receipts; the old string-only observed shape no longer validates.
- Nucleus cohort evidence (M1 + A1 receipts and the manifest resolution)
  repinned to the implementation commit `93266dad9`; payloads differ only in
  `source_commit`. GPUI A11y snapshots are byte-identical (no snapshot
  committed changed).

## Oracle evidence (unit tests, no window opened)

| Invariant | Counterexample | Result |
| --- | --- | --- |
| Operator remains free | editor → browser → editor while capture pid never frontmost | `Proved`; all transitions retained in `observed` (`unrelated_frontmost_transitions_are_admissible_and_retained`) |
| Native capture never fronts | one sample whose pid equals the capture pid | `SelfFrontmost` typed failure (`a_later_self_frontmost_sample_is_a_typed_failure`) |
| Bad baseline never passes | capture pid frontmost before its first window | `SelfFrontmost` (`a_capture_process_baseline_is_a_typed_failure`) |
| Evidence fails closed | unreadable baseline / failed required read / too few samples | `Unprovable` (`an_absent_baseline_is_never_proof`, `a_failed_required_read_is_never_proof`, `too_few_samples_is_never_proof`) |
| Self-frontmost outranks damage | short watch, failed reads, absent baseline + one self sample | `SelfFrontmost` (`a_self_frontmost_sample_outranks_other_unprovable_causes`) |
| Wire contract | verdict + evidence field names | `the_verdict_serialises_as_a_closed_lowercase_string`, `the_evidence_serialises_under_the_contracted_names` |

The TypeScript side plants the same negatives: null baseline, empty
baseline identity, baseline without pid, empty observed, self-frontmost
observation, string-only observed (pre-v3), non-positive pid, too few
samples, `failed_reads > 0`, missing `capturer_pid`, and every verdict
other than `proved` are rejected by the verifier
(`compare.test.ts` "foreground evidence (gpui)").

## Validation

- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --bin
  poodle-window-capture --features window-capture transport::` — 16 passed
  (all foreground/verdict/evidence tests, plus png/size tests).
- Full capture target suite — 63 passed; 2 failed
  (`cohort_capture::tests::every_registered_scenario_*`). Both failures are
  pre-existing on the clean base: scenario `agent-transcript.json` gained an
  action the capture binary's scenario parser does not know
  (`programmatic_append`); reproduced with the whole change set stashed at
  the planning base. Not in scope; recorded, not repaired.
- `effigy smoke:gpui-window-capture` — every check passes except the unit
  test line, which fails only on the same two pre-existing scenario-drift
  tests. Windowless (no windowed selector was run).
- `bun test test/visual/button-comparison/compare.test.ts` — 44 passed.
- `effigy regressions:native` — 233 passed; emitted the repinned M1/A1
  receipt cohort (payload delta = `source_commit` only).
- `bunx tsc --noEmit -p tsconfig.json` — no errors in any changed file
  (repo-wide pre-existing errors elsewhere unchanged).
- `effigy docs:check` — passed after the cohort repin.
- `git diff --check` — clean.
- One windowless capture-binary build (`cargo build --features
  window-capture`) — passed. The pre-existing unused-import warning in
  `transport.rs` is present at the base commit too and was left alone.

No `*-windowed` selector was run. The worker stopped after one pushed PR for
independent exact-head review. The reviewer found no blocking findings; the
three non-blocking observations are deferred to Lab acceptance or future
planning: a transient `failed_reads` can make a batch unprovable, stale
sibling g16 wording remains outside this card's owned paths, and the
contracted wire spelling `selffrontmost` should remain unchanged. The
coordinator merged PR #230, and the integration checkout is synchronized with
`origin/main` at `583aa173935dd66ea0d8bd17196115f5b211a01b`.

## Pre-existing reds (not introduced here)

`test/nucleus-a11y/scenarios/agent-transcript.json` declares action type
`programmatic_append`; `window_capture/cohort_capture.rs`'s scenario action
enum only knows `pointer_activate` and `key`. Two capture-target tests fail
at the planning base with the same message. The fix belongs to whoever
extended the scenario vocabulary past the capture parser (an A1-era card),
not to g17.003.
