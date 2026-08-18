<script lang="ts">
  import SpecimenLayout from "../src/components/SpecimenLayout.svelte";
</script>

<SpecimenLayout showSizes={false} showDensities={false}>
  {#snippet sizes(size)}
    <p data-size={size}>size {size}</p>
  {/snippet}
  {#snippet densities(density)}
    <p data-density={density}>density {density}</p>
  {/snippet}
  <p>examples content</p>
</SpecimenLayout>