# g16.099 — React Prop Port Tranche

Status: complete — merged in PR #204 at `660b9510d`; all five pending-port baseline entries cleared
Type: React parity port — no Svelte or contract change
Opened: 2026-09-04
Depends on: merged `g16.095` (satisfied at `f297774f4`)
Governing refs: `../../contracts/001-working-rules.md` (Svelte is the
reference; a capability in Svelte and absent elsewhere is a gap to port),
`095-react-prop-drift-gate.md`, the `g16.095` execution log's grouped
findings
Dispatch manifest: `../dispatch.md`

## Goal

Clear every `pending-port` entry from the React drift baseline by porting the
Svelte props the React shells lack, so the ratchet shrinks to
`framework-idiom` and `needs-decision` entries only.

## Fixed Boundary

- Port, with matching defaults and documented behaviour: `Button`
  `formenctype`, `formmethod`, `style`; `Calendar` `today`; `SplitView`
  `divider`; `AppHeader` `element` (React form: a forwarded ref or
  `elementRef` callback, documented in the contract's runtime notes);
  `DockRegion` `showCollapseToggle`. Take the exact list from the seeded
  baseline at merge time; if `g16.095`'s revision changes it, the baseline
  is authoritative.
- Delete each cleared `pending-port` entry in the same PR; the gate must pass
  with the entry removed and fail with the port reverted.
- Do not touch `framework-idiom` entries. Do not decide `needs-decision`
  entries; leave them and their reasons intact.
- Svelte-inclusion candidates (`Tree.onEditingChange`,
  `OrderBy.onActiveSortChange`) are out of scope; they return to Chatterbox.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Port is real | revert one ported prop | `docs:react-prop-drift` exits 1 naming it |
| Baseline shrank | a cleared `pending-port` entry remains | gate refuses a baselined prop that no longer drifts |
| Defaults match | React default differs from Svelte | default-drift finding |
| Svelte untouched | any diff under `packages/svelte/` | lane is red |

## Validation

`effigy docs:react-prop-drift`, focused React component tests for each ported
prop, `effigy ci:web`, `effigy docs:check`, `git diff --check
origin/main...HEAD`.

## Owned Paths

`packages/react/components/src/{Button,Calendar,SplitView,AppHeader,DockRegion}.tsx`
and their tests, the `BASELINE` register in
`packages/svelte/preview/scripts/react-prop-drift.ts` (removals only),
contract runtime-notes lines for `AppHeader` `element` if the React form
needs one sentence, this card's execution log, root `PAPERCUTS.md` (append
only).

## Stop Conditions

Stop when a port needs a contract change beyond a runtime note, or when a
`pending-port` entry turns out to be a `needs-decision`. Escalation owner:
Chatterbox.
