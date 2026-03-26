<script lang="ts">
  import { EditableLabel, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let title = "My project title";
  let emptyValue = "";
  let flushValue = "Inline heading";
  let lastEvent = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Double-click to edit (default)</Eyebrow>
    <EditableLabel
      bind:value={title}
      ariaLabel="Project title"
      on:commit={(e) => (lastEvent = `Committed: "${e.detail.value}" (was: "${e.detail.previousValue}")`)}
      on:cancel={() => (lastEvent = "Edit cancelled")}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <EditableLabel value={size.toUpperCase()} ariaLabel={"Label at " + size} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Click to edit with icon</Eyebrow>
    <EditableLabel
      bind:value={title}
      ariaLabel="Project title"
      activationMode="enterOrSpace"
      showEditIcon
      on:commit={(e) => (lastEvent = `Committed: "${e.detail.value}"`)}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Empty state</Eyebrow>
    <EditableLabel
      bind:value={emptyValue}
      ariaLabel="Description"
      activationMode="enterOrSpace"
      emptyText="Add a description…"
      on:commit={(e) => (lastEvent = `Committed: "${e.detail.value}"`)}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Flush variant</Eyebrow>
    <EditableLabel
      bind:value={flushValue}
      ariaLabel="Heading"
      variant="flush"
      activationMode="enterOrSpace"
      showEditIcon
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>With max length</Eyebrow>
    <EditableLabel
      value="Short text"
      ariaLabel="Short text"
      activationMode="enterOrSpace"
      maxLength={20}
      placeholder="Enter text…"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <EditableLabel
      value="Read-only value"
      ariaLabel="Read-only"
      disabled
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

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  p { margin: 0; }
</style>
