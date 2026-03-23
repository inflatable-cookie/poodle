<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";

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
      <span class="checkbox__mark"><Icon name="minus" size="sm" /></span>
    {:else if currentChecked}
      <span class="checkbox__mark"><Icon name="check" size="sm" /></span>
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
    gap: var(--flint-space-inline-sm);
    color: var(--flint-color-text-primary);
    cursor: pointer;
  }

  .checkbox[data-disabled="true"] {
    opacity: var(--flint-state-opacity-disabled);
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
    width: calc(var(--flint-size-icon-default) + 0.125rem);
    height: calc(var(--flint-size-icon-default) + 0.125rem);
    border: 0.0625rem solid var(--flint-color-border-default);
    border-radius: 0.3125rem;
    background: var(--flint-color-background-surface);
    color: var(--flint-color-text-inverse);
  }

  .checkbox__control:checked + .checkbox__indicator,
  .checkbox__control:indeterminate + .checkbox__indicator {
    border-color: var(--flint-color-accent-base);
    background: var(--flint-color-accent-base);
  }

  .checkbox__control:focus-visible + .checkbox__indicator {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .checkbox__mark {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: calc(var(--flint-size-icon-default) - 0.125rem);
    height: calc(var(--flint-size-icon-default) - 0.125rem);
    line-height: 1;
  }

  .checkbox__label {
    font-family: var(--flint-typography-label-family);
    font-size: var(--flint-typography-label-size);
    font-weight: var(--flint-typography-label-weight);
    line-height: var(--flint-typography-label-lineHeight);
  }
</style>
