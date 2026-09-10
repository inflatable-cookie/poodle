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
| [`g18.003`](003-tiptap-prosemirror-rich-text-editor.md) — TipTap/ProseMirror rich-text editor | complete | PR #237 (merge `fb0b73732b5c0a2a9361fddd75962eccd2710b0f`) |
| [`g18.004`](004-tabs-card-inactive-surfaces.md) — Tabs card inactive surfaces | complete | PR #239 (merge `ed6ed66050c5ba8bf62aaf27eee795ca5be052fa`) |
| [`g18.005`](005-v040-release-preflight.md) — v0.4.0 release preflight | verifying | retained PR #238; g18.007 repair merged, validation retry pending |
| [`g18.006`](006-v040-web-editor-release-and-desktop-unblock.md) — v0.4.0 web editor release and Desktop unblock | planned | g18.003, g18.004, g18.005, and g18.008; final source recheck and explicit operator release authority |
| [`g18.007`](007-ordinary-changelog-maintenance-scope.md) — ordinary changelog maintenance scope | complete | PR #240 (merge `ef2e46bb949a766e844e48f071119c9576c6f723`); retained g18.005 PR #238 retries next |
| [`g18.008`](008-web-editor-preview-specimens.md) — web editor preview specimens | complete | PR #241 (merge `998b6ddc69f94e405b515f6bddd682a2e8916ea5`); Svelte/React catalogue admission via the web-only supplement, serial before g18.006 |
| First functional repair tranche | planning horizon | compile from accepted g18.001 missing-capability output |
| Remaining mounted behaviour tranches | planning horizon | bounded by dependency and interaction substrate, not arbitrary component count |
| GPUI visual expansion | planning horizon | functional tranche stable; operator-approved background-safe capture |
| GPUI keyboard-origin focus | planning horizon | census identifies affected focus-bearing rows |
| A2 platform accessibility | held | `gpui-apple` publishes a buildable crate and live non-activating tree proof is available |
| Nucleus V2/M2/switch packet | external horizon | Nucleus-owned seeding and journeys plus Lab V2 and A2 |

## Approved frontier

`g18.003` and `g18.004` are complete after independent review and plugin-owned
merge. The retained `g18.005` preflight remains bounded to the existing
changelog/parser blocker and its `g18.007` structural repair; release, Desktop
adoption, native editor work, and GPUI repair tranches do not auto-start.

g18.005 is implemented and independently accepted, but ordinary web CI rejected
every changelog change by path. The operator-selected g18.007 structural,
content-aware maintenance rule merged as `ef2e46bb949a766e844e48f071119c9576c6f723`
(PR #240). PR #238 and its Queue threads remain retained; Queue retries
validation now that g18.007 merged.

The merged editor tasks had omitted their baseline preview catalogue pages.
Operator-confirmed g18.008 added separate CodeEditor, RichTextEditor, and
RichTextRenderer specimens to both web previews through the existing web-only
supplement (PR #241, merged `998b6ddc69f94e405b515f6bddd682a2e8916ea5`). It ran
independently of g18.007 and added no native parity.

After preflight closes, `g18.006` prepares and publishes the
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
