# g15.014 — Release-Gate Remediation: Security Advisory Prerequisite

Status: complete — remediation landed, `effigy qa` green (PR pending)
Date: 2026-08-17
Card: `docs/roadmaps/g15/014-release-gate-remediation.md`
Handoff: `docs/handoffs/20260817-010454-g15-014-release-gate-remediation.md`
Governing refs: `docs/roadmaps/g15/release-gap-register.md`,
`docs/contracts/001-working-rules.md`

## Advisory

GHSA-2v37-7h3g-55p8 — `nanoid < 3.3.18`, high severity: custom generators can
loop indefinitely when size is zero. `bun audit` reported it as the one open
release gate on `effigy qa`.

## Dependency Path

The root workspace pins a blanket `nanoid` override in
`package.json` → `overrides.nanoid = "^3.3.16"`, which resolved `nanoid@3.3.17`
in `bun.lock`. `nanoid@3.3.17` is consumed by `postcss@8.5.26`
(`nanoid ^3.3.17`), which `vite@8.2.1` depends on. `bun audit` reports the
chain from the owning workspace as `poodle-react-preview › vite`. No package
depends on a nanoid major other than 3, and the override is the single pinning
surface.

## Change (Batch A then Batch B)

Batch A confirmed the path and that the fix is a patch-level bump, not an
override removal, a replacement, or a Vite/toolchain move. Batch B applied the
smallest supported change:

- `package.json`: `overrides.nanoid` `^3.3.16` → `^3.3.18` (patched line,
  latest 3.x).
- `bun.lock`: regenerated with `bun install`; only the `nanoid` entry moved
  `3.3.17` → `3.3.18` (version plus integrity hash). `postcss`'s `^3.3.17`
  requirement remains satisfied.

No override removal, no `bun update` sweep, no unrelated lockfile churn. The
full diff is three lines: one override in `package.json`, two lockfile lines.

## Validation

| Gate | Result |
| --- | --- |
| `effigy audit:security` | pass — `bun audit`: "No vulnerabilities found"; cargo deny advisories ok x4; security hygiene clean |
| `effigy qa` | pass (exit 0) — full headless board incl. `ci`, `ci:native`, `test:web-pack-install`, `audit:licenses`, `audit:security` |
| `effigy react:build` | pass — vite v8.2.1 build completed |
| `effigy test:components` | pass — 335 files, 2604 tests |
| `effigy docs:check` | pass |
| `git diff --check origin/main...HEAD` | pass |

No `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or Jetstream
selector ran.

## Parallel Safety

This lane touched only the root dependency manifest, `bun.lock`, and this log.
It did not enter PR #29's surfaces (component code, tests, contracts,
roster/register). Card, roadmap, register, and dispatch status were not
modified by the worker.

## Change Footprint

`package.json` (override line), `bun.lock` (nanoid entry), this log. No
component, contract, specimen, public API, or version-pinned config beyond the
override changed.
