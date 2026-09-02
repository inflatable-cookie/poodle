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
Proof commit: `fb35a2eb83ddd060ca5d37377ff99c71c9c12189`
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
- Artifact-set ID: `1cf40d3bdd516f76af5ef44d246141a2ad148d948003a6fac91e207b95ed0347`.

### Deterministic receipt

Receipt SHA-256: `f45f5f143df32da1e906c77acfb25e22935bfff88f8cba0efba4ee2ca80010bf`

```json
{
  "schemaVersion": 1,
  "kind": "poodle-installed-web-distribution",
  "sourceCommit": "fb35a2eb83ddd060ca5d37377ff99c71c9c12189",
  "svelteFloor": "5.56.8",
  "belowFloorNegative": "5.38.6",
  "rosterDenominator": 176,
  "rosterNamesSha256": "f497bfa0a47e1627a1ee7076016ac5566d83584d458b3f3693b688885a02a84a",
  "artifactSetId": "1cf40d3bdd516f76af5ef44d246141a2ad148d948003a6fac91e207b95ed0347",
  "packages": {
    "@inflatable-cookie/poodle-core": {
      "archiveSha256": "a20d18cfc85cb0d7f73cef6054427c1039c09e2e9ad8a89e49b73128c7f7ff3b",
      "buildReceiptSha256": "c11b72992391a7bb876fdf08e21d073aae1ff72b7b6319ed6d99012fa517b033",
      "version": "0.2.3"
    },
    "@inflatable-cookie/poodle-react": {
      "archiveSha256": "2f9df2c4eba97270a6104faa91cfb8598fd84720dc85439195819b8bcaf4761c",
      "buildReceiptSha256": "10882d4fb89bdcd05136ab95a29b2b2be96deba44878408e537ff21afa820486",
      "version": "0.2.3"
    },
    "@inflatable-cookie/poodle-svelte": {
      "archiveSha256": "31f0121dd32717ca71105e891d7bf89e5e3163228404925dd9fb1838ec061702",
      "buildReceiptSha256": "8a396cb075578f4737d31d9fb37c7244f571969ea9563dd8af5f7335bb060469",
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
- The receipt certifies `fb35a2eb83ddd060ca5d37377ff99c71c9c12189`. This log and
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
