# DetailSection

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `DetailSection`
- Layer: `composites`
- Summary: a titled grouping of related detail rows, supporting copy, and
  optional section-level actions
- In scope: section heading, supportive description, grouped body content,
  optional actions, divider posture
- Out of scope: page-level header identity, domain-specific row contents,
  editable form submission logic

## 2. Anatomy

```text
[Root Section]
  ├── [Section Header]
  │     ├── [Title Block]
  │     └── [Actions] (optional)
  └── [Section Body]
        └── [Detail Rows or Custom Content]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Section | yes | grouped detail region | spacing, separator |
| Section Header | no | heading and optional actions | typography, gap |
| Title Block | no | title and support text | typography, text color |
| Section Body | yes | grouped rows/content | spacing, surface |
| Actions | no | section-scoped actions | action spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string \| null` | `null` | no | section heading |
| `description` | `string \| null` | `null` | no | support text |
| `isSeparated` | `boolean` | `true` | no | visual separation from surrounding sections |
| `ariaLabel` | `string \| null` | `null` | no | optional label when no visible title exists |

### Controlled And Uncontrolled

- declarative grouping composite
- child content remains host-owned

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| simple | body only | compact grouped body |
| titled | title present | heading-led section |
| actionable | actions present | header split layout |
| separated | `isSeparated=true` | divider or spacing separation |

### Component States

State table is sufficient.

## 5. Events

No component-owned events beyond child action behavior.

## 6. Accessibility

### Semantics

- Role: region, group, or neutral section container depending on heading and
  context
- Required attributes: heading association when the section is addressable
- Optional attributes: region label override
- Labeling rules: if the section is meant to be navigable, it needs a stable
  accessible name

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves through section actions and body content in logical order |

### Focus And Announcement

- focus entry: the section container is not focusable by default
- focus exit: section-level actions should not interrupt child row order
- live-region behavior: none
- GPUI-native accessibility mapping notes: GPUI must preserve heading-to-group
  relationships when the section acts as a named region or addressable group

## 7. Layout

### Sizing

- sections fill available parent width
- body spacing stays consistent across simple and descriptive headers

### Composition

- parent expectations: detail shells, settings pages, inspector sections
- child expectations: `DetailRow`, `Card`, or light custom body content
- resizing rules: actions may wrap below the title block on narrow widths
- composition rule: sections may mix summary cards and detail rows, but should
  still preserve a clear section heading and subordinate hierarchy

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Section | spacing and separator roles | grouping |
| Title Block | heading/subdued text roles | hierarchy |
| Section Body | spacing and surface roles | content container |
| Actions | action spacing roles | control cluster |

## 9. Svelte Notes

- expected substrate: `Stack`, `Inline`, `Separator`, and heading semantics
- wrapper strategy: section slots stay Pug-owned; action and body children are
  composed rather than hard-coded

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::composites::detail_section`
- implementation-only details: GPUI may use grouped layout views and named
  subtrees instead of HTML sections, but addressable-section semantics remain
  required

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] heading/group semantics match
- [ ] section action ordering matches
- [ ] body composition remains semantically neutral and consistent

### Tier 2: Visual Parity

- [ ] section spacing, dividers, and title hierarchy use comparable token roles

### Tier 3: Implementation Freedom

- [ ] body slot mounting and wrapping behavior stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| divider rendering may differ | runtime drawing techniques differ | allowed | keep grouping semantics strict |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### With title and rows

A titled section with description and detail rows:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With title and rows | `title="Project details"`, `description="Core metadata for this project."`, four DetailRows (Name, Owner, Created, Status) | heading-led section with description text and vertically stacked detail rows |

### With actions

A titled section with a header action button:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With actions | `title="Billing"`, actions slot with secondary sm Edit button, three DetailRows (Plan, Billing cycle, Next invoice) | header split layout with title on start and Edit button on end, detail rows below |

### DetailRow with description

A section demonstrating detail rows that include description text:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| DetailRow with description | `title="Configuration"`, two DetailRows with `description` prop; first row has `truncateValue` | detail rows with label, value, and supporting description; first value truncates with ellipsis |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: settings scopes, inspector sections, metadata groupings
- future follow-up: align with deeper form sections and validation groups later

## Next Task

Use `DetailSection` to group related readonly information under a local heading,
and keep page-level identity in `PageHeader` or `DetailShell`.
