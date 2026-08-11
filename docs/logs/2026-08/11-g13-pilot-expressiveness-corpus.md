---
title: g13 batch 005 — pilot contract expressiveness corpus
status: complete
owner: Poodle core
updated: 2026-08-11
tags: [log, g13, pilot, IR, corpus, button, range-slider, text-input]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/005-pilot-contract-expressiveness-corpus.md`
on branch `thread/g13-pilot-expressiveness-corpus`: extracted the semantic
vocabulary the g13.002 pilot IR must express from the Button, RangeSlider, and
TextInput contracts (semantic authority) plus current Svelte, React,
core-machine/style, Rust spec/headless/render, GPUI, Jetstream, specimen,
interaction, accessibility, recipe, and visual evidence.

## Deliverables (only the scoped writes)

- `docs/roadmaps/g13/pilot-expressiveness-corpus.md` — the corpus
- `docs/logs/2026-08/11-g13-pilot-expressiveness-corpus.md` — this log
- `PAPERCUTS.md` — one new entry (React ports omit contract-listed web-native
  props)

No contracts, specs, architecture, roadmaps, dispatch state, implementation,
tests, registries, generated artifacts, or manifests were touched. No IR
schema, representation, or crate recommendation appears in the corpus.

## Method

- Read the three component contracts end to end (§1–§14), spec
  `063-rust-authored-component-and-scene-ir.md`, and the current
  implementations and evidence for all four runtimes plus the four preview
  shells.
- Assigned stable IDs: `CROSS-*` (21), `BTN-*` (29), `RNG-*` (29), `TXT-*`
  (32), `SHELL-*` (10), `NEG-*` (8), plus registers `UNKNOWN-*` (2),
  `OBS-*` (6), `GAP-*` (7).
- Classified every requirement as `SDD` (shared declarative definition),
  `GTA` (generated target artifact), `AC` (adapter capability),
  `CV` (conformance vector), or `EXT` (candidate explicit runtime extension),
  without proposing representation.
- Every requirement cites its contract section and at least one current
  evidence path; missing evidence is named in the `GAP-*` register instead of
  inferred.
- Negative cases (`NEG-01`–`NEG-08`) directly exercise IR-03–IR-06.

## Requirement counts

| Set | Count |
|-----|-------|
| CROSS-* | 21 |
| BTN-* | 29 |
| RNG-* | 29 |
| TXT-* | 32 |
| SHELL-* | 10 |
| NEG-* | 8 |
| Total | 129 |
| UNKNOWN-* | 2 |
| OBS-* | 6 |
| GAP-* | 7 |

Classification totals: SDD 80, GTA 2, AC 22, CV 39, EXT 10 (dual-coded rows
count in both columns; NEG rows are IR-boundary cases). Counts verified by
grepping the corpus file for unique IDs after writing.

## Stop-condition check

No semantic contradiction was found; the contradiction register is empty. The
closest candidates are recorded as observations:

- `OBS-01` — R §9 Svelte note describes input `min`/`max` clamping the current
  implementations no longer use (machine clamps instead). Observable contract
  (invariant, per-thumb semantics, aria) holds.
- `OBS-02` — `data-state="active"` reflects pointer drag only; R §4 also lists
  keyboard adjustment.
- `OBS-03` — React Button lacks `formenctype`/`formmethod`; React TextInput
  lacks `autocorrect`. Recorded as evidence gap + papercut.

Open semantic questions (`UNKNOWN-01`, `UNKNOWN-02`): embedded-variant
`aria-orientation` scope in R §6; Rust spec enum superset
(`ButtonVariant::Danger`, `ButtonTone::Success`) beyond the B §3 union.

## Validation

| Command | Exit state |
|---------|-----------|
| `git diff --check` | 0 |
| `git status --porcelain` | 0 — only the three scoped files modified |
| requirement-ID count sweep (grep, 9 patterns) | 0 — counts match the corpus tables |

## Not done

Per batch card and worker rules: no merge, no roadmap/status/dispatch edits,
no schema or implementation work. The corpus becomes the acceptance input for
the g13.002 schema card on review.
