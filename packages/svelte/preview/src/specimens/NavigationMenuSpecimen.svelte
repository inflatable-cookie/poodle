<script lang="ts">
  import { NavigationMenu, type NavigationMenuItem } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const items: NavigationMenuItem[] = [
    { value: "home", label: "Home" },
    { value: "components", label: "Components" },
    { value: "tokens", label: "Tokens" },
    { value: "guides", label: "Guides" },
    { value: "changelog", label: "Changelog", disabled: true },
  ];

  let active = $state("components");
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

  <!-- The default trigger is borderless since g13.016; activeOutline opts
       the border back in. Solid fill covers the open trigger with
       accent-base + text-inverse, and must survive hover. -->
  <SpecimenGroup label="Navigation menu (active outline)">
    <NavigationMenu {items} value="components" activeOutline ariaLabel="Outlined main navigation" />
  </SpecimenGroup>

  <SpecimenGroup label="Navigation menu (solid fill)">
    <NavigationMenu {items} value="components" activeFill="solid" ariaLabel="Solid main navigation" />
  </SpecimenGroup>

  <SpecimenGroup label="Navigation menu (solid fill — hover the open trigger)">
    <div class="poodle-specimen__frame">
      <NavigationMenu {items} value="components" activeFill="solid" ariaLabel="Solid hovered main navigation" />
    </div>
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

  .poodle-specimen__frame {
    border: 1px dashed var(--poodle-color-border-subtle);
    padding: 0.5rem;
  }
</style>
