<script lang="ts">
  import { Icon, ListCard, ListCardCounter } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  let linkedAction = $state("No linked counter clicked yet.");
</script>

<SpecimenGroup label="Static footer counters">
  <ListCard title="Design system" subtitle="12 contributors" leadingShape="rounded-square" interactive>
    {#snippet leading()}
      <Icon icon="folder" />
    {/snippet}
    {#snippet footer()}
      <ListCardCounter icon="file-text" count={24} tooltip="24 documents" />
      <ListCardCounter icon="image" count={8} tooltip="8 images" />
    {/snippet}
  </ListCard>
</SpecimenGroup>

<SpecimenGroup label="Linked footer counter">
  <ListCard title="Brand guidelines" subtitle="Last updated 2 weeks ago" leadingShape="rounded-square" interactive>
    {#snippet leading()}
      <Icon icon="folder" />
    {/snippet}
    {#snippet footer()}
      <ListCardCounter
        icon="layers"
        count={3}
        tooltip="3 sub-folders"
        href="#sub-folders"
        onClick={() => (linkedAction = "Opened sub-folders")}
      />
    {/snippet}
  </ListCard>
  <p class="poodle-specimen__hint">{linkedAction}</p>
</SpecimenGroup>

<style>
  .poodle-specimen__hint {
    margin: 0.75rem 0 0;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
