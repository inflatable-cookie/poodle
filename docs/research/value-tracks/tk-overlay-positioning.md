# Value Track: Overlay Positioning

Status: complete (findings documented)
Created: 2026-03-11
Updated: 2026-03-11
Priority: high (informs g01.010)

## Purpose

Research popover, dialog, menu, and tooltip positioning patterns to inform:
- Overlay primitive design (g01.010)
- Floating element positioning
- Z-index/layering strategies
- Collision detection and viewport handling

---

## Key Findings

### CSS Anchor Positioning (Emerging)

**Native browser support (Chrome, Safari, Firefox coming):**
```css
.trigger {
  anchor-name: --menu-trigger;
}

.popover {
  position: absolute;
  position-anchor: --menu-trigger;
  top: anchor(bottom);
  left: anchor(left);
  position-try-fallbacks: flip-block, flip-inline;
}
```

**Benefits:**
- No JavaScript for positioning
- Automatic fallback handling
- Better performance
- Native scroll anchoring

**Status:** Expected baseline 2026. Not yet ready for Pug's initial implementation.

### Floating UI (Current Standard)

**Features:**
- Anchor positioning relative to trigger
- Middleware system for behavior
- Collision detection (flip, shift, hide)
- Auto-update on scroll/resize
- Framework agnostic

**Core Concepts:**
```
useFloating({
  middleware: [
    offset(10),        // Gap between anchor and floating
    flip(),            // Flip to opposite side if no space
    shift(),           // Shift along edge if no space
    autoPlacement(),   // Choose best placement automatically
    hide()             // Hide when anchor leaves viewport
  ]
})
```

### Positioning Strategies

#### 1. Absolute Positioning with Portal

**How it works:**
- Element rendered in portal (body or container)
- Absolute positioning relative to viewport
- Position calculated based on anchor rect

**Pros:**
- Escapes overflow:hidden containers
- Can use z-index layering
- Consistent positioning

**Cons:**
- Requires JavaScript for positioning
- Must handle scroll/resize updates

#### 2. Fixed Positioning

**Use case:**
- Modals that cover entire viewport
- Tooltips that follow cursor
- Elements that should stay in view

**Pros:**
- Positioned relative to viewport
- Not affected by scroll

**Cons:**
- Doesn't scroll with anchor
- Limited use cases

### Placement Options

**Standard Placements:**
| Placement | Description |
|-----------|-------------|
| `top` | Above anchor, centered |
| `top-start` | Above anchor, left-aligned |
| `top-end` | Above anchor, right-aligned |
| `bottom` | Below anchor, centered |
| `bottom-start` | Below anchor, left-aligned |
| `bottom-end` | Below anchor, right-aligned |
| `left` | Left of anchor, centered |
| `right` | Right of anchor, centered |

### Collision Detection

**Flip Strategy:**
- If no space on chosen side, flip to opposite
- Example: `top` → `bottom`

**Shift Strategy:**
- If no space for full width, shift along edge
- Maintains placement but adjusts position

**Auto Placement:**
- Chooses best placement based on available space
- Considers all sides and picks optimal

**Hide Strategy:**
- Hide floating element when anchor leaves viewport
- Prevents orphaned overlays

### Z-Index / Layering

**Common Z-Index Scale:**
```css
:root {
  --z-dropdown: 1000;
  --z-sticky: 1020;
  --z-fixed: 1030;
  --z-modal-backdrop: 1040;
  --z-modal: 1050;
  --z-popover: 1060;
  --z-tooltip: 1070;
}
```

**Portal Strategy:**
- Render overlays in dedicated portal container
- Container has high z-index
- Elements within ordered by type

**GPUI Considerations:**
- GPUI uses painter's algorithm (last rendered on top)
- No CSS z-index equivalent
- Layering controlled by render order

### Modality

**Modal Overlays:**
- Trap focus within overlay
- Block interaction with background
- Backdrop click dismisses
- `aria-modal="true"`

**Non-Modal Overlays:**
- Don't trap focus
- Background remains interactive
- Click outside may dismiss
- `aria-modal="false"`

### Components Using Overlays

| Component | Modal | Positioning |
|-----------|-------|-------------|
| Dialog/Modal | Yes | Centered or custom |
| Popover | Optional | Relative to trigger |
| Dropdown Menu | No | Below trigger |
| Tooltip | No | Above/below trigger |
| Context Menu | No | At cursor position |
| Toast | No | Fixed corner position |

---

## Recommendations for Pug

### Overlay Component Suite

```
Layer 1 - Overlays
├── Dialog/Modal
│   ├── Modal.Backdrop
│   ├── Modal.Content
│   └── Modal.Close
├── Popover
│   ├── Popover.Trigger
│   ├── Popover.Content
│   └── Popover.Arrow
├── Tooltip
├── DropdownMenu
└── Toast/Notification
```

### Positioning API

**Popover Positioning Props:**
```typescript
interface PositioningProps {
  placement: 'top' | 'top-start' | 'top-end' |
             'bottom' | 'bottom-start' | 'bottom-end' |
             'left' | 'right';
  offset?: number;           // Gap in pixels
  flip?: boolean;            // Enable flip on collision
  shift?: boolean;           // Enable shift on collision
  sameWidth?: boolean;       // Match trigger width
  align?: 'start' | 'center' | 'end';
}
```

### Svelte Implementation

**Bits UI provides:**
- Portal component
- Positioning via computed styles
- Collision detection
- Arrow positioning

**Pug adds:**
- Token-based styling
- Consistent animation patterns
- Positioning presets

### GPUI Implementation

**Considerations:**
- GPUI doesn't have CSS positioning
- Elements positioned via layout
- Overlays rendered in separate layer

**Approach:**
```rust
// GPUI overlay positioning
Overlay::new(
  anchor_element,
  |cx| {
    div()
      .absolute()  // GPUI absolute positioning
      .top_0()
      .left_0()
      .child(content)
  }
)
```

### Z-Index Strategy

**Token-Based:**
```css
/* Pug tokens */
--pug-z-dropdown: 1000;
--pug-z-popover: 1010;
--pug-z-tooltip: 1020;
--pug-modal-backdrop: 1030;
--pug-z-modal: 1040;
```

**GPUI Equivalent:**
- Render order determines layering
- Explicit layer management
- Document as delta

---

## Related

- Floating UI: https://floating-ui.com/
- CSS Anchor Positioning: https://developer.mozilla.org/en-US/docs/Web/CSS/anchor
- Milestone: [g01.010](../../roadmaps/g01/010-overlay-navigation-and-interaction-primitives.md)

---

## Next Task

Create Popover and Dialog component contracts with positioning specifications.
