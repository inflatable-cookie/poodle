<script lang="ts">
  import { EditableLabel } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let title = $state("My project title");
  let emptyValue = $state("");
  let flushValue = $state("Inline heading");
  let lastEvent = $state("");
</script>

<SpecimenLayout>
  <SpecimenGroup label="Double-click to edit (default)">
    <EditableLabel
      bind:value={title}
      ariaLabel="Project title"
      onCommit={(detail) => (lastEvent = `Committed: "${detail.value}" (was: "${detail.previousValue}")`)}
      onCancel={() => (lastEvent = "Edit cancelled")}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Click to edit with icon">
    <EditableLabel
      bind:value={title}
      ariaLabel="Project title"
      activationMode="enterOrSpace"
      showEditIcon
      onCommit={(detail) => (lastEvent = `Committed: "${detail.value}"`)}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Empty state">
    <EditableLabel
      bind:value={emptyValue}
      ariaLabel="Description"
      activationMode="enterOrSpace"
      emptyText="Add a description…"
      onCommit={(detail) => (lastEvent = `Committed: "${detail.value}"`)}
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

  {#snippet sizes(size)}
    <EditableLabel value={size.toUpperCase()} {size} ariaLabel={"Label at " + size} />
  {/snippet}

  {#snippet densities(density)}
    <EditableLabel value="Edit me" {density} />
  {/snippet}
</SpecimenLayout>

<style>
  p { margin: 0; }
</style>
