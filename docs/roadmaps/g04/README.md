# g04 Underlay Parity, Component Depth, And Workflow Coverage

Status: completed
Updated: 2026-03-14

## Context

`g04` begins after `g03` completed hardening, migration policy, parity
automation, first downstream adoption tranches, validation, and mature extension
support. The next priority is whether the Svelte component surface covers enough
real-world UI patterns to replace Underlay as the primary component library for
downstream applications. GPUI native parity work follows in `g05` once the
expanded Svelte component suite is established.

An exhaustive audit of the Underlay component library identified approximately
39 missing components, patterns, and feature gaps that Poodle must address before
downstream projects can migrate away from Underlay without regressions. This
generation organizes that work into sequenced milestones covering new component
families, depth extensions to existing components, and specialized editing and
media surfaces.

## Starting State

- `g03` hardening, adoption, accessibility, release, and onboarding baselines
  are complete
- Svelte primitives cover 64 components with full specimen coverage
- Svelte composites cover 21 components with full specimen coverage
- Underlay parity audit has identified ~39 gaps across dialog patterns, file
  handling, input depth, list interactions, media, editing, and operational
  display
- the preview app is restructured into a three-section catalogue with
  per-component pages

## Exit State

- all Underlay-parity components are implemented and documented
- existing component feature gaps are resolved
- downstream projects can adopt Poodle as a full Underlay replacement for UI
  components
- new component specimens are integrated into the preview catalogue
- the block editor baseline is evaluated and scoped for follow-on work
- GPUI counterparts for new components are scoped but not required in this
  generation

## Milestone Status

| ID | Milestone | Depends On | Class | Status |
|----|-----------|------------|-------|--------|
| 001 | Underlay parity audit, gap register, and implementation priority | g03.014 | Foundation | Completed |
| 002 | Dialog and confirmation patterns | 001 | Core build | Completed |
| 003 | File upload, media input, and embed patterns | 001 | Core build | Completed |
| 004 | Button and card pattern extensions | 001 | Core build | Completed |
| 005 | Input depth, text field, and specialized entry patterns | 001 | Core build | Completed |
| 006 | Temporal display and duration input | 001 | Core build | Completed |
| 007 | List interaction, reordering, and sort controls | 001 | Core build | Completed |
| 008 | Code display and color selection | 001 | Core build | Completed |
| 009 | Navigation cards and list card patterns | 001 | Core build | Completed |
| 010 | Loading, skeleton presets, and progress patterns | 001 | Depth | Completed |
| 011 | DataTable and Select depth | 002-010 | Depth | Completed |
| 012 | Operational display and monitoring patterns | 002-010 | Depth | Completed |
| 013 | Rich text and markdown editing | 001 | Specialist | Completed |
| 014 | Media playback components | 001 | Specialist | Completed |
| 015 | Block editor exploration and Nightfire-informed baseline | 013 | Specialist | Completed |
| 016 | Existing component feature gap hardening | 002-012 | Hardening | Completed |
| 017 | Preview specimen coverage and documentation | 002-016 | Docs | Completed |
| 018 | Generation closeout and `g05` cutover plan | 017 | Closure | Completed |

## Dependency Shape

```text
001 Underlay Parity Audit / Gap Register
  -> 002 Dialog / Confirmation Patterns
  -> 003 File Upload / Media Input / Embed
  -> 004 Button / Card Pattern Extensions
  -> 005 Input Depth / Text Field / Specialized Entry
  -> 006 Temporal Display / Duration Input
  -> 007 List Interaction / Reordering / Sort
  -> 008 Code Display / Color Selection
  -> 009 Navigation Cards / List Cards
      -> 010 Loading / Skeleton / Progress
      -> 011 DataTable / Select Depth
      -> 012 Operational Display / Monitoring
          -> 016 Feature Gap Hardening
              -> 017 Specimen Coverage / Docs
                  -> 018 Closeout / g05 Cutover
  -> 013 Rich Text / Markdown Editing
      -> 015 Block Editor Exploration
  -> 014 Media Playback
```

## Execution Lanes

### Lane A: Foundation

`001`

### Lane B: Core New Components

`002 -> 003 -> 004 -> 005 -> 006 -> 007 -> 008 -> 009`

### Lane C: Component Depth

`010 -> 011 -> 012`

### Lane D: Specialist Surfaces

`013 -> 015` and `014` (independent)

### Lane E: Hardening And Closeout

`016 -> 017 -> 018`

## Next Task

Open `g04.001` and formalize the Underlay parity gap register with explicit
implementation priority, contract requirements, and layer assignment for each
identified component.
