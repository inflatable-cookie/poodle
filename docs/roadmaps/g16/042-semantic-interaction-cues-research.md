# g16.042 — Semantic Interaction Cues Research

Status: planned — research starts after accepted and merged `g16.034`
Opened: 2026-09-01
Depends on: accepted and merged `g16.034` as policy/lifecycle evidence; current
visual feedback and toast ownership
Governing refs: `../../architecture/012-semantic-motion-policy.md`,
`../../contracts/001-working-rules.md`
Intake: DesEngs candidate 6, merged in PR #126
Source leads: [Cuelume](https://cuelume.dev),
[@web-kits/audio](https://audio.raphaelsalaja.com/)

## Goal

Research whether Poodle should own optional semantic interaction cue roles
across web and native hosts. Sound, haptics, policy names, synthesis, unlock,
volume, and sample delivery remain open questions.

This card authorizes research only. `CuePolicy`, `full | muted | silent`, and a
closed role list are hypotheses, not public API decisions.

## Questions

- Which cue meanings are generalized and useful without audio being required
  for correctness?
- Should policy govern availability, user preference, host capability, or all
  three, and is restriction-only inheritance appropriate?
- Who owns first-gesture unlock, permission, volume, routing, concurrency,
  teardown, background behavior, and assistive-technology interaction?
- Can synthesized web cues and native host cues share meaning without sharing
  waveforms?
- Are haptics part of the same semantic family or a separate later capability?

## Required Evidence

- Inspect Cuelume and its audio engine from durable primary or pinned sources;
  record licensing and dependency boundaries.
- Survey platform accessibility guidance and browser/native capability limits
  from authoritative sources.
- Audit candidate Poodle states and at least two real host policies; reject
  product-specific jingles and sample libraries.
- Cover muted environments, missing devices, repeated cues, overlap, failure,
  reduced sensory preference, capture/testing, and deterministic evidence.

## Deliverable And Promotion Gate

Write `docs/research/value-tracks/semantic-interaction-cues.md` with an
architecture/recipe/consumer-owned/reject recommendation. Promotion requires
operator decisions on policy, roles, runtime ownership, accessibility, and
evidence; no sound implementation is implied.

## Writable Scope

The dossier only, plus `PAPERCUTS.md` for new execution friction. Do not edit
architecture, contracts, source, packages, roadmaps, triage, or consumers.

## Validation

Run `effigy docs:lint` and `git diff --check`.
