<script lang="ts">
  import { ModMatrixGrid } from "@inflatable-cookie/poodle-svelte";
  import type { ModMatrixCell, ModMatrixHeader } from "@inflatable-cookie/poodle-core";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  const sources: ModMatrixHeader[] = [{ id: "one", label: "Source 1" }, { id: "two", label: "Source 2" }, { id: "three", label: "Source 3" }];
  const destinations: ModMatrixHeader[] = [{ id: "a", label: "Dest A" }, { id: "b", label: "Dest B" }, { id: "c", label: "Dest C" }];
  let cells = $state<ModMatrixCell[]>([{ sourceId: "one", destinationId: "a", amount: .75, enabled: true }, { sourceId: "one", destinationId: "b", amount: -.5, enabled: true }, { sourceId: "one", destinationId: "c", amount: .35, enabled: true, parameters: { min: 0, max: 1, step: .05 } }, { sourceId: "two", destinationId: "c", amount: 0, enabled: true }]);
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  <SpecimenGroup label="Sparse generic matrix"><ModMatrixGrid {sources} {destinations} bind:cells ariaLabel="Generic modulation matrix" /></SpecimenGroup>
  <SpecimenGroup label="Bipolar / negative / unipolar"><ModMatrixGrid {sources} {destinations} {cells} ariaLabel="Mixed parameter amounts" /></SpecimenGroup>
  <SpecimenGroup label="Keyboard navigation and toggle"><ModMatrixGrid sources={sources.slice(0, 2)} {destinations} {cells} ariaLabel="Keyboard matrix" /></SpecimenGroup>
  <SpecimenGroup label="Empty axes"><ModMatrixGrid ariaLabel="Empty matrix" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><ModMatrixGrid {sources} {destinations} {cells} disabled ariaLabel="Disabled matrix" /></SpecimenGroup>
</div>
  {#snippet sizes(size)}<ModMatrixGrid sources={sources.slice(0,2)} destinations={destinations.slice(0,2)} {cells} {size} ariaLabel={`Mod matrix ${size} size`} />{/snippet}
  {#snippet densities(density)}<ModMatrixGrid sources={sources.slice(0,2)} destinations={destinations.slice(0,2)} {cells} {density} ariaLabel={`Mod matrix ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page { display: grid; gap: 1.5rem; }</style>
