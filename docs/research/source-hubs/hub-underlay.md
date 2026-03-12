# Source Hub: Underlay

Status: planned
Created: 2026-03-11
Updated: 2026-03-11

## Purpose

Document Underlay's token/runtime system and integration constraints to inform:
- Underlay bridge design (g01.013)
- Token ingestion rules for Underlay
- Wrapper preservation strategy
- Zero-leak adoption goal

---

## Source Inventory

### Internal Sources

| Source | Location | Type | Access |
|--------|----------|------|--------|
| Underlay Token System | *(underlay repo)* | Implementation | Internal |
| Underlay Component System | *(underlay repo)* | Implementation | Internal |
| Underlay CSS/Runtime | *(underlay repo)* | Implementation | Internal |

---

## Key Questions

### Token Integration

- [ ] How does Underlay currently handle design tokens?
- [ ] What token format would Underlay consume from Pug?
- [ ] How would Underlay map Pug tokens to Underlay CSS/runtime?
- [ ] What namespace/ownership boundaries are needed?

### Component Integration

- [ ] How would Underlay wrap Pug components?
- [ ] What is the wrapper preservation rule?
- [ ] How does Underlay maintain its public API while using Pug internally?
- [ ] Which Underlay components could be Pug-backed vs. stay Underlay-owned?

### Adoption Path

- [ ] What is the migration strategy for Underlay?
- [ ] What are the breaking change risks?
- [ ] How can adoption be incremental?

---

## Findings

*(To be filled when research begins - not immediate priority)*

---

## Implications for Pug

*(To be synthesized when research begins)*

---

## Related

- Milestone: [g01.013](../../roadmaps/g01/013-underlay-bridge-and-token-ingestion-baseline.md)
- Milestone: `g02.013` now covers preview/docs usability hardening instead of adoption sequencing
- Milestone: [g03.007](../../roadmaps/g03/007-underlay-bridge-hardening-and-zero-leak-adoption-proof.md)

---

## Next Task

Defer until g01.002-g01.006 are underway; start research in parallel with g01.007-012.
