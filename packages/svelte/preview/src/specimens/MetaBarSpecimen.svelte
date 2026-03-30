<script lang="ts">
  import { Code, Eyebrow, MetaBar, MetaItem, Pill } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Header metadata</Eyebrow>
    <MetaBar ariaLabel="Project metadata">
      <MetaItem label="ID">
        <Code inline source="proj_01JX9G9NVV1W3M4P6K8Q8T2D5A" showCopyButton />
      </MetaItem>
      <Pill tone="success">Active</Pill>
      <MetaItem label="Owner">Clay</MetaItem>
      <MetaItem label="Updated">2 hours ago</MetaItem>
    </MetaBar>
  </div>

  <div class="specimen__group">
    <Eyebrow>No separators</Eyebrow>
    <MetaBar showSeparators={false}>
      <MetaItem label="Type">Media</MetaItem>
      <Pill tone="info">Public</Pill>
      <MetaItem>1920 × 1080</MetaItem>
    </MetaBar>
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>
