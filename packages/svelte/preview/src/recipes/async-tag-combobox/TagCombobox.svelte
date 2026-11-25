<script lang="ts">
  import {
    findNextEnabledIndex,
    firstEnabledIndex,
    layerContains,
    registerDismissLayer,
    tokenBackspaceRemoves,
  } from "@inflatable-cookie/poodle-core";
  import { anchored, Icon } from "@inflatable-cookie/poodle-svelte";

  type Tag = { id: string; label: string; disabled?: boolean };
  type SearchState =
    | { kind: "idle" }
    | { kind: "loading" }
    | { kind: "ready"; tags: readonly Tag[] }
    | { kind: "error" };

  let { searchTags }: {
    searchTags: (query: string, signal: AbortSignal) => Promise<readonly Tag[]>;
  } = $props();

  const inputId = $props.id();
  const listId = `${inputId}-options`;
  let root = $state<HTMLElement>();
  let input = $state<HTMLInputElement>();
  let listbox = $state<HTMLElement>();
  let retry = $state<HTMLButtonElement>();

  let query = $state("");
  let selected = $state.raw<Tag[]>([]);
  let result = $state.raw<SearchState>({ kind: "idle" });
  let activeId = $state<string | null>(null);
  let selectionMessage = $state("");
  let composing = false;
  let request: AbortController | undefined;
  let pendingDirection: 1 | -1 | undefined;

  const options = $derived(
    result.kind === "ready"
      ? result.tags.filter((tag) => !selected.some((chosen) => chosen.id === tag.id))
      : [],
  );
  const activeIndex = $derived(options.findIndex((tag) => tag.id === activeId));
  const expanded = $derived(options.length > 0);
  const searchOpen = $derived(result.kind !== "idle");
  const optionId = (index: number) => `${listId}-${index}`;

  const searchMessage = $derived.by(() => {
    if (result.kind === "loading") return "Searching for tags…";
    if (result.kind === "error") return "Could not load tags. Press Enter to retry.";
    if (result.kind === "idle") return "";
    if (!expanded) return "No matching tags.";
    return options.length === 1 ? "1 suggestion." : `${options.length} suggestions.`;
  });

  function moveActive(direction: 1 | -1) {
    const next = direction === 1 && activeIndex === -1
      ? firstEnabledIndex(options)
      : findNextEnabledIndex(options, Math.max(0, activeIndex), direction);
    const tag = options[next];
    activeId = tag && !tag.disabled ? tag.id : null;
  }

  async function search(direction?: 1 | -1) {
    request?.abort();
    const current = new AbortController();
    request = current;
    pendingDirection = direction;
    activeId = null;
    result = { kind: "loading" };

    try {
      const tags = await searchTags(query, current.signal);
      if (current.signal.aborted) return;
      result = { kind: "ready", tags };
      if (pendingDirection) moveActive(pendingDirection);
    } catch {
      if (current.signal.aborted) return;
      result = { kind: "error" };
    }
  }

  function close() {
    request?.abort();
    request = undefined;
    pendingDirection = undefined;
    activeId = null;
    result = { kind: "idle" };
  }

  function edit(value: string) {
    if (value === query) return;
    query = value;
    void search();
  }

  function announceSelection(action: "Added" | "Removed", tag: Tag) {
    const count = selected.length;
    selectionMessage = `${action} ${tag.label}. ${count} ${count === 1 ? "tag" : "tags"} selected.`;
  }

  function addTag(tag: Tag) {
    if (tag.disabled || selected.some((chosen) => chosen.id === tag.id)) return;
    selected = [...selected, tag];
    query = "";
    close();
    input?.focus();
    announceSelection("Added", tag);
  }

  function removeTag(tag: Tag) {
    selected = selected.filter((chosen) => chosen.id !== tag.id);
    input?.focus();
    announceSelection("Removed", tag);
  }

  function handleKeydown(event: KeyboardEvent) {
    if (composing || event.isComposing) {
      // The document's dismiss layer also sees Escape.
      if (event.key === "Escape") event.stopPropagation();
      return;
    }
    if (event.altKey || event.ctrlKey || event.metaKey || event.shiftKey) return;

    if (event.key === "ArrowDown" || event.key === "ArrowUp") {
      if (result.kind === "error" || (result.kind === "ready" && !expanded)) return;
      event.preventDefault();
      const direction = event.key === "ArrowDown" ? 1 : -1;
      if (result.kind === "idle") void search(direction);
      else if (result.kind === "loading") pendingDirection = direction;
      else moveActive(direction);
    } else if (event.key === "Enter") {
      const active = options[activeIndex];
      if (active) {
        event.preventDefault();
        addTag(active);
      } else if (result.kind === "error") {
        event.preventDefault();
        void search();
      }
    } else if (event.key === "Backspace" && tokenBackspaceRemoves(query, selected.length)) {
      const last = selected.at(-1);
      if (last) {
        event.preventDefault();
        removeTag(last);
      }
    } else if (activeId !== null && ["ArrowLeft", "ArrowRight", "Home", "End"].includes(event.key)) {
      activeId = null;
    }
  }

  $effect(() => {
    if (!searchOpen) return;
    return registerDismissLayer({
      hostElement: root,
      contains: (target) => layerContains(target, root, listbox),
      dismissOnOutsideInteract: true,
      onDismiss: close,
    });
  });

  $effect(() => {
    if (activeIndex >= 0) {
      listbox?.ownerDocument.getElementById(optionId(activeIndex))?.scrollIntoView({ block: "nearest" });
    }
  });

  $effect(() => () => request?.abort());
</script>

<div class="tag-combobox" bind:this={root}>
  <label for={inputId}>Tags</label>
  {#if selected.length}
    <!-- Keep list semantics when resetting markers in CSS. -->
    <ul class="tags" role="list" aria-label="Selected tags">
      {#each selected as tag (tag.id)}
        <li>
          <button type="button" class="tag" aria-label={`Remove ${tag.label}`} onclick={() => removeTag(tag)}>
            {tag.label}<Icon name="x" size="sm" />
          </button>
        </li>
      {/each}
    </ul>
  {/if}
  <input
    bind:this={input}
    id={inputId}
    type="text"
    role="combobox"
    autocomplete="off"
    aria-autocomplete="list"
    aria-expanded={expanded}
    aria-controls={expanded ? listId : undefined}
    aria-activedescendant={activeIndex >= 0 ? optionId(activeIndex) : undefined}
    placeholder="Search tags"
    value={query}
    oninput={(event) => {
      if (!composing && !(event instanceof InputEvent && event.isComposing)) edit(event.currentTarget.value);
    }}
    oncompositionstart={() => {
      composing = true;
      close();
    }}
    oncompositionend={(event) => {
      composing = false;
      const field = event.currentTarget;
      if (field === field.ownerDocument.activeElement) edit(field.value);
      else query = field.value;
    }}
    onkeydown={handleKeydown}
    onclick={() => {
      if (!composing && result.kind === "idle") void search();
    }}
    onblur={(event) => {
      if (event.relatedTarget !== retry) close();
    }}
  />

  <p role="status" class:visually-hidden={expanded || !searchMessage}>{searchMessage}</p>
  {#if result.kind === "error"}
    <button
      bind:this={retry}
      type="button"
      class="retry"
      onblur={close}
      onclick={() => {
        input?.focus();
        void search();
      }}
    >Try again</button>
  {/if}
  <p role="status" class="visually-hidden">{selectionMessage}</p>
</div>

{#if expanded}
  <ul
    bind:this={listbox}
    use:anchored={{ anchor: input ?? null, matchWidth: true }}
    id={listId}
    class="suggestions"
    role="listbox"
    aria-label="Tag suggestions"
  >
    {#each options as tag, index (tag.id)}
      <!-- svelte-ignore a11y_click_events_have_key_events (The combobox input handles option navigation and activation.) -->
      <li
        id={optionId(index)}
        role="option"
        aria-selected={index === activeIndex}
        aria-disabled={tag.disabled || undefined}
        onmousedown={(event) => event.preventDefault()}
        onclick={() => addTag(tag)}
      >{tag.label}</li>
    {/each}
  </ul>
{/if}

<style>
  .tag-combobox {
    display: grid;
    gap: var(--poodle-space-stack-sm);
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
  }

  label {
    font-weight: var(--poodle-typography-label-weight);
  }

  input {
    box-sizing: border-box;
    width: 100%;
    min-height: var(--poodle-size-control-height);
    padding: var(--poodle-space-control-y) var(--poodle-space-control-x);
    border: var(--poodle-border-width-default) solid var(--poodle-recipe-text-input-border, var(--poodle-color-border-default));
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-recipe-text-input-fill, var(--poodle-color-background-surface));
    color: inherit;
    font: inherit;
  }

  input::placeholder {
    color: var(--poodle-color-text-secondary);
  }

  :is(input, button):focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-recipe-text-input-border-focus, var(--poodle-color-accent-focusRing));
  }

  .tags, .suggestions {
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-xs);
  }

  .tag, .retry {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-xs);
    min-height: var(--poodle-size-control-xs);
    padding-inline: var(--poodle-space-inline-sm);
    border: var(--poodle-border-width-default) solid var(--poodle-recipe-pill-border, var(--poodle-color-border-default));
    border-radius: var(--poodle-radius-pill);
    background: var(--poodle-recipe-pill-fill, var(--poodle-color-background-panel));
    color: var(--poodle-recipe-pill-text, var(--poodle-color-text-primary));
    font: inherit;
    cursor: pointer;
  }

  .retry {
    justify-self: start;
  }

  .suggestions {
    box-sizing: border-box;
    z-index: var(--poodle-overlay-z-menu);
    max-height: 40dvh;
    overflow: auto;
    padding: var(--poodle-space-inline-xs);
    border: var(--poodle-border-width-default) solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-recipe-select-listbox-fill, var(--poodle-color-background-elevated));
    box-shadow: var(--poodle-recipe-select-listbox-shadow, var(--poodle-elevation-overlay));
    color: var(--poodle-recipe-select-option-text, var(--poodle-color-text-primary));
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
  }

  [role="option"] {
    padding: var(--poodle-space-control-y) var(--poodle-space-control-x);
    border-radius: var(--poodle-radius-control);
    overflow-wrap: anywhere;
    cursor: pointer;
  }

  [role="option"]:is([aria-selected="true"], :hover):not([aria-disabled="true"]) {
    background: var(--poodle-recipe-select-highlighted-hover-option-fill, var(--poodle-color-background-panel));
  }

  [role="option"][aria-selected="true"] {
    outline: var(--poodle-border-width-default) solid var(--poodle-color-accent-focusRing);
    outline-offset: calc(-1 * var(--poodle-border-width-default));
  }

  [aria-disabled="true"] {
    color: var(--poodle-color-text-secondary);
    cursor: default;
  }

  p {
    margin: 0;
  }

  .visually-hidden {
    position: absolute;
    inline-size: 1px;
    block-size: 1px;
    overflow: hidden;
    clip-path: inset(50%);
    white-space: nowrap;
  }
</style>
