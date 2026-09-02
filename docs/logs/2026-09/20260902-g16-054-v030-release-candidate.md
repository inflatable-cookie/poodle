# g16.054 — HistoryCenter v0.3.0 Candidate Certification

Status: candidate certified — awaiting orchestrator review
Date: 2026-09-02
Card: `docs/roadmaps/g16/054-historycenter-v030-release-candidate.md`
Handoff: `docs/handoffs/20260902-163000-g16-054-v030-candidate.md`
Branch: `release/g16-054-v030-candidate`
Candidate base: `0a4a329733dfb63bd5e70977d54581e4fef000c8`
Candidate tree commit: `9b451c48d2fc8ea990313fcd441056169a40f26c`
g16.059 prerequisite proof commit: `b37f404737062f9603c097dee0ee8b8916595d1f`
g16.059 prerequisite PR #163 merge object: `536c9f7435c779e5829016432394d0c01ec442ee`
Separate main docs correction: `d82ba7202` — candidate was not rebased

## Outcome

The immutable `0.3.0` candidate is certified from a clean detached checkout of
the exact candidate tree commit. The candidate tree is the first semantic
commit after the frozen dispatch tip. The second semantic commit contains only
this log and the card's evidence record.

The candidate is not a release. No tag, publish, workflow dispatch, registry
mutation, sibling write, React admission, Loophole change, or release command
was performed.

## Review repairs

PR review
[#165#issuecomment-5515821870](https://github.com/inflatable-cookie/poodle/pull/165#issuecomment-5515821870)
required four repairs; all changed candidate inputs or harness, so the
candidate was re-frozen and fully recertified:

1. Candidate scope admitted arbitrary content inside allowlisted Cargo
   manifests. The production guard now restricts every changed line of the 17
   allowlisted `Cargo.toml` files to `[package]` `version = "0.2.3"` →
   `"0.3.0"` and same-identity intra-repository Poodle requirement version
   changes (`name` and `path` must be preserved, only `0.2.3` → `0.3.0` may
   change, `[dependencies]`/`[dev-dependencies]` only). Publication,
   registry/source, `[patch]`, and `[replace]` content is rejected, as are
   unpaired adds/removes and every other manifest shape.
2. `docs/release-notes/0.2.3.md` now states that `0.2.3` is historical
   repository state only and is not installable from any registry; web
   consumers remain on released `0.2.2` versions.
3. Candidate mode no longer derives its required base as `proofCommit^`. It
   derives the real branch base (merge-base with `origin/main`) and requires
   the certified source to be the direct one-commit child of that base, so the
   evidence head and hidden prior candidate commits are rejected as candidate
   sources by the production guard.
4. This log embeds the exact deterministic receipt JSON bytes; the receipt
   SHA-256 below is the digest of those bytes plus the trailing newline.

## Scope guard repair

The existing g16.059 installed-certification guard remains strict when
`POODLE_WEB_PACK_INSTALL_SCOPE_MODE` is unset or `strict`. The ordinary
`effigy test:web-pack-install` invocation therefore remains strict and rejects
this candidate before build/pack.

The only additional mode is the explicit
`g16.054-candidate` value. Its production guard derives the real branch base
as the merge-base of the certified commit with `origin/main`, requires the
certified source to be the direct one-commit child of that base, derives
actual changed paths from that base/commit range, and accepts only:

- the existing g16.059 certification harness, card, log, and papercut surfaces;
- the exact candidate package manifests, Cargo manifests, and tracked Cargo
  locks;
- the exact generated IR, catalogue, and version-stamp output paths;
- `bun.lock`, `CHANGELOG.md`, the `0.2.3`/`0.3.0` release-note surfaces, and
  the three package README surfaces.

Workflow/action, release transport, registry/publish, tag, sibling,
component-behaviour, React-admission, and every other path remain rejected.
Candidate npm package-manifest content is also checked: only the required
version and core dependency leaves may change, all candidate package versions
are `0.3.0`, and React remains `private: true`. Candidate Cargo content is
restricted as described in the review repairs above.

## Deterministic receipt

Certification command:
`POODLE_WEB_PACK_INSTALL_SCOPE_MODE=g16.054-candidate effigy test:web-pack-install`

The clean wrapper certified 11 test files / 22 tests, two build/pack rounds,
matching dist inventories and archive bytes, fresh archive-file consumers,
browser and Node SSR lanes, the Svelte floor, packed declarations, and the
canonical 176-name roster.

Receipt SHA-256: `55d8d8df7431f5da9ef7ab9bf8b11c290fbf17f1513f9b5d4f12b7d5bc5f9464`
Artifact-set ID: `beb27d07c468559dbdf1366b72e1274f78badce7a393f04ecad6a592a1bdd99d`
Roster names SHA-256: `f497bfa0a47e1627a1ee7076016ac5566d83584d458b3f3693b688885a02a84a`

```json
{
  "schemaVersion": 1,
  "kind": "poodle-installed-web-distribution",
  "sourceCommit": "9b451c48d2fc8ea990313fcd441056169a40f26c",
  "svelteFloor": "5.56.8",
  "belowFloorNegative": "5.38.6",
  "rosterDenominator": 176,
  "rosterNamesSha256": "f497bfa0a47e1627a1ee7076016ac5566d83584d458b3f3693b688885a02a84a",
  "artifactSetId": "beb27d07c468559dbdf1366b72e1274f78badce7a393f04ecad6a592a1bdd99d",
  "packages": {
    "@inflatable-cookie/poodle-core": {
      "archiveSha256": "8a14b853065a7aaaae10630949cb76304bbd16df50cddf847e8675b1b52e91c4",
      "buildReceiptSha256": "1acaea01198683959e3ebd1804ba4c3d7d479ec6d1246ed018d0d687f6265112",
      "version": "0.3.0"
    },
    "@inflatable-cookie/poodle-react": {
      "archiveSha256": "bdc9f62874a26b510e03233d55acfc06b0625ae9d208c794d1f848b53c477716",
      "buildReceiptSha256": "038f3c23c1c5fae9005e7717aa85ab8d69a55e8e428bbac29befdf97dcdb450c",
      "version": "0.3.0"
    },
    "@inflatable-cookie/poodle-svelte": {
      "archiveSha256": "2d83c859a3d8f7fe92d867deb50414873d4dd5e5f07d2785459780757e4e8b4d",
      "buildReceiptSha256": "a46349ef48e3d806d5f4ca63521f7bcd2d30d0bccf2c4a6220bdd2cdb8597ae6",
      "version": "0.3.0"
    }
  }
}
```

| Package | Version | Archive SHA-256 | Build receipt SHA-256 |
| --- | --- | --- | --- |
| `@inflatable-cookie/poodle-core` | `0.3.0` | `8a14b853065a7aaaae10630949cb76304bbd16df50cddf847e8675b1b52e91c4` | `1acaea01198683959e3ebd1804ba4c3d7d479ec6d1246ed018d0d687f6265112` |
| `@inflatable-cookie/poodle-react` | `0.3.0` | `bdc9f62874a26b510e03233d55acfc06b0625ae9d208c794d1f848b53c477716` | `038f3c23c1c5fae9005e7717aa85ab8d69a55e8e428bbac29befdf97dcdb450c` |
| `@inflatable-cookie/poodle-svelte` | `0.3.0` | `2d83c859a3d8f7fe92d867deb50414873d4dd5e5f07d2785459780757e4e8b4d` | `a46349ef48e3d806d5f4ca63521f7bcd2d30d0bccf2c4a6220bdd2cdb8597ae6` |

## Falsification receipts

All plants used the production `assertCertificationScope` guard and actual
two-commit ranges. Each was expected to fail and did fail:

| Oracle | Failure receipt |
| --- | --- |
| Default installed certification stays strict | `effigy test:web-pack-install` without the mode exited 1 on `CHANGELOG.md` and candidate version surfaces before build/pack |
| installed archive must not resolve to source | `installed package resolved to sibling source` |
| browser plus SSR lanes are both required | `browser client SSR rejection observed: expected client/server rejection` |
| Svelte floor is truthful | `Svelte 5.38.6 runtime failure` |
| unsuppressed packed declaration negatives bite | `branchCount, SliderAppearance, and Tree reorder callback fixtures returned unsuppressed TypeScript errors` |
| receipt identity changes when evidence is edited | `edited receipt hash differed from the certified identity` |
| canonical roster denominator rejects a 175-name plant | `Frozen roster denominator changed: expected 176, found 175` |
| certification scope rejects a real workflow mutation | `certification scope rejected forbidden workflow surface: .github/workflows/release.yml` |
| candidate scope rejects an unauthorized source path | `paths outside writable allowlist: packages/core/src/unauthorized.ts` |
| candidate scope rejects a workflow transport mutation | `workflow surface: .github/workflows/release.yml` |
| candidate scope rejects a publish transport mutation | `release surface: scripts/publish/release.ts, registry surface: scripts/publish/release.ts` |
| candidate scope rejects Cargo publish content in an allowed manifest | `Cargo publication/registry/source content in packages/contracts/tokens/Cargo.toml: publish = false, publish = true` |
| candidate scope rejects Cargo registry content in an allowed manifest | `Cargo publication/registry/source content in packages/contracts/tokens/Cargo.toml: registry = "https://registry.example.invalid"` |
| candidate scope rejects retargeted intra-repository Cargo requirements | `unauthorized Cargo manifest change in packages/contracts/tokens/Cargo.toml: poodle-ir = { version = "0.2.3", path = "../../contracts/ir" } -> poodle-ir = { version = "0.3.0", path = "../../contracts/evil" }` |
| candidate scope rejects the evidence head as the candidate source | `Error: candidate scope requires the certified source to be the direct one-commit child of 69640b6838af5ffe9c21805fc26e120dfcd0162a; evidence heads or hidden prior candidate commits are rejected (source 8d78148e06fc833b983afbc938c256b145c9ba4f, first parent 1a596d376251d7f9bb8f1c925d8fccfb6dd8305b, distance 2)` |

## Validation

- `effigy ir:build` / `effigy ir:check` — pass; all generated artifacts current.
- `effigy catalogue:build` / `effigy catalogue:check` — pass.
- `effigy test --plan` — pass; default Vitest plan selected.
- `effigy check:release-automation` — pass; 5 retained workflows, Effigy gate,
  alias, and publish set checked.
- `effigy audit:licenses` — pass; 9 package manifests, 17 Cargo manifests,
  and 4 notice surfaces clean.
- `effigy audit:security` — pass; 4,798 repository files, no credential
  patterns, lifecycle hooks, or remote dependencies; Bun audit found no
  vulnerabilities.
- `effigy drift:gpui-consumer-identity` — pass; one crates.io GPUI identity and
  the wrong-type negative control both verified.
- `effigy docs:check` — pass; docs lint, generated reports, package builds, and
  clean-tree writer guard passed.
- `POODLE_WEB_PACK_INSTALL_SCOPE_MODE=g16.054-candidate effigy qa` — pass;
  full headless board passed.
- `POODLE_WEB_PACK_INSTALL_SCOPE_MODE=g16.054-candidate effigy release gates
  --json` — pass; the one configured `headless` gate passed. The no-mode form
  was also checked and correctly failed at the strict scope guard.
- `git diff --check` — pass after candidate validation.

## Tag and registry absence

- Local `v0.3.0` and `v0.2.4` tag checks — no output.
- Remote `origin` `v0.3.0` and `v0.2.4` tag checks — no output.
- npm `@inflatable-cookie/poodle-core` — `0.2.2`, `latest=0.2.2`.
- npm `@inflatable-cookie/poodle-svelte` — `0.2.2`, `latest=0.2.2`.
- npm `@inflatable-cookie/poodle-react` — not found; remains unpublished/private.

## Limits

This is candidate evidence only. Do not merge or begin adoption from this
worker lane. The next authority gate is orchestrator review of the exact PR
head. Separate authorization is required for any later tag, release workflow,
npm/registry publication, crate publication, React admission, Jetstream
admission, Loophole adoption, or sibling change.

No `release prepare`, `release execute`, `release simulate`, tag creation,
publication, workflow dispatch/edit, registry mutation, sibling edit, merge,
windowed selector, or native-visual selector was run.
