# g16.039 — Agent Task List Research

Status: research-ready
Opened: 2026-09-01
Depends on: current ToolCallGroup, AgentPlan, Progress, and ChangedFiles
contracts; independent of `g16.034` and `g16.036`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/tool-call-group.md`,
`../../contracts/components/agent-plan.md`
Intake: DesEngs candidate 3, merged in PR #126
Source leads: [Beautiful UI](https://beautifului.dev),
[AICSS To-do List](https://www.aicss.dev/r/task-list)

## Goal

Determine whether a live agent-maintained checklist is a reusable Poodle
composite, an extension of ToolCallGroup, or consumer-owned content. Preserve
the distinction between accepted plans, executed tool runs, progress values,
and a host-authored task list.

This card authorizes research only. It does not add AgentTaskList or extend
ToolCallGroup.

## Questions

- What semantic distinction survives across consumers between a plan step, a
  task, a tool call, and a progress row?
- Is a flat stable-id list sufficient, or is hierarchy a real generalized
  requirement?
- Which pending, in-progress, done, failed, skipped, blocked, count, and detail
  states are necessary rather than decorative?
- Who owns reorder, retry, expansion, selection, replacement, and settled
  history?
- How are changes announced without replaying the whole list?

## Required Evidence

- Inspect Beautiful UI and AICSS examples from durable primary or pinned
  sources and record licensing limits.
- Compare live Poodle contracts and at least two host task models.
- Test the proposed semantics against rapid status changes, insertion/removal,
  long labels, failure detail, empty lists, and restored history.
- Map active-cohort rendering and accessibility without product status names.

## Deliverable And Promotion Gate

Write `docs/research/value-tracks/agent-task-list.md` with an
extend/add/compose/reject recommendation and a bounded semantic model if one is
justified. Promotion requires operator acceptance of ownership relative to
AgentPlan and ToolCallGroup.

## Writable Scope

The dossier only, plus `PAPERCUTS.md` for new execution friction. Do not edit
contracts, source, packages, roadmaps, triage, or consumers.

## Validation

Run `effigy docs:lint` and `git diff --check`.
