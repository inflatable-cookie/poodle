# Flint Svelte Developer Guide

End-to-end implementation guide for building applications with the Flint design system's Svelte packages.

## Quick Start

### 1. Install packages

```bash
bun add @flint/svelte-tokens @flint/svelte-primitives @flint/svelte-composites @flint/icons-lucide
```

`@flint/svelte-tokens` provides the CSS custom properties and theme helpers, `@flint/svelte-primitives` provides the foundational UI components, `@flint/svelte-composites` provides higher-level compositions, and `@flint/icons-lucide` provides tree-shakeable icon imports.

### 2. Import the token stylesheet

In your app's entry point (e.g. `+layout.svelte` or `App.svelte`), import the token CSS:

```svelte
<script>
  import "@flint/svelte-tokens/styles.css";
</script>
```

If you prefer the legacy CSS subpath, that remains available too:

```css
@import "@flint/svelte-tokens/css/flint-tokens.css";
```

This loads the full set of CSS custom properties (`--flint-*`) that all components resolve their visual properties from.

### 3. Set theme attributes

Flint themes are activated via `data-*` attributes on a parent element. Use the runtime helper or set them manually:

```svelte
<script>
  import { applyThemeAttributes } from "@flint/svelte-tokens/runtime";
  import { onMount } from "svelte";

  let shell;

  onMount(() => {
    applyThemeAttributes(shell, {
      theme: "dark",          // "light" | "dark" | "loophole-studio"
      density: "compact",     // "comfortable" | "compact"
      controlSize: "md",      // "sm" | "md" | "lg"
    });
  });
</script>

<div bind:this={shell} data-theme="dark" data-density="compact" data-control-size="md">
  <!-- your app -->
</div>
```

Or simply apply the data attributes in your HTML:

```html
<div data-theme="dark" data-density="compact">
  <!-- everything inside inherits these theme tokens -->
</div>
```

### 4. Use components

```svelte
<script>
  import { Button, TextInput, Field, Select } from "@flint/svelte-primitives";
  import { search } from "@flint/icons-lucide";
</script>

<Button variant="primary" on:click={() => console.log("clicked")}>
  Save Changes
</Button>

<Button variant="secondary" leadingIcon={search}>
  Search
</Button>
```

---

## Package Architecture

```
@flint/svelte-tokens       — CSS custom properties, theme/density/size helpers
@flint/svelte-primitives    — 83 foundational UI components
@flint/svelte-composites    — 34 higher-level compositions built on primitives
@flint/icons-lucide         — 1700+ tree-shakeable Lucide icon exports
```

### Dependency graph

```
@flint/svelte-tokens
    ↑
@flint/svelte-primitives (depends on tokens)
    ↑
@flint/svelte-composites (depends on primitives, tokens)

@flint/icons-lucide (standalone — no dependency on other Flint packages)
```

### Import entry points

| Package | Import | Purpose |
|---------|--------|---------|
| `@flint/svelte-primitives` | Default | All components |
| `@flint/svelte-primitives/types` | Type-only | TypeScript types |
| `@flint/svelte-composites` | Default | All composites |
| `@flint/svelte-composites/types` | Type-only | TypeScript types |
| `@flint/svelte-tokens` | Default | Token values and metadata |
| `@flint/svelte-tokens/runtime` | `applyThemeAttributes()` | Theme attribute helper |
| `@flint/svelte-tokens/themes` | `themes`, `densityModes`, `controlSizes` | Theme definitions |
| `@flint/icons-lucide` | Named exports | Individual icon data |

---

## Design Token System

Every visual property in Flint resolves from CSS custom properties. Components never hardcode colors, spacing, radii, or typography — they reference `--flint-*` tokens.

### Token hierarchy

```
Primitive tokens (raw values)
  --flint-color-neutral-900: #131a22
  --flint-space-4: 1rem
  --flint-radius-md: 0.375rem

    ↓ resolved into

Semantic tokens (contextual purpose)
  --flint-color-text-primary: var(--flint-color-neutral-900)
  --flint-space-stack-md: 0.75rem
  --flint-radius-control: 0.375rem

    ↓ optionally overridden by

Treatment tokens (interactive-state styling)
  --flint-treatment-interactive-fill
  --flint-treatment-interactive-border
  --flint-treatment-interactive-radius
```

### Key semantic tokens

**Colors:**
```css
--flint-color-background-canvas      /* page background */
--flint-color-background-surface     /* card/input backgrounds */
--flint-color-background-panel       /* sidebar/panel backgrounds */
--flint-color-background-elevated    /* floating/elevated surfaces */
--flint-color-background-overlay     /* scrim behind modals */
--flint-color-text-primary           /* main text */
--flint-color-text-secondary         /* muted/helper text */
--flint-color-text-inverse           /* text on primary fills */
--flint-color-border-subtle          /* light dividers */
--flint-color-border-default         /* standard borders */
--flint-color-border-strong          /* emphasis borders */
--flint-color-accent-base            /* primary accent (buttons, links) */
--flint-color-accent-hover           /* accent hover state */
--flint-color-accent-focusRing       /* focus ring color */
--flint-color-status-success         /* success state */
--flint-color-status-warning         /* warning state */
--flint-color-status-danger          /* error/danger state */
--flint-color-icon-primary           /* icon default color */
--flint-color-icon-muted             /* icon muted color */
```

**Spacing:**
```css
--flint-space-stack-sm: 0.5rem       /* vertical gaps (small) */
--flint-space-stack-md: 0.75rem      /* vertical gaps (medium) */
--flint-space-stack-lg: 1.25rem      /* vertical gaps (large) */
--flint-space-inline-sm: 0.5rem      /* horizontal gaps (small) */
--flint-space-inline-md: 0.75rem     /* horizontal gaps (medium) */
--flint-space-inline-lg: 1rem        /* horizontal gaps (large) */
--flint-space-control-x: 0.75rem     /* control horizontal padding */
--flint-space-control-y: 0.5rem      /* control vertical padding */
--flint-space-panel-x: 1rem          /* panel horizontal padding */
--flint-space-panel-y: 0.75rem       /* panel vertical padding */
```

**Sizing:**
```css
--flint-size-control-height: 2.25rem /* default control height */
--flint-size-icon-sm: 0.75rem
--flint-size-icon-md: 1rem
--flint-size-icon-lg: 1.25rem
```

**Typography:**
```css
--flint-typography-body-family / -size / -lineHeight / -weight
--flint-typography-label-family / -size / -lineHeight / -weight
--flint-typography-heading-family / -size / -lineHeight / -weight
--flint-typography-code-family / -size / -lineHeight / -weight
```

**Radius, elevation, motion:**
```css
--flint-radius-control: 0.375rem
--flint-radius-surface: 0.625rem
--flint-radius-pill: 999rem
--flint-elevation-surface / -overlay / -dialog
--flint-motion-duration-interaction: 180ms
--flint-motion-easing-standard: cubic-bezier(0.2, 0, 0, 1)
--flint-state-opacity-disabled: 0.48
```

### Themes

Three built-in themes, activated via `data-theme` attribute:

| Theme | Description |
|-------|-------------|
| `light` | Default neutral light theme |
| `dark` | Dark neutral theme |
| `loophole-studio` | Custom branded dark theme |

Two density modes via `data-density`:

| Density | Description |
|---------|-------------|
| `comfortable` | Relaxed spacing (default) |
| `compact` | Tighter spacing for dense UIs |

Three control sizes via `data-control-size`:

| Size | Control height |
|------|---------------|
| `sm` | 1.75rem |
| `md` | 2.25rem (default) |
| `lg` | 2.75rem |

---

## Component Conventions

### Prop patterns

All components follow consistent naming conventions:

**State props** use `is` prefix:
```svelte
<TextInput isDisabled />
<TextInput isReadOnly />
<Button isLoading />
<Checkbox isMixed />
```

**Value props** follow the controlled/uncontrolled pattern:
```svelte
<!-- Controlled: parent owns state -->
<TextInput id="name" value={name} on:valueChange={(e) => name = e.detail.value} />

<!-- Uncontrolled: component owns state -->
<TextInput id="name" defaultValue="Alice" on:valueChange={(e) => console.log(e.detail.value)} />
```

The pattern works identically across all value-bearing components:
- `value` / `defaultValue` — text, select, radio
- `checked` / `defaultChecked` — checkboxes, switches
- When `value` is `null`, the component is uncontrolled. When it's a non-null value, the component is controlled.

**Presentation props:**
```svelte
<Button variant="primary" />     <!-- "primary" | "secondary" | "ghost" -->
<Button tone="danger" />         <!-- "default" | "danger" -->
<Button size="sm" />             <!-- "sm" | "md" | "lg" -->
<Stack direction="horizontal" /> <!-- layout orientation -->
```

**Accessibility props:**
```svelte
<IconButton icon={trash2} ariaLabel="Delete item" />
<TextInput id="email" describedBy="email-help" />
```

Every interactive component requires an `id` prop for form label association or an `ariaLabel` for icon-only controls.

### Event patterns

Components dispatch typed events via Svelte's `createEventDispatcher`:

```svelte
<TextInput
  id="email"
  on:valueChange={(e) => email = e.detail.value}
  on:submit={(e) => handleSubmit(e.detail.value)}
  on:cancel={() => resetForm()}
  on:focus={(e) => handleFocus(e.detail)}
  on:blur={(e) => handleBlur(e.detail)}
/>

<Select
  id="role"
  options={roleOptions}
  on:valueChange={(e) => role = e.detail.value}
/>

<Checkbox
  id="agree"
  on:checkedChange={(e) => agreed = e.detail.checked}
/>

<Dialog
  open={showDialog}
  on:openChange={(e) => showDialog = e.detail.open}
  on:requestClose={() => showDialog = false}
/>

<Button on:click={(e) => handleClick(e.detail)} />
```

**Naming conventions:**
- `valueChange` — value-bearing controls (TextInput, Select, RadioGroup)
- `checkedChange` — boolean toggles (Checkbox, Switch)
- `pressedChange` — toggle buttons (Toggle)
- `openChange` — overlays (Dialog, Drawer, Popover)
- `requestClose` — modal dismiss requests
- `submit` — TextInput on Enter
- `cancel` — TextInput on Escape
- `click`, `focus`, `blur` — standard DOM events

### Slot patterns

Components use named slots for flexible composition:

```svelte
<!-- Button: leading/trailing slots for icons or custom content -->
<Button variant="secondary">
  <Icon icon={download} slot="leading" />
  Download Report
  <Pill slot="trailing" tone="success" size="xxs">New</Pill>
</Button>

<!-- Field: default slot receives accessibility bindings -->
<Field id="email" label="Email" validationState="invalid" error="Required">
  <TextInput
    slot:let={{ describedBy, validationState }}
    id="email"
    {describedBy}
    {validationState}
  />
</Field>

<!-- Dialog: default slot for content, actions slot for buttons -->
<Dialog title="Confirm" open={show} on:requestClose={() => show = false}>
  <p>Are you sure?</p>
  <svelte:fragment slot="actions">
    <Button variant="ghost" on:click={() => show = false}>Cancel</Button>
    <Button variant="primary" tone="danger" on:click={handleDelete}>Delete</Button>
  </svelte:fragment>
</Dialog>
```

---

## Icons

Flint uses a layered icon system with three consumption patterns.

### Pattern 1: Direct import (tree-shakeable)

Import individual icons from `@flint/icons-lucide`. Only icons you use end up in the bundle:

```svelte
<script>
  import { Icon } from "@flint/svelte-primitives";
  import { search, heart, settings, trash2 } from "@flint/icons-lucide";
</script>

<Icon icon={search} size="md" />
<Icon icon={heart} size="sm" />
<Icon icon={settings} size="lg" ariaLabel="Settings" />
```

This is the **recommended approach** for application code. Each icon is a standalone `IconNodes` array — an `[tagName, attributes][]` tuple describing SVG children.

Icon names use camelCase identifiers converted from Lucide's kebab-case names:
- `arrow-down` → `arrowDown`
- `circle-check` → `circleCheck`
- `trash-2` → `trash2`
- JS reserved words get an `Icon` suffix: `delete` → `deleteIcon`

### Pattern 2: String names (built-in internals)

35 icons are embedded directly in the framework for component chrome. These work with string names and require no imports or setup:

```svelte
<Icon icon="chevron-down" size="sm" />
<Icon icon="check" />
<Icon icon="x" />
```

Built-in icons: `arrow-down`, `arrow-right`, `arrow-up`, `check`, `chevron-down`, `chevron-left`, `chevron-right`, `chevron-up`, `circle-alert`, `circle-check`, `circle-x`, `columns-3`, `download`, `ellipsis`, `ellipsis-vertical`, `external-link`, `file-text`, `grip-vertical`, `image`, `inbox`, `info`, `list-filter`, `loader`, `lock-open`, `minus`, `music`, `pencil`, `play`, `plus`, `search`, `star`, `trending-down`, `trending-up`, `triangle-alert`, `x`.

Legacy aliases are supported: `edit` → `pencil`, `filter` → `list-filter`, `more-horizontal` → `ellipsis`, etc.

### Pattern 3: Bulk icon set via IconProvider

For scenarios where you need the full icon catalogue available by name (e.g., CMS icon pickers, admin UIs):

```svelte
<script>
  import { Icon, IconProvider } from "@flint/svelte-primitives";
  import iconNodes from "lucide-static/icon-nodes.json";
</script>

<IconProvider icons={iconNodes}>
  <!-- All 1700+ Lucide icons available by string name -->
  <Icon icon="rocket" />
  <Icon icon="flame" />
  <Icon icon="shield-check" />
</IconProvider>
```

`IconProvider` accepts any `Record<string, IconNodes>` — it doesn't have to be Lucide. You can pass a Phosphor equivalent, a custom icon set, or a filtered subset.

String lookups check the `IconProvider` set first, then fall back to the 35 built-in internals.

### Using icons in components

Components that accept icons use the `IconProp` type (`IconNodes | string`):

```svelte
<script>
  import { Button, IconButton } from "@flint/svelte-primitives";
  import { save, trash2, plus } from "@flint/icons-lucide";
</script>

<Button variant="primary" leadingIcon={save}>Save</Button>
<Button variant="secondary" leadingIcon={plus}>Add Item</Button>
<IconButton icon={trash2} ariaLabel="Delete" tone="danger" variant="ghost" />

<!-- String names also work for built-in icons -->
<IconButton icon="search" ariaLabel="Search" variant="secondary" />
```

### Icon sizing

Icons support three sizes via the `size` prop:

| Size | Dimension |
|------|-----------|
| `sm` | 0.75rem (12px) |
| `md` | 1rem (16px) — default |
| `lg` | 1.25rem (20px) |

Icons inherit `currentColor` from their parent, so they automatically match text color.

---

## Forms

### Field + control pattern

The `Field` component handles labels, descriptions, hints, validation messages, and accessibility bindings. Wrap any form control in a `Field`:

```svelte
<script>
  import { Field, TextInput, Select, Checkbox } from "@flint/svelte-primitives";

  let name = "";
  let role = "";
  let agreed = false;
</script>

<Field id="name" label="Full Name" isRequired>
  <TextInput
    id="name"
    value={name}
    on:valueChange={(e) => name = e.detail.value}
    placeholder="Enter your name"
  />
</Field>

<Field
  id="role"
  label="Role"
  description="Select the user's primary role"
  validationState={role ? "valid" : "invalid"}
  error={role ? null : "Role is required"}
>
  <Select
    id="role"
    options={[
      { value: "admin", label: "Administrator" },
      { value: "editor", label: "Editor" },
      { value: "viewer", label: "Viewer" },
    ]}
    value={role}
    on:valueChange={(e) => role = e.detail.value}
  />
</Field>
```

The `Field` component:
- Renders the `<label>` associated with the control via `for={id}`
- Shows required indicator (`*`) when `isRequired` is true
- Shows "Optional" label when not required (configurable via `optionalLabel`)
- Shows description text below the label (always visible)
- Shows a `hint` tooltip via an info icon next to the label (progressive disclosure for longer help text)
- Shows error or pending messages based on `validationState`
- Provides `describedBy` to child controls via slot props for ARIA binding
- Supports grid layout via `span` and `gridArea` props

### Hints vs descriptions

Use `description` for always-visible help text that users need to see. Use `hint` for supplemental guidance that can be revealed on demand:

```svelte
<!-- Always-visible help -->
<Field id="pw" label="Password" description="Must be at least 8 characters." isRequired>
  <TextInput id="pw" type="password" />
</Field>

<!-- Progressive-disclosure hint -->
<Field id="slug" label="URL Slug" hint="Lowercase letters, numbers, and hyphens only.">
  <TextInput id="slug" />
</Field>

<!-- Both together -->
<Field id="key" label="API Key" description="Your personal key." hint="Rotate periodically for security." isRequired>
  <TextInput id="key" />
</Field>
```

### Validation states

```svelte
<Field id="email" label="Email" validationState="invalid" error="Invalid email format">
  <TextInput id="email" validationState="invalid" />
</Field>

<Field id="slug" label="Slug" validationState="pending" pendingMessage="Checking availability...">
  <TextInput id="slug" validationState="pending" />
</Field>

<Field id="username" label="Username" validationState="valid">
  <TextInput id="username" validationState="valid" />
</Field>
```

`ValidationState` values: `"none"` | `"invalid"` | `"valid"` | `"pending"`

### FormLayout (composite)

For multi-field forms, use `FormLayout` from composites:

```svelte
<script>
  import { FormLayout } from "@flint/svelte-composites";
  import { Field, TextInput, Select, Button } from "@flint/svelte-primitives";
</script>

<FormLayout columns={2}>
  <Field id="first" label="First Name" isRequired>
    <TextInput id="first" />
  </Field>
  <Field id="last" label="Last Name" isRequired>
    <TextInput id="last" />
  </Field>
  <Field id="email" label="Email" span="full">
    <TextInput id="email" type="email" />
  </Field>

  <svelte:fragment slot="actions">
    <Button variant="ghost">Cancel</Button>
    <Button variant="primary">Save</Button>
  </svelte:fragment>
</FormLayout>
```

`FormLayout` provides responsive grid layout (6-col desktop → 2-col tablet → 1-col mobile) and renders form-level error/success messages via `error` and `success` props.

### FieldSet (semantic grouping)

Use `FieldSet` to group related fields with a semantic `<fieldset>` and `<legend>`. Screen readers announce the legend as the group name:

```svelte
<script>
  import { FieldSet, Field, TextInput, Select } from "@flint/svelte-primitives";
</script>

<FieldSet legend="Contact Information">
  <Field id="name" label="Full Name" isRequired>
    <TextInput id="name" />
  </Field>
  <Field id="email" label="Email" isRequired>
    <TextInput id="email" type="email" />
  </Field>
</FieldSet>

<FieldSet legend="Address" columns={2}>
  <Field id="street" label="Street" span="full">
    <TextInput id="street" />
  </Field>
  <Field id="city" label="City">
    <TextInput id="city" />
  </Field>
  <Field id="state" label="State">
    <Select id="state" options={stateOptions} />
  </Field>
</FieldSet>
```

Props: `legend` (group label), `columns` (grid columns, default 1), `gap` (`SpaceScale`, default "md"), `span` (column span in parent grid). The legend is styled as an uppercase eyebrow. Child `Field` components can use `span="full"` to span all columns.

---

## Layout Components

### Stack

Flex container with direction and gap control:

```svelte
<script>
  import { Stack, Button } from "@flint/svelte-primitives";
</script>

<Stack direction="vertical" gap="md">
  <Button>First</Button>
  <Button>Second</Button>
  <Button>Third</Button>
</Stack>

<Stack direction="horizontal" gap="sm" align="center">
  <Icon icon={info} />
  <span>Aligned content</span>
</Stack>
```

### Grid

CSS Grid layout container:

```svelte
<script>
  import { Grid, Surface } from "@flint/svelte-primitives";
</script>

<Grid columns={3} gap="md">
  <Surface tone="panel">Cell 1</Surface>
  <Surface tone="panel">Cell 2</Surface>
  <Surface tone="panel">Cell 3</Surface>
</Grid>
```

### Surface

Themed container with elevation levels:

```svelte
<Surface tone="canvas">Page background</Surface>
<Surface tone="panel" border="subtle">Sidebar panel</Surface>
<Surface tone="elevated" border="default">Card or floating element</Surface>
```

### Box

Raw layout primitive with inline style control for margins, padding, and sizing.

### Spacer

Flex-aware spacer element that pushes siblings apart.

---

## Overlays

### Dialog

```svelte
<script>
  import { Dialog, Button } from "@flint/svelte-primitives";
  let open = false;
</script>

<Button on:click={() => open = true}>Open Dialog</Button>

<Dialog title="Confirm Action" {open} on:requestClose={() => open = false}>
  <p>This action cannot be undone. Are you sure?</p>
  <svelte:fragment slot="actions">
    <Button variant="ghost" on:click={() => open = false}>Cancel</Button>
    <Button variant="primary" tone="danger" on:click={handleConfirm}>Delete</Button>
  </svelte:fragment>
</Dialog>
```

### Drawer

Slide-out panel from a screen edge:

```svelte
<Drawer title="Settings" edge="right" open={showDrawer} on:requestClose={() => showDrawer = false}>
  <!-- drawer content -->
</Drawer>
```

`edge`: `"left"` | `"right"` | `"top"` | `"bottom"`

### Popover

Positioned floating content anchored to a trigger:

```svelte
<Popover placement="bottom-start">
  <Button slot="trigger">Options</Button>
  <div>Popover content here</div>
</Popover>
```

`placement`: `"top"` | `"top-start"` | `"top-end"` | `"bottom"` | `"bottom-start"` | `"bottom-end"` | `"left"` | `"right"` | etc.

### Tooltip

```svelte
<Tooltip content="Save your changes">
  <Button variant="primary">Save</Button>
</Tooltip>
```

---

## Data Display

### Table

```svelte
<script>
  import { Table } from "@flint/svelte-primitives";
  import type { TableColumn, TableRow } from "@flint/svelte-primitives";

  const columns: TableColumn[] = [
    { id: "name", label: "Name", isRowHeader: true },
    { id: "role", label: "Role" },
    { id: "status", label: "Status", align: "end" },
  ];

  const rows: TableRow[] = [
    { id: "1", cells: { name: "Alice", role: "Admin", status: "Active" } },
    { id: "2", cells: { name: "Bob", role: "Editor", status: "Inactive" } },
  ];
</script>

<Table {columns} {rows} />
```

### DataTable (composite)

Full-featured data table with sorting, column visibility, bulk actions, and export:

```svelte
<script>
  import { DataTable } from "@flint/svelte-composites";
  import type { TableColumn, TableRow } from "@flint/svelte-composites";
</script>

<DataTable
  {columns}
  {rows}
  selectable
  sortable
  on:sortChange={(e) => handleSort(e.detail)}
  on:selectionChange={(e) => handleSelection(e.detail)}
/>
```

### Tabs

```svelte
<script>
  import { Tabs } from "@flint/svelte-primitives";
  import type { TabItem } from "@flint/svelte-primitives";

  const tabs: TabItem[] = [
    { value: "general", label: "General" },
    { value: "security", label: "Security" },
    { value: "billing", label: "Billing", icon: "star" },
  ];

  let activeTab = "general";
</script>

<Tabs
  items={tabs}
  value={activeTab}
  variant="underline"
  on:valueChange={(e) => activeTab = e.detail.value}
/>
```

Tab variants: `"underline"` | `"card"` | `"pill"` | `"strip"`

---

## Navigation

### Breadcrumbs

```svelte
<script>
  import { Breadcrumbs } from "@flint/svelte-primitives";
  import type { BreadcrumbItem } from "@flint/svelte-primitives";

  const items: BreadcrumbItem[] = [
    { value: "home", label: "Home", href: "/" },
    { value: "users", label: "Users", href: "/users" },
    { value: "alice", label: "Alice Johnson", isCurrent: true },
  ];
</script>

<Breadcrumbs {items} />
```

### Pagination

```svelte
<script>
  import { Pagination } from "@flint/svelte-primitives";
  let page = 1;
</script>

<Pagination
  totalPages={20}
  currentPage={page}
  on:pageChange={(e) => page = e.detail.page}
/>
```

### Menu

```svelte
<script>
  import { Menu, Button } from "@flint/svelte-primitives";
  import type { MenuItem } from "@flint/svelte-primitives";

  const items: MenuItem[] = [
    { value: "edit", label: "Edit", shortcutLabel: "⌘E" },
    { value: "duplicate", label: "Duplicate" },
    { value: "sep", label: "", kind: "separator" },
    { value: "delete", label: "Delete", kind: "action" },
  ];
</script>

<Menu {items} on:select={(e) => handleAction(e.detail.value)}>
  <Button slot="trigger" variant="ghost">Actions</Button>
</Menu>
```

---

## Feedback Components

### Callout

```svelte
<script>
  import { Callout } from "@flint/svelte-primitives";
</script>

<Callout tone="info">This is an informational message.</Callout>
<Callout tone="warning">Check your settings before continuing.</Callout>
<Callout tone="danger" isDismissible>Something went wrong.</Callout>
<Callout tone="success">Changes saved successfully.</Callout>
```

### StatusIndicator

```svelte
<StatusIndicator tone="success" label="Online" />
<StatusIndicator tone="danger" label="Offline" />
<StatusIndicator tone="pending" label="Syncing" />
```

### Progress

```svelte
<Progress value={65} max={100} />
```

### ToastStack (composite)

```svelte
<script>
  import { ToastStack } from "@flint/svelte-composites";
  import type { ToastItem } from "@flint/svelte-composites";

  let toasts: ToastItem[] = [];

  function addToast() {
    toasts = [...toasts, {
      id: crypto.randomUUID(),
      title: "Saved",
      message: "Your changes have been saved.",
      tone: "success",
    }];
  }
</script>

<ToastStack {toasts} on:dismiss={(e) => toasts = toasts.filter(t => t.id !== e.detail.id)} />
```

---

## Selection Controls

### Checkbox

```svelte
<Checkbox
  id="terms"
  label="I agree to the terms"
  checked={agreed}
  on:checkedChange={(e) => agreed = e.detail.checked}
/>
```

### Switch

```svelte
<Switch
  id="notifications"
  label="Enable notifications"
  checked={enabled}
  on:checkedChange={(e) => enabled = e.detail.checked}
/>
```

### RadioGroup

```svelte
<RadioGroup
  id="plan"
  options={[
    { value: "free", label: "Free" },
    { value: "pro", label: "Pro" },
    { value: "enterprise", label: "Enterprise", isDisabled: true },
  ]}
  value={plan}
  on:valueChange={(e) => plan = e.detail.value}
/>
```

### SegmentedControl

```svelte
<SegmentedControl
  id="view"
  options={[
    { value: "grid", label: "Grid" },
    { value: "list", label: "List" },
  ]}
  value={view}
  on:valueChange={(e) => view = e.detail.value}
/>
```

### Select

```svelte
<Select
  id="country"
  options={[
    { value: "us", label: "United States" },
    { value: "uk", label: "United Kingdom" },
    { value: "ca", label: "Canada" },
  ]}
  value={country}
  placeholder="Select a country"
  on:valueChange={(e) => country = e.detail.value}
/>
```

Grouped options:

```svelte
<Select
  id="tz"
  options={[
    {
      label: "Americas",
      options: [
        { value: "est", label: "Eastern" },
        { value: "pst", label: "Pacific" },
      ],
    },
    {
      label: "Europe",
      options: [
        { value: "gmt", label: "GMT" },
        { value: "cet", label: "CET" },
      ],
    },
  ]}
/>
```

### Combobox

Searchable select with type-ahead:

```svelte
<Combobox
  id="assignee"
  options={users}
  value={assignee}
  placeholder="Search users..."
  on:valueChange={(e) => assignee = e.detail.value}
/>
```

---

## Composite Patterns

### PageHeader + DetailShell

```svelte
<script>
  import { PageHeader, DetailShell, DetailSection, DetailRow } from "@flint/svelte-composites";
  import { Breadcrumbs } from "@flint/svelte-primitives";
</script>

<PageHeader title="User Details">
  <Breadcrumbs slot="breadcrumbs" items={breadcrumbItems} />
</PageHeader>

<DetailShell>
  <DetailSection title="Personal Information">
    <DetailRow label="Name">{user.name}</DetailRow>
    <DetailRow label="Email">{user.email}</DetailRow>
    <DetailRow label="Role">{user.role}</DetailRow>
  </DetailSection>
</DetailShell>
```

### EmptyState

```svelte
<EmptyState
  variant="firstRun"
  title="No projects yet"
  description="Create your first project to get started."
>
  <Button variant="primary" slot="action">Create Project</Button>
</EmptyState>
```

### CommandPalette

```svelte
<script>
  import { CommandPalette } from "@flint/svelte-composites";
  import type { CommandActionItem } from "@flint/svelte-composites";

  let open = false;
  const actions: CommandActionItem[] = [
    { id: "new", title: "New Document", shortcut: "⌘N", group: "File" },
    { id: "open", title: "Open...", shortcut: "⌘O", group: "File" },
    { id: "settings", title: "Settings", shortcut: "⌘,", group: "App" },
  ];
</script>

<CommandPalette {open} {actions} on:select={(e) => handleAction(e.detail)} on:close={() => open = false} />
```

---

## TypeScript Types

### Importing types

```typescript
// Primitive types
import type {
  ButtonVariant,
  ButtonTone,
  ControlSize,
  ValidationState,
  IconProp,
  SelectOption,
  TabItem,
  MenuItem,
  RadioGroupOption,
  DateRangeValue,
  DateTimeValue,
  OverlayPlacement,
} from "@flint/svelte-primitives";

// Composite types
import type {
  TableColumn,
  TableRow,
  ToastItem,
  CommandActionItem,
  WorkspaceLayoutSnapshot,
  DockEdge,
  PanelTabItem,
} from "@flint/svelte-composites";

// Icon types
import type { IconNodes, IconNodeElement, IconSet } from "@flint/svelte-primitives";
```

### Key type definitions

```typescript
type ControlSize = "sm" | "md" | "lg";
type ButtonVariant = "primary" | "secondary" | "ghost";
type ButtonTone = "default" | "danger";
type ValidationState = "none" | "invalid" | "valid" | "pending";
type StatusTone = "neutral" | "info" | "success" | "warning" | "danger" | "pending";
type IconProp = IconNodes | string;      // direct data or string name
type IconNodes = IconNodeElement[];       // [tag, attrs][]
type IconNodeElement = [string, Record<string, string>];

interface SelectOption {
  value: string;
  label: string;
  isDisabled?: boolean;
  group?: string;
}

interface TabItem {
  value: string;
  label: string;
  icon?: IconProp;
  isDisabled?: boolean;
  isClosable?: boolean;
}

interface MenuItem {
  value: string;
  label: string;
  isDisabled?: boolean;
  isChecked?: boolean;
  shortcutLabel?: string;
  kind?: "action" | "checkbox" | "radio" | "separator";
}
```

---

## Common Recipes

### Themed page layout

```svelte
<script>
  import { Surface, Stack } from "@flint/svelte-primitives";
</script>

<div data-theme="dark" data-density="compact">
  <Surface tone="canvas" style="min-height: 100vh; padding: var(--flint-space-panel-y) var(--flint-space-panel-x)">
    <Stack gap="lg">
      <!-- page content -->
    </Stack>
  </Surface>
</div>
```

### Form with validation

```svelte
<script>
  import { Field, TextInput, Select, Button, Stack } from "@flint/svelte-primitives";
  import { FormLayout } from "@flint/svelte-composites";

  let name = "";
  let email = "";
  let submitted = false;

  $: nameError = submitted && !name ? "Name is required" : null;
  $: emailError = submitted && !email ? "Email is required" : null;
</script>

<FormLayout columns={1}>
  <Field
    id="name"
    label="Name"
    isRequired
    validationState={nameError ? "invalid" : "none"}
    error={nameError}
  >
    <TextInput
      id="name"
      value={name}
      validationState={nameError ? "invalid" : "none"}
      on:valueChange={(e) => name = e.detail.value}
    />
  </Field>

  <Field
    id="email"
    label="Email"
    isRequired
    validationState={emailError ? "invalid" : "none"}
    error={emailError}
  >
    <TextInput
      id="email"
      type="email"
      value={email}
      validationState={emailError ? "invalid" : "none"}
      on:valueChange={(e) => email = e.detail.value}
    />
  </Field>

  <svelte:fragment slot="actions">
    <Button variant="primary" on:click={() => { submitted = true; }}>Submit</Button>
  </svelte:fragment>
</FormLayout>
```

### Icon button toolbar

```svelte
<script>
  import { Toolbar, IconButton, Separator } from "@flint/svelte-primitives";
  import { bold, italic, underline, link, image } from "@flint/icons-lucide";
</script>

<Toolbar>
  <IconButton icon={bold} ariaLabel="Bold" />
  <IconButton icon={italic} ariaLabel="Italic" />
  <IconButton icon={underline} ariaLabel="Underline" />
  <Separator orientation="vertical" />
  <IconButton icon={link} ariaLabel="Insert Link" />
  <IconButton icon={image} ariaLabel="Insert Image" />
</Toolbar>
```

### Confirmation dialog

```svelte
<script>
  import { AlertDialog, Button } from "@flint/svelte-primitives";
  let showConfirm = false;
</script>

<Button tone="danger" on:click={() => showConfirm = true}>Delete Account</Button>

<AlertDialog
  open={showConfirm}
  tone="danger"
  title="Delete Account"
  description="This will permanently delete your account and all associated data. This action cannot be undone."
  confirmLabel="Delete"
  cancelLabel="Cancel"
  on:confirm={handleDelete}
  on:cancel={() => showConfirm = false}
/>
```

---

## Full Component Reference

### Primitives (83 components)

**Action:** Button, IconButton, SplitButton
**Input:** TextInput, TextArea, NumberEntry, SearchField, PinInput, DurationInput, ColorPicker
**Selection:** Checkbox, Switch, RadioGroup, Select, Combobox, SegmentedControl, Toggle, ToggleGroup, TriStateSwitch, Slider, RangeSlider, Rating
**Date/Time:** Calendar, RangeCalendar, DatePicker, DateRangePicker, DateTimePicker, DateTimeRangePicker, ZonedDateTimePicker, TimeField, TimeZoneSelect
**Layout:** Box, Grid, Stack, Spacer, Separator, Surface, Region, ScrollShell
**Navigation:** Breadcrumbs, Pagination, PaginationSummary, Tabs, Menu, Menubar, NavigationMenu
**Overlay:** Dialog, AlertDialog, Drawer, Popover, HoverCard, Tooltip, ContextMenu
**Feedback:** Callout, Progress, Meter, Skeleton, StatusIndicator
**Display:** Accordion, Card, Code, Pill, Icon, IconButton, IconProvider, Table, EditableLabel, Eyebrow, ListCard, ListCardCounter, NavCard, NavCardGrid, FormActions, Field, Collapsible, CollapseToggle, FileUpload, BulkActionBar, OrderBy, ResizeHandle, StatusBar, Toolbar

### Composites (34 components)

**Data:** DataTable, FilterToolbar, SelectionSummary, PaginationSummary
**Detail:** DetailShell, DetailSection, DetailRow
**Forms:** FormLayout, FormDialog, ConfirmAction, SlugField, EmbedInput
**Browse:** PageHeader, EmptyState, PageLoading
**Media:** MediaThumbnail, MediaPreview, MediaPicker, AudioPlayer, VideoPlayer, EmbedPreview
**Editor:** MarkdownEditor, BlockEditor, EditableList, ReorderableList
**Picker:** PickerShell, RelationPicker, CardRadioGroup
**Layout:** SplitView, DockRegion, AppHeader, StatusBar, ResizeHandle
**Feedback:** ToastStack, LogList
**Discovery:** CommandPalette, ActionDiscoveryPanel
**Metric:** MetricTile

---

## Running the Preview App

The interactive component preview lives at `packages/svelte/preview`:

```bash
bun install
bun run tokens:build
cd packages/svelte/preview
bun run dev
```

Open `http://localhost:4173`. The preview app shows every component specimen with theme/density/size controls.

---

## Contracts

Every component has a contract document in `docs/contracts/foundation/<component>.md` that defines:

- **Anatomy** — which DOM parts exist and their nesting
- **Props** — every prop, its type, and default value
- **Token targets** — which semantic token controls each visual property
- **States** — hover, active, focus, disabled, loading behavior
- **Accessibility** — ARIA attributes, roles, keyboard behavior
- **Sizing** — exact dimensions from tokens

When implementing new components or extending existing ones, always read the contract first. The Svelte implementation is the reference implementation — if the contract is ambiguous, the Svelte code is authoritative.
