# g18 — GPUI functional completion

Status: active
Opened: 2026-09-09
Updated: 2026-09-10
Governing refs: `../../../README.md`, `../../README.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../architecture/003-component-docs-ia-and-implementation-substrates.md`,
`../../contracts/001-working-rules.md`,
`../../contracts/003-native-accessibility.md`,
`../../specs/008-parity-evidence-documented-delta-and-downstream-extension-rules.md`,
`../../evidence/nucleus/parity-evidence-ledger.md`,
`../generation-index.md`

## Generation outcome

Turn the 175-component portable GPUI surface from construction coverage into
contract-bound functional evidence, repair real gaps in bounded tranches, and
make it impossible to call a component complete from a specimen route or test
name alone.

This generation does not revive the rejected g14 conformance corpus. Component
contracts remain semantic authority; mounted runtime evidence proves only the
capabilities it actually drives.

## Current state

- 176 public components; 175 portable native targets; MeterSurface is the one
  contract-approved web-only row.
- 175/175 GPUI specimen routes construct headlessly.
- 29/175 components have validated mounted-behaviour receipts through the
  Nucleus cohort; 146 remain missing in the current ledger.
- Of those 146, 44 have retained named mounted-test expectations but no
  validated execution receipt. The other 102 have no admitted mounted receipt
  or retained expected-test entry.
- GPUI visual evidence covers the 29-row Nucleus cohort; 146 rows remain
  missing. Broad platform accessibility proof remains held separately.
- g18.001 census (PR #235): 73/175 portable rows carry at least one admitted mounted capability, 24/175 fully admitted; 224 refusals and 11 substrate groups await tranche compilation.

Construction is not functional completion. A bounded regression is not whole-
contract proof. The first task establishes the capability-level denominator
needed to compile honest repair tranches.

## Generation runway

| Task or planning horizon | State | Dependency or checkpoint |
| --- | --- | --- |
| [`g18.001`](001-contract-bound-gpui-functionality-census.md) — contract-bound GPUI functionality census | complete | PR #235 (merge `8185a9758f146e901499a8a8704a41202f99ca9e`) |
| [`g18.002`](002-codemirror-web-code-editor.md) — CodeMirror web CodeEditor | complete | PR #236 (merge `308fa52c5cd68d9c776f320c368e4fb0896e4d4d`) |
| [`g18.003`](003-tiptap-prosemirror-rich-text-editor.md) — TipTap/ProseMirror rich-text editor | in execution | Queue task `edb2d303-38c9-44a8-aa2c-a6ff5821643e`; web-only staged admission |
| [`g18.004`](004-tabs-card-inactive-surfaces.md) — Tabs card inactive surfaces | ready after dependency and separate operator go | `g18.003` closeout; serial generated docs/evidence |
| [`g18.005`](005-v040-web-editor-release-and-desktop-unblock.md) — v0.4.0 web editor release and Desktop unblock | planned | `g18.003` and `g18.004`; final source recheck and explicit operator release authority |
| First functional repair tranche | planning horizon | compile from accepted g18.001 missing-capability output |
| Remaining mounted behaviour tranches | planning horizon | bounded by dependency and interaction substrate, not arbitrary component count |
| GPUI visual expansion | planning horizon | functional tranche stable; operator-approved background-safe capture |
| GPUI keyboard-origin focus | planning horizon | census identifies affected focus-bearing rows |
| A2 platform accessibility | held | `gpui-apple` publishes a buildable crate and live non-activating tree proof is available |
| Nucleus V2/M2/switch packet | external horizon | Nucleus-owned seeding and journeys plus Lab V2 and A2 |

## Approved frontier

`g18.003` is executing through Northstar Queue. `g18.004` is the sole approved
next task: a fill-only correction that makes every card Tabs item carry the
semantic surface background while preserving existing selection strength. It
waits for `g18.003` closeout and a separate operator go. Release, Desktop
adoption, native editor work, and GPUI repair tranches do not auto-start.

After both component tasks close, `g18.005` prepares and publishes the
SemVer-minor `0.4.0` web release to unblock Desktop's retained g02.058 task and
PR #215. It is not ready: Chatterbox must recheck the final package trees and
the operator must explicitly authorize release mutation.

## Held and recurring work

- Web-pair composite extraction remains an operator checkpoint when a shared
  composite is next touched or a second React consumer arrives.
- Contributor design-guidance pilot remains operator-gated on named reviewers,
  approvals, and run custody.
- Jetstream admission remains a separate held programme.
- GPUI and shared-Rust CodeEditor work is a future planning horizon. The web
  admission earns no native parity credit.
- GPUI and shared-Rust rich-text editing are future planning horizons. The
  `g18.003` web admission earns no native parity credit.
- Citations, nested menus, keyboard geometry, consumer requests, repository
  settings, and `gpui-unofficial` adoption gates remain in current triage.

## Completion rule

g18 completes only when every portable row has capability-level disposition:
validated mounted proof, a repaired and proved implementation, or a current
contract/platform hold with a named owner and recheck. Aggregate construction
counts and source presence cannot satisfy this rule.
