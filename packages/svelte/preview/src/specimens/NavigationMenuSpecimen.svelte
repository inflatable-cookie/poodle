<script lang="ts">
  import { NavigationMenu, Eyebrow, type NavigationMenuItem } from "@poodle/svelte-primitives";

  const items: NavigationMenuItem[] = [
    { value: "home", label: "Home" },
    { value: "components", label: "Components" },
    { value: "tokens", label: "Tokens" },
    { value: "guides", label: "Guides" },
    { value: "changelog", label: "Changelog", disabled: true },
  ];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let active = "components";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Horizontal navigation</Eyebrow>
    <NavigationMenu
      {items}
      value={active}
      ariaLabel="Main navigation"
      on:valueChange={(e) => { if (e.detail.value) active = e.detail.value; }}
    >
      <p>Active section: <strong>{active}</strong></p>
    </NavigationMenu>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <NavigationMenu {items} value="components" ariaLabel={size + " navigation"} {size} />
      {/each}
    </div>
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
