# Build an async tag combobox from core behaviour

Use this recipe when users need to find and add several existing tags. The
[standalone preview example](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte)
uses the [supported core exports](../contracts/core-behaviour.md) and the
public Svelte [`anchored` action](../contracts/002-anchored-overlays.md).
Run `effigy svelte:preview` and open
`http://localhost:4173/async-tag-combobox.html`. Type `error` to see a failed
search, or enter text that matches no tags to see the empty state.

## Build it in an app

1. Render an editable input and a listbox of suggestions. Supply
   `searchTags(query, signal)` from your application. Return tags with stable,
   unique IDs, display labels and an optional `disabled` boolean. Parse service
   responses before handing them to the recipe and pass the signal to your
   request. Each search may return an entirely different set of tags.
2. Store chosen tags separately and filter them out of suggestions by ID.
   Arrow keys navigate suggestions and skip disabled rows. Enter or a click
   adds the active or clicked tag, clears the query and keeps focus on the
   input. Backspace on an empty query removes the last tag. Each tag also has a
   named remove button.
3. Use the [editable combobox pattern](https://www.w3.org/WAI/ARIA/apg/patterns/combobox/).
   Keep DOM focus on the input and expose the provisional option via
   `aria-activedescendant`. Keep `aria-controls` pointed at the mounted
   listbox. Each opening offers one addition, and the committed tags live in
   their own labeled list. A persistent status region announces loading,
   results, empty and error states. A second region announces tag changes.
   Escape and outside interaction dismiss the search without committing a tag.
4. Import base and theme styles as shown in the
   [example entry](../../packages/svelte/preview/src/recipes/async-tag-combobox/main.ts).
   The recipe uses semantic tokens for metrics and reads the public
   `text-input`, `pill` and `select`
   [appearance hooks](../knowledge/architecture/007-appearance-recipe-contract.md)
   with token defaults. Set overrides on the theme root when they must also
   reach the portalled listbox.

## Behaviour still written in the recipe

| Piece | What the recipe supplies | Source |
| --- | --- | --- |
| Active option | Keep an active ID, project it into the current unchosen results, seed first or last on open, and clear it on query changes and close. Core supplies enabled-index traversal; the recipe guards its all-disabled return. | [Active state and navigation](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L29-L62), [search and close](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L64-L89) |
| Persistent tag selection | Store tag records outside the result set, hide chosen IDs, guard duplicate or disabled additions, and remove by ID while preserving order. Core supplies only the empty-input Backspace predicate here. | [Suggestion projection](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L38-L42), [add and remove](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L102-L115), [Backspace](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L141-L145) |
| Search lifecycle | Project loading, ready and error; abort superseded or closed requests and ignore either outcome after abort. Remember arrow direction during loading, retry the same query and abort on unmount. The host supplies fetching and matching. | [Search and close](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L64-L89), [pending navigation and retry](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L125-L140), [unmount](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L168) |
| Keyboard and text input | Map arrows, Enter and Backspace to intents; preserve ordinary cursor keys and browser editing shortcuts. Suppress commands, requests and dismissal while composing text, then search the committed text. | [Keyboard mapping](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L117-L150), [input and IME events](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L197-L210) |
| Focus and scroll | Keep DOM focus on the input during navigation and pointer selection. Return focus after tag removal or retry. Reveal the active row after the DOM updates. | [Tag mutations](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L102-L115), [active row scroll](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L162-L166), [retry](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L221-L230), [option pointer events](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L251-L252) |
| Overlay lifecycle | Bind open state and the two DOM roots to the core dismissal layer, pass its unregister function to Svelte, and close on focus leaving the input except when moving to retry. The Svelte action owns positioning, portalling and observation. | [Dismissal registration](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L152-L160), [blur and retry](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L214-L230), [anchoring](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L235-L243) |
| Accessible parts | Allocate instance, listbox and option IDs; bind expanded, controls and active descendant to mounted suggestions; label the list and tag buttons, and project disabled and provisional selection on rows. | [IDs](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L22-L23), [option IDs](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L46), [label, tags and input](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L171-L194), [listbox and options](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L235-L255) |
| Announcements | Convert search outcomes and tag commits into messages and keep the live nodes mounted while their content changes. The application supplies copy. | [Search messages](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L48-L54), [tag messages](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L97-L115), [live regions](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L219-L232) |

The active-option and persistent-selection rules are portable policy candidates.
Key events, focus, ARIA and overlay lifecycle belong in bindings. Search
transport and tag vocabulary belong to the application.
