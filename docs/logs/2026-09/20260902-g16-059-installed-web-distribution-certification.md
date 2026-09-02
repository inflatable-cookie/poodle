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
Proof commit: `b37f404737062f9603c097dee0ee8b8916595d1f`
Worker PR: https://github.com/inflatable-cookie/poodle/pull/163

## Outcome

`test:web-pack-install` is now the permanent installed web certification
harness for compiled core, Svelte, and private React. It builds and packs all
three packages twice from one exact commit, compares inventories and archive
hashes, installs archive `file:` references into fresh no-workspace consumers,
and records one deterministic receipt. The clean-checkout wrapper uses a
temporary bare clone and passes an explicit inner-run marker, so attached
worktree artifacts cannot recurse into or contaminate the certification.

The clean-main repairs are included:

- declaration bootstrap serializes the nested TypeScript `6.0.3` install and
  validates that resolver path instead of allowing root TypeScript `7`;
- `docs:check` and `health` build the contracted Svelte/React `dist/` outputs
  before export-target and docs audits.
- declaration-tools carries the repository-required exact `MIT` license
  metadata; dependencies and package behavior are unchanged.
- The scope guard derives the proof commit's changed paths against the
  required base, enforces the explicit writable-path allowlist, and rejects
  workflow, release, version, and registry surfaces.

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
- Artifact-set ID: `2d2fac98557cd50cc4e01d265405771401049060b88776cfe6d4627523903577`.

### Deterministic receipt

Receipt SHA-256: `5cb4b6d2f848e426de868fd56501b1d2c02c368f6ece7f2fb32692394c9f733c`

```json
{
  "schemaVersion": 1,
  "kind": "poodle-installed-web-distribution",
  "sourceCommit": "b37f404737062f9603c097dee0ee8b8916595d1f",
  "svelteFloor": "5.56.8",
  "belowFloorNegative": "5.38.6",
  "rosterDenominator": 176,
  "rosterNamesSha256": "f497bfa0a47e1627a1ee7076016ac5566d83584d458b3f3693b688885a02a84a",
  "artifactSetId": "2d2fac98557cd50cc4e01d265405771401049060b88776cfe6d4627523903577",
  "packages": {
    "@inflatable-cookie/poodle-core": {
      "archiveSha256": "48a6bb351cfa6e0deb9a58749439761b6144ac28d5904da91cf22629d7073793",
      "buildReceiptSha256": "ab491a66187f7eebe5204d84ce19285229177a505a87f555e17fc6de1ee5e28e",
      "version": "0.2.3"
    },
    "@inflatable-cookie/poodle-react": {
      "archiveSha256": "7bdd27881379ab965441caff19980c3d6ce6c1cb526e91ea7850c645c9caed12",
      "buildReceiptSha256": "13207cf17a9f4d89c78ca5fd167b34d6a77ae7b3b8b78891524f04aa7d572699",
      "version": "0.2.3"
    },
    "@inflatable-cookie/poodle-svelte": {
      "archiveSha256": "0ddc8b8caa36b1d9b96d2ff0b43828239be1c98287ade025e12f90ab5799f2f3",
      "buildReceiptSha256": "87b6508be16a44a58d94c0906a7b69a8bf1c8ca7ee1c90c6193b5627e5a3bdb3",
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
| Green certification is not a release | `Error: certification scope rejected forbidden workflow surface: .github/workflows/release.yml` |

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
- Scope falsification — a real temporary two-commit repository changed
  `.github/workflows/release.yml`; the production guard rejected it.
- `effigy docs:check` — pass.
- `effigy qa` — pass. The full headless board passed release automation, web,
  Rust/headless, GPUI consumer, license, security, and advisory checks; license
  compliance reported 9 package manifests, 17 Cargo manifests, and 4 notice
  surfaces clean, with no vulnerabilities.
- `git diff --check origin/main...HEAD` — pass.

No windowed, native-visual, release, workflow-dispatch, merge, or `g16.054`
selector/action was run.

## Limits

- This worker has not merged, released, dispatched a workflow, run a windowed
  selector, or dispatched `g16.054`. The single authorized worker PR is
  [#163](https://github.com/inflatable-cookie/poodle/pull/163).
- The receipt certifies `b37f404737062f9603c097dee0ee8b8916595d1f`. This log and
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
