# Action Pattern Recipes

Reusable action-surface composition rules for Poodle-based Svelte apps.

## Purpose

Use this guide when contracts tell you what `Button`, `IconButton`,
`SplitButton`, or `Menu` do, but you need a stable answer for how to express
common app actions without rebuilding wrapper churn.

## Save Intent Actions

### Default posture

Poodle should not own a `SaveSplitButton` wrapper.

The useful generic behavior belongs in `SplitButton` itself. The actual action
vocabulary, such as “Save” and “Save and close”, belongs in app code.

### Recommended pattern

```svelte
<script lang="ts">
  import { SplitButton } from "@poodle/svelte-primitives";

  let intent = "save";

  const items = [
    { value: "save", label: "Save" },
    { value: "save-and-close", label: "Save and close" }
  ];

  function handleIntentChange(event: CustomEvent<{ value: string }>) {
    intent = event.detail.value;
  }
</script>

<input type="hidden" name="intent" value={intent} />

<SplitButton
  type="submit"
  variant="primary"
  items={items}
  on:action={handleIntentChange}
>
  {intent === "save-and-close" ? "Save and close" : "Save"}
</SplitButton>
```

### Decision

- keep submit semantics on the real button
- keep action labels and hidden form intent in app code
- improve `SplitButton` if a generic save-intent workflow feels awkward

## Action Menus

### Default posture

Do not promote app-specific action wrappers like `CopyActionsMenu` or
`EntityActionsMenu` into Poodle.

Those wrappers mostly encode app vocabulary, not reusable UI semantics.

### Recommended pattern

```svelte
<script lang="ts">
  import { Menu, Button } from "@poodle/svelte-primitives";

  const items = [
    { value: "duplicate", label: "Duplicate" },
    { value: "archive", label: "Archive" },
    { value: "delete", label: "Delete", tone: "danger" }
  ];

  function handleAction(event: CustomEvent<{ value: string }>) {
    const action = event.detail.value;
    // map to app-owned behavior here
  }
</script>

<Menu items={items} on:action={handleAction}>
  <Button variant="secondary">Actions</Button>
</Menu>
```

### Decision

- keep app action vocabularies out of Poodle
- improve `Menu`, confirm-dialog composition, or helper utilities if needed
- do not recreate convenience wrappers as canonical Poodle composites

## Grouped Icon Actions

### Default posture

For grouped page-level actions:

- primary action: `IconButton` with `variant="primary"`
- secondary sibling actions: `IconButton` with `variant="secondary"`
- toolbar utility actions: `IconButton` with `variant="ghost"` when they live
  inside `FilterToolbar`

### Decision

- do not use `ghost` for secondary clustered actions when they are peers of a
  primary add/create action
- use built-in `IconButton` tooltip behavior instead of wrapping it in another
  tooltip primitive

## Decision Rules

- If the pattern is generic and awkward, improve Poodle.
- If the pattern is domain vocabulary, keep it in app/shared code.
- If a wrapper only saves a few menu items or a hidden input, it probably does
  not belong in Poodle.
- If the same recipe appears in multiple apps, document it in a focused guide
  before adding new surface area.

## Related Contracts

- [Button](/Users/betterthanclay/Dev/projects/poodle/docs/contracts/foundation/button.md)
- [IconButton](/Users/betterthanclay/Dev/projects/poodle/docs/contracts/foundation/icon-button.md)
- [SplitButton](/Users/betterthanclay/Dev/projects/poodle/docs/contracts/foundation/split-button.md)
- [Menu](/Users/betterthanclay/Dev/projects/poodle/docs/contracts/foundation/menu.md)

## Next Task

Add the next action guide once the next reusable workflow hardens in real app
work, most likely dialog-trigger recipes and confirmation-action composition so
teams have a stable rule for destructive flows without recreating old Underlay
helper wrappers.
