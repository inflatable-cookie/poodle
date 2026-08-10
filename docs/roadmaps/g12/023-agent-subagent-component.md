# 023 Agent Subagent Component

Status: completed
Owner: Tom
Created: 2026-08-10
Repo: this one; work in the worktree on the branch below
Branch: `thread/023-agent-subagent-component`

## Worker Rules

- You are an execution worker. Execute this card exactly — scope, steps,
  acceptance criteria, stop conditions. No planning authority.
- Do NOT spawn sub-agents or parallel research tasks; read sources directly.
- Do NOT touch roadmap/status files in any repo. Write only the deliverables
  in Scope plus the batch log.
- Where sources conflict or a needed surface is missing, STOP and report —
  never improvise. Poodle's own docs and existing components are
  authoritative over this card's assumptions.
- Commit on the branch above and push with
  `git push -u origin thread/023-agent-subagent-component`. Do not merge.

## Context

Nucleus renders provider sub-agent (child agent) work in its agent
transcript. The chosen product pattern (from
`/Users/tom/Dev/projects/nucleus/docs/research/source-hubs/harness-subagent-rendering.md`) is an
inline child-work group: header with identity + status, a live one-line
activity sub-line while running, expandable detail, and a click-through
action. No control affordances — the underlying model (Swallowtail contract
045) is observation-only.

The exact template to mirror is the `AgentPlan`/`AgentPlanRecord` pair,
which landed recently and demonstrates every file a new agent-surface
component touches. `AgentQuestion` is the older precedent.

## Governing Refs (read first)

- `AGENTS.md` (poodle root) — effigy tasks, validation tiers
- `docs/contracts/template/component-contract-template.md` and
  `docs/specs/002-component-contract-template-and-parity-rules.md` —
  contract-first order
- `docs/contracts/components/agent-plan.md` and `agent-question.md` — the
  16-section shape to follow
- `docs/contracts/components/agent-transcript.md` — for the new item kind
- Existing code to mirror: `packages/core/src/agent-plan.ts`,
  `packages/svelte/components/src/AgentPlan.svelte`,
  `packages/core/src/styles/agent-plan.css` (note: styles live in
  `packages/core/src/styles/` since the release restructure),
  `packages/svelte/preview/src/specimens/AgentPlanSpecimen.svelte`
- Swallowtail status vocabulary to mirror exactly:
  `unknown | pending | running | waiting | completed | failed | interrupted | shutdown`
  (`swallowtail-runtime/src/activity/subagent.rs`, `SubagentStatus`)

## Scope

A new component `AgentSubagent` (no record variant needed — the settled
state is the same component with a terminal status). All the files the
AgentPlan precedent touches, adapted to the current layout:

1. Contract doc `docs/contracts/components/agent-subagent.md` (16-section
   shape, status `draft`) + index entries in `docs/contracts/README.md` and
   `docs/contracts/components/README.md`
2. Headless core `packages/core/src/agent-subagent.ts`: types
   `AgentSubagentStatus` (the eight-value vocabulary above),
   `AgentSubagentItem` ({ id, label, status, activityLine?, summary? }),
   and pure helpers (status label/terminal mapping, e.g.
   `subagentStatusLabel`, `isTerminalSubagentStatus`); export from
   `packages/core/src/index.ts`
3. Conformance vectors `packages/contracts/headless/vectors/agent-subagent.json`
   + TS runner + Rust mirror (`packages/contracts/headless/src/agent_subagent.rs`,
   lib.rs mod, conformance test)
4. Spec crate `packages/contracts/components/src/agent_subagent.rs`
   (+ lib.rs) and renderer `packages/render/src/agent_subagent.rs` (+ lib.rs)
5. Svelte `packages/svelte/components/src/AgentSubagent.svelte` + exports
   (`index.ts`, `types.ts`)
6. Styles `packages/core/src/styles/agent-subagent.css`
7. Transcript: new `TranscriptItem` kind `"subagent-group"` in
   `packages/core/src/agent-transcript.ts` (+ Rust mirror + vectors),
   rendered by `AgentTranscript.svelte` via `AgentSubagent`
8. Specimen `packages/svelte/preview/src/specimens/AgentSubagentSpecimen.svelte`
   + registry entries (`specimens/registry.ts`, `component-registry.ts`,
   `parity.ts`) + `test/fixtures/component-props.ts`
9. Batch log `docs/logs/2026-08-10-agent-subagent-component.md`

## Component Semantics (pin these in the contract)

- Props: `item: AgentSubagentItem`; `expanded?: boolean` ($bindable,
  default false); `onToggle?(expanded)`; `onOpenChild?: () => void`
  (click-through affordance, label "Open child work"); `size`/`sizeRole`/
  `density` ladder matching AgentPlan.
- Header: child label + status badge. `running` shows a spinner (poodle
  `Spinner`, dots variant); terminal statuses show no spinner.
- `activityLine` renders only while non-terminal; `summary` only once
  terminal. Both are plain host-supplied strings.
- Expanded state reveals a detail region (a simple block list of the
  child's recent activity lines is enough for v1 — prop
  `detailLines?: string[]`).
- `unknown` status renders literally as "Unknown" — never inferred or
  prettified. No control affordances anywhere (no stop/cancel/steer).

## Acceptance Criteria

- Contract doc exists at draft status and drift gates pass
  (`effigy docs:contract-drift`, `effigy docs:spec-drift`)
- `effigy test:core`, `effigy test:components`, `effigy test:contracts`,
  `effigy test:parity` pass
- The specimen renders running/waiting/completed/failed/unknown states and
  the expanded detail, visible in the preview app
- Batch log records commands + exit states

## Stop Conditions

- The repo layout contradicts this card (files moved by the release
  restructure) → stop with the correct paths cited
- The transcript union change breaks existing vectors → stop with the
  failing vector names
- Scope pressure toward React/GPUI/Jetstream variants → stop; natives are
  out of scope (Known Delta row in the contract doc instead)
- PAPERCUTS.md friction goes in the batch log

## Closeout

Merged `bf5dc91f`. All acceptance gates re-run green on the branch before
merge (core 373, components 32 files, contracts 24 suites, parity, both
drift gates). Specimen verified visually in the preview app: all five status
states plus expanded detail render as designed. React/GPUI/Jetstream
variants remain a Known Delta in the contract doc, which stays `draft` until
nucleus integration proves the API.
