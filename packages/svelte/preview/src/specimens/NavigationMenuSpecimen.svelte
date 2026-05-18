<script lang="ts">
  import { NavigationMenu, type NavigationMenuItem } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const items: NavigationMenuItem[] = [
    { value: "home", label: "Home" },
    { value: "components", label: "Components" },
    { value: "tokens", label: "Tokens" },
    { value: "guides", label: "Guides" },
    { value: "changelog", label: "Changelog", disabled: true },
  ];

  let active = "components";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Horizontal navigation">
    <NavigationMenu
      {items}
      value={active}
      ariaLabel="Main navigation"
      onValueChange={(value) => { if (value) active = value; }}
    >
      {#snippet children(activeValue)}
      <p>Active section: <strong>{active}</strong></p>
      {/snippet}
    </NavigationMenu>
  </SpecimenGroup>

  {#snippet sizes(size)}
    <NavigationMenu {items} value="components" {size} ariaLabel={size + " navigation"} />
  {/snippet}

  {#snippet densities(density)}
    <NavigationMenu {items} value="components" {density} ariaLabel={density + " navigation"} />
  {/snippet}
</SpecimenLayout>

<style>
  p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
