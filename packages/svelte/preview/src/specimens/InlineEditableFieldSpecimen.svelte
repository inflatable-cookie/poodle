<script lang="ts">
  import { InlineEditableField } from "@pug/svelte-composites";
  import { Eyebrow } from "@pug/svelte-primitives";

  let title = "My project title";
  let emptyValue = "";
  let lastEvent = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Click to edit</Eyebrow>
    <InlineEditableField
      bind:value={title}
      ariaLabel="Project title"
      on:save={(e) => (lastEvent = `Saved: "${e.detail.value}" (was: "${e.detail.previousValue}")`)}
      on:cancel={() => (lastEvent = "Edit cancelled")}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Empty state</Eyebrow>
    <InlineEditableField
      bind:value={emptyValue}
      ariaLabel="Description"
      emptyText="Add a description…"
      on:save={(e) => (lastEvent = `Saved: "${e.detail.value}"`)}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>With max length</Eyebrow>
    <InlineEditableField
      value="Short text"
      ariaLabel="Short text field"
      maxLength={20}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <InlineEditableField
      value="Read-only value"
      ariaLabel="Read-only field"
      isDisabled
    />
  </div>

  {#if lastEvent}
    <div class="specimen__group">
      <Eyebrow>Last event</Eyebrow>
      <p>{lastEvent}</p>
    </div>
  {/if}
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
    gap: 0.5rem;
  }

  p { margin: 0; }
</style>
