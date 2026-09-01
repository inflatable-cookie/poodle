# g16.037 — Transcript-Inline Agent Approval Research

Status: research-complete — PR #130; reject a new generic approval surface,
compose existing AgentQuestion and AgentPlan
Opened: 2026-09-01
Depends on: current AgentQuestion, AgentPlan, AgentTranscript, and
ConfirmAction contracts; independent of `g16.034` and `g16.036`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/agent-question.md`,
`../../contracts/components/agent-plan.md`,
`../../contracts/components/agent-transcript.md`
Intake: DesEngs candidate 1, merged in PR #126
Source leads: [Beautiful UI](https://beautifului.dev),
[AICSS Approval Card](https://www.aicss.dev/components/approval-card),
[Fluid Functionalism AskUserQuestions](https://www.fluidfunctionalism.com/docs/ask-user-questions)

## Goal

Determine whether Poodle needs a transcript-inline, no-dialog approval surface
for an agent command, short plan, or bounded recommendation. Preserve the
composer-owned AgentQuestion and AgentPlan paths and the settled transcript
records unless evidence shows an additive placement is coherent.

This card authorizes research only. It does not add a transcript item kind,
approval API, recommendation schema, or second text input.

## Questions

- Is this one reusable approval semantic or several product workflows?
- Can an existing AgentQuestion or AgentPlan model be placed inline without
  weakening focus, live-region, blocking, or settled-record ownership?
- What distinguishes turn blocking from action blocking?
- Which command, plan, confidence, alternative, refusal, expiry, and settled
  states are genuinely shared across products?
- How do Svelte, React, shared Rust, and GPUI preserve the same keyboard and
  announcement result without a scrim or focus trap?

## Required Evidence

- Inspect Beautiful UI, AICSS, and Fluid Functionalism from durable primary or
  pinned sources; record licensing and mutable-source limits.
- Compare the live Poodle contracts and at least two real consumer workflows.
- Trace focus entry/return, repeated activation, replacement, cancellation,
  disabled actions, and transition from live decision to settled record.
- Separate reusable semantics from agent-specific copy and domain payloads.

## Deliverable And Promotion Gate

Write `docs/research/value-tracks/transcript-inline-agent-approval.md` with a
recommendation of extend, add, compose, or reject. Name unresolved operator
decisions and the smallest cross-runtime proof. No implementation card is
created until the operator accepts the placement, blocking model, and public
semantic boundary.

## Writable Scope

The dossier only, plus `PAPERCUTS.md` for new execution friction. Do not edit
contracts, source, specimens, packages, roadmaps, triage, or consumers.

## Validation

Run `effigy docs:lint` and `git diff --check`.
