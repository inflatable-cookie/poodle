# Research Master Index

Status: active
Updated: 2026-03-11

## Research Program Overview

Flint requires comparative research across three external substrate systems (GPUI, Bits, Underlay) and multiple design domain precedents before freezing implementation details. This research program follows the Northstar structure:

- **[source-hubs/](./source-hubs/)** - External system intelligence (GPUI, Bits, Underlay)
- **[value-tracks/](./value-tracks/)** - Domain research topics with implementation implications
- **[specimen-dossiers/](./specimen-dossiers/)** - Specific case studies and examples
- **[translation-memos/](./translation-memos/)** - Synthesis documents that feed into architecture
- **[templates/](./templates/)** - Reusable research document templates

---

## Source Hubs

Source hubs document external systems that Flint builds upon or integrates with. Each hub tracks capabilities, constraints, and API surfaces.

| Hub | Purpose | Critical For | Status |
|-----|---------|--------------|--------|
| [hub-gpui](./source-hubs/hub-gpui.md) | GPUI component model, styling capabilities, element tree patterns | g01.006, g01.007-012, parity rules | 🟢 **complete** |
| [hub-bits](./source-hubs/hub-bits.md) | Bits Svelte primitives, extension limits, composition patterns | g01.005, Svelte substrate policy | 🟢 **complete** |
| [hub-underlay](./source-hubs/hub-underlay.md) | Underlay token/runtime integration, wrapper patterns | g01.013, g03.007 | 🟡 planned |

---

## Value Tracks

Value tracks research domain precedents and patterns. They answer "what are the options?" and "what should Flint adopt?"

### Phase 1: Foundation (blocks g01.002-g01.006)

| Track | Purpose | Critical For | Status |
|-------|---------|--------------|--------|
| [tk-design-token-systems](./value-tracks/tk-design-token-systems.md) | Token taxonomy patterns (DTCG, Style Dictionary, Theo) | g01.002, g01.003 | 🟢 **complete** |
| [tk-cross-framework-contracts](./value-tracks/tk-cross-framework-contracts.md) | Multi-platform component contracts | g01.004, contract template | 🟢 **complete** |
| [tk-svelte-headless-patterns](./value-tracks/tk-svelte-headless-patterns.md) | Svelte component library architectures | g01.005, Bits policy | 🟡 planned |
| [tk-gpui-idioms](./value-tracks/tk-gpui-idioms.md) | GPUI-specific implementation patterns | g01.006, GPUI substrate | 🟡 planned |

### Phase 2: Primitives & Composites (informs g01.007-g01.012)

| Track | Purpose | Critical For | Status |
|-------|---------|--------------|--------|
| [tk-workstation-ui-patterns](./value-tracks/tk-workstation-ui-patterns.md) | DAW/pro-tool: docks, panels, split views, persistence | g01.012, workstation shell | 🟢 **complete** |
| [tk-accessibility-cross-platform](./value-tracks/tk-accessibility-cross-platform.md) | ARIA, focus, keyboard across web/native | g01.007-010, g02.011 | 🟢 **complete** |
| [tk-overlay-positioning](./value-tracks/tk-overlay-positioning.md) | Popover, dialog, menu, tooltip layering | g01.010, overlay primitives | 🟢 **complete** |
| [tk-form-validation-patterns](./value-tracks/tk-form-validation-patterns.md) | Field states, validation, error handling | g01.008, g02.001 | 🟢 **complete** |

### Phase 3: Scale & Hardening (prepares g02/g03)

| Track | Purpose | Critical For | Status |
|-------|---------|--------------|--------|
| [tk-data-table-patterns](./value-tracks/tk-data-table-patterns.md) | Tables, virtualization, bulk actions | g02.002, g02.003 | 🟡 planned |
| [tk-command-palette-patterns](./value-tracks/tk-command-palette-patterns.md) | Action discovery, fuzzy search | g02.008, g01.012 | ✅ landed |
| [tk-parity-testing](./value-tracks/tk-parity-testing.md) | Cross-platform testing strategies | g03.002, g03.012 | 🟡 planned |
| [tk-docs-site-tooling](./value-tracks/tk-docs-site-tooling.md) | Component documentation platforms | g02.012, g03.003 | 🟡 planned |

---

## Translation Memos

Translation memos synthesize research into architecture decisions.

| Memo | Source Research | Target Architecture | Status |
|------|-----------------|---------------------|--------|
| [tm-token-system](./translation-memos/tm-token-system.md) | hub-gpui, tk-design-token-systems | Token system design | 🟢 ready for review |
| [tm-contract-template](./translation-memos/tm-contract-template.md) | tk-cross-framework-contracts | Component contracts | 🟢 ready for review |
| [tm-svelte-substrate](./translation-memos/tm-svelte-substrate.md) | hub-bits | Svelte substrate policy | 🟢 ready for review |

---

## Specimen Dossiers

Specimen dossiers document specific implementations as reference cases.

| Dossier | Subject | Purpose | Status |
|---------|---------|---------|--------|
| *(none yet)* | | | |

---

## Research Priorities

### ✅ Phase 1 Complete (unblocks g01.002-g01.006)

All immediate priority research is **COMPLETE**:

1. ✅ **hub-gpui** - GPUI capabilities understood
2. ✅ **hub-bits** - Bits coverage understood
3. ✅ **tk-design-token-systems** - Token taxonomy complete
4. ✅ **tk-cross-framework-contracts** - Contract patterns complete

**Result:** g01.002 through g01.006 are **unblocked**.

### ✅ Phase 2 Near-Term Complete (informs g01.007-g01.012)

Phase 2 value tracks now **COMPLETE**:

5. ✅ **tk-workstation-ui-patterns** - Panel/dock patterns documented
6. ✅ **tk-accessibility-cross-platform** - ARIA/keyboard patterns documented
7. ✅ **tk-overlay-positioning** - Overlay positioning patterns documented
8. ✅ **tk-form-validation-patterns** - Form validation patterns documented

**Result:** g01.007 through g01.012 have research guidance.

### 🟡 Remaining Phase 2

9. **tk-gpui-idioms** - GPUI implementation patterns (reference material)
10. **tk-svelte-headless-patterns** - Svelte patterns (reference material)

### 🟡 Phase 3 Future (g02/g03 preparation)

11. **hub-underlay** - Integration patterns for g01.013
12. **tk-data-table-patterns** - Virtualization for g02.002
13. **tk-command-palette-patterns** - Command palette baseline landed in g02.008
14. **tk-parity-testing** - Testing strategies for g03.002
15. **tk-docs-site-tooling** - Docs platforms for g02.012

---

## Research Summary

### Completed Research (12 documents)

**Source Hubs (2):**
- hub-gpui.md - GPUI capabilities and token consumption
- hub-bits.md - Bits UI primitives and extension model

**Value Tracks (8):**
- tk-design-token-systems.md - DTCG, Style Dictionary
- tk-cross-framework-contracts.md - React Aria, Zag.js patterns
- tk-workstation-ui-patterns.md - VS Code, DAW panel systems
- tk-accessibility-cross-platform.md - ARIA, keyboard navigation
- tk-overlay-positioning.md - Floating UI, positioning strategies
- tk-form-validation-patterns.md - Field states, validation
- tk-svelte-headless-patterns.md - (placeholder)
- tk-gpui-idioms.md - (placeholder)

**Translation Memos (3):**
- tm-token-system.md - Token system decisions
- tm-contract-template.md - Contract template decisions
- tm-svelte-substrate.md - Svelte substrate decisions

### Architecture Integration

All research has been integrated into:
- `docs/architecture/001-flint-system-shape.md`
- `docs/architecture/002-token-system-and-package-layout.md`

### Ready for Implementation

Research now supports:
- g01.002 Token Schema
- g01.003 Token Artifacts
- g01.004 Contract Template
- g01.005 Svelte Substrate
- g01.006 GPUI Substrate
- g01.007-012 Primitive suite with patterns for:
  - Accessibility (tk-accessibility-cross-platform)
  - Forms (tk-form-validation-patterns)
  - Overlays (tk-overlay-positioning)
  - Workstation shell (tk-workstation-ui-patterns)

---

## Cross-References

- Research findings feed into: [research-to-architecture-crossref.md](./research-to-architecture-crossref.md)
- Research informs implementation via: [research-to-implementation-playbook.md](./research-to-implementation-playbook.md)

---

## Next Task

1. Review translation memos with stakeholders
2. Begin g01.002 token schema implementation
3. Create first component contracts using research findings
4. Plan Phase 3 research as g01 implementation progresses
