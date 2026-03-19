# g08.008 Visual Parity Verification And Delta Register

Status: planned
Owner: Pug Core
Depends on: g08.007

## Goals

Systematically compare every GPUI specimen page against the Svelte reference.
Document all differences. Fix bugs. Record intentional deltas with rationale.

## Execution Checklist

### Systematic Comparison

- [ ] Run both Svelte and GPUI preview apps
- [ ] Compare every specimen page
- [ ] Classify differences as bugs (fix) or intentional deltas (document)

### Known GPUI-Specific Deltas (expected)

These are rendering differences inherent to the GPUI platform:
- Font rendering (subpixel antialiasing, hinting differences)
- SVG rendering via luminance-based alpha mask
- Element-level opacity vs per-color alpha
- Animation timing may differ slightly

### Contract Compliance Verification

- [ ] For each component, verify against contract checklist:
  - All dimensions from tokens
  - All colors from tokens
  - Anatomy matches contract
  - All props supported
  - Disabled/loading states correct
  - Focus ring present on interactive components
  - ARIA attributes applied

### Delta Register

- [ ] Create or update `docs/roadmaps/g08/delta-register.md`
- [ ] Record each intentional delta with:
  - Component name
  - What differs
  - Why (platform constraint)
  - Severity (cosmetic / functional)

## Acceptance Criteria

- [ ] Every component page compared
- [ ] All bugs found and fixed
- [ ] Delta register documents all intentional differences
- [ ] Contract compliance verified for all components
