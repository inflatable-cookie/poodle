<script lang="ts">
  import SpecimenLayout from "../src/components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  {#snippet densities(density)}
    <p data-density={density}>density {density}</p>
  {/snippet}
  <p>examples content</p>
</SpecimenLayout>