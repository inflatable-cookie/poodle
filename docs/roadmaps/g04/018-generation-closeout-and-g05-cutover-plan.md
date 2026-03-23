# g04.018 Generation Closeout And `g05` Cutover Plan

Status: planned
Owner: Poodle Core
Updated: 2026-03-14
Depends on: g04.017
Primary repos: `poodle`

## Goals

- [ ] verify all g04 milestones are complete or explicitly deferred
- [ ] update generation status and documentation
- [ ] confirm g05 scope accounts for expanded component surface

## Execution Checklist

- [ ] audit all g04 milestones for completion status
- [ ] record any deferred items with justification and g05 target
- [ ] update g04 README.md with final milestone status table
- [ ] update generation-index.md to mark g04 as completed
- [ ] confirm g05 GPUI parity scope covers:
  - GPUI counterparts for all new g04 components
  - block editor full implementation (if deferred from g04.015)
  - downstream migration support and tooling
  - any remaining Underlay parity gaps
- [ ] update roadmaps README.md to reflect g05 as active generation

## Acceptance Criteria

- [ ] all g04 milestones are marked completed or explicitly deferred
- [ ] g04 README.md reflects final status
- [ ] generation-index.md is updated
- [ ] g05 scope is confirmed with enough detail to begin execution
- [ ] no orphaned work items remain untracked

## Next Task

Begin `g05` execution based on the confirmed scope, starting with GPUI parity
for g04-added components.
