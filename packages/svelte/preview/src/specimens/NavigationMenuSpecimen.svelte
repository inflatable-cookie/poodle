<script lang="ts">
  import { NavigationMenu, type NavigationMenuItem, type ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

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
  <SpecimenGroup label="Horizontal navigation">
    <NavigationMenu
      {items}
      value={active}
      ariaLabel="Main navigation"
      on:valueChange={(e) => { if (e.detail.value) active = e.detail.value; }}
    >
      <p>Active section: <strong>{active}</strong></p>
    </NavigationMenu>
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <NavigationMenu {items} value="components" ariaLabel={size + " navigation"} {size} />
      {/each}
    </div>
  </SpecimenGroup>
  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <NavigationMenu {items} value="components" ariaLabel="{density} navigation" {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }

  p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
