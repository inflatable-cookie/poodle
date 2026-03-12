# SelectionSummary

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `SelectionSummary`
- Layer: `composites`
- Summary: a compact summary of the current selected scope with optional removal and clear-selection affordances
- In scope: selected item count, selected chips/tags, clear action, overflow count
- Out of scope: candidate browsing, confirm/cancel workflow, pagination-aware selection semantics

## 2. Accessibility

- summary text must stay textual and explicit
- chip removal actions need accessible names tied to the selected item
- clear-selection remains distinct from confirm/cancel semantics
- GPUI-native accessibility mapping notes: GPUI must preserve selection summary and per-item removal meaning rather than rendering them as decorative tags

## 3. Next Task

Use `SelectionSummary` inside `RelationPicker`, picker shells, and future selection-heavy workflows instead of rebuilding selected-chip summaries ad hoc.
