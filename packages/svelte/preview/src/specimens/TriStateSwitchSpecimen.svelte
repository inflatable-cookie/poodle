<script lang="ts">
  import { TriStateSwitch, Eyebrow, Surface, type TriStateValue } from "@poodle/svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let filter: TriStateValue = "default";
</script>

<SpecimenLayout>
  <Surface tone="panel" border="subtle" padding="md">
    <div class="poodle-specimen">
      <div class="poodle-specimen__item">
        <Eyebrow>Default</Eyebrow>
        <TriStateSwitch value={filter} ariaLabel="Filter mode" onValueChange={(value) => (filter = value)} />
        <span class="poodle-specimen__value">{filter}</span>
      </div>

      <div class="poodle-specimen__item">
        <Eyebrow>Custom labels</Eyebrow>
        <TriStateSwitch options={{ excluded: "Hide", default: "All", included: "Show" }} ariaLabel="Visibility filter" />
      </div>

      <div class="poodle-specimen__item">
        <Eyebrow>Custom colors</Eyebrow>
        <TriStateSwitch value={filter} excludedColor="#ef4444" defaultColor="#64748b" includedColor="#22c55e" ariaLabel="Custom colors" onValueChange={(value) => (filter = value)} />
      </div>

      <div class="poodle-specimen__item">
        <Eyebrow>Disabled</Eyebrow>
        <TriStateSwitch value="included" disabled ariaLabel="Disabled" />
      </div>
    </div>
  </Surface>

  <svelte:fragment slot="sizes" let:size>
    <TriStateSwitch value="default" {size} ariaLabel={"Switch at " + size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <TriStateSwitch value="default" {density} ariaLabel={"Switch at " + density} />
  </svelte:fragment>
</SpecimenLayout>

<style>
  .poodle-specimen { display: flex; flex-direction: column; gap: 0.75rem; }
  .poodle-specimen__item { display: flex; align-items: center; gap: 0.75rem; }
  .poodle-specimen__value { font-size: 0.75rem; color: var(--poodle-color-text-secondary); }
</style>
