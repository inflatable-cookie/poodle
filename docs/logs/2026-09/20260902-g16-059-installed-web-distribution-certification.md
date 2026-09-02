# g16.059 — Installed Web Distribution Certification

Status: complete — awaiting orchestrator review
Date: 2026-09-02
Card: `docs/roadmaps/g16/059-installed-web-distribution-certification.md`
Handoff: `docs/handoffs/20260902-141500-g16-059-installed-web-certification.md`
Governing refs: `docs/architecture/014-compiled-web-package-distribution.md`,
`docs/specs/070-compiled-web-distribution-contract.md`
Branch: `feature/g16-059-installed-web-certification`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-059-installed-web-certification`
Base: `origin/main` at `721b17791946a6a7dcf254de0775f353d47090eb`
Required ancestor: PR #162 merge `b43481dff4e25b70fc0b0b076cee116f5e97d93b`
Proof commit: `ba08070f9f89fb5c405d853095df68e95546db5c`
Worker PR: not opened — launcher instruction forbids PR creation

## Outcome

`test:web-pack-install` is now the permanent installed web certification
harness for compiled core, Svelte, and private React. It builds and packs all
three packages twice from one exact commit, compares inventories and archive
hashes, installs archive `file:` references into fresh no-workspace consumers,
and records one deterministic receipt. The clean-checkout wrapper uses a
temporary bare clone and passes an explicit inner-run marker, so attached
worktree artifacts cannot recurse into or contaminate the certification.

The two named clean-main repairs are included:

- declaration bootstrap serializes the nested TypeScript `6.0.3` install and
  validates that resolver path instead of allowing root TypeScript `7`;
- `docs:check` and `health` build the contracted Svelte/React `dist/` outputs
  before export-target and docs audits.

React remains `private: true`; no version, release, workflow, registry,
publication, native, sibling, or component-behavior surface changed.

## Evidence

- Clean exact-commit outer selector: `effigy test:web-pack-install` — 11 test
  files / 22 tests passed.
- Two build/pack rounds matched on output inventories, file hashes, archive
  SHA-256 values, build receipts, provenance, notices, export targets,
  conditions, CSS/parser graphs, and installed declaration surfaces.
- Installed browser and Node SSR probes covered Svelte and React roots/direct
  Button/Select plus all five `./markdown` components. Default Node resolution
  selected server output; direct client output was rejected by SSR.
- Svelte `5.56.8` passed browser and SSR. The named below-floor `5.38.6` leg
  failed visibly with `TypeError: $.delegated is not a function`.
- Existing HistoryEntry, Slider, and Tree positive/negative declaration proofs
  remained active and unsuppressed. Packed negatives produced diagnostics for
  `branchCount`, `SliderAppearance`, and forbidden Tree reorder callbacks.
- Roster denominator: `176`.
- Roster names SHA-256: `f497bfa0a47e1627a1ee7076016ac5566d83584d458b3f3693b688885a02a84a`.
- Artifact-set ID: `0316246f1afc88032066113f2d3b07ede4c48b57ef992dec949f47fd11ae3dbc`.

### Deterministic receipt

Receipt SHA-256: `5d44f1c16ff8843c8271399f7ed39a7928fc811e362b08604bc18bc4daa8b58e`

```json
{
  "schemaVersion": 1,
  "kind": "poodle-installed-web-distribution",
  "sourceCommit": "ba08070f9f89fb5c405d853095df68e95546db5c",
  "svelteFloor": "5.56.8",
  "belowFloorNegative": "5.38.6",
  "rosterDenominator": 176,
  "rosterNamesSha256": "f497bfa0a47e1627a1ee7076016ac5566d83584d458b3f3693b688885a02a84a",
  "artifactSetId": "0316246f1afc88032066113f2d3b07ede4c48b57ef992dec949f47fd11ae3dbc",
  "packages": {
    "@inflatable-cookie/poodle-core": {
      "archiveSha256": "32bb872aa6dc8b92c99970b1530b1820ccddc951b1d5ccb38474b791b69d94c7",
      "buildReceiptSha256": "e47c0704d64f9b7d919b4075886d2941915e8d8b79543c055fafb6c7147174bb",
      "version": "0.2.3"
    },
    "@inflatable-cookie/poodle-react": {
      "archiveSha256": "78abc5eff0e07b111715e240eaa45977706df008a706b36472bcb1c8284c0ed5",
      "buildReceiptSha256": "1fdbfbdf3100eab6b7704418567a0a2cb4ad0138dbf18790f44f737b7dbcefa4",
      "version": "0.2.3"
    },
    "@inflatable-cookie/poodle-svelte": {
      "archiveSha256": "50d6e367137429f95483e4ab3917c86c602b4c6fb3369f87d1f79efba2b12ceb",
      "buildReceiptSha256": "7b98abfd4356fc3e2034fe317184c924afc198d42ed2285cc4483edf7280711b",
      "version": "0.2.3"
    }
  }
}
```

## Falsification receipts

Each oracle was planted at the committed proof point, observed failing, and
restored before the passing run.

| Oracle | Failure receipt |
| --- | --- |
| Installed archive must not resolve to source | `Error: installed package resolved to sibling source` |
| Browser and SSR lanes are both required | `browser client SSR rejection observed: expected client/server rejection: Error: https://svelte.dev/e/effect_orphan` |
| Svelte floor is truthful | `Svelte 5.38.6 runtime failure: TypeError: $.delegated is not a function` |
| Unsuppressed packed declaration negatives bite | `branchCount`, `SliderAppearance`, and Tree reorder callback fixtures returned unsuppressed TypeScript errors with `suppressed: false` |
| Receipt identity changes when evidence is edited | `Error: edited receipt hash differs from the certified identity` |
| Canonical roster denominator rejects a 175-name plant | `Error: Frozen roster denominator changed: expected 176, found 175` |
| Green certification is not a release | `Error: release/workflow path rejected: .github/workflows/release.yml` |

## Baseline repairs

- Cold nested declaration bootstrap reproduced the merged-main race: the first
  concurrent declaration check resolved root TypeScript `7.0.2` instead of
  pinned nested `6.0.3`, then failed on cascaded declarations. The repaired
  lock-and-realpath gate preserves exact `6.0.3`.
- Clean-like `effigy docs:check` reproduced export-target failures when docs
  lint ran before Svelte/React `dist/` existed. Selector ordering now builds
  both shell distributions before audits.

## Validation

- `effigy test:shell-build` from a cold nested declaration-tools cache — 31
  pass / 0 fail, 1,084 expects; nested TypeScript `6.0.3` was installed before
  parallel test workers started.
- `effigy test:web-pack-install` — 11 files / 22 tests passed from the clean
  detached proof checkout.
- `effigy docs:check` — pass.
- `effigy qa` — broad headless board reached the existing license-audit failure:
  `scripts/web-distribution/declaration-tools/package.json: expected license "MIT"`.
  Release automation, web 3,606-test, Rust/headless, and GPUI consumer sections
  passed before that out-of-scope failure.
- `git diff --check origin/main...HEAD` — pass.

No windowed, native-visual, release, workflow-dispatch, merge, or `g16.054`
selector/action was run.

## Limits

- The launcher explicitly forbade creation of a PR, so no PR URL exists for
  this worker branch. The orchestrator owns any PR creation/review needed for
  integration.
- The receipt certifies `ba08070f9f89fb5c405d853095df68e95546db5c`. This log and
  card are documentation-only closeout changes; changing certified code,
  package inputs, or receipt inputs requires rerunning the installed oracle.
- The receipt is promotion evidence, not a release or publication authority.

## Diff scope

Owned surfaces: installed certification harness and fixtures/helpers, the two
named build/selector repairs, package-install docs, this card, this log. No
versions, release surfaces, workflows, tags, registries, native, sibling, or
`g16.054` changes.

## Continuation

Orchestrator review and exact-head integration remain. `g16.054` must stay
blocked until the orchestrator accepts this receipt and separately dispatches
its candidate lane.
