# g16.061 — Installed Web Smoke / Certification Routing

Status: complete — awaiting orchestrator review
Date: 2026-09-02
Card: `docs/roadmaps/g16/061-installed-web-smoke-certification-routing.md`
Handoff: `docs/handoffs/20260902-215605-g16-061-installed-routing.md`
Governing refs: `docs/architecture/014-compiled-web-package-distribution.md`,
`docs/roadmaps/g16/059-installed-web-distribution-certification.md`,
`docs/roadmaps/g16/054-historycenter-v030-release-candidate.md`
Branch: `fix/g16-061-installed-routing`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-061-installed-routing`
Base: `origin/main` at `ab5ab1dce6df39c98b3a51160e41fac4da1d6d49`
Required ancestor: PR #165 merge `9e38e7971d3acc6d530e83f5e640feac27bbff00`

## Outcome

Default `test:web-pack-install` is ordinary receipt-free installed smoke.
Unset `POODLE_WEB_PACK_INSTALL_SCOPE_MODE` is `ordinary`. Exact certification
requires `strict`. Candidate certification remains `g16.054-candidate`.
Unknown modes reject before clone. Changed filenames never promote a run
into certification.

The shared proof body is unchanged: two clean builds and packs, archive
inspection, installed browser and Node SSR, Svelte floor, below-floor
negative, declarations, CSS/parser, roster. Receipt construction, receipt
hashing, certification-scope evidence, and the receipt-identity plant run
only in explicit certification modes.

No component, package, version, release note, workflow, registry, tag,
sibling, Figmatic, or PR #164 change. No tag, publish, workflow dispatch,
or windowed/native-visual selector.

## Mode table

| Mode | How selected | Empty range | Feature source | Forbidden release surfaces | Receipt |
| --- | --- | --- | --- | --- | --- |
| `ordinary` | unset or `ordinary` | pass | pass | reject before build/pack | none |
| `strict` | `POODLE_WEB_PACK_INSTALL_SCOPE_MODE=strict` | reject | g16.059 allowlist only | reject | written + printed |
| `g16.054-candidate` | explicit env | reject (also not a direct child) | candidate allowlist + Cargo/manifest honesty | reject except candidate honesty paths | written + printed |
| unknown | any other value | n/a | n/a | n/a | reject at parser |

## Reproduction

On current main (`HEAD == origin/main`) before the repair, the inner
production path failed immediately:

```text
error: certification scope found no changed paths
      at assertCertificationScope (.../web-preview.ts:616:15)
```

That is the ordinary `ci:web` failure: unset mode was still g16.059 strict.

## Falsification receipts

All plants used the production `assertInstalledScope` / `readInstalledScopeMode`
router and actual committed two-commit ranges, then restored by deleting the
plant. Each expected failure failed; pass plants returned ordinary proofs
with no receipt emission:

| Oracle | Result |
| --- | --- |
| Unset mode is ordinary, not strict | `readInstalledScopeMode(undefined) === "ordinary"`; `emitsCertificationReceipt` is false |
| Ordinary empty `origin/main..HEAD` | production guard passed; inner smoke 11 files / 22 tests; stdout `mode: ordinary`; no `receiptSha256`; `installed-receipt.json` absent; evidence has no `receipt` or `certificationScope` |
| Ordinary feature source path | `packages/svelte/components/src/Tabs.svelte` accepted |
| Filenames do not infer certification | `PAPERCUTS.md`-only ordinary range stays `ordinary` |
| Ordinary workflow | `workflow surface: .github/workflows/release.yml` |
| Ordinary version | `version surface: packages/core/package.json` |
| Ordinary release | `release surface: CHANGELOG.md` |
| Ordinary registry | `release surface: scripts/publish/release.ts, registry surface: scripts/publish/release.ts` |
| Strict empty range | `certification scope found no changed paths` |
| Strict allowlist | feature `Tabs.svelte` rejected; `PAPERCUTS.md` accepted |
| Candidate unauthorized source | `paths outside writable allowlist: packages/core/src/unauthorized.ts` |
| Candidate Cargo publish | `publish = false, publish = true` |
| Candidate Cargo registry | `registry = "https://registry.example.invalid"` |
| Candidate retargeted requirement | `path = "../../contracts/evil"` |
| Candidate evidence head | `direct one-commit child` + `distance 2` |
| Unknown mode | production entry `must be ordinary, strict, or g16.054-candidate: typo` (exit 1 before clone) |

Candidate Cargo and evidence-head messages match the merged g16.054 log
wording.

## Limits

- This worker has not merged, rebased PR #164, tagged, published, dispatched a
  workflow, run a windowed selector, or contacted Figmatic. Rebase and
  revalidation of PR #164 stay with the orchestrator after merge.
- Strict full-pack receipt emission was not rerun on this branch: the
  branch range includes routing docs outside the g16.059 allowlist, so
  strict correctly rejects it. Strict allowlist/empty/receipt-routing plants
  used the production guard and the shared formatter.

## Continuation

Orchestrator review and exact-head integration remain. After merge, rebase
PR #164 onto current main and rerun its `ci:web`. Tagging/publishing `0.3.0`
remains a separate operator-authorized release mutation.
