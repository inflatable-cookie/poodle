<script lang="ts">
  import { TriStateSwitch, type TriStateValue } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let filter: TriStateValue = $state("default");
</script>

<SpecimenLayout>
    <SpecimenGroup label="Default">
    <TriStateSwitch value={filter} ariaLabel="Filter mode" onValueChange={(value) => (filter = value)} />
        <span class="poodle-specimen__value">{filter}</span>
  </SpecimenGroup>

        <SpecimenGroup label="Custom labels">
    <TriStateSwitch options={{ excluded: "Hide", default: "All", included: "Show" }} ariaLabel="Visibility filter" />
  </SpecimenGroup>

        <SpecimenGroup label="Custom colors">
    <TriStateSwitch value={filter} excludedColor="#ef4444" defaultColor="#64748b" includedColor="#22c55e" ariaLabel="Custom colors" onValueChange={(value) => (filter = value)} />
  </SpecimenGroup>

        <SpecimenGroup label="Disabled">
    <TriStateSwitch value="included" disabled ariaLabel="Disabled" />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <TriStateSwitch value="default" {size} ariaLabel={"Switch at " + size} />
  {/snippet}

  {#snippet densities(density)}
    <TriStateSwitch value="default" {density} ariaLabel={"Switch at " + density} />
  {/snippet}
</SpecimenLayout>

<style>
    .poodle-specimen__item { display: flex; align-items: center; gap: 0.75rem; }
  .poodle-specimen__value { font-size: 0.75rem; color: var(--poodle-color-text-secondary); }
</style>
