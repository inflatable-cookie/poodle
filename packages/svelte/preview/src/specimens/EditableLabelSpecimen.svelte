<script lang="ts">
  import { EditableLabel } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let title = "My project title";
  let emptyValue = "";
  let flushValue = "Inline heading";
  let lastEvent = "";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Double-click to edit (default)">
    <EditableLabel
      bind:value={title}
      ariaLabel="Project title"
      on:commit={(e) => (lastEvent = `Committed: "${e.detail.value}" (was: "${e.detail.previousValue}")`)}
      on:cancel={() => (lastEvent = "Edit cancelled")}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Click to edit with icon">
    <EditableLabel
      bind:value={title}
      ariaLabel="Project title"
      activationMode="enterOrSpace"
      showEditIcon
      on:commit={(e) => (lastEvent = `Committed: "${e.detail.value}"`)}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Empty state">
    <EditableLabel
      bind:value={emptyValue}
      ariaLabel="Description"
      activationMode="enterOrSpace"
      emptyText="Add a description…"
      on:commit={(e) => (lastEvent = `Committed: "${e.detail.value}"`)}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Flush variant">
    <EditableLabel
      bind:value={flushValue}
      ariaLabel="Heading"
      variant="flush"
      activationMode="enterOrSpace"
      showEditIcon
    />
  </SpecimenGroup>

  <SpecimenGroup label="With max length">
    <EditableLabel
      value="Short text"
      ariaLabel="Short text"
      activationMode="enterOrSpace"
      maxLength={20}
      placeholder="Enter text…"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <EditableLabel
      value="Read-only value"
      ariaLabel="Read-only"
      disabled
    />
  </SpecimenGroup>

  {#if lastEvent}
    <SpecimenGroup label="Last event">
      <p>{lastEvent}</p>
    </SpecimenGroup>
  {/if}

  <svelte:fragment slot="sizes" let:size>
    <EditableLabel value={size.toUpperCase()} {size} ariaLabel={"Label at " + size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <EditableLabel value="Edit me" {density} />
  </svelte:fragment>
</SpecimenLayout>

<style>
  p { margin: 0; }
</style>
