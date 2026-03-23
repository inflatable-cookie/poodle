# Value Track: Cross-Framework Component Contracts

Status: complete (findings documented)
Created: 2026-03-11
Updated: 2026-03-11
Priority: critical (blocks g01.004)

## Purpose

Research how design systems define component contracts that work across multiple frameworks to inform Poodle's:
- Component contract template (g01.004)
- Documentation structure
- Parity definition between Svelte and GPUI
- Contract versioning strategy

---

## Key Findings

### React Aria Architecture

**Three-Layer Architecture:**

1. **React Stately** - Framework-agnostic state management
   - Hooks for state logic
   - No assumptions about platform
   - No theme/design system logic
   - Returns state interface for reading/updating

2. **React Aria** - Web-specific behavior & accessibility
   - WAI-ARIA Authoring Practices implementation
   - Screen reader & keyboard navigation
   - Mouse, touch, keyboard interactions
   - Internationalization (30+ languages)
   - Returns DOM props to spread on elements

3. **React Spectrum** - Adobe's themed component library
   - Built on React Aria + Stately
   - Spectrum design system styling

**Key Insight for Poodle:**
- Separate state/behavior from rendering
- State layer can be shared (conceptually) across frameworks
- Each framework implements rendering with native patterns

### React Aria Contract Elements

Per-component documentation includes:

| Section | Contents |
|---------|----------|
| **Purpose** | What the component does |
| **Anatomy** | Structural parts (root, label, input, etc.) |
| **Props/Options** | Configuration with types |
| **State** | Internal state values |
| **Events** | Callbacks and when they fire |
| **Accessibility** | ARIA roles, keyboard behavior |
| **Styling** | Slots, states for styling |
| **Examples** | Usage patterns |

**Props API Pattern:**
```typescript
interface ButtonProps {
  // Appearance
  variant?: 'primary' | 'secondary' | 'ghost';
  size?: 'sm' | 'md' | 'lg';
  
  // State
  isDisabled?: boolean;
  isLoading?: boolean;
  
  // Events
  onPress?: (e: PressEvent) => void;
  onPressStart?: (e: PressEvent) => void;
  onPressEnd?: (e: PressEvent) => void;
  
  // A11y
  'aria-label'?: string;
  'aria-describedby'?: string;
}
```

### Zag.js State Machine Approach

**Architecture:**
- Component logic in framework-agnostic state machines (XState-based)
- Thin framework adapters (React, Vue, Solid, Svelte)
- Framework handles rendering, Zag handles behavior

**Component Definition:**
```javascript
// State machine definition
const machine = createMachine({
  id: "number-input",
  initial: "idle",
  context: {
    value: "0",
    min: 0,
    max: 100,
    step: 1,
  },
  states: {
    idle: {
      on: {
        INCREMENT: { actions: "increment" },
        DECREMENT: { actions: "decrement" },
        CHANGE: { actions: "setValue" },
      },
    },
    // ... more states
  },
});

// Framework adapter (React example)
export function NumberInput() {
  const service = useMachine(machine, { id: useId() });
  const api = numberInput.connect(service, normalizeProps);

  return (
    <div {...api.getRootProps()}>
      <label {...api.getLabelProps()}>Enter number:</label>
      <button {...api.getDecrementTriggerProps()}>-</button>
      <input {...api.getInputProps()} />
      <button {...api.getIncrementTriggerProps()}>+</button>
    </div>
  );
}
```

**Key Insight for Poodle:**
- State machines provide formal behavior specification
- Framework adapters can be thin
- Poodle could define contracts that map to both Svelte and GPUI implementations

### Contract Template Analysis

**Common Elements Across Systems:**

| Element | React Aria | Zag.js | Radix | Open UI |
|---------|------------|--------|-------|---------|
| Purpose | ✅ | ✅ | ✅ | ✅ |
| Anatomy | ✅ | ✅ | ✅ | ✅ |
| Props/Inputs | ✅ | ✅ | ✅ | ✅ |
| State | ✅ | ✅ | ✅ | ✅ |
| Events | ✅ | ✅ | ✅ | ✅ |
| Accessibility | ✅ | ✅ | ✅ | ✅ |
| Styling API | Slots | Data attrs | CSS vars | - |
| Framework Notes | Web only | Adapters | React only | Standard |

**Poodle-Specific Needs (not in precedent systems):**
- Cross-framework parity documentation
- Platform-specific implementation notes
- Delta documentation (what differs and why)

### Parity Definition Patterns

**React Aria's Approach:**
- "Adaptive interactions" - works on mouse, touch, keyboard
- Tested across screen readers
- Behavior consistency prioritized over visual consistency

**Zag.js Approach:**
- State machine ensures behavioral consistency
- Framework adapters ensure idiomatic API in each framework
- Rendering completely delegated (styling fully custom)

**For Poodle's Svelte + GPUI:**

Parity should be defined at these levels:

1. **Semantic Inputs** - Same props achieve same results
2. **State Behavior** - Same states, same transitions
3. **Accessibility** - Same ARIA, same keyboard behavior
4. **Events** - Same callbacks fire at same times
5. **Layout** - Same sizing/positioning expectations

**Acceptable Differences:**
- Visual implementation (CSS vs GPUI styling)
- Internal event handling (DOM vs GPUI events)
- Focus ring appearance (OS native vs custom)
- Animation approach (CSS vs GPUI transitions)

---

## Recommendations for Poodle

### Contract Template Structure

```markdown
# Component Name

## Purpose
One-sentence description.

## Anatomy
```
[diagram showing parts: root, label, input, etc.]
```

## Props/Inputs

### Appearance
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| variant | 'default' \| 'primary' | 'default' | Visual style |
| size | 'sm' \| 'md' \| 'lg' | 'md' | Control size |

### State
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| disabled | boolean | false | Non-interactive state |
| loading | boolean | false | Shows loading indicator |

### Events
| Prop | Type | Description |
|------|------|-------------|
| onClick | (e: ClickEvent) => void | Fired when activated |
| onFocus | (e: FocusEvent) => void | Fired when focused |

## States

### Visual States
- **Default** - Normal appearance
- **Hover** - Pointer over component
- **Focus** - Keyboard focused
- **Active/Pressed** - Being activated
- **Disabled** - Non-interactive
- **Loading** - Processing

### Component States
- State machine diagram (for complex components)
- State transition table

## Accessibility

### ARIA
- **Role**: button
- **Required**: aria-label (when no text)
- **Optional**: aria-describedby, aria-expanded

### Keyboard
| Key | Action |
|-----|--------|
| Enter/Space | Activate |
| Tab | Focus |

## Layout

### Sizing
- Min/max dimensions
- How it responds to container

### Spacing
- Internal padding
- Icon/text gaps

## Token Usage

| Element | Token |
|---------|-------|
| Background | semantic.color.background.primary |
| Text | semantic.color.text.primary |
| Border | semantic.color.border.default |

## Implementation Notes

### Svelte
- Bits usage (if applicable)
- Implementation specifics
- Known limitations

### GPUI
- GPUI patterns used
- Implementation specifics
- Known limitations

## Parity Checklist

- [ ] Same props have same semantic effect
- [ ] Same states exist
- [ ] Same events fire at same times
- [ ] Same keyboard behavior
- [ ] Same ARIA roles/attributes
- [ ] Same layout behavior

## Known Deltas

| Aspect | Svelte | GPUI | Rationale |
|--------|--------|------|-----------|
| Focus ring | CSS outline | GPUI native | Platform conventions |
| Animation | CSS transitions | GPUI transitions | Implementation detail |
```

### Parity Definition

**Three-Tier Parity Model:**

1. **Strict Parity** - Must match exactly
   - Semantic behavior
   - Accessibility
   - State transitions
   - Event timing

2. **Visual Parity** - Should match closely
   - Colors (via tokens)
   - Spacing (via tokens)
   - Typography (via tokens)
   - Overall proportions

3. **Implementation Freedom** - Can differ
   - Internal state management
   - Event handling details
   - Rendering approach
   - Animation mechanism

### Documentation Strategy

**Contract-First Development:**
1. Write contract document first
2. Get review/approval
3. Implement in Svelte
4. Implement in GPUI
5. Verify against parity checklist

**Per-Component Docs Location:**
- `docs/specs/components/button.md`
- `docs/specs/components/input.md`
- etc.

---

## Related

- Source hub: [hub-gpui](../source-hubs/hub-gpui.md) (GPUI capabilities affect parity)
- Source hub: [hub-bits](../source-hubs/hub-bits.md) (Bits affects Svelte implementation)
- Translation memo: [tm-contract-template](../translation-memos/tm-contract-template.md) (pending)
- Milestone: [g01.004](../../roadmaps/g01/004-component-contract-template-and-documentation-ia.md)

---

## Next Task

Create translation memo synthesizing contract template recommendations for g01.004.
