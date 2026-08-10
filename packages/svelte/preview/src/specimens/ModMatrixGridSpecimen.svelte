<script lang="ts">
  import { ModMatrixGrid } from "@inflatable-cookie/poodle-svelte";
  import type { ModMatrixCell, ModMatrixHeader } from "@inflatable-cookie/poodle-core";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  const sources: ModMatrixHeader[] = [{ id: "one", label: "Source 1" }, { id: "two", label: "Source 2" }, { id: "three", label: "Source 3" }];
  const destinations: ModMatrixHeader[] = [{ id: "a", label: "Dest A" }, { id: "b", label: "Dest B" }, { id: "c", label: "Dest C" }];
  let cells = $state<ModMatrixCell[]>([{ sourceId: "one", destinationId: "a", amount: .75, enabled: true }, { sourceId: "one", destinationId: "b", amount: -.5, enabled: true }, { sourceId: "two", destinationId: "c", amount: 0, enabled: true }]);
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  <section><h3>Sparse generic matrix</h3><ModMatrixGrid {sources} {destinations} bind:cells ariaLabel="Generic modulation matrix" /></section>
  <section><h3>Positive / negative / zero</h3><ModMatrixGrid {sources} destinations={destinations.slice(0, 2)} {cells} ariaLabel="Bipolar amounts" /></section>
  <section><h3>Keyboard navigation and toggle</h3><ModMatrixGrid sources={sources.slice(0, 2)} {destinations} {cells} ariaLabel="Keyboard matrix" /></section>
  <section><h3>Empty axes</h3><ModMatrixGrid ariaLabel="Empty matrix" /></section>
  <section><h3>Disabled</h3><ModMatrixGrid {sources} {destinations} {cells} disabled ariaLabel="Disabled matrix" /></section>
</div>
  {#snippet sizes(size)}<ModMatrixGrid sources={sources.slice(0,2)} destinations={destinations.slice(0,2)} {cells} {size} ariaLabel={`Mod matrix ${size} size`} />{/snippet}
  {#snippet densities(density)}<ModMatrixGrid sources={sources.slice(0,2)} destinations={destinations.slice(0,2)} {cells} {density} ariaLabel={`Mod matrix ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page{display:grid;gap:1.5rem}section{display:grid;gap:.75rem}h3{margin:0;color:var(--poodle-color-text-secondary);font-size:.75rem}</style>
