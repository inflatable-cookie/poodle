# Source Hub: Bits UI

Status: complete (findings documented)
Created: 2026-03-11
Updated: 2026-03-11

## Purpose

Document Bits UI's primitives, extension limits, and API patterns to inform:
- Svelte substrate policy (g01.005)
- What Bits can accelerate vs. what Pug must own
- How Bits-backed components remain Pug-contract-compliant

---

## Source Inventory

### Official Sources

| Source | URL | Type | Last Checked |
|--------|-----|------|--------------|
| Bits UI Docs | https://bits-ui.com/docs | Official | 2026-03-11 |
| Getting Started | https://bits-ui.com/docs/getting-started | Official | 2026-03-11 |
| GitHub | https://github.com/huntabyte/bits-ui | Source | 2026-03-11 |

### Related Ecosystem

| Source | URL | Relationship | Notes |
|--------|-----|--------------|-------|
| shadcn-svelte | https://shadcn-svelte.com/ | Primary consumer | Built on top of Bits UI |
| Melt UI | https://melt-ui.com/ | Predecessor/inspiration | Bits UI v1 is successor to Melt UI |
| Radix UI | https://www.radix-ui.com/ | React equivalent | API design inspiration |

---

## Key Findings

### Overview

Bits UI is a **headless component library for Svelte 5** focused on:
- Developer experience
- Accessibility (WAI-ARIA compliance)
- Full creative control (bring your own styles)

**Key Philosophy:** Most components ship completely unstyled. You bring styles via standard `class` props or `data-*` attributes.

### Component Coverage

Bits UI provides **40+ headless primitives** organized into categories:

#### Layout & Structure
| Component | Parts | Description |
|-----------|-------|-------------|
| Accordion | Root, Item, Header, Trigger, Content | Collapsible sections |
| Collapsible | Root, Trigger, Content | Simple expand/collapse |
| Tabs | Root, List, Trigger, Content | Tabbed interface |
| Separator | Root | Visual divider |
| Scroll Area | Root, Viewport, Corner | Custom scrollable container |

#### Overlays
| Component | Parts | Description |
|-----------|-------|-------------|
| Dialog | Root, Trigger, Portal, Overlay, Content, Title, Description, Close | Modal dialogs |
| Alert Dialog | Root, Trigger, Portal, Overlay, Content, Title, Description, Action, Cancel | Confirmation dialogs |
| Popover | Root, Trigger, Portal, Content, Arrow | Floating content |
| Dropdown Menu | Root, Trigger, Portal, Content, Item, Group, Label, Separator, CheckboxItem, RadioGroup, RadioItem, Sub, SubTrigger, SubContent | Context menus |
| Context Menu | Same as Dropdown Menu | Right-click menus |
| Menubar | Root, Menu, Trigger, Portal, Content... | Horizontal menu bar |
| Tooltip | Root, Trigger, Portal, Content, Arrow | Hover information |
| Hover Card | Root, Trigger, Portal, Content | Preview on hover |
| Navigation Menu | Root, List, Item, Trigger, Content, Sub, Viewport, Indicator | Site navigation |

#### Forms & Inputs
| Component | Parts | Description |
|-----------|-------|-------------|
| Button | Root | Clickable actions |
| Checkbox | Root, Indicator | Binary selection |
| Switch | Root, Thumb, HiddenInput | Toggle on/off |
| Radio Group | Root, Item, HiddenInput | Single selection |
| Select | Root, Trigger, Portal, Content, Item, Group, Label, Separator, ScrollUpButton, ScrollDownButton | Dropdown selection |
| Slider | Root, Range, Thumb, Tick | Range selection |
| Label | Root | Form labels |
| Input | (through other components) | Text entry |

#### Data Display
| Component | Parts | Description |
|-----------|-------|-------------|
| Calendar | Root, Header, Heading, Grid, GridRow, GridHead, HeadCell, GridBody, Cell, Day | Date picker |
| Range Calendar | Same as Calendar | Date range selection |
| Date Picker | Composes DateField + Popover + Calendar | Full date input |
| Date Range Picker | Composes DateRangeField + Popover + RangeCalendar | Date range input |
| Table | Root, Header, Body, Footer, Row, Head, Cell, Caption | Data tables |
| Pagination | Root, PrevButton, NextButton, Item, Ellipsis | Page navigation |
| Meter | Root, Label, Value, ValueLabel | Progress indicator |
| Progress | Root, Label, Value, ValueLabel | Completion indicator |

#### Advanced Inputs
| Component | Parts | Description |
|-----------|-------|-------------|
| PIN Input | Root, HiddenInput, Cell, CellInput | OTP/2FA input |
| Combobox | Root, Input, Portal, Content, Item... | Autocomplete |
| Command | Root, Input, List, Empty, Group, Item, Separator | Command palette |
| Toggle | Root | Press/toggle button |
| Toggle Group | Root, Item | Group of toggles |
| Toolbar | Root, Group, Button, Link, Separator | Toolbars |
| Rating Group | Root, Item | Star ratings |

### API Patterns

#### Compound Component Pattern
All Bits UI components use **compound component pattern** (similar to Radix):

```svelte
<script>
  import { Dialog } from "bits-ui";
</script>

<Dialog.Root>
  <Dialog.Trigger>Open</Dialog.Trigger>
  <Dialog.Portal>
    <Dialog.Overlay />
    <Dialog.Content>
      <Dialog.Title>Title</Dialog.Title>
      <Dialog.Description>Description</Dialog.Description>
      <Dialog.Close>Close</Dialog.Close>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
```

#### Controlled vs Uncontrolled
Bits supports both patterns via Svelte 5 runes:

```svelte
<!-- Uncontrolled -->
<Dialog.Root>
  <Dialog.Trigger>Open</Dialog.Trigger>
  ...
</Dialog.Root>

<!-- Controlled -->
<script>
  let open = $state(false);
</script>
<Dialog.Root bind:open>
  <Dialog.Trigger>Open</Dialog.Trigger>
  ...
</Dialog.Root>

<!-- Function binding (fully controlled) -->
<Dialog.Root bind:open={() => open, (v) => open = v}>
```

#### Render Delegation
Bits uses Svelte 5's `child` snippet for complete rendering control:

```svelte
<Accordion.Content>
  {#snippet child({ props, open })}
    {#if open}
      <div {...props} transition:slide>
        Content here
      </div>
    {/if}
  {/snippet}
</Accordion.Content>
```

### Styling Integration

#### Class Props
All components accept `class` props for styling:

```svelte
<Button.Root class="bg-blue-500 text-white px-4 py-2 rounded">
  Click me
</Button.Root>
```

#### Data Attributes
Components expose state via `data-*` attributes for CSS targeting:

```svelte
<Switch.Root class="data-[state=checked]:bg-green-500">
  <Switch.Thumb class="data-[state=checked]:translate-x-4" />
</Switch.Root>
```

Common data attributes:
- `data-state` - "checked", "unchecked", "open", "closed", etc.
- `data-orientation` - "horizontal", "vertical"
- `data-side` - "top", "bottom", "left", "right"
- `data-align` - "start", "center", "end"
- `data-highlighted` - present when item is highlighted
- `data-disabled` - present when disabled

### Extension Model

#### What Can Be Extended
- **Styling**: Complete control via `class` and `data-*` attributes
- **Structure**: Can add wrapper elements, reorder parts
- **Behavior**: Event callbacks can be chained or overridden
- **Animation**: Full control via Svelte transitions and `child` snippet

#### Extension Limits
- **Component logic**: Encapsulated in Bits (cannot change internal state machine)
- **DOM structure**: Must maintain required parent-child relationships
- **Accessibility**: ARIA attributes managed by Bits (should not override)
- **Focus management**: Handled internally (should not interfere)

#### Creating Reusable Wrappers

Recommended pattern for design systems:

```svelte
<!-- MyButton.svelte -->
<script lang="ts">
  import { Button, type WithoutChildrenOrChild } from "bits-ui";
  import { cn } from "$lib/utils";
  
  type Props = WithoutChildrenOrChild<Button.RootProps> & {
    variant?: "default" | "primary" | "ghost";
    size?: "sm" | "md" | "lg";
  };
  
  let {
    variant = "default",
    size = "md",
    class: className,
    ...rest
  }: Props = $props();
  
  const variants = {
    default: "bg-white border border-gray-300",
    primary: "bg-blue-500 text-white",
    ghost: "bg-transparent hover:bg-gray-100"
  };
  
  const sizes = {
    sm: "px-2 py-1 text-sm",
    md: "px-4 py-2 text-base",
    lg: "px-6 py-3 text-lg"
  };
</script>

<Button.Root
  class={cn(
    "rounded font-medium transition-colors",
    variants[variant],
    sizes[size],
    className
  )}
  {...rest}
/>
```

### Token Integration Strategy

Bits UI components accept standard `class` props, making Pug token integration straightforward:

```svelte
<script>
  import { Button } from "bits-ui";
  import { tokens } from "@pug/tokens/svelte";
</script>

<Button.Root 
  class="bg-[var(--pug-color-background-primary)] text-[var(--pug-color-text-primary)]"
>
  Click me
</Button.Root>

<!-- Or with Tailwind + CSS variables -->
<Button.Root class="bg-background-primary text-text-primary">
  Click me
</Button.Root>
```

### Accessibility Features

Built-in accessibility (no extra work required):
- **WAI-ARIA compliance**: Proper roles, states, properties
- **Keyboard navigation**: Arrow keys, Tab, Enter, Escape, Space
- **Focus management**: Focus trapping in modals, focus restoration
- **Screen reader support**: Labels, descriptions, live regions
- **Focus visible**: Keyboard vs mouse focus distinction

Components handle:
- `aria-expanded`, `aria-controls`, `aria-labelledby`
- `aria-selected`, `aria-checked`, `aria-pressed`
- `aria-orientation`, `aria-haspopup`
- `aria-describedby`, `aria-errormessage`
- Focus rings (via `data-highlighted`)

### SSR Compatibility

Bits UI is fully SSR-compatible with SvelteKit:
- No hydration mismatches
- Proper server-side rendering
- No flash of unstyled content (when used with `class` props)

---

## Critical Questions Answered

### Primitive Coverage

**Q: What primitives does Bits provide?**
A: 40+ primitives covering: Accordion, Dialog, Dropdown Menu, Popover, Tooltip, Tabs, Calendar, Date Picker, Select, Slider, Switch, Checkbox, Radio Group, Table, Command, Navigation Menu, Menubar, Context Menu, Pagination, PIN Input, and more.

**Q: What primitives are missing?**
A: Based on Pug's architecture (001-pug-system-shape.md), Bits covers most Layer 1 primitives. Missing or limited:
- Layout primitives (Box, Stack, Grid) - Bits doesn't provide these
- Advanced workstation components (dock, split panel) - out of scope for Bits
- Some density/control-size variants may need custom handling

### Extension Model

**Q: How extensible are Bits components?**
A: Highly extensible for styling and composition. Limited for core behavior changes (by design).

**Q: Can Bits primitives be wrapped while preserving Pug semantics?**
A: Yes. Wrapper components can:
- Apply Pug token-based styling
- Add Pug-specific props (variant, size mapped to Pug tokens)
- Maintain Bits accessibility and behavior
- Document as Pug components

### API Patterns

**Q: What is Bits' component API pattern?**
A: Compound components (Root + subcomponents), Svelte 5 runes for state, `class` for styling, `child` snippet for render control.

**Q: How does Bits handle compound components?**
A: All complex components use compound pattern (Dialog.Root, Dialog.Trigger, etc.), enabling flexible composition.

### Token Integration

**Q: How can Bits components consume Pug tokens?**
A: Via standard `class` props with CSS custom properties or Tailwind classes that reference Pug tokens.

**Q: What styling hooks does Bits expose?**
A: `class` props on all components, `data-*` attributes for state-based styling, `child` snippet for complete render control.

### Maintenance & Risk

**Q: What is the maintenance status of Bits?**
A: Actively maintained by Hunter Johnston. Version 1.0 released for Svelte 5. Used by shadcn-svelte (major consumer).

**Q: How stable are the APIs?**
A: v1.0 is stable. Breaking changes from v0 (Melt UI era) have settled. Svelte 5 runes-based API is the future.

**Q: What is the migration path if Bits changes?**
A: Since Pug wraps Bits components (not exposes them directly), internal Bits changes can be absorbed in wrapper updates without affecting Pug's public API.

---

## Implications for Pug

### Bits Integration Policy

**Recommended Approach:**

1. **Bits is an implementation detail**
   - Pug components wrap Bits primitives
   - Pug defines the public contract
   - Bits can be swapped later if needed

2. **What Bits provides**
   - Accessibility (ARIA, keyboard, focus)
   - State management (controlled/uncontrolled)
   - Event handling
   - Portal/overlay management

3. **What Pug adds**
   - Token-based styling system
   - Semantic variant/size props mapped to tokens
   - Component-specific styling
   - Pug-specific documentation

### What Pug Must Own

- Component contract (props, behavior, events)
- Token application (how tokens map to component parts)
- Visual variants (primary, secondary, ghost → token mappings)
- Sizing (sm, md, lg → density/control-size tokens)
- Documentation and examples
- Parity with GPUI implementation

### What Bits Can Provide

- Accessibility compliance
- Keyboard navigation
- Focus management
- State machines
- Event handling
- Portal/overlay behavior
- Form integration (for input components)

### Wrapper Pattern Example

```svelte
<!-- PugButton.svelte - Pug's public Button -->
<script lang="ts">
  import { Button as BitsButton } from "bits-ui";
  import type { ButtonProps } from "./Button.contract";
  
  let { variant = "default", size = "md", ...rest }: ButtonProps = $props();
  
  // Map Pug variant/size to token-based classes
  const variantClasses = {
    default: "bg-[var(--pug-color-background-primary)]",
    primary: "bg-[var(--pug-color-accent)]",
    // ...
  };
</script>

<BitsButton.Root
  class="{variantClasses[variant]} {sizeClasses[size]}"
  {...rest}
/>
```

---

## Related

- Value track: [tk-svelte-headless-patterns](../value-tracks/tk-svelte-headless-patterns.md)
- Translation memo: [tm-svelte-substrate](../translation-memos/tm-svelte-substrate.md) (pending)
- Milestone: [g01.005](../../roadmaps/g01/005-svelte-substrate-and-bits-integration-policy.md)
- shadcn-svelte: https://shadcn-svelte.com/ (Bits consumer example)

---

## Next Task

Create translation memo synthesizing Svelte substrate policy recommendations for g01.005.
