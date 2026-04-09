<script lang="ts">
  import { Code, MetaBar, MetaItem, Pill } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Header metadata">
    <MetaBar ariaLabel="Project metadata">
      <MetaItem label="ID">
        <Code inline source="proj_01JX9G9NVV1W3M4P6K8Q8T2D5A" showCopyButton />
      </MetaItem>
      <Pill tone="success">Active</Pill>
      <MetaItem label="Owner">Clay</MetaItem>
      <MetaItem label="Updated">2 hours ago</MetaItem>
    </MetaBar>
  </SpecimenGroup>

  <SpecimenGroup label="No separators">
    <MetaBar showSeparators={false}>
      <MetaItem label="Type">Media</MetaItem>
      <Pill tone="info">Public</Pill>
      <MetaItem>1920 × 1080</MetaItem>
    </MetaBar>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

</style>
