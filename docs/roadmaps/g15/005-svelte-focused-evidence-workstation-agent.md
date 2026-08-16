# g15.005 — Svelte Focused Evidence: Workstation Systems & Agent Surfaces

Status: **blocked** — pending orchestrator review of `g15.001`
Depends on: `g15.001`
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../contracts/001-working-rules.md`

## Outcome

Close the focused-evidence gap for the 24 workstation and agent-surface
components measured in `g15.001`. Each component gains focused, owner-local
test evidence that asserts contract behaviour — not an anatomy smoke case.
Agent surfaces are largely downstream-shared vocabulary; their focused tests
assert transcript semantics (roles, settled plans, tool calls, changed files)
without turning the component tests into a cross-runtime corpus.

## Scope

Workstation systems: StatusBar, StatusIndicator, Surface, Text, TextLink,
Table, TokenInput, TimeInput, TimeZoneSelect, ToggleGroup, Toolbar, Tooltip,
TriStateSwitch, UiPresentationProvider, VideoPlayer, DateTimeZonePicker

Agent surfaces: AgentMessage, AgentPlanRecord, AgentQuestion,
AgentQuestionRecord, AgentSubagent, ChangedFiles, ToolCall, ToolCallGroup

Priority: downstream-used components first (StatusBar, StatusIndicator,
Surface, TextLink, Table, ToggleGroup, Toolbar, Tooltip,
UiPresentationProvider — see roster Downstream use column).

## Goals

- [ ] One focused test file (or named cases in a family test) per component,
      asserting contract semantics: transcript rendering, keyboard and focus
      behaviour, accessibility projection, token use.
- [ ] Evidence names exact files and cases; aggregate selectors do not count.
- [ ] No component API, runtime code, specimen, or contract changes to
      produce evidence.

## Acceptance

- [ ] Every scoped component has a named focused test case beyond the anatomy
      smoke.
- [ ] `effigy check:svelte`, `effigy test:components`, `effigy docs:check`
      pass.
- [ ] The register's row for each component flips to evidence-present.

## Stop Conditions

- A test asserts the same anatomy smoke asserts.
- Work expands beyond the scoped component list without a new card.
- A specimen or contract is changed to make a test pass.

## Writable Scope

- focused tests and bounded harness fixtures beside the components
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `effigy test:components` (narrow: the touched test files)
- `effigy check:svelte`
- `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
