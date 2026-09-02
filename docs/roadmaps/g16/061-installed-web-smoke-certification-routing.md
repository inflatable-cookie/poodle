# g16.061 — Installed Web Smoke / Certification Routing

Status: complete — awaiting orchestrator review
Type: validation infrastructure repair
Opened: 2026-09-02
Depends on: merged `g16.054` candidate (`9e38e7971`); completed `g16.059`
Governing refs: `059-installed-web-distribution-certification.md`,
`054-historycenter-v030-release-candidate.md`,
`../../architecture/014-compiled-web-package-distribution.md`,
`../../contracts/001-working-rules.md`

## Goal

Make the permanent installed-package harness usable by ordinary feature PRs
without producing receipt-shaped certification evidence. Keep strict and
candidate certification explicit, fail closed, and unchanged in substance.

## Fixed Boundary

- `effigy test:web-pack-install` with no scope-mode environment variable is an
  ordinary installed-package smoke. It runs the full installed archive,
  browser/SSR, declaration, negative, CSS/parser, deterministic build/pack,
  and roster checks, but emits no certification receipt or receipt hash.
- Ordinary mode derives the real `origin/main` merge-base and changed paths.
  Empty ranges are valid. Workflow/action, release, version, registry/publish,
  tag/transport, and sibling surfaces still reject before build/pack.
- Exact certification requires
  `POODLE_WEB_PACK_INSTALL_SCOPE_MODE=strict`. It preserves the g16.059
  writable allowlist, non-empty range, receipt bytes, receipt hash, and
  falsification behavior.
- Candidate certification remains
  `POODLE_WEB_PACK_INSTALL_SCOPE_MODE=g16.054-candidate`. Its direct-child,
  candidate allowlist, manifest/Cargo honesty, receipt, and falsification
  behavior are unchanged.
- Do not infer certification authority from filenames or a range that happens
  to fit an allowlist. A receipt exists only in an explicit certification
  mode.
- No component, package output, public API, version, release note, workflow,
  registry, tag, or sibling change belongs here.

## Ordered Work

1. Split scope routing into `ordinary`, explicit `strict`, and explicit
   `g16.054-candidate`; make unset mode resolve to `ordinary`.
2. Keep one shared installed-package proof body. Route receipt construction,
   receipt hashing, certification-scope evidence, and receipt-specific
   falsifications only through explicit certification modes.
3. Add production-guard counterexamples:
   ordinary feature source path passes and produces no receipt; empty main
   range passes and produces no receipt; ordinary workflow/release/version/
   registry paths reject; explicit strict retains the g16.059 allowlist and
   receipt; explicit candidate retains direct-child and Cargo guards; unknown
   mode rejects.
4. Update canonical certification docs and the open papercut. Rebase accepted
   PR #164 onto the merged repair and rerun its required `ci:web` separately.

## Acceptance

- An ordinary source-only feature range completes `test:web-pack-install` and
  cannot write or print a certification receipt/hash.
- An empty `origin/main..HEAD` range completes the same receipt-free smoke.
- Ordinary forbidden release-bearing ranges fail before build/pack.
- Explicit strict mode reproduces the g16.059 receipt semantics; explicit
  candidate mode reproduces the merged g16.054 candidate guard semantics.
- `ci:web` becomes green for the rebased g16.060 implementation without any
  bypass, source resolution, reduced installed proof, or receipt ambiguity.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Ordinary smoke is not certification | ordinary feature run prints `receiptSha256` | output/receipt absence test fails |
| Ordinary validation remains real | ordinary mode skips archive/browser/SSR/type tests | retained installed assertions fail |
| Release surfaces fail closed | ordinary range edits `package.json` version or workflow | production scope guard rejects before build |
| Strict stays exact | unset mode silently emits the g16.059 receipt | explicit-mode test fails |
| Candidate stays exact | evidence head or Cargo publish metadata passes candidate mode | merged candidate falsifications fail |
| Unknown modes fail | typo becomes ordinary | mode parser rejects |

## Writable Scope

`test/package-install/web-preview.ts`, its focused helpers/tests if required,
`PAPERCUTS.md`, this card, one execution log, g16.059 routing clarification,
and the minimum Effigy selector configuration only if the existing selector
cannot express the fixed boundary. Do not edit component code, packages,
versions, release notes, workflows, registries, tags, siblings, or PR #164.

## Validation

Run focused routing/guard plants, ordinary `effigy test:web-pack-install`,
explicit strict reproduction against a suitable committed fixture/range,
explicit candidate guard regressions, `effigy ci:web`, `effigy docs:check`,
`effigy qa`, and `git diff --check origin/main...HEAD`. Never run release,
workflow-dispatch, windowed, or native-visual selectors.

## Continuation

After accepted merge, rebase PR #164 onto current main, rerun its required
boards, re-review the changed exact head, merge it, and return the merged SHA
and local-link instructions to Figmatic. Tagging/publishing the accepted
`0.3.0` candidate remains a separate operator-authorized release mutation.

## Evidence

Production router lives in `test/package-install/scope.ts`. Unset mode is
ordinary. The empty-range inner smoke on `ab5ab1dce6df39c98b3a51160e41fac4da1d6d49`
passed 11 files / 22 tests with `mode: ordinary` and no receipt or receipt
hash. Focused plants and the merged candidate Cargo/evidence-head failures
are recorded in
`docs/logs/2026-09/20260902-g16-061-installed-web-smoke-certification-routing.md`.
PR #164 rebase stays with the orchestrator.
