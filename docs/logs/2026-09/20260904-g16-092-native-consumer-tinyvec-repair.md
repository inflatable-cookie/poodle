# g16.092 — Native Fresh-Consumer tinyvec Build Repair

Status: review — repair and exact receipt repin pushed for re-review
Date: 2026-09-04
Card: `docs/roadmaps/g16/092-native-consumer-tinyvec-build.md` (widened by `d5cecdf6c`)
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-223000-g16-092-native-consumer-tinyvec-build.md`
Branch: `fix/g16-092-native-consumer-tinyvec`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-092-native-consumer-tinyvec`
Planning base: `5020cab3b`, rebased onto widened `d5cecdf6c`, then onto post-g16.094 main `26c69e37bf84da3f64f09fc8bb1025338cc9f39c`
C1 runtime: `ed9699dd887405cbcc1287f3ddf5abb37dc4f57b`
C2 evidence head: recorded in PR #199
PR: `#199`

## Outcome

The fresh dual-dependency consumer compiles again under the repository's Rust
1.95 minimum and the stable toolchain. `effigy drift:gpui-consumer-identity`
passes with all positive identity checks intact and the negative control
failing as its intended type mismatch. All 28 Nucleus scenario tests executed
under the std-unified graph from C1 and emitted a complete fresh receipt
cohort; manifest, receipts, compiled lock digest, and generated ledger agree.

## Failure and root cause

crates.io `tinyvec 1.13.0` cannot compile its alloc-only path: the new
`with_initial_len` calls `vec!` where only `alloc::vec::{self, Vec}` is
imported (the module, not the macro), and without the `std` feature the crate
is `no_std`, so `vec!` never resolves. The fresh consumer has no lockfile, so
it resolved 1.13.0 (newest compatible) over `gpui → usvg → fontdb → tinyvec`
with only `tinyvec feature "alloc"`. Committed locks still pinned 1.12.0,
which is why warm in-repo boards stayed green. The negative control degraded
to the same macro error instead of its type mismatch.

## Commits

- Counterexample `553ffd273`: the identity gate names the repaired shape and
  prints the resolved tinyvec version plus the alloc-only feature tree; red
  before the repair, positive identity checks and negative control preserved.
- C1 `ed9699dd8`: node-backend names `tinyvec = { version = "1", features =
  ["std"] }`; feature unification enables `std` graph-wide while resolution
  stays floating. Locks refresh with one added edge each. The compiled
  `LOCKFILE_SHA256` in `packages/gpui/preview/src/nucleus_receipts.rs` advances
  to the new preview lock digest. No evidence JSON in C1.
- After the g16.094 rebase onto `26c69e37b`, the full sequence was replayed and
  the cohort re-emitted from the replayed C1: the runtime tree under
  `SOURCE_PATHS` is byte-identical across the replay, and the fresh emission
  carries the replayed C1 source commit. Carried-forward receipts were not
  used.
- C2: manifest `resolution.lockfile_sha256` and `resolution.source_commit`
  advance to the freshly observed C1 identity, the complete 28-receipt cohort
  is replaced by the C1 emission, and this card and log are updated. The
  generated ledger reproduced byte-identical. No runtime or package source
  after C1.

## Receipt identity

| Field | Value |
| --- | --- |
| Emission command | `effigy regressions:native` from C1, `POODLE_NUCLEUS_RECEIPT_DIR={repo}/target/nucleus-receipts` |
| Suite | 202/202 passed; 28 scenario tests emitted |
| source_commit | `ed9699dd887405cbcc1287f3ddf5abb37dc4f57b` in all 28 receipts |
| lockfile | `packages/gpui/preview/Cargo.lock` |
| lockfile_sha256 | `c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c` |
| Outcome | `passed` |

## Validation

- `effigy regressions:native` from committed C1 — 202/202 passed, 28 fresh
  receipts emitted with the C1 source commit and new lock digest; no machine
  paths in any receipt.
- `effigy check:parity-evidence-ledger` — 176 rows validated after the repin.
- `bun test scripts/parity-evidence-ledger.test.ts` — 6 passed.
- `effigy drift:gpui-consumer-identity` — pass on the stable toolchain and
  under `RUSTUP_TOOLCHAIN=1.95.0`; positive identities pass, the negative
  control fails as the intended type mismatch.
- `effigy ci:rust` — clean.
- `effigy docs:check`, `effigy ci:web`, `effigy ci:native`, and
  `git diff --check origin/main...HEAD` results are recorded in the PR #199
  review comment.

## Limits

- g16.092 only. No other Nucleus row advances; the cohort stays 28/29 mounted.
- No A1, V1, web, workflow, release, version, or native-visual change.
- MessageCenter remains planning-only; its shared receipt finalization waits
  for this repair on main.
- No windowed or native-visual selector ran. Not merged.
