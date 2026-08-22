# g15.048 — Packed Roster Reachability

Status: **complete** — PR #64 merged as `1bb30492`
Depends on: all pre-release web public API changes, including `g15.041`
Unblocks: `g15.050`
Governing refs: `../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`release-baseline-roster.md`, `013-v020-release-certification.md`

## Goal

Prove the full public Svelte and React rosters are reachable from clean packed
tarballs without turning package installation into a second 175-component
specimen or behavior suite.

## Scope

- Pack core, Svelte, and React from the repository under test and install them
  into a clean consumer with no workspace aliases, sibling source imports, or
  undeclared dependencies.
- Generate or check one exact import inventory from the frozen 175-component
  roster for each web runtime. Every public component export must resolve from
  the package root.
- Keep a small representative mount set for runtime machinery: styles/tokens,
  snippets/render props, overlays, context providers, composite state, and the
  public APIs changed late in g15.
- Verify core public subpaths and generated assets survive packing.
- Update the roster's pack-install column mechanically from the new proof.

## Acceptance

- [x] All 175 Svelte and all 175 React component names import from clean packed
      roots; a missing or extra export fails with the exact name.
- [x] Core, Svelte, and React tarballs contain their declared entry points,
      types, styles, generated tokens/icons, licence, README, and manifests.
- [x] Representative Svelte and React mounts run from installed tarballs.
- [x] The proof does not maintain 175 fake prop fixtures or claim behavior from
      import success.
- [x] The React package is packed and certified even if `g15.050` keeps it
      unpublished.

## Completion Evidence

PR #64 was accepted at `db1e2632` and merged as `1bb30492`. The packed-root
proof imports exactly 175 Svelte and 175 React components, rejects named extra
exports, retains a bounded 9-Svelte/13-React runtime mount set, and passed an
independent temporary `0.2.0` package-version mutation. This unblocks the
package-proof input to `g15.050`; visual conformance, native presentation
context, release automation, and the remaining release gates stay separate.

## Stop Conditions

- A component can only be imported through workspace source or an undeclared
  dependency.
- The roster and public roots disagree about the 175-name denominator.
- The proof starts duplicating focused component behavior evidence.

## Writable Scope

- `test/package-install/`
- generated/checkable roster import inventory
- `release-baseline-roster.md` and one August batch log
- package metadata only where required to fix a real packed-boundary defect
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `effigy test:web-pack-install`
- relevant Svelte/React package checks
- `effigy docs:check`
- `git diff --check origin/main...HEAD`
