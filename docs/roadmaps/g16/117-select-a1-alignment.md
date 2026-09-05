# g16.117 — Select A1 Alignment

Status: complete — merged in PR #218 at `ef9049f15`; the chevron toggle followed the card default (contract decides, no consumer override)
Type: bounded parity repair — contract, Svelte, shared Rust render; A1 receipt
Opened: 2026-09-05
Depends on: merged `g16.111` (`3dea40372`)
Governing refs: `../../contracts/components/select.md` (accessibility section,
lines ~257–304), `../../contracts/001-working-rules.md` (Svelte is the
reference; contracts remain the semantic authority),
`../../contracts/003-native-accessibility.md`,
`nucleus-parity-receipts/a1-divergences/select/` (both snapshots and the
16-entry diff, reproduced byte-for-byte by independent review),
`packages/svelte/components/src/Select.svelte`, `packages/render/src/select.rs`
Dispatch manifest: `../dispatch.md`

## What the divergence is

Five real differences; the rest of the 16 entries are index shifts caused by
the first one.

| # | Svelte (reference) | GPUI node tree | Contract says | Verdict |
| --- | --- | --- | --- | --- |
| 1 | non-searchable trigger is a `button` with `aria-expanded`, `aria-haspopup="listbox"`, `aria-controls` | trigger role `combobox` | line 268: non-searchable trigger button; `combobox` only for the searchable trigger area | GPUI repairs: `ButtonLike` role with expanded/haspopup/controls when not searchable |
| 2 | trigger has no value text; the selected label is part of the name | trigger `value_text` "Banana" | silent | Svelte is reference: drop trigger `value_text`; name carries the label |
| 3 | a focusable chevron toggle button named "Open options"/"Close options" (`Select.svelte:479,524`) | no such node | line 170 names a chevron indicator; silent on focusability | **Decision below** |
| 4 | listbox named "Fruit" via `aria-label` (`:560`) | listbox name `null` | line 269 silent on name; 003's root-only rule is written for Jetstream | Svelte is reference: project the label onto the listbox as well; contract gains one line |
| 5 | options are sequential focus stops (`<button role="option">`, no `tabindex="-1"`) | options not focusable; highlight via active descendant | line 302: highlight moves via `aria-activedescendant`, no DOM focus change | Svelte repairs: options `tabindex="-1"`; one tab stop per Select |

## Contract Decision (operator)

Row 3. The chevron is a second tab stop inside a Select. The APG
select-only combobox pattern has one tab stop; the contract names the
chevron as an indicator, not a control. Recommendation: make the chevron
non-focusable and decorative in Svelte (`tabindex="-1"`, `aria-hidden`),
keep pointer toggling, and leave GPUI without a chevron node. Alternative:
keep it focusable and make GPUI expose a matching `button` node named
"Open options"/"Close options". Default if unanswered when the worker
reaches it: the recommendation, recorded in the contract.

## Fixed Boundary

- Contract: add the listbox-name line and the chevron decision to
  `select.md`; no other contract change.
- Svelte: options `tabindex="-1"`; chevron per the decision. No behaviour
  change for pointer or keyboard selection.
- Render: non-searchable trigger role per line 268 with expanded, haspopup
  (listbox), and controls; drop trigger `value_text`; project `aria_label`
  onto the listbox; searchable path unchanged.
- React: mirror the Svelte changes (options, chevron) so the React drift
  gate stays green.
- Re-run the A1 receipt for Select through `g16.111`'s paired runner; the
  diff must be empty and the receipt validated; the row's ledger cell moves
  to `mounted`. Delete the divergence store for Select in the same PR.

## Runtime Identity (rule, 2026-09-05)

The receipt checker binds every receipt to `manifest.resolution.source_commit`
and its lock digest, and verifies the runtime source paths are unchanged since
that commit. A lane that changes any runtime source path (this includes
`packages/gpui/preview` A1 tests and extractor edits) therefore MUST, at its
final exact head after rebasing onto `main`: repin `resolution.source_commit`
and the lock digest, re-emit the entire Nucleus cohort (all M1 receipts and
every A1 receipt already on `main`) through the real selectors, and validate
the cohort. This is the `g16.105`/`106`/`111` practice, not a scope widening.
Manifest edits are limited to the resolution block; the 29-row cohort and
scenario ids never change. When several tranches are open, the coordinator
merges them one at a time and each later head re-emits at its rebase; the
reviewer checks the cohort validates at the exact merged head. The lane's
PR also commits the regenerated parity ledger
(`docs/roadmaps/g16/parity-evidence-ledger.md`) at that head; the ledger is
generated evidence, not a coordinator-reserved closeout surface.

A row whose paired snapshots diverge on real semantics is recorded (diff,
both snapshots, exact attributes) with no receipt and no `mounted` cell,
exactly as `g16.111` did for Select. If the cause is a missing value that
`poodle-render` already has the input for, the one-line projection fix is in
scope; anything else becomes a bounded repair card (`g16.117` shape).

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Contract decides | a repair that contradicts line 268 or 302 | reviewer rejects |
| One tab stop | Tab from the trigger lands on an option or chevron | Svelte test and A1 focus order |
| Paired receipt is empty-diff | any remaining entry | receipt validation fails |
| Searchable unchanged | searchable Select snapshot | identical to before |
| React parity | React Select still exposes focusable options | `docs:react-prop-drift` and React test |

## Validation

Svelte and React Select tests, `cargo test -p poodle-render`,
`effigy regressions:native` (A1 Select run), `effigy test:a11y`,
`effigy check:parity-evidence-ledger`, `effigy docs:check`, `git diff --check
origin/main...HEAD`.

## Owned Paths

`docs/contracts/components/select.md`, `packages/svelte/components/src/Select.svelte`
and tests, `packages/react/components/src/Select.tsx` and tests,
`packages/render/src/select.rs` and tests, `test/nucleus-a11y/scenarios/select.json`,
the Select A1 receipt and divergence store, execution log, `PAPERCUTS.md`
(append only).

## Stop Conditions

Stop if the searchable path needs a change, or if the chevron decision
would alter pointer behaviour. Escalation owner: Chatterbox.
