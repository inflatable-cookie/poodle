<script lang="ts">
  import { EditableLabel } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let title = "My project title";
  let emptyValue = "";
  let flushValue = "Inline heading";
  let lastEvent = "";
</script>

<div class="specimen">
  <SpecimenGroup label="Double-click to edit (default)">
    <EditableLabel
      bind:value={title}
      ariaLabel="Project title"
      on:commit={(e) => (lastEvent = `Committed: "${e.detail.value}" (was: "${e.detail.previousValue}")`)}
      on:cancel={() => (lastEvent = "Edit cancelled")}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <EditableLabel value={size.toUpperCase()} ariaLabel={"Label at " + size} {size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <EditableLabel value="Edit me" {density} />
        </div>
      {/each}
    </div>
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
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }

  p { margin: 0; }
</style>
