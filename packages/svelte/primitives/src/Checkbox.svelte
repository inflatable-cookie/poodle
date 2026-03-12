<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let id: string | undefined = undefined;
  export let isChecked = false;
  export let defaultChecked = false;
  export let isMixed = false;
  export let isDisabled = false;
  export let isReadOnly = false;
  export let label: string | null = null;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;

  const dispatch = createEventDispatcher<{
    checkedChange: { checked: boolean };
  }>();

  let input: HTMLInputElement | null = null;
  let uncontrolledChecked = defaultChecked;

  $: currentChecked = isChecked ?? uncontrolledChecked;
  $: if (input) {
    input.indeterminate = isMixed;
  }

  function handleChange(event: Event): void {
    const nextChecked = (event.currentTarget as HTMLInputElement).checked;

    if (isReadOnly) {
      (event.currentTarget as HTMLInputElement).checked = currentChecked;
      return;
    }

    uncontrolledChecked = nextChecked;
    dispatch("checkedChange", { checked: nextChecked });
  }
</script>

<label class="checkbox" data-disabled={isDisabled}>
  <input
    bind:this={input}
    {id}
    class="checkbox__control"
    type="checkbox"
    checked={currentChecked}
    disabled={isDisabled}
    aria-label={label ? undefined : ariaLabel ?? undefined}
    aria-describedby={describedBy ?? undefined}
    aria-readonly={isReadOnly ? "true" : undefined}
    on:change={handleChange}
  />
  <span class="checkbox__indicator" aria-hidden="true">
    {#if isMixed}
      <span class="checkbox__mark">−</span>
    {:else if currentChecked}
      <span class="checkbox__mark">✓</span>
    {/if}
  </span>
  {#if label}
    <span class="checkbox__label">{label}</span>
  {/if}
</label>

<style>
  .checkbox {
    display: inline-flex;
    align-items: center;
    gap: var(--pug-space-inline-sm);
    color: var(--pug-color-text-primary);
    cursor: pointer;
  }

  .checkbox[data-disabled="true"] {
    opacity: var(--pug-state-opacity-disabled);
    cursor: not-allowed;
  }

  .checkbox__control {
    position: absolute;
    opacity: 0;
    pointer-events: none;
  }

  .checkbox__indicator {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 18px;
    height: 18px;
    border: 1px solid var(--pug-color-border-default);
    border-radius: 5px;
    background: var(--pug-color-background-surface);
    color: var(--pug-color-text-inverse);
  }

  .checkbox__control:checked + .checkbox__indicator,
  .checkbox__control:indeterminate + .checkbox__indicator {
    border-color: var(--pug-color-accent-base);
    background: var(--pug-color-accent-base);
  }

  .checkbox__control:focus-visible + .checkbox__indicator {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 2px;
  }

  .checkbox__mark {
    font-family: var(--pug-typography-code-family);
    font-size: 12px;
    line-height: 1;
  }

  .checkbox__label {
    font-family: var(--pug-typography-label-family);
    font-size: var(--pug-typography-label-size);
    font-weight: var(--pug-typography-label-weight);
    line-height: var(--pug-typography-label-lineHeight);
  }
</style>
