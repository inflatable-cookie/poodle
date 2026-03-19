# DetailShell

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `DetailShell`
- Layer: `composites`
- Summary: a reusable information-display shell for a single record, entity, or
  settings scope
- In scope: header region, scrollable detail body, empty/loading/error posture,
  section stack
- Out of scope: domain-specific data fetching, editable form workflows,
  workstation panel chrome

## 2. Anatomy

```text
[Root Shell]
  ├── [Header Region] (optional)
  ├── [State Region] (optional)
  └── [Body Scroll Region]
        └── [Detail Sections or Custom Content]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Shell | yes | overall detail container | spacing, background |
| Header Region | no | page identity and top actions | spacing, separator |
| State Region | no | empty, loading, or callout state | spacing |
| Body Scroll Region | yes | main content viewport | scroll, spacing, surface |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string \| null` | `null` | no | convenience shorthand when no external header composite is supplied |
| `scrollMode` | `"shell" \| "body"` | `"body"` | no | who owns vertical scrolling |
| `state` | `"ready" \| "empty" \| "loading" \| "error"` | `"ready"` | no | high-level content posture |
| `ariaLabel` | `string \| null` | `null` | no | optional region label |

### Controlled And Uncontrolled

- declarative composite shell
- state posture is controlled by the host application

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | `state="ready"` | content visible |
| empty | `state="empty"` | empty-state region replaces content |
| loading | `state="loading"` | skeleton/progress region visible |
| error | `state="error"` | callout region visible |

### Component States

State table is sufficient.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onScroll` | body viewport scrolls | framework-native event | optional passthrough |

## 6. Accessibility

### Semantics

- Role: usually `main`, `region`, or grouped content shell depending on parent
  context
- Required attributes: accessible name when the shell is a named destination
- Optional attributes: heading association and state-region descriptions
- Labeling rules: when the shell hosts a primary detail destination, heading
  hierarchy should make that destination obvious

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | enters header actions, state content, and body content in logical order |
| scroll keys | operate on the documented scroll owner when focus enters it |

### Focus And Announcement

- focus entry: the shell itself is not focusable by default unless it owns a
  scroll destination
- focus exit: state transitions should preserve or restore a sensible focus
  target when content changes materially
- live-region behavior: loading and error transitions should surface through
  child `Progress`, `Skeleton`, or `Callout` semantics rather than
  ad hoc shell announcements
- GPUI-native accessibility mapping notes: GPUI must preserve named-region
  semantics, scroll ownership, and sensible focus continuity when loading, error,
  or empty content replaces the body

## 7. Layout

### Sizing

- shell fills assigned parent region
- body scroll viewport should tolerate long section stacks

### Composition

- parent expectations: product page bodies, settings scopes, inspector-like
  detail panes in product apps
- child expectations: `PageHeader`, `DetailSection`, `ScrollShell`,
  `EmptyState`, `Callout`, and feedback primitives
- resizing rules: header and state regions remain visible without obscuring body
  reachability
- state rule: `DetailShell` uses `ready`, `empty`, `loading`, and `error`;
  browse-specific `no-results` posture does not belong here

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Shell | background and spacing roles | page shell |
| Header Region | separator and spacing roles | top identity |
| State Region | spacing roles | transient state messages |
| Body Scroll Region | `ScrollShell` and surface roles | content viewport |

## 9. Svelte Notes

- expected substrate: `Stack` plus `ScrollShell` and other composites
- wrapper strategy: host should compose explicit `PageHeader` and `DetailSection`
  children rather than overloading the shell with domain fields

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::composites::detail_shell`
- implementation-only details: GPUI may model the body as a native scroll view,
  but state swaps and focus continuity still require deliberate handling

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] named-region and scroll-owner semantics match
- [ ] empty/loading/error posture meaning matches
- [ ] focus continuity across state changes matches

### Tier 2: Visual Parity

- [ ] shell spacing and section stack hierarchy use comparable token roles

### Tier 3: Implementation Freedom

- [ ] viewport implementation and mount strategy stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| scroll indicator styling may differ | runtime scroll rendering differs | allowed | keep scroll ownership and focus rules strict |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Layout structure

A shell showing placeholder regions for anatomy visualization:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Layout structure | header slot with colored Region, three body Regions (Section 1, 2, 3) | shell with distinct header region and stacked body sections, each shown as colored placeholder blocks |

### Multi-section layout with header

A full detail view with PageHeader and multiple DetailSections:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Multi-section layout with header | PageHeader with title, eyebrow, subtitle, Badge and Edit button; three DetailSections (General, Configuration with Reset action, Integrations) separated by Separators | complete detail page with identity header, action controls, and grouped detail rows across sections |

### Loading state

A shell in loading posture:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Loading state | `title="Loading"`, `state="loading"` | shell with loading/progress indicator replacing body content |

### Error state

A shell in error posture:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Error state | `title="Error"`, `state="error"`, `stateTitle="Failed to load"`, `stateMessage="Something went wrong. Please try again."` | shell with error callout replacing body content, showing title and message |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: settings scopes, detail inspectors, entity pages
- future follow-up: connect editable detail and form workflows in later milestones

## Next Task

Use `DetailShell` as the framing layer for local identity, summary cards, and
structured detail sections while keeping models, fetch state, and mutation
logic outside the composite.
