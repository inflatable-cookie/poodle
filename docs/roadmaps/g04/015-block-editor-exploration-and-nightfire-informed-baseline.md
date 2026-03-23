# g04.015 Block Editor Exploration And Nightfire-Informed Baseline

Status: planned
Owner: Flint Core
Updated: 2026-03-14
Depends on: g04.013
Primary repos: `flint`

## Goals

- [ ] evaluate the Nightfire block editor architecture for portability to Flint
- [ ] define the scope and contract for a Flint block editor component
- [ ] implement a minimal block editor baseline if feasible within this
  generation

## Execution Checklist

- [ ] audit Nightfire block system: block types, toolbar, drag reorder, nested
  blocks, serialization format
- [ ] identify Nightfire dependencies that are Underlay-specific vs portable
- [ ] determine whether Flint block editor should wrap an existing engine
  (ProseMirror, Tiptap, BlockNote) or adapt Nightfire directly
- [ ] write a scope document defining which block types are in-scope for v1:
  paragraph, heading, image, code, quote, list, divider
- [ ] if feasible: implement BlockEditor composite with basic block type support
- [ ] if not feasible: document findings and defer full implementation to g05
- [ ] create specimen if implementation lands
- [ ] register in component-registry.ts and specimen registry if implemented

## Acceptance Criteria

- [ ] Nightfire architecture audit is documented with portability assessment
- [ ] block editor scope document defines v1 block types and serialization
  format
- [ ] decision is recorded: implement in g04 vs defer to g05
- [ ] if implemented: BlockEditor renders blocks, supports add/remove/reorder,
  and serializes to a documented format
- [ ] if deferred: g05 scope includes explicit block editor milestones

## Next Task

Open `g04.016` and harden existing component feature gaps.
