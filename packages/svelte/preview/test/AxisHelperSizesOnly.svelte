<script lang="ts">
  import SpecimenLayout from "../src/components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  {#snippet sizes(size)}
    <p data-size={size}>size {size}</p>
  {/snippet}
  <p>examples content</p>
</SpecimenLayout>