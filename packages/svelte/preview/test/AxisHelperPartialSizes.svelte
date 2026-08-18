<script lang="ts">
  import SpecimenLayout from "../src/components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  {#snippet sizes(size)}
    {#if size === "xs"}
      <p data-size={size}>only xs</p>
    {/if}
  {/snippet}
  <p>examples content</p>
</SpecimenLayout>
