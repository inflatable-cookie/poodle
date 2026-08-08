# Select — a searchable select renders empty when given a value

Date: 2026-07-27
From: Jetstream thread (editor inspector)
Component: `packages/svelte/components/src/Select.svelte`

## Symptom

```svelte
<Select value="builtin/cube" options={[{value: 'builtin/cube', label: 'builtin/cube'}]} searchable />
```

renders an empty input showing the placeholder. The value is set, the option
exists, and nothing is displayed. Choosing from the menu fixes it until the
component is remounted.

Non-searchable is fine — the trigger renders `selectedOption`.

## Cause

In `searchable` mode the input's displayed text is `query` (line ~378), and
`query` is:

```ts
let query = $state("");
```

It is assigned in exactly two places: when an option is chosen (`query =
option.label`) and when the user types. Nothing seeds it from `value` on mount,
so a controlled `Select` starts blank.

## Fix

Seed it from the selected option, and keep it in step when `value` changes from
outside:

```ts
let query = $state("");
$effect(() => {
  if (!open) query = selectedOption?.label ?? "";
});
```

Guarding on `!open` matters — otherwise typing a filter would be overwritten by
the current selection on every keystroke.

## Worth a test

`render(Select, {value, options, searchable})` then asserting the input's value
is the selected label. The current behaviour passes any test that only checks
the component mounts.

## What we did meanwhile

Dropped `searchable` and appended the current value to `options` when the
catalog does not contain it, so a key a scene names but the catalog has lost
still displays (marked `(missing)`).
