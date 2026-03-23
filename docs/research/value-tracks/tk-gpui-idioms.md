# Value Track: GPUI Idioms

Status: planned
Created: 2026-03-11
Updated: 2026-03-11
Priority: high (informs g01.006)

## Purpose

Research GPUI-specific implementation patterns and idioms to inform:
- GPUI substrate policy (g01.006)
- How Poodle components translate to GPUI
- GPUI-native patterns that differ from web
- Performance considerations

---

## Research Questions

### Component Patterns

- [ ] What is the idiomatic GPUI component structure?
- [ ] How do `RenderOnce`, `Render`, and `Element` differ?
- [ ] What is the lifecycle of a GPUI component?
- [ ] How do parent-child relationships work?

### State & Reactivity

- [ ] How does state management work in GPUI?
- [ ] What is the `Model` pattern?
- [ ] How are subscriptions and updates handled?
- [ ] How does this compare to Svelte's reactivity?

### Styling & Appearance

- [ ] How is styling done in GPUI (shared styles, inline, etc.)?
- [ ] What is the `Theme` struct and how is it extended?
- [ ] How are colors, spacing, and typography handled?
- [ ] How does styling differ from CSS?

### Layout

- [ ] What layout primitives exist in GPUI?
- [ ] How does GPUI's layout model compare to CSS Flexbox?
- [ ] What are the sizing and constraint systems?
- [ ] How is responsive design handled?

### Events & Input

- [ ] How are mouse, keyboard, and focus events handled?
- [ ] What is the event flow and bubbling?
- [ ] How do actions/keybindings work?

---

## Sources

### GPUI Documentation

| Source | Location | Notes |
|--------|----------|-------|
| GPUI Docs | zed/crates/gpui/docs | Official documentation |
| GPUI Examples | zed/crates/gpui/examples | Code samples |
| Zed Source | zed/crates/zed | Real-world usage |

### Community Resources

| Source | URL | Notes |
|--------|-----|-------|
| GPUI Discord/Discussions | Zed community | Q&A and patterns |

---

## Findings

*(To be filled when research begins)*

---

## Implications for Poodle

*(To be synthesized)*

---

## Related

- Source hub: [hub-gpui](../source-hubs/hub-gpui.md)
- Milestone: [g01.006](../../roadmaps/g01/006-gpui-substrate-and-rust-token-binding-baseline.md)

---

## Next Task

Study GPUI examples and Zed source code to document component patterns.
