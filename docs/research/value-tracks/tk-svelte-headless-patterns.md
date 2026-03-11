# Value Track: Svelte Headless Patterns

Status: planned
Created: 2026-03-11
Updated: 2026-03-11
Priority: high (informs g01.005)

## Purpose

Research Svelte headless UI patterns and component library architectures to inform:
- Svelte substrate policy (g01.005)
- Bits integration approach
- Component composition patterns
- State management in Svelte components

---

## Research Questions

### Headless Patterns

- [ ] What is the "headless UI" pattern in Svelte?
- [ ] How do builders/actions work (Melt UI pattern)?
- [ ] What are the tradeoffs between component-based vs. builder-based APIs?
- [ ] How does composition work in Svelte headless libraries?

### State Management

- [ ] How is component state managed in Svelte headless libraries?
- [ ] What patterns exist for sharing state between components?
- [ ] How is controlled vs. uncontrolled component state handled?

### Styling Integration

- [ ] How do headless components expose styling hooks?
- [ ] What is the relationship between headless primitives and design tokens?
- [ ] How are CSS custom properties integrated?

### Precedents

- [ ] How does Melt UI structure its component builders?
- [ ] How does Bits Svelte (public) handle composition?
- [ ] What can be learned from React headless libraries (Radix, Headless UI)?

---

## Sources

### Svelte Headless Libraries

| Library | URL | Pattern | Notes |
|---------|-----|---------|-------|
| Melt UI | https://melt-ui.com/ | Builder/actions | Modern Svelte headless |
| Bits UI (public) | https://bits-ui.com/ | Components | shadcn's Svelte port |
| Headless UI (React/Vue) | https://headlessui.com/ | Components | Tailwind Labs |
| Radix Primitives | https://www.radix-ui.com/primitives | Components | React, influential |

### State Management

| Source | URL | Notes |
|--------|-----|-------|
| Svelte Runes | https://svelte.dev/docs/runes | Svelte 5 state model |
| Svelte Stores | https://svelte.dev/docs/svelte-store | Legacy but still used |
| Runes tutorial | https://svelte.dev/tutorial/runes | State patterns |

---

## Findings

*(To be filled when research begins)*

---

## Implications for Pug

*(To be synthesized)*

---

## Related

- Source hub: [hub-bits](../source-hubs/hub-bits.md)
- Milestone: [g01.005](../../roadmaps/g01/005-svelte-substrate-and-bits-integration-policy.md)

---

## Next Task

Research Melt UI builder patterns and compare to component-based approaches.
