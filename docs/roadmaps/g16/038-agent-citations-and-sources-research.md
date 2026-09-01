# g16.038 — Agent Citations And Sources Research

Status: research-complete — PR #129; compose existing primitives, promotion
remains gated on a named consumer and stable source identity
Opened: 2026-09-01
Depends on: current AgentMessage and AgentTranscript contracts; independent of
`g16.034` and `g16.036`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/agent-message.md`,
`../../contracts/components/agent-transcript.md`
Intake: DesEngs candidate 2, merged in PR #126
Source leads: [Beautiful UI](https://beautifului.dev),
[AICSS Inline Citations](https://www.aicss.dev/r/inline-citations)

## Goal

Research a generalized composition for inline citation marks and a compact
source list around streamed or settled agent prose. Keep source identity,
titles, hrefs, and follow-up actions host-owned. Do not turn delivery cadence,
word reveal, or shimmer into citation semantics.

This card authorizes research only. It does not add Citation, source-list, or
follow-up-chip APIs.

## Questions

- Are marks and the source list one composite, two primitives, or authored
  AgentMessage content?
- How are stable source ids mapped when markdown streams, reparses, or settles?
- What is announced for a mark, repeated source, broken reference, or source
  list, and how does keyboard traversal avoid fragmenting message reading?
- Which link safety, truncation, disclosure, and unavailable-source rules are
  reusable rather than host policy?
- Can the same semantic surface project through shared Rust and GPUI without
  exposing a markdown-parser implementation detail?

## Required Evidence

- Inspect Beautiful UI and AICSS citation examples from durable primary or
  pinned sources and record licensing limits.
- Audit AgentMessage parsing, streaming replacement, TextLink, focus, copy,
  selection, and live-region ownership.
- Compare at least two consumer source-record shapes without importing either
  vocabulary into Poodle.
- Include dense, repeated, missing, unsafe-link, narrow-width, reduced-motion,
  and assistive-technology cases.

## Deliverable And Promotion Gate

Write `docs/research/value-tracks/agent-citations-and-sources.md` with an
extend/add/compose/reject recommendation, candidate semantic data shape, active-
cohort feasibility, and smallest proof. Promotion requires operator acceptance
of identity, placement, and accessibility ownership.

## Writable Scope

The dossier only, plus `PAPERCUTS.md` for new execution friction. Do not edit
contracts, source, packages, roadmaps, triage, or consumers.

## Validation

Run `effigy docs:lint` and `git diff --check`.
