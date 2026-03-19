# ProjectHeader

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `ProjectHeader`
- Layer: `workstation`
- Summary: a focused workstation header for project identity, file/workspace
  actions, and project-scoped status
- In scope: project title, dirty state cue, project actions, workspace/layout
  selectors, scoped status indicators
- Out of scope: app-global shell identity, DAW-specific transport or editing
  controls

## 2. Anatomy

```text
[Root Header]
  ├── [Project Identity]
  ├── [Scoped Actions]
  └── [Status Region] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Header | yes | project-level shell row | background, border, spacing |
| Project Identity | yes | title and state cue | typography, text color |
| Scoped Actions | no | project/workspace commands | action spacing |
| Status Region | no | dirty, sync, or workspace status | status, subdued text |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | none | yes | visible project/workspace title |
| `isDirty` | `boolean` | `false` | no | unsaved-changes cue |
| `subtitle` | `string \| null` | `null` | no | optional supporting metadata |
| `ariaLabel` | `string \| null` | `null` | no | optional region label |

### Controlled And Uncontrolled

- declarative shell header
- commands, selectors, and status content remain host-owned children

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| clean | default | neutral project header |
| dirty | `isDirty=true` | unsaved-changes cue visible |
| descriptive | subtitle or status visible | expanded metadata posture |

### Component States

State table is sufficient.

## 5. Events

No component-owned events beyond child action behavior.

## 6. Accessibility

### Semantics

- Role: complementary header region or toolbar-adjacent section depending on
  shell placement
- Required attributes: clear project identity text
- Optional attributes: region label and state descriptions
- Labeling rules: dirty-state cues must not rely only on color or punctuation

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | reaches project actions and status affordances in logical order |

### Focus And Announcement

- focus entry: the header container is not focusable by default
- focus exit: dirty-state changes should be conveyed through text or status
  semantics when materially important
- live-region behavior: none by default; child status elements may announce
  meaningful project changes
- GPUI-native accessibility mapping notes: GPUI must preserve project identity
  and state cues as named shell context rather than a decorative toolbar row

## 7. Layout

### Sizing

- fixed header height with allowance for narrow stacked layouts
- action clusters may wrap or collapse according to host policy

### Composition

- parent expectations: workspace shell beneath `AppHeader`, focused project
  windows, shell-local tool surfaces
- child expectations: actions, selectors, and status chips/indicators
- resizing rules: project identity remains primary under width pressure

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Header | shell background, border, and spacing roles | project chrome |
| Project Identity | title and subdued text roles | project context |
| Dirty cue | warning/accent roles | unsaved-state visibility |
| Scoped Actions | action spacing roles | command cluster |

## 9. Svelte Notes

- expected substrate: `Inline`, `Surface`, action primitives, and status
  primitives
- wrapper strategy: action wiring remains external to the contract

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::workstation::project_header`
- implementation-only details: GPUI may embed layout selectors or status chips
  natively, but project identity and dirty-state semantics remain required

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] project identity and dirty-state meaning match
- [ ] action/status ordering matches

### Tier 2: Visual Parity

- [ ] title prominence and status hierarchy use comparable token roles

### Tier 3: Implementation Freedom

- [ ] selector and action packing strategy stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact dirty cue styling may differ | runtime shell chrome differs | allowed | keep non-color semantics strict |

## 13. Specimen Definitions

> **Note:** The `ProjectHeaderSpecimen.svelte` has been removed and needs to be recreated. The following specimen groups should be implemented when the specimen is restored.

### Clean State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Clean state | `title="My Project"` | Neutral project header with title, no dirty indicator |

### Dirty State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Dirty state | `title="My Project"`, `isDirty=true` | Project header with unsaved-changes cue visible alongside title |

### With Subtitle And Actions

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With subtitle and actions | `title="My Project"`, `subtitle="Last saved 2 minutes ago"`, scoped action buttons in actions slot | Header with title, subtitle metadata below, and action controls trailing |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: project windows, focused workspace shells
- future follow-up: connect richer layout-selector depth in later workstation
  milestones

## Next Task

Use `ProjectHeader` for project-scoped shell context and keep app-global
identity in `AppHeader`.
