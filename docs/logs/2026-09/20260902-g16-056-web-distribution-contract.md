# g16.056 — Web Distribution Contract

Status: complete — awaiting orchestrator review
Date: 2026-09-02
PR: pending
Card: `docs/roadmaps/g16/056-web-distribution-contract.md`
Handoff: `docs/handoffs/20260902-095101-g16-056-web-distribution-contract.md`
Governing refs: `docs/architecture/014-compiled-web-package-distribution.md`,
`docs/specs/070-compiled-web-distribution-contract.md`,
`docs/specs/022-packaging-versioning-and-release-channel-rules.md`
Branch: `docs/g16-056-web-distribution-contract`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-056-web-distribution-contract`
Base: `origin/main` at `9e8e646f25a1dfde818083c798ffba53adea3e95`
Planning base `a809792c6cd9873f9716b4954d2b4b803c6b65eb` is an ancestor.
The handoff's 40-char planning SHA was a padded short hash and is not an object.

## Outcome

The compiled web boundary is now an exact contract. Spec 070 freezes the 176
roster names, 167 core CSS files, 108 icon modules, Svelte client/server and
React single-lane maps, files/sideEffects, Svelte floor, optional `marked`,
receipt JSON, forbidden archive content, and the root-to-`./markdown` break
without a shim. Successors inherit writable scopes and oracles from that spec.
No package was built, packed, versioned, or released.

## Current-state drift recorded, not repaired

- `files` still ship `src`; Svelte still has a `svelte` field and condition
- Svelte peer is `>=5.38.6 <6`; `marked` is a hard shell dependency
- React exports only `.`; `private: true` already
- 178 Svelte files exist; 176 are roster components. `DragDropProvider` and
  `MenuSurface` become chunks, not public `*.svelte` matches
- `packages/release-manifest.json` still lists React as preview public-intent

## Oracle

| Row | Plant | Result |
| --- | --- | --- |
| SSR choice is explicit | `"import"` before `"default"` on Svelte `.` | forbidden-selector law and "import is absent" reject it |
| Types reachability | `./types` with only `"types"` | "Declarations-only fails" rejects it |
| Source-free wildcard | `./*.svelte` → `./dist/*.svelte` | source class in the forbidden matrix rejects it |
| Parser isolated | ordinary Button/Select graph owns `marked` | dependency table (markdown-only `marked`) rejects it |
| Release authority separate | created `docs/release-notes/0.3.0.md` | g16.056 writable scope / oracle rejects it |

All plants restored. No release-note file remains.

## Validation

- Inventory proof: spec 070 component/CSS/icon lists equal disk and the 176
  frozen roster names
- `effigy svelte:surface-audit` — 176/176 public exports
- `effigy check:parity-evidence-ledger` — 176 rows
- `effigy docs:check` — pass
- `git diff --check origin/main...HEAD` — pass

No windowed, native-visual, build, pack, or release selector.

## Diff scope

Owned surfaces only: architecture 014, spec 070, spec 022 package-boundary
clarification, spec index, g15 roster denominator, g16.056 card, this log,
PAPERCUTS. No build scripts, manifests, versions, workflows, or successor
cards.

## Continuation

Accepted merge unlocks `g16.057` only.
