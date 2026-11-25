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
2. Use `selectionTransition` in multiple mode with the tag ID as the key.
   Store its result across searches and filter chosen tags out of suggestions.
   Use `moveActiveOption` to navigate suggestions with the arrow keys and
   skip disabled rows. Enter or a click adds a tag, clears the query and
   keeps focus on the input. Backspace on an empty query removes the last
   tag. Each tag also has a named remove button.
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

## Behaviour supplied by core

- `moveActiveOption` tracks the active value through wrapping and disabled
  options, including lists with no enabled option. The recipe projects tag
  IDs into option values, clears the active value on a new query or close,
  and passes the active index to the DOM. Call `reconcileActiveOption` if
  your control refreshes a list while keeping navigation active.
- `selectionTransition` keeps chosen tag records across searches, rejects
  duplicate or disabled additions, and removes by tag ID. The recipe stores
  the selected records and uses them to hide already-chosen suggestions.
  `tokenBackspaceRemoves` checks whether Backspace can remove a tag.

## Behaviour still written in the recipe

| Piece | What the recipe supplies | Source |
| --- | --- | --- |
| Search lifecycle | Track loading, ready and error. Abort superseded or closed requests and ignore either outcome after abort. Remember arrow direction during loading, retry the same query and abort on unmount. The host supplies fetching and matching. | [Search and close](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L60-L85), [pending navigation and retry](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L122-L137), [unmount](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L165) |
| Keyboard and text input | Map arrows, Enter and Backspace to intents. Preserve ordinary cursor keys and browser editing shortcuts. Suppress commands, requests and dismissal while composing text, then search the committed text. | [Keyboard mapping](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L114-L147), [input and IME events](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L194-L206) |
| Focus and scroll | Keep DOM focus on the input during navigation and pointer selection. Return focus after tag removal or retry. Reveal the active row after the DOM updates. | [Tag mutations](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L98-L112), [active row scroll](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L159-L163), [retry](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L217-L227), [option pointer events](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L248-L249) |
| Overlay lifecycle | Bind open state and the two DOM roots to the core dismissal layer, pass its unregister function to Svelte, and close on focus leaving the input except when moving to retry. The Svelte action owns positioning, portalling and observation. | [Dismissal registration](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L149-L157), [blur and retry](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L211-L227), [anchoring](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L232-L240) |
| Accessible parts | Allocate instance, listbox and option IDs. Bind expanded, controls and active descendant to mounted suggestions, label the list and tag buttons, and project disabled and provisional selection on rows. | [IDs](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L25-L26), [option IDs](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L50), [label, tags and input](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L168-L191), [listbox and options](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L232-L252) |
| Announcements | Convert search outcomes and tag commits into messages and keep the live nodes mounted while their content changes. The application supplies copy. | [Search messages](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L52-L58), [tag messages](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L93-L112), [live regions](../../packages/svelte/preview/src/recipes/async-tag-combobox/TagCombobox.svelte#L216-L229) |
