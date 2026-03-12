# Panel Surface

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `PanelSurface`
- Layer: `workstation`
- Summary: a reusable container for docked or floating workstation panels with
  optional header, tabs, and body regions
- In scope: shell-level panel chrome, header slotting, active/inactive state,
  body scrolling boundary
- Out of scope: transport strips, timeline rows, console channels, or any
  Loophole-specific DAW content inside the panel body

## 2. Anatomy

```text
[Root Surface]
  ├── [Header] (optional)
  │     ├── [Title Area]
  │     ├── [Tab Strip] (optional)
  │     └── [Utility Actions] (optional)
  └── [Body Viewport]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Surface | yes | panel shell and border context | background, border, radius, elevation |
| Header | no | shell chrome for title, tabs, actions | background, separator, spacing |
| Title Area | no | current panel identity | typography, text color |
| Tab Strip | no | panel-local navigation host | spacing, border, active indicator |
| Utility Actions | no | panel-scoped shell actions | icon color, gap |
| Body Viewport | yes | content region with panel-owned scrolling boundary | surface background, body padding |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string \| null` | `null` | no | visible title when header is present |
| `isActive` | `boolean` | `false` | no | indicates currently focused/selected panel |
| `isElevated` | `boolean` | `false` | no | floating or prominent shell state |
| `hasHeader` | `boolean` | `true` | no | allows pure body surfaces when shell chrome lives elsewhere |
| `bodyPadding` | `"none" \| "sm" \| "md"` | `"md"` | no | semantic body spacing only |
| `scrollMode` | `"panel" \| "content"` | `"panel"` | no | whether viewport or child content owns scrolling |
| `headerActions` | `slot/children` | none | no | shell-owned action affordances |
| `tabs` | `slot/children` | none | no | panel-local tab surface |

### Naming Rules

- keep shell semantics generic: `isActive`, `isElevated`, `hasHeader`
- do not leak app-specific panel taxonomies into the public API

### Controlled And Uncontrolled

- no value control on the surface itself
- panel selection, tab selection, and dock placement are owned by higher shell
  systems

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| inactive | default | neutral panel shell |
| active | `isActive=true` | emphasized border or title treatment |
| elevated | `isElevated=true` | stronger elevation and contrast |
| headerless | `hasHeader=false` | body region fills top boundary |
| scroll-contained | `scrollMode="panel"` | body viewport owns scroll clipping |

### Component States

State table is sufficient. Panel docking, persistence, and transfer state live
in higher-order shell contracts rather than in `PanelSurface` itself.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onFocusWithinChange` | focus enters or exits the surface subtree | boolean | optional shell signal |
| `onHeaderAction` | header utility action triggers | app-defined | only for generic shell actions |
| `onBodyScroll` | body viewport scrolls | framework-native event | optional passthrough |

## 6. Accessibility

### Semantics

- Role: usually `region` when the panel has a labelable heading; may be neutral
  container when shell semantics live outside
- Required attributes: accessible name through visible title or host-provided
  labeling when the panel is an addressable region
- Optional attributes: `aria-labelledby`, `aria-describedby`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | enters next focusable child within the panel |
| `Shift+Tab` | exits or moves backwards through the panel subtree |
| panel-local shortcuts | handled by higher shell contracts, not by `PanelSurface` itself |

### Focus And Announcement

- focus entry: active styling may follow `focus-within` or explicit shell state
- focus exit: active emphasis clears when shell focus moves away unless
  selection model keeps it active
- live-region behavior: none
- GPUI-native accessibility mapping notes: when the panel is exposed as a named
  region, GPUI must surface an equivalent region/group node and preserve label
  relationships for assistive technology

## 7. Layout

### Sizing

- minimum size: must preserve readable header chrome and body viewport
- maximum size: fills region assigned by dock/workspace system
- overflow behavior: root clips overflow; body viewport is the default scrolling
  container when `scrollMode="panel"`

### Composition

- parent expectations: dock region, split-view slot, floating workspace window,
  or standalone inspector shell
- child expectations: body content must not assume direct knowledge of dock or
  window mechanics
- resizing rules: parent layout system owns size negotiation; `PanelSurface`
  owns only its internal header/body split

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Surface | `semantic.color.background.panel` | default chrome fill |
| Root Surface | `semantic.color.border.subtle` and `semantic.color.border.default` | shell boundary |
| Root Surface | `semantic.radius.surface` | shared shell rounding |
| Root Surface | `semantic.elevation.surface` and `semantic.elevation.overlay` | normal vs elevated shell |
| Header | `semantic.size.panel.header` | header height |
| Header | `semantic.space.panel.x` and `semantic.space.panel.y` | shell chrome spacing |
| Title | `semantic.typography.label.*` | title typography |
| Title | `semantic.color.text.primary` and `semantic.color.text.secondary` | active/inactive labeling |
| Body Viewport | `semantic.space.panel.*` | body padding baseline |
| Active emphasis | `semantic.color.accent.base` or `semantic.color.accent.focusRing` | selected/focused shell cue |

## 9. Svelte Notes

- expected substrate: native Svelte component with token-driven layout; Bits may
  participate for tab strips or menus inside the header but not for the panel
  shell container itself
- wrapper strategy: panel body and header slots are Pug-owned layout semantics
- implementation-only details: data attributes may represent active/elevated
  shell states for styling
- known browser-specific deltas: `position: sticky` headers are allowed when
  scroll containment remains semantically equivalent

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::workstation::panel_surface`
- theme access strategy: shell colors, spacing, and elevation come from
  generated token helpers via the GPUI theme layer
- implementation-only details: header/body layout should use GPUI-native flex
  or split primitives
- known GPUI-native deltas: scroll indicators and focus treatment may follow
  GPUI conventions if semantic shell states remain intact

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] header/body semantics match
- [ ] active/elevated meaning matches
- [ ] scroll ownership and clipping semantics match
- [ ] labeled-region accessibility rules match

### Tier 2: Visual Parity

- [ ] header height and spacing use the same token roles
- [ ] active/inactive panel emphasis uses the same semantic cues
- [ ] elevated panels feel proportionally consistent

### Tier 3: Implementation Freedom

- [ ] browser sticky-header or GPUI-native layout internals stay implementation-only
- [ ] tab/menu internals inside the header do not redefine panel semantics

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending first implementation | review once both runtimes host real panel content |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: Aura shell rebuild, Spark shell rebuild, future Underlay
  workstation-style tools
- future follow-up: keep later panel orchestration work building on this shell
  surface rather than reopening its semantics

## Next Task

Use this panel contract as the base shell surface beneath `PanelHeader`,
`PanelTabs`, and `DockRegion`.
