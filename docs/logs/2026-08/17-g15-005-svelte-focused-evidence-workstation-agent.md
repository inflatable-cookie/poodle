# g15.005 — Svelte focused evidence: workstation systems & agent surfaces

Status: complete — all three batches landed (PR pending)
Date: 2026-08-17
Card: `docs/roadmaps/g15/005-svelte-focused-evidence-workstation-agent.md`
Governing refs: `docs/roadmaps/g15/release-baseline-roster.md`,
`docs/roadmaps/g15/release-gap-register.md`,
`docs/contracts/001-working-rules.md`

## Batches

The card's three named batches were executed in order. The roster and register
evidence rows were updated once all three were green under the full validation
round.

- **Batch A — typography, status & toolbar systems (8):** StatusBar,
  StatusIndicator, Surface, Text, TextLink, Toolbar, Tooltip, ToggleGroup
- **Batch B — workstation inputs, tables & pickers (8):** Table, TokenInput,
  TimeInput, TimeZoneSelect, TriStateSwitch, UiPresentationProvider,
  VideoPlayer, DateTimeZonePicker
- **Batch C — agent surfaces (8):** AgentMessage, AgentPlanRecord,
  AgentQuestion, AgentQuestionRecord, AgentSubagent, ChangedFiles, ToolCall,
  ToolCallGroup

## Evidence Landed

Every scoped component has a named focused test file on the Svelte side
(`packages/svelte/components/test/<Name>.test.ts`) and the mirrored contract
cases on the React side (`packages/react/components/test/<Name>.test.tsx`).
`AgentSubagent` already carried React evidence from an earlier tranche, so this
card closed 24 Svelte gaps and 23 React gaps. The anatomy smoke
(`smoke.test.ts`) is not reused as evidence; every case asserts observable
contract behaviour beyond mounting.

| Component | Svelte evidence | React evidence |
| --- | --- | --- |
| StatusBar | `StatusBar.test.ts` | `StatusBar.test.tsx` |
| StatusIndicator | `StatusIndicator.test.ts` | `StatusIndicator.test.tsx` |
| Surface | `Surface.test.ts` | `Surface.test.tsx` |
| Text | `Text.test.ts` | `Text.test.tsx` |
| TextLink | `TextLink.test.ts` | `TextLink.test.tsx` |
| Toolbar | `Toolbar.test.ts` | `Toolbar.test.tsx` |
| Tooltip | `Tooltip.test.ts` | `Tooltip.test.tsx` |
| ToggleGroup | `ToggleGroup.test.ts` | `ToggleGroup.test.tsx` |
| Table | `Table.test.ts` | `Table.test.tsx` |
| TokenInput | `TokenInput.test.ts` | `TokenInput.test.tsx` |
| TimeInput | `TimeInput.test.ts` | `TimeInput.test.tsx` |
| TimeZoneSelect | `TimeZoneSelect.test.ts` | `TimeZoneSelect.test.tsx` |
| TriStateSwitch | `TriStateSwitch.test.ts` | `TriStateSwitch.test.tsx` |
| UiPresentationProvider | `UiPresentationProvider.test.ts` | `UiPresentationProvider.test.tsx` |
| VideoPlayer | `VideoPlayer.test.ts` | `VideoPlayer.test.tsx` |
| DateTimeZonePicker | `DateTimeZonePicker.test.ts` | `DateTimeZonePicker.test.tsx` |
| AgentMessage | `AgentMessage.test.ts` | `AgentMessage.test.tsx` |
| AgentPlanRecord | `AgentPlanRecord.test.ts` | `AgentPlanRecord.test.tsx` |
| AgentQuestion | `AgentQuestion.test.ts` | `AgentQuestion.test.tsx` |
| AgentQuestionRecord | `AgentQuestionRecord.test.ts` | `AgentQuestionRecord.test.tsx` |
| AgentSubagent | `AgentSubagent.test.ts` | `AgentSubagent.test.tsx` (pre-existing) |
| ChangedFiles | `ChangedFiles.test.ts` | `ChangedFiles.test.tsx` |
| ToolCall | `ToolCall.test.ts` | `ToolCall.test.tsx` |
| ToolCallGroup | `ToolCallGroup.test.ts` | `ToolCallGroup.test.tsx` |

Five compiled Svelte harnesses back cases that need snippet or context
composition: `StatusBarSnippetsHarness.svelte`,
`StatusIndicatorSlotHarness.svelte`, `ToolbarRovingHarness.svelte`,
`TooltipHarness.svelte`, and `UiPresentationHarness.svelte`.

Behaviour asserted, by batch:

- **Batch A** — StatusBar footer landmark and the summary-then-`ariaLabel`
  name resolution, leading/trailing region gating; StatusIndicator tone
  projection with the dot hidden from assistive tech and the dot-only state;
  Surface tone/border/elevation and the accessibility-neutral default that
  only opts into `region` semantics with a label; Text semantic element
  selection and clamp/spacing projection with no injected ARIA; TextLink
  anchor-versus-button resolution, including the disabled case that must not
  leave a dead navigation target; Toolbar roving focus with arrow-key wrap and
  orientation gating; Tooltip controlled/uncontrolled open, `aria-describedby`
  lifetime, hover- and focus-driven delayed open, and Escape dismissal;
  ToggleGroup single/multiple role switch, value emission, and the
  `allowDeactivation` clearing rule.
- **Batch B** — Table native header scoping, row-header `th`, caption, empty
  row `colspan`, and accessible name placement; TokenInput commit/split/dedupe/
  remove/Backspace flows, hidden inputs, and `resolveToken` rejection;
  TimeInput controlled/uncontrolled value flow and null-on-clear; TimeZoneSelect
  combobox fallback list and pre-selected zone label; TriStateSwitch fixed
  three-radio order and controlled value; UiPresentationProvider CSS custom
  properties and semantic size propagation to descendants; VideoPlayer
  play/mute/seek/volume labelling, captions gating, and time formatting;
  DateTimeZonePicker dialog composition, Escape and outside-dismiss without
  value loss, and disabled inertness.
- **Batch C** — AgentMessage markdown block model (headings, lists,
  blockquotes, separators, code), streaming caret, empty-message silence, and
  link interception; AgentPlanRecord summary/plan exclusivity, budget
  truncation, and disclosure state; AgentQuestion radiogroup-versus-group
  semantics, single-select submit, override clearing, batch progress, and
  declined dismissal; AgentQuestionRecord per-outcome projection; AgentSubagent
  running/terminal/unknown status rendering and disclosure gating;
  ChangedFiles header totals, chip limit, tree expansion, and empty-list
  silence; ToolCall accessible-name composition and the non-interactive row
  when there is no output; ToolCallGroup collapsed-newest rule, ordered
  expansion, and failure-outranks-running status rollup.

## Bounded Fixes (contract-first)

Four observable defects surfaced, all fixed within the scoped components. Each
is covered by a case that fails when the fix is reverted (verified by
reverting all four source changes and re-running the paired suites: 11 cases
failed across both runtimes, then passed again once restored).

- **Table accessible name (both runtimes)** — `aria-label` was applied to the
  `.poodle-table-shell` `<div>`, a scroll container with no table semantics, so
  the name never reached the grid. Both runtimes now place it on `<table>`.
  The contract already carried the stronger requirement and recorded the Svelte
  placement as a known gap; `docs/contracts/components/table.md` §6 now states
  the rule and the reason instead of the stale gap note.
- **Tooltip forced-open surface (both runtimes)** — a tooltip shown through
  `open`/`defaultOpen` never ran the hover ENTER transition, so the default
  first-child anchor stayed unresolved and the hover machine stayed `closed`:
  nothing was announced through `aria-describedby` and Escape was inert. Both
  runtimes now resolve the default anchor once for a forced-open surface and
  seed the machine to `open`. No contract change — the contract already
  required first-child anchoring, `aria-describedby` while open, and Escape
  dismissal.
- **Select placeholder resolution (both runtimes)** — `hasSelection` compared
  the current value against the raw clear value, so a non-clearable select with
  a `defaultValue` reported itself as empty and showed the placeholder instead
  of the value. It now compares against the clearable-aware placeholder value.
  This is what the contract already describes (the placeholder-reset semantics
  are scoped to `clearable`, and the placeholder state means "no value
  selected"), so no contract change. `TimeZoneSelect`'s documented uncontrolled
  initial zone was the visible symptom.
- **React `UiPresentationProvider` (React only)** — the React provider set only
  the context and never rendered the contract-required root `<div>` carrying
  the four CSS custom properties, so descendant primitives lost every spacing
  and control-height override. It now renders the root div and computes the
  documented values. No contract change — the Svelte reference already matched
  the contract.

## Inherited-Work Repair

The three batch commits were authored in an earlier worker run that never
completed the full validation round. This run rebased them onto current
`origin/main` and repaired what the gates then caught:

- **`AgentMessage.test.tsx` fixture was inert** — the markdown fixture was
  passed as a JSX string attribute, where `\n` stays a literal backslash and
  `n`. The whole list/blockquote/separator fixture collapsed to one line and
  the case failed on the list count. Now passed as an expression literal.
- **`ToolCallGroup` fixtures were type-invalid (both runtimes)** — the
  `TranscriptToolCall` discriminant `kind: "tool-call"` was missing from all
  eight fixtures. `effigy check:svelte` failed on it; the React side was the
  same defect but no repo gate typechecks React test files (see PAPERCUTS).
  Both fixture sets now carry the discriminant, and the arrays are typed
  `TranscriptToolCall[]` so a future omission fails loudly.
- **Svelte `Tooltip` rerender was passing for the wrong reason** —
  `rerender({ props: { open: false } })` sets a prop literally named `props`;
  the assertion only passed because svelte-testing-library resets unlisted
  props to their defaults, which made the tooltip uncontrolled rather than
  controlled-closed. Corrected to the repo-wide `rerender({ open: false })`
  shape, so the case now proves what it claims.
- **`UiPresentationHarness.svelte` prop types were `string`** — widened past
  `ControlDensity`/`ControlSize`; `effigy check:svelte` failed on it. Now typed.
- **Tooltip pairing was asymmetric** — Svelte covered hover-driven open and
  React covered focus-driven open, so neither trigger was paired. Both suites
  now cover both, which is what the card's pairing requirement asks for.

## Observations (no change made)

- React synthesizes `pointerenter` from a bubbling `pointerover`, so the React
  hover case must dispatch `pointerOver`; the non-bubbling `pointerEnter` event
  never reaches the handler. The Svelte case dispatches `pointerEnter` directly
  on the root.
- Seven of the 24 components (StatusBar, StatusIndicator, Surface, Text, Table,
  UiPresentationProvider, AgentQuestionRecord) carry static-only evidence — no
  synthetic events. That is deliberate: their load-bearing contract behaviour
  is accessibility projection and composed-token output, both of which the card
  names as valid evidence. The assertions are behavioural (name resolution,
  scope placement, custom-property values, per-outcome projection), not
  anatomy.
- The four existing `effigy check:svelte` warnings and the `effigy doctor`
  findings are baseline board health and were left untouched.

## Validation

| Command | Result |
| --- | --- |
| `effigy test:components` | pass (335 files, 2598 tests) |
| `effigy check:svelte` | pass (880 files, 0 errors, 4 baseline warnings) |
| `effigy react:build` | pass (772 modules) |
| `effigy docs:check` | pass |
| `git diff --check origin/main...HEAD` | pass |
| Fix-coverage check (revert all four source fixes, re-run paired suites) | 11 cases failed as expected, then passed once restored |

No `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or any
Jetstream selector ran.

## Outcome

The roster reads 175/0 focused Svelte evidence and 175/0 focused React
evidence. No Svelte-denominator gap class and no React mirror gap class remains
open for the v0.2.0 release baseline.
