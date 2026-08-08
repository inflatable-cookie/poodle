<script lang="ts">
  import "@poodle/styles/sidebar-nav.css";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  import type { SidebarNavGroup, SidebarNavItem } from "./types";

  interface Props {
    groups?: SidebarNavGroup[];
    value?: string | null;
    ariaLabel?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onValueChange?: ((value: string) => void) | undefined;
  }

  let {
    groups = [],
    value = $bindable<string | null>(null),
    ariaLabel = null,
    size = null,
    sizeRole = "chrome",
    density = null,
    onValueChange = undefined,
  }: Props = $props();

  const visibleGroups = $derived(groups.filter((group) => group.items.length > 0));

  function handleItemActivation(item: SidebarNavItem): void {
    if (item.disabled) return;
    value = item.value;
    onValueChange?.(item.value);
  }
</script>

<nav
  class="poodle-sidebar-nav"
  data-size={size ?? undefined}
  data-density={density ?? undefined}
  data-size-role={sizeRole}
  aria-label={ariaLabel ?? undefined}
>
  {#each visibleGroups as group (group.id)}
    <section
      class="poodle-sidebar-nav__group"
      data-separated={visibleGroups.length > 1}
      aria-label={group.label ?? undefined}
    >
      {#if group.label}
        <h2 class="poodle-sidebar-nav__group-title">{group.label}</h2>
      {/if}

      <ul class="poodle-sidebar-nav__list">
        {#each group.items as item (item.value)}
          <li>
            {#if item.href && !item.disabled}
              <a
                class="poodle-sidebar-nav__item"
                class:poodle-sidebar-nav__item--active={item.value === value}
                href={item.href}
                aria-current={item.value === value ? "page" : undefined}
                onclick={() => handleItemActivation(item)}
              >
                {item.label}
              </a>
            {:else}
              <button
                type="button"
                class="poodle-sidebar-nav__item"
                class:poodle-sidebar-nav__item--active={item.value === value}
                aria-current={item.value === value ? "page" : undefined}
                disabled={item.disabled}
                onclick={() => handleItemActivation(item)}
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

