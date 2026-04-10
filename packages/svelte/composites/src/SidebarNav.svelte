<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import {
    type ControlDensity,
    type ControlSize,
    type SemanticControlSizeRole,
  } from "@poodle/svelte-primitives";

  import type { SidebarNavGroup, SidebarNavItem } from "./types";

  export let groups: SidebarNavGroup[] = [];
  export let value: string | null = null;
  export let ariaLabel: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "chrome";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
  }>();

  $: visibleGroups = groups.filter((group) => group.items.length > 0);

  function handleItemActivation(item: SidebarNavItem): void {
    if (item.disabled) return;
    dispatch("valueChange", { value: item.value });
  }
</script>

<nav
  class="sidebar-nav"
  data-size={size ?? undefined}
  data-density={density ?? undefined}
  data-size-role={sizeRole}
  aria-label={ariaLabel ?? undefined}
>
  {#each visibleGroups as group (group.id)}
    <section
      class="sidebar-nav__group"
      data-separated={visibleGroups.length > 1}
      aria-label={group.label ?? undefined}
    >
      {#if group.label}
        <h2 class="sidebar-nav__group-title">{group.label}</h2>
      {/if}

      <ul class="sidebar-nav__list">
        {#each group.items as item (item.value)}
          <li>
            {#if item.href && !item.disabled}
              <a
                class="sidebar-nav__item"
                class:sidebar-nav__item--active={item.value === value}
                href={item.href}
                aria-current={item.value === value ? "page" : undefined}
                on:click={() => handleItemActivation(item)}
              >
                {item.label}
              </a>
            {:else}
              <button
                type="button"
                class="sidebar-nav__item"
                class:sidebar-nav__item--active={item.value === value}
                aria-current={item.value === value ? "page" : undefined}
                disabled={item.disabled}
                on:click={() => handleItemActivation(item)}
              >
                {item.label}
              </button>
            {/if}
          </li>
        {/each}
      </ul>
    </section>
  {/each}
</nav>

<style>
  .sidebar-nav {
    --poodle-sidebar-nav-item-height: 1.875rem;
    --poodle-sidebar-nav-group-gap: var(--poodle-space-panel-y);
    --poodle-sidebar-nav-item-padding-inline: var(--poodle-space-control-x);
    --poodle-sidebar-nav-item-padding-block: 0.375rem;
    --poodle-sidebar-nav-item-font-size: var(--poodle-typography-label-size);
    --poodle-sidebar-nav-title-font-size: calc(var(--poodle-typography-label-size) * 0.75);
    --poodle-sidebar-nav-title-letter-spacing: 0.18em;
    --poodle-sidebar-nav-title-gap: calc(var(--poodle-space-panel-y) * 0.375);
    display: grid;
    gap: var(--poodle-sidebar-nav-group-gap);
    min-width: 0;
    align-content: start;
    padding: var(--poodle-space-panel-y) 0.375rem;
  }

  .sidebar-nav[data-size="xs"] {
    --poodle-sidebar-nav-item-height: 1.375rem;
    --poodle-sidebar-nav-item-font-size: 0.6875rem;
    --poodle-sidebar-nav-title-font-size: 0.46875rem;
  }

  .sidebar-nav[data-size="sm"] {
    --poodle-sidebar-nav-item-height: 1.625rem;
    --poodle-sidebar-nav-item-font-size: 0.75rem;
    --poodle-sidebar-nav-title-font-size: 0.5rem;
  }

  .sidebar-nav[data-size="md"] {
    --poodle-sidebar-nav-item-height: 1.875rem;
    --poodle-sidebar-nav-item-font-size: 0.8125rem;
    --poodle-sidebar-nav-title-font-size: 0.5625rem;
  }

  .sidebar-nav[data-size="lg"] {
    --poodle-sidebar-nav-item-height: 2.125rem;
    --poodle-sidebar-nav-item-font-size: 0.875rem;
    --poodle-sidebar-nav-title-font-size: 0.59375rem;
  }

  .sidebar-nav[data-size="xl"] {
    --poodle-sidebar-nav-item-height: 2.375rem;
    --poodle-sidebar-nav-item-font-size: 0.9375rem;
    --poodle-sidebar-nav-title-font-size: 0.625rem;
  }

  .sidebar-nav[data-density="compact"] {
    --poodle-sidebar-nav-group-gap: 0.625rem;
    --poodle-sidebar-nav-item-padding-inline: 0.5rem;
    --poodle-sidebar-nav-item-padding-block: 0.3125rem;
    --poodle-sidebar-nav-title-letter-spacing: 0.2em;
    --poodle-sidebar-nav-title-gap: 0.125rem;
  }

  .sidebar-nav[data-density="default"] {
    --poodle-sidebar-nav-group-gap: 0.75rem;
    --poodle-sidebar-nav-item-padding-inline: 0.75rem;
    --poodle-sidebar-nav-item-padding-block: 0.375rem;
    --poodle-sidebar-nav-title-letter-spacing: 0.18em;
    --poodle-sidebar-nav-title-gap: 0.1875rem;
  }

  .sidebar-nav[data-density="comfortable"] {
    --poodle-sidebar-nav-group-gap: 0.875rem;
    --poodle-sidebar-nav-item-padding-inline: 0.875rem;
    --poodle-sidebar-nav-item-padding-block: 0.4375rem;
    --poodle-sidebar-nav-title-letter-spacing: 0.16em;
    --poodle-sidebar-nav-title-gap: 0.25rem;
  }

  .sidebar-nav__group {
    display: grid;
    gap: 0.3125rem;
    min-width: 0;
  }

  .sidebar-nav__group[data-separated="true"] + .sidebar-nav__group {
    margin-top: 0.125rem;
    padding-top: calc(var(--poodle-sidebar-nav-group-gap) - 0.125rem);
    border-top: 0.0625rem solid
      color-mix(in srgb, var(--poodle-color-border-subtle) 54%, transparent);
  }

  .sidebar-nav__group-title {
    margin: 0;
    padding: 0 var(--poodle-sidebar-nav-item-padding-inline) var(--poodle-sidebar-nav-title-gap);
    color: var(--poodle-color-accent-base);
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-sidebar-nav-title-font-size);
    font-weight: 700;
    letter-spacing: var(--poodle-sidebar-nav-title-letter-spacing);
    line-height: 1.2;
    text-transform: uppercase;
  }

  .sidebar-nav__list {
    display: grid;
    gap: 0.125rem;
    min-width: 0;
    list-style: none;
    margin: 0;
    padding: 0;
  }

  .sidebar-nav__item {
    position: relative;
    display: block;
    width: 100%;
    min-width: 0;
    min-height: var(--poodle-sidebar-nav-item-height);
    padding: var(--poodle-sidebar-nav-item-padding-block) var(--poodle-sidebar-nav-item-padding-inline);
    border: 0;
    border-left: 0.1875rem solid transparent;
    border-radius: 0.2rem calc(var(--poodle-radius-control) - 0.125rem) calc(var(--poodle-radius-control) - 0.125rem) 0.2rem;
    background: transparent;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-sidebar-nav-item-font-size);
    font-weight: 500;
    line-height: 1.3;
    text-align: left;
    text-decoration: none;
    cursor: pointer;
    transition:
      color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }


  .sidebar-nav__item:hover:not(:disabled) {
    color: var(--poodle-color-text-primary);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 60%, transparent);
  }

  .sidebar-nav__item--active {
    color: var(--poodle-color-text-primary);
    font-weight: 600;
    background: color-mix(in srgb, var(--poodle-color-accent-base) 10%, transparent);
    box-shadow: inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-accent-base) 20%, transparent);
  }

  .sidebar-nav__item--active {
    border-left-color: var(--poodle-color-accent-base);
  }

  .sidebar-nav__item:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .sidebar-nav__item:disabled {
    opacity: var(--poodle-state-opacity-disabled);
    cursor: not-allowed;
  }
</style>
