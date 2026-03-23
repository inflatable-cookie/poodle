<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let id: string | undefined = undefined;
  export let isChecked: boolean | null = null;
  export let defaultChecked = false;
  export let isDisabled = false;
  export let isReadOnly = false;
  export let label: string | null = null;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let name: string | undefined = undefined;

  const dispatch = createEventDispatcher<{
    checkedChange: { checked: boolean };
  }>();

  let uncontrolledChecked = defaultChecked;

  $: isControlled = isChecked !== null;
  $: currentChecked = isControlled ? isChecked === true : uncontrolledChecked;

  function handleChange(event: Event): void {
    const nextChecked = (event.currentTarget as HTMLInputElement).checked;

    if (isReadOnly) {
      (event.currentTarget as HTMLInputElement).checked = currentChecked;
      return;
    }

    if (!isControlled) {
      uncontrolledChecked = nextChecked;
    }

    dispatch("checkedChange", { checked: nextChecked });
  }
</script>

<label class="switch" data-disabled={isDisabled} data-read-only={isReadOnly}>
  <input
    {id}
    {name}
    class="switch__control"
    type="checkbox"
    role="switch"
    checked={currentChecked}
    disabled={isDisabled}
    aria-label={label ? undefined : ariaLabel ?? undefined}
    aria-describedby={describedBy ?? undefined}
    aria-readonly={isReadOnly ? "true" : undefined}
    on:change={handleChange}
  />
  <span class="switch__track" aria-hidden="true">
    <span class="switch__thumb"></span>
  </span>
  {#if label}
    <span class="switch__label">{label}</span>
  {/if}
</label>

<style>
  .switch {
    display: inline-flex;
    align-items: center;
    gap: var(--flint-space-inline-sm);
    color: var(--flint-color-text-primary);
    cursor: pointer;
  }

  .switch[data-disabled="true"] {
    cursor: not-allowed;
    opacity: var(--flint-state-opacity-disabled);
  }

  .switch[data-read-only="true"] {
    cursor: default;
  }

  .switch__control {
    position: absolute;
    opacity: 0;
    pointer-events: none;
  }

  .switch__track {
    display: inline-flex;
    align-items: center;
    width: calc(var(--flint-size-icon-default) * 2 + 0.125rem);
    height: calc(var(--flint-size-icon-default) + 0.25rem);
    padding: 0.125rem;
    border: 0.0625rem solid var(--flint-color-border-default);
    border-radius: 999px;
    background: color-mix(in srgb, var(--flint-color-background-surface) 86%, transparent);
    box-shadow: inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent);
    transition:
      background var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      border-color var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      box-shadow var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard);
  }

  .switch__thumb {
    width: calc(var(--flint-size-icon-default) - 0.125rem);
    height: calc(var(--flint-size-icon-default) - 0.125rem);
    border-radius: 999px;
    background: var(--flint-color-text-primary);
    box-shadow: 0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
    transform: translateX(0);
    transition:
      transform var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      background var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard);
  }

  .switch__control:checked + .switch__track {
    border-color: color-mix(in srgb, var(--flint-color-accent-base) 58%, var(--flint-color-border-default));
    background: color-mix(in srgb, var(--flint-color-accent-base) 24%, var(--flint-color-background-surface));
  }

  .switch__control:checked + .switch__track .switch__thumb {
    background: var(--flint-color-accent-base);
    transform: translateX(calc(var(--flint-size-icon-default) - 0.125rem));
  }

  .switch__control:focus-visible + .switch__track {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .switch__label {
    font-family: var(--flint-typography-label-family);
    font-size: var(--flint-typography-label-size);
    font-weight: var(--flint-typography-label-weight);
    line-height: var(--flint-typography-label-lineHeight);
  }
</style>
