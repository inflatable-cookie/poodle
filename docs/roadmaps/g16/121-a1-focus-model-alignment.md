# g16.121 — A1 Focus-Model Alignment (last four rows)

Status: ready
Type: A1 evidence repair — Svelte extractor law, one scenario, one Svelte
roving-focus fix; four receipts to reach 29/29
Opened: 2026-09-05
Depends on: merged `g16.119` (`cceb6646a`)
Governing refs: `../../contracts/components/{radio-group,segmented-control,menu,agent-transcript}.md`,
`test/nucleus-a11y/` (Svelte extractor and scenarios), `111` snapshot law,
`nucleus-parity-receipts/a1-divergences/{radio-group,segmented-control,menu,agent-transcript}/`,
`docs/logs/2026-09/20260905-g16-119-a1-focus-and-state-semantics.md`
Dispatch manifest: `../dispatch.md`

## What remains, and why it is not a GPUI defect

| Row | Remaining diff | Contract | Ruling |
| --- | --- | --- | --- |
| RadioGroup | Svelte reports two `focus_order` stops; GPUI one | roving focus via the native radio group: Tab reaches one radio | the Svelte extractor over-counts native radios (every `tabindex="0"` input); the browser exposes one stop. Extractor law: a native radio group is one sequential stop, on the checked or first enabled radio |
| SegmentedControl | same | same (hidden native radios) | same extractor law |
| AgentTranscript | Svelte `focused: true` after the action; GPUI `null` | contract: no focus change on append | the scenario's action is a pointer click, which focuses the clicked element in a browser. The scenario must exercise append the way the contract means it (programmatic append, no pointer), on both sides |
| Menu | Svelte menu item is a sequential stop (`focus_order` 3); GPUI none | items are reached by arrows from one roving stop | Svelte defect: `menuitem` buttons need `tabindex="-1"` with roving focus, the same repair `g16.117` made for Select options |

## Fixed Boundary

- Extractor: implement the native-radio-group focus law in the Svelte
  extractor (`test/nucleus-a11y`), with a unit test on a two-radio fixture
  proving one stop. Do not special-case component names; key on
  `input[type=radio][name]` groups.
- Scenario: change `agent-transcript.json` to a programmatic append action
  consumed by both extractors; the GPUI side already dispatches appends
  without focus.
- Svelte Menu: `tabindex="-1"` on menu items with the existing roving
  highlight; keyboard behaviour per contract unchanged; React mirrors it;
  `docs:react-prop-drift` stays green.
- Proof: four empty-diff A1 receipts; divergence stores deleted; cohort
  repin and re-emit; ledger regenerated to 29/29 on the GPUI accessibility
  column. Then delete the divergence store README's consumed sections.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Extractor law matches the browser | a checked radio that is not the reported stop | unit test with `document.activeElement` after a real Tab in happy-dom or a documented browser rule |
| No focus on append | the transcript scenario still clicks | scenario has no pointer action |
| Menu is one stop | Tab from the trigger lands on an item | Svelte test fails |
| 29/29 | any row without a receipt | ledger check |

## Validation

`test/nucleus-a11y` project, Svelte and React Menu tests, `effigy
regressions:native`, `effigy check:parity-evidence-ledger`, `effigy
docs:check`, `git diff --check origin/main...HEAD`.

## Owned Paths

`test/nucleus-a11y/**` (extractor law, unit test, transcript scenario),
`packages/svelte/components/src/Menu.svelte`, `packages/react/components/src/Menu.tsx`
and their tests, the four rows' receipts and stores, manifest `resolution`,
ledger, execution log, `PAPERCUTS.md` (append).

## Stop Conditions

Stop if the extractor law would change any already-receipted row's snapshot
(re-run the cohort and report), or if the Menu change alters keyboard
behaviour the contract names. Escalation owner: Chatterbox.
