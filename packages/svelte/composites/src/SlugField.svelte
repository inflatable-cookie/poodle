<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { TextInput, Field } from "@flint/svelte-primitives";

  import type { ValidationState } from "@flint/svelte-primitives";

  export let id: string;
  export let label = "Slug";
  export let source = "";
  export let value = "";
  export let placeholder = "auto-generated-slug";
  export let isManualOverride = false;
  export let isDisabled = false;
  export let validationState: ValidationState = "none";
  export let error: string | null = null;
  export let description: string | null = "URL-safe identifier auto-generated from the title.";
  export let maxLength: number | null = null;
  export let prefix: string | null = null;

  const dispatch = createEventDispatcher<{
    change: { value: string; isManual: boolean };
  }>();

  $: autoSlug = slugify(source);
  $: if (!isManualOverride && autoSlug !== value) {
    value = autoSlug;
  }
  $: displayPrefix = prefix ?? "/";

  function slugify(text: string): string {
    return text
      .toLowerCase()
      .trim()
      .replace(/[^\w\s-]/g, "")
      .replace(/[\s_]+/g, "-")
      .replace(/-+/g, "-")
      .replace(/^-|-$/g, "");
  }

  function handleValueChange(event: CustomEvent<{ value: string }>): void {
    const raw = event.detail.value;
    const slugged = raw
      .toLowerCase()
      .replace(/[^\w\s-]/g, "")
      .replace(/\s+/g, "-");

    if (slugged !== value) {
      isManualOverride = true;
      value = slugged;
      dispatch("change", { value: slugged, isManual: true });
    }
  }

  function resetToAuto(): void {
    isManualOverride = false;
    value = autoSlug;
    dispatch("change", { value: autoSlug, isManual: false });
  }
</script>

<div class="slug-field">
  <Field
    {id}
    {label}
    {description}
    {error}
    {validationState}
  >
    <svelte:fragment let:describedBy>
      <div class="slug-field__row">
        <TextInput
          {id}
          value={value}
          {placeholder}
          prefix={displayPrefix}
          isDisabled={isDisabled}
          {validationState}
          {describedBy}
          {maxLength}
          on:valueChange={handleValueChange}
        />
        {#if isManualOverride}
          <button
            type="button"
            class="slug-field__reset"
            on:click={resetToAuto}
            aria-label="Reset to auto-generated slug"
          >
            Reset
          </button>
        {/if}
      </div>
    </svelte:fragment>
  </Field>

  {#if isManualOverride}
    <p class="slug-field__hint">
      Manual override active. Auto-slug would be: <code>{autoSlug || "—"}</code>
    </p>
  {/if}
</div>

<style>
  .slug-field {
    display: grid;
    gap: var(--flint-space-stack-sm);
  }

  .slug-field__row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .slug-field__row :global(.text-input) {
    flex: 1;
  }

  .slug-field__reset {
    display: inline-flex;
    align-items: center;
    height: var(--flint-size-control-height);
    padding: 0 0.625rem;
    border: 0.0625rem solid var(--flint-color-border-default);
    border-radius: var(--flint-radius-control);
    background: var(--flint-color-background-surface);
    color: var(--flint-color-text-secondary);
    cursor: pointer;
    font-family: var(--flint-typography-label-family);
    font-size: 0.75rem;
    font-weight: var(--flint-typography-label-weight);
    white-space: nowrap;
    transition:
      background var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      border-color var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard);
  }

  .slug-field__reset:hover {
    background: color-mix(in srgb, var(--flint-color-background-surface) 84%, var(--flint-color-background-elevated));
    border-color: color-mix(in srgb, var(--flint-color-border-default) 78%, var(--flint-color-text-primary));
  }

  .slug-field__reset:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .slug-field__hint {
    margin: 0;
    font-size: 0.75rem;
    color: var(--flint-color-text-secondary);
  }

  .slug-field__hint code {
    font-family: var(--flint-typography-code-family);
    font-size: 0.6875rem;
    padding: 0.0625rem 0.25rem;
    border-radius: 0.1875rem;
    background: color-mix(in srgb, var(--flint-color-background-panel) 72%, var(--flint-color-background-elevated));
  }
</style>
