# Research To Implementation Playbook

Status: active
Updated: 2026-03-11

## Default Flow

1. **Create a source hub** for external systems (GPUI, Bits, Underlay)
2. **Research value tracks** for domain patterns and precedents
3. **Record findings** in source hubs and value tracks
4. **Synthesize** into translation memos when decisions need to be made
5. **Update architecture** with research-backed decisions
6. **Implement** only after decisions are explicit in architecture

## Research Triggers

Start research when:
- An external system's capabilities are unknown but needed for a decision
- Multiple valid approaches exist and tradeoffs need documentation
- A decision has long-term consequences that are hard to reverse
- Cross-platform parity requirements need clarification

## Translation Memo Triggers

Create a translation memo when:
- Research findings need to become architecture decisions
- Multiple stakeholders need to review/approve an approach
- A decision affects multiple milestones or component areas
- The decision is non-obvious or has significant tradeoffs

## Implementation Gates

Do not implement until:
- [ ] Relevant research is documented
- [ ] Architecture is updated (if affected)
- [ ] Translation memo exists (for significant decisions)
- [ ] Roadmap acceptance criteria can be met

## Research Priority by Milestone

### g01.002 - Token Schema

**Must research first:**
- [hub-gpui](./source-hubs/hub-gpui.md) - GPUI token consumption
- [tk-design-token-systems](./value-tracks/tk-design-token-systems.md) - Token taxonomy patterns

### g01.003 - Token Artifacts

**Must research first:**
- [tk-design-token-systems](./value-tracks/tk-design-token-systems.md) - Emission tools and formats

### g01.004 - Contract Template

**Must research first:**
- [tk-cross-framework-contracts](./value-tracks/tk-cross-framework-contracts.md) - Contract patterns

### g01.005 - Svelte Substrate

**Must research first:**
- [hub-bits](./source-hubs/hub-bits.md) - Bits coverage and limits
- [tk-svelte-headless-patterns](./value-tracks/tk-svelte-headless-patterns.md) - Svelte patterns

### g01.006 - GPUI Substrate

**Must research first:**
- [hub-gpui](./source-hubs/hub-gpui.md) - GPUI capabilities
- [tk-gpui-idioms](./value-tracks/tk-gpui-idioms.md) - GPUI patterns

### g01.007-012 - Primitives & Shell

**Should research:**
- [tk-workstation-ui-patterns](./value-tracks/tk-workstation-ui-patterns.md) - Panel/dock patterns
- [tk-accessibility-cross-platform](./value-tracks/tk-accessibility-cross-platform.md) - ARIA/keyboard
- [tk-overlay-positioning](./value-tracks/tk-overlay-positioning.md) - Overlay behavior

### g01.013 - Underlay Bridge

**Must research first:**
- [hub-underlay](./source-hubs/hub-underlay.md) - Integration patterns

## Next Task

Begin research on immediate priority items: hub-gpui, tk-design-token-systems, hub-bits, tk-cross-framework-contracts.
