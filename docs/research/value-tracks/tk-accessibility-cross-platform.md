# Value Track: Accessibility Cross-Platform

Status: complete (findings documented)
Created: 2026-03-11
Updated: 2026-03-11
Priority: high (informs g01.007-010, g02.011)

## Purpose

Research ARIA patterns, keyboard navigation, and focus management that apply across web (Svelte) and native (GPUI) to inform:
- Accessibility requirements for all primitives
- Keyboard navigation patterns
- Focus management strategies
- Cross-platform parity for accessibility

---

## Key Findings

### ARIA Authoring Practices (APG)

The W3C ARIA Authoring Practices Guide defines standard patterns for accessible components.

**Key Principles:**
1. **Use semantic HTML first** - Only add ARIA when HTML semantics insufficient
2. **Keyboard operable** - All interactive elements must work with keyboard
3. **Visible focus** - Focus indicator must be visible at all times
4. **Logical tab order** - Tab sequence follows visual layout

### Keyboard Navigation Conventions

**Tab Sequence:**
- `Tab` - Move to next focusable element
- `Shift + Tab` - Move to previous focusable element
- Only ONE element of a composite component in tab sequence

**Arrow Key Navigation (inside components):**
- Arrow keys navigate within composite components
- Examples: radio groups, menus, tabs, listboxes, grids

**Common Key Patterns:**

| Key | Action |
|-----|--------|
| `Enter` / `Space` | Activate button, toggle, select |
| `Escape` | Close modal, menu, popover |
| `Arrow Up/Down` | Navigate vertical lists, menus |
| `Arrow Left/Right` | Navigate horizontal tabs, sliders |
| `Home` / `End` | Jump to first/last item |
| `Page Up` / `Page Down` | Scroll large lists |

### Focus Management Patterns

#### 1. Roving Tabindex

For composite components (radio groups, menus, tabs):

```
Initial state:
- First item: tabindex="0"
- Others: tabindex="-1"

After arrow navigation:
- Previously focused: tabindex="-1"
- Newly focused: tabindex="0"
```

**Benefits:**
- Only one item in tab sequence
- Arrow keys navigate within component
- Tab moves out of component

#### 2. Focus Trapping (Modals/Dialogs)

When modal opens:
1. Focus moves to first focusable element in modal
2. Tab cycles within modal only
3. Background content is inert

When modal closes:
1. Focus returns to element that opened modal

#### 3. Focus Restoration

Always return focus to trigger element when:
- Modal/dialog closes
- Menu closes
- Popover closes

### ARIA Roles by Component

| Component | Role | Required Attributes |
|-----------|------|---------------------|
| Button | `button` | `aria-label` (if no text) |
| Checkbox | `checkbox` | `aria-checked` |
| Dialog | `dialog` | `aria-modal`, `aria-labelledby` |
| Menu | `menu` | `aria-orientation` |
| Menu Item | `menuitem` | - |
| Radio Group | `radiogroup` | - |
| Radio | `radio` | `aria-checked` |
| Select | `combobox` | `aria-expanded`, `aria-controls` |
| Slider | `slider` | `aria-valuenow`, `aria-valuemin`, `aria-valuemax` |
| Switch | `switch` | `aria-checked` |
| Tabs | `tablist` | `aria-orientation` |
| Tab | `tab` | `aria-selected`, `aria-controls` |
| Tab Panel | `tabpanel` | - |
| Tooltip | `tooltip` | - |

### ARIA States & Properties

**Common States:**
- `aria-expanded` - Dropdown, accordion open state
- `aria-selected` - Selected item in list
- `aria-checked` - Checkbox, radio, switch state
- `aria-pressed` - Toggle button state
- `aria-hidden` - Element not visible/accessible
- `aria-disabled` - Non-interactive but focusable

**Relationship Properties:**
- `aria-controls` - Element controls another
- `aria-labelledby` - Labels element by ID
- `aria-describedby` - Describes element by ID
- `aria-owns` - Parent of child elements

**Live Regions:**
- `aria-live="polite"` - Announce when user idle
- `aria-live="assertive""` - Announce immediately
- `role="alert"` - Important alert message
- `role="status"` - Status update

### Focus Visible Patterns

**CSS for Web (Svelte):**
```css
/* Don't remove focus outline */
*:focus {
  outline: 2px solid var(--poodle-color-focus);
  outline-offset: 2px;
}

/* Or use :focus-visible for keyboard only */
*:focus-visible {
  outline: 2px solid var(--poodle-color-focus);
  outline-offset: 2px;
}
```

**GPUI Considerations:**
- GPUI has native focus rings
- May differ from web styling
- Should be documented as known delta

### Cross-Platform Parity for Accessibility

| Aspect | Web (Svelte) | GPUI | Parity Level |
|--------|--------------|------|--------------|
| Keyboard navigation | Full ARIA patterns | Platform conventions | Strict |
| Focus rings | CSS :focus-visible | Native focus | Visual delta acceptable |
| Screen readers | ARIA attributes | Platform APIs | Strict behavior |
| Tab order | Tabindex | Focus order | Strict |
| Focus trapping | JS focus management | GPUI modal behavior | Strict |
| Live regions | aria-live | Platform announcements | Strict |

### Common Accessibility Mistakes

1. **Removing focus outlines** without replacement
2. **Overusing ARIA** when semantic HTML works
3. **Inconsistent keyboard behavior** from platform conventions
4. **Missing focus management** in modals/menus
5. **Not announcing dynamic changes** via live regions
6. **Using `aria-hidden` on focusable elements**
7. **Wrong ARIA roles** for component type

### Testing Checklist

**Keyboard Testing:**
- [ ] All interactive elements reachable by Tab
- [ ] Tab order is logical
- [ ] Focus visible at all times
- [ ] Arrow keys work in composite components
- [ ] Escape closes overlays
- [ ] Focus returns to trigger on close

**Screen Reader Testing:**
- [ ] Roles announced correctly
- [ ] States update when changed
- [ ] Labels associated properly
- [ ] Dynamic content announced
- [ ] No duplicate announcements

**Automated Testing:**
- [ ] axe-core or similar passes
- [ ] Lighthouse accessibility score
- [ ] No ARIA validation errors

---

## Recommendations for Poodle

### Accessibility Requirements for All Components

**Tier 1 (Strict Parity):**
- Keyboard navigation patterns
- Focus management behavior
- Screen reader semantics
- Tab order logic

**Tier 2 (Visual Parity):**
- Focus ring appearance
- High contrast support
- Reduced motion support

### Component-Specific Keyboard Patterns

**Button:**
- `Enter` or `Space` to activate
- `Tab` to focus

**Checkbox:**
- `Space` to toggle
- `Tab` to move between checkboxes

**Radio Group:**
- `Arrow Up/Down` or `Arrow Left/Right` to select
- Only selected radio in tab sequence

**Select/Dropdown:**
- `Space` or `Enter` to open
- `Arrow Up/Down` to navigate options
- `Enter` to select
- `Escape` to close

**Dialog/Modal:**
- `Tab` cycles within dialog
- `Escape` closes
- Focus returns to trigger

**Tabs:**
- `Tab` enters tablist
- `Arrow Left/Right` changes tabs
- `Tab` again enters tab panel

### Focus Management Guidelines

1. **Visible focus required** - Never hide focus indicator
2. **Logical tab order** - Match visual flow
3. **Focus restoration** - Return to trigger on close
4. **Focus trapping** - Trap in modals/menus
5. **Skip links** - Provide for repetitive content

### Documentation Requirements

Each component contract must document:
- ARIA role(s)
- Keyboard interaction
- Focus behavior
- Screen reader expectations

---

## Related

- W3C ARIA Authoring Practices: https://www.w3.org/WAI/ARIA/apg/
- Milestone: [g01.007-010](../../roadmaps/g01/) - Primitive implementation
- Milestone: [g02.011](../../roadmaps/g02/011-accessibility-focus-keyboard-and-state-semantics-hardening.md)

---

## Next Task

Create accessibility guidelines document for component authors.
