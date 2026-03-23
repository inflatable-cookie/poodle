# g01 Foundation, Tokens, Contracts, And First Shared Surface

Status: completed
Updated: 2026-03-11

## Context

`g01` is the generation that turns Flint from a repo concept into a real shared
UI-system program. It freezes the ownership model, creates the token and
contract baseline, builds the first cross-framework primitives, defines the
workstation-shell layer, and proves the first Underlay bridge.

## Starting State

- the repo exists but has no implementation packages
- vision and architecture are new and intentionally lean
- no canonical token schema exists yet
- no Svelte or GPUI component contract has been frozen yet
- Underlay and downstream app integration rules are still conceptual

## Exit State

- the package boundary model is explicit
- the token system is defined and emits consumer-facing artifacts
- the component contract template exists and governs the first shared surface
- Svelte and GPUI implementation substrates are explicit
- the first primitive suite and first workstation-shell composites are planned
  and bounded
- Underlay bridge and downstream extension rules are explicit enough to start
  adoption work in `g02`

## Milestone Status

| ID | Milestone | Depends On | Class | Status |
|----|-----------|------------|-------|--------|
| 001 | Northstar bootstrap and scope freeze | - | Foundation | Completed |
| 002 | Canonical token schema and naming system | 001 | Foundation | Completed |
| 003 | Token artifact emission, themes, and density modes | 002 | Foundation | Completed |
| 004 | Component contract template and documentation IA | 001, 002 | Foundation | Completed |
| 005 | Svelte substrate and Bits integration policy | 002, 004 | Foundation | Completed |
| 006 | GPUI substrate and Rust token binding baseline | 002, 004 | Foundation | Completed |
| 007 | Layout, surface, and scrolling primitives | 003-006 | Core build | Completed |
| 008 | Action and text-entry primitives | 003-006 | Core build | Completed |
| 009 | Selection, value, and feedback primitives | 003-006 | Core build | Completed |
| 010 | Overlay, navigation, and interaction primitives | 003-006 | Core build | Completed |
| 011 | Product composites and information architecture baseline | 007-010 | Depth | Completed |
| 012 | Workstation-shell composites and panel system baseline | 007-010 | Depth | Completed |
| 013 | Underlay bridge and token-ingestion baseline | 003, 011, 012 | Adoption | Completed |
| 014 | Parity evidence, downstream extension contract, and `g02` cutover | 005-013 | Closure | Completed |

## Dependency Shape

```text
001 Bootstrap
  -> 002 Token Schema
      -> 003 Token Artifacts/Themes/Density
  -> 004 Contract Template/Docs IA
      -> 005 Svelte Substrate
      -> 006 GPUI Substrate
          -> 007 Layout/Surface Primitives
          -> 008 Action/Text-Entry Primitives
          -> 009 Selection/Value/Feedback Primitives
          -> 010 Overlay/Navigation/Interaction Primitives
              -> 011 Product Composites
              -> 012 Workstation Shell
                  -> 013 Underlay Bridge
                      -> 014 Parity/Extension/Cutover
```

## Execution Lanes

### Lane A: Contract And Token Core

`001 -> 002 -> 003 -> 004`

### Lane B: Implementation Substrates

`004 -> 005 -> 006`

### Lane C: Primitive Surface

`007 -> 008 -> 009 -> 010`

### Lane D: First Real Consumers

`011 -> 012 -> 013 -> 014`

## Next Task

`g01` is complete. Start `g02.001` next so the now-stable baseline can expand
into forms and validation system depth without reopening foundation questions.
