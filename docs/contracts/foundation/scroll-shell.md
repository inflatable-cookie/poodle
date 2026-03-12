# Scroll Shell

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `ScrollShell`
- Layer: `foundation`
- Summary: a reusable scrolling boundary with explicit viewport ownership,
  focus behavior, and assistive-technology expectations
- In scope: viewport shell, overflow, keyboard reachability, optional region
  labeling
- Out of scope: virtualized list semantics, custom scrollbars as a contract
  requirement, inertial physics tuning

## 2. Anatomy

```text
[Root]
  └── [Viewport]
        └── [Content]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | shell boundary for scroll area | optional surface/border tokens |
| Viewport | yes | element or node that owns scrolling | spacing and overflow |
| Content | yes | scrolled content subtree | caller-owned |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `direction` | `"vertical" \| "horizontal" \| "both"` | `"vertical"` | no | owned scroll axis |
| `padding` | `"none" \| "sm" \| "md"` | `"none"` | no | viewport interior spacing |
| `asRole` | `"region" \| "group" \| null` | `null` | no | semantic opt-in when the scroller is a named destination |
| `label` | `string \| null` | `null` | no | required when the shell exposes a named region without visible label |
| `isFocusable` | `boolean` | `false` | no | allows keyboard users to move focus to the scroll container when needed |

### Controlled And Uncontrolled

- no controlled value model
- scroll position APIs may be added later if cross-runtime parity requires them

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | neutral scrolling shell |
| focus | `isFocusable=true` and container focused | visible focus treatment |
| overflowed | content exceeds viewport | scrolling available on declared axis |

### Component States

State table is sufficient. Scroll position is runtime state, not a public value
model in this baseline contract.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onScroll` | viewport scroll position changes | runtime-native scroll event | optional passthrough |
| `onReachStart` | scroll reaches start boundary | none | optional later extension |
| `onReachEnd` | scroll reaches end boundary | none | optional later extension |

## 6. Accessibility

### Semantics

- Role: none by default; `region` or `group` only by explicit opt-in
- Required attributes: accessible label when the shell is an addressable region
  and no visible labeling relationship exists
- Optional attributes: description relationships for instructions such as
  “scroll for more content”

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | reaches the scroll shell only when `isFocusable=true` or focusable children are present |
| `Arrow keys` | scroll along owned axis when the viewport itself is focused and platform behavior allows |
| `Page Up/Page Down` | scroll larger increments when the viewport itself is focused |
| `Home/End` | move to start/end when the viewport itself is focused |

### Focus And Announcement

- focus entry: the shell should only enter the tab order when focusable scroll
  behavior is intentional
- focus exit: focus should move to children or out of the shell without trap
  behavior
- live-region behavior: none by default

### GPUI Accessibility Expectations

- GPUI implementations must expose the scroll container as a native accessible
  region when the contract opts in
- keyboard scrolling must be implemented intentionally where the platform does
  not provide it automatically
- the accessible node should communicate scrollability and region labeling
  without requiring HTML/ARIA mechanics

## 7. Layout

### Sizing

- requires explicit or inherited size constraints to create a scrolling
  boundary
- viewport clips content on the owned axis

### Composition

- parent expectations: constrained surface or layout region
- child expectations: arbitrary content, including focusable descendants
- resizing rules: viewport expands or contracts with parent constraints while
  preserving declared scroll ownership

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `semantic.radius.surface` and optional border tokens | shell boundary when visible |
| Viewport | `semantic.space.panel.*` or semantic padding roles | interior spacing |
| Focus treatment | `semantic.color.accent.focusRing` and `semantic.border.width.focus` | visible keyboard focus |

## 9. Svelte Notes

- native browser scrolling should be preferred
- semantic HTML plus browser focus behavior should do the default work where
  possible
- custom scrollbars are allowed but must not replace keyboard reachability or
  accessible naming

## 10. GPUI Notes

- GPUI scroll ownership, focusability, and assistive-technology signaling must
  be implemented explicitly
- if GPUI lacks automatic parity for keyboard scrolling or spoken scroll-region
  semantics, Pug must add that behavior rather than documenting it away
- visual scrollbar appearance may differ, but scrollability and focus behavior
  must not

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] scroll-axis ownership matches
- [ ] focusability rules match
- [ ] keyboard scrolling behavior matches when the viewport is focused
- [ ] named-region semantics match

### Tier 2: Visual Parity

- [ ] focus treatment and shell padding remain proportionally aligned
- [ ] overflow clipping and scrollbar presence communicate the same intent

### Tier 3: Implementation Freedom

- [ ] native browser scrollbars vs GPUI-native scroll rendering stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| scrollbar visuals may differ | platform-native rendering is acceptable | allowed | keep behavior parity strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: lists, shell panels, menus, inspectors, browsers
- future follow-up: add virtualization guidance when list/grid contracts arrive

## Next Task

Use `ScrollShell` as the baseline for future scrollable composites so keyboard
reachability and GPUI accessibility do not get reinvented component by
component.
