# Research

Status: active
Updated: 2026-03-11

Research is enabled because Flint will need comparative work across GPUI, Bits, Underlay, and downstream app needs before freezing implementation details.

## Quick Start

1. Read the [master-index.md](./master-index.md) for the research program overview
2. Check active [source-hubs/](./source-hubs/) for external system intelligence
3. Check active [value-tracks/](./value-tracks/) for domain research
4. See [research-to-architecture-crossref.md](./research-to-architecture-crossref.md) for how research feeds into decisions

## Directory Structure

```
docs/research/
├── README.md                              # This file
├── master-index.md                        # Research program overview and priorities
├── research-to-architecture-crossref.md   # Links research to architecture updates
├── research-to-implementation-playbook.md # When and how to use research
├── source-hubs/                           # External system documentation
│   ├── hub-gpui.md                       # GPUI capabilities and constraints
│   ├── hub-bits.md                       # Bits Svelte integration
│   └── hub-underlay.md                   # Underlay integration patterns
├── value-tracks/                          # Domain research topics
│   ├── tk-design-token-systems.md        # Token taxonomy patterns
│   ├── tk-cross-framework-contracts.md   # Component contract patterns
│   ├── tk-svelte-headless-patterns.md    # Svelte component patterns
│   ├── tk-gpui-idioms.md                 # GPUI-specific patterns
│   ├── tk-workstation-ui-patterns.md     # DAW/pro-tool UI patterns
│   └── ...                               # Additional tracks as needed
├── translation-memos/                     # Synthesis documents
├── specimen-dossiers/                     # Specific case studies
└── templates/                             # Reusable document templates
```

## Core Files

| File | Purpose |
|------|---------|
| [master-index.md](./master-index.md) | Research program overview, priorities, and status |
| [research-to-architecture-crossref.md](./research-to-architecture-crossref.md) | Links research to architecture decisions |
| [research-to-implementation-playbook.md](./research-to-implementation-playbook.md) | How and when to use research |

## Working Rules

1. **Use source hubs** to document external systems (GPUI, Bits, Underlay)
2. **Use value tracks** to research domain patterns and precedents
3. **Use translation memos** to synthesize research into architecture decisions
4. **Use specimen dossiers** to document specific case studies
5. **Promote findings** into architecture only after synthesis
6. **Don't implement** until research gates are met (see playbook)

## Immediate Priorities

Research that blocks current milestones:

1. **[hub-gpui](./source-hubs/hub-gpui.md)** - GPUI token system and capabilities (blocks g01.002, g01.006)
2. **[tk-design-token-systems](./value-tracks/tk-design-token-systems.md)** - Token taxonomy patterns (blocks g01.002)
3. **[hub-bits](./source-hubs/hub-bits.md)** - Bits extension limits (blocks g01.005)
4. **[tk-cross-framework-contracts](./value-tracks/tk-cross-framework-contracts.md)** - Contract patterns (blocks g01.004)

## Next Task

Execute research on the four immediate priority items to unblock g01 token and contract work.
