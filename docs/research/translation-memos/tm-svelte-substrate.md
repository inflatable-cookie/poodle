# Translation Memo: Svelte Substrate and Bits Integration

Status: draft
Created: 2026-03-11
Updated: 2026-03-11
Target: g01.005

## Summary

Pug's Svelte implementation will use Bits UI as an implementation substrate for headless primitives (accessibility, state management, event handling) while Pug maintains ownership of the public contract, token-based styling, and semantic component API. Bits UI is treated as an internal implementation detail, not a public dependency.

---

## Sources

| Source | Link | Relevant Findings |
|--------|------|-------------------|
| hub-bits | [../source-hubs/hub-bits.md](../source-hubs/hub-bits.md) | Bits provides 40+ primitives, compound component API, full styling control |
| tk-svelte-headless-patterns | [../value-tracks/tk-svelte-headless-patterns.md](../value-tracks/tk-svelte-headless-patterns.md) | Svelte headless patterns, Melt UI vs Bits |
| shadcn-svelte | https://shadcn-svelte.com/ | Reference implementation pattern using Bits |

---

## Decisions

### 1. Bits UI as Implementation Substrate

**Decision:** Pug will use Bits UI v1.0+ as the implementation substrate for Svelte components.

**Rationale:**
- Bits provides 40+ high-quality headless primitives
- Full accessibility (WAI-ARIA, keyboard, focus) built-in
- Svelte 5 native (uses runes)
- Full styling control via `class` and `data-*` attributes
- Active maintenance and community adoption (used by shadcn-svelte)
- No styling assumptions (truly headless)

**Scope of Bits Usage:**
- ✅ Accessibility (ARIA attributes, roles)
- ✅ Keyboard navigation
- ✅ Focus management
- ✅ State machines (controlled/uncontrolled)
- ✅ Event handling
- ✅ Portal/overlay management
- ✅ Form integration

**Out of Scope for Bits:**
- ❌ Layout primitives (Box, Stack, Grid) - Pug implements
- ❌ Styling (tokens, variants) - Pug implements
- ❌ Workstation components (dock, split) - Pug implements

### 2. Wrapper Component Pattern

**Decision:** Pug components will be wrappers around Bits primitives.

**Pattern Structure:**
```
Pug Component (public API)
    ↓ wraps
Bits UI Primitive (behavior/accessibility)
    ↓ applies
Pug Tokens (styling)
```

**Example:**
```svelte
<!-- PugButton.svelte -->
<script lang="ts">
  import { Button as BitsButton } from "bits-ui";
  import type { ButtonProps } from "./contract";
  
  let { 
    variant = "default", 
    size = "md",
    ...rest 
  }: ButtonProps = $props();
  
  // Pug defines variant/size mapping to tokens
  const classes = getTokenClasses({ variant, size, component: 'button' });
</script>

<BitsButton.Root class={classes} {...rest}>
  {@render rest.children?.()}
</BitsButton.Root>
```

**Benefits:**
- Pug owns public API contract
- Bits can be swapped without changing Pug API
- Token system fully controlled by Pug
- Pug-specific documentation

### 3. Public Contract Ownership

**Decision:** Pug defines and owns the public component contract, not Bits.

**Pug Owns:**
- Component props interface (naming, types, defaults)
- Semantic variants (primary, secondary, ghost)
- Control sizes (sm, md, lg)
- Event callback signatures
- Documentation and examples
- Parity requirements with GPUI

**Bits Provides (Implementation Detail):**
- Underlying behavior
- Accessibility implementation
- State management

**Consequences:**
- Pug props may differ from Bits props (intentionally)
- Pug adds semantic layer on top of Bits primitives
- Pug documentation doesn't reference Bits

### 4. Token Integration Strategy

**Decision:** Pug tokens applied via `class` props using CSS custom properties.

**Approach:**
```svelte
<BitsButton.Root 
  class="bg-[var(--pug-color-background-primary)] 
         text-[var(--pug-color-text-primary)]
         hover:bg-[var(--pug-color-background-hover)]"
>
```

**Alternative with Tailwind:**
```svelte
<!-- tailwind.config.ts references Pug tokens -->
<BitsButton.Root class="bg-background-primary text-text-primary">
```

**Token Application:**
- Pug components map semantic props to token classes
- Data attributes (`data-state`, `data-highlighted`) for state-based styling
- Pug provides `getTokenClasses()` utility for consistent mapping

### 5. Compound Component Exposure

**Decision:** Pug may expose compound component patterns where appropriate.

**Options:**

**Option A: Wrapped Compound (Recommended)**
```svelte
<script>
  import { Dialog } from "@pug/svelte";
</script>

<Dialog.Root>
  <Dialog.Trigger>Open</Dialog.Trigger>
  <Dialog.Content>
    <Dialog.Title>Title</Dialog.Title>
  </Dialog.Content>
</Dialog.Root>
```
- Pug exports wrapped compound components
- Full control over each part's styling
- Consistent with Bits/shadcn patterns

**Option B: Simplified Single Component**
```svelte
<Dialog trigger="Open" title="Title">Content</Dialog>
```
- Simpler API but less flexible
- May not work for complex components

**Decision:** Use Option A (compound) for complex components, Option B (simple) where appropriate.

### 6. Extension Limits

**Decision:** Document that Pug components are not directly extensible from Bits.

**Policy:**
- Apps should use Pug components, not Bits directly
- If Bits-level customization needed, use Bits directly (outside Pug)
- Pug doesn't expose Bits types or internals
- Pug component props are the extension point

**Rationale:**
- Maintains Pug's contract ownership
- Prevents accidental coupling to Bits
- Allows Pug to swap Bits in future if needed

### 7. Form Integration

**Decision:** Pug form components use Bits form primitives with Pug styling.

**Bits Form Primitives:**
- Checkbox
- Radio Group
- Select
- Slider
- Switch
- Date Picker / Date Field
- PIN Input

**Pug Adds:**
- Field wrappers with labels, error messages
- Validation state styling
- Form layout components (FormRow, FormGroup)
- Integration with Pug validation patterns

### 8. Version and Maintenance Policy

**Decision:** Bits UI updates absorbed internally; no breaking change exposure.

**Policy:**
- Pug specifies compatible Bits version in dependencies
- Minor Bits updates: absorb in Pug patch release
- Major Bits updates: evaluate impact, may require Pug major release
- Pug's public API remains stable regardless of Bits changes

**Current Recommendation:**
- Target Bits UI v1.0.x (stable, Svelte 5)
- Pin version in `package.json`
- Update after testing in g03 regression phase

---

## Action Items

- [ ] Add Bits UI v1.0.x as dependency in `svelte/package.json`
- [ ] Create wrapper component patterns in `svelte/src/components/`
- [ ] Implement token-to-class mapping utility
- [ ] Create first wrapped component (Button) as pattern example
- [ ] Document "Pug owns the contract" rule in architecture
- [ ] Update g01.005 milestone as complete

---

## Related

- Source hub: [hub-bits](../source-hubs/hub-bits.md)
- Translation memo: [tm-token-system](./tm-token-system.md) (token integration)
- Milestone: [g01.005](../../roadmaps/g01/005-svelte-substrate-and-bits-integration-policy.md)

---

## Appendix: Component Mapping

| Pug Component | Bits Primitive | Notes |
|---------------|----------------|-------|
| Button | Button | Direct wrapper |
| Checkbox | Checkbox | + label wrapper |
| Dialog | Dialog | Full compound |
| DropdownMenu | DropdownMenu | Full compound |
| Input | (implement) | Bits provides low-level, Pug adds styled input |
| Popover | Popover | Full compound |
| RadioGroup | RadioGroup | + label wrapper |
| Select | Select | Full compound |
| Slider | Slider | Full compound |
| Switch | Switch | + label wrapper |
| Tabs | Tabs | Full compound |
| Tooltip | Tooltip | Full compound |
| Calendar | Calendar | Pug adds styling |
| DatePicker | DatePicker | Composed primitive |
| Command | Command | Pug adds styling |
| Table | Table | Pug adds token styling |

---

## Appendix: Wrapper Template

```svelte
<!-- Template for Pug component wrapping Bits -->
<script lang="ts">
  import { Component as BitsComponent } from "bits-ui";
  import { cn } from "$lib/utils";
  import { getTokenClasses } from "$lib/tokens";
  import type { ComponentProps } from "./contract";
  
  let {
    variant = "default",
    size = "md",
    class: className,
    ...rest
  }: ComponentProps = $props();
  
  const baseClasses = getTokenClasses("component", { variant, size });
</script>

<BitsComponent.Root 
  class={cn(baseClasses, className)} 
  {...rest}
>
  {@render rest.children?.()}
</BitsComponent.Root>
```
