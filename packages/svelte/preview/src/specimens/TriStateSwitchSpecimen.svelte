<script lang="ts">
  import {
    TriStateSwitch,
    Eyebrow,
    UiPresentationProvider,
    type TriStateValue,
  } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  let filter: TriStateValue = "default";
  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;
</script>

<div class="specimen">
  <SpecimenGroup label="Default">
    <TriStateSwitch
      value={filter}
      ariaLabel="Filter mode"
      on:valueChange={(e) => (filter = e.detail.value)}
    />
    <p>Value: <strong>{filter}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Custom labels">
    <TriStateSwitch
      options={{ excluded: "Hide", default: "All", included: "Show" }}
      ariaLabel="Visibility filter"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Semantic sizes">
    <div class="specimen__row">
      {#each controlSizes as size}
        <TriStateSwitch value="default" {size} ariaLabel={`Tri-state switch at ${size}`} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Chrome vs prominent role offset">
    <UiPresentationProvider density="compact" sizeScale="sm">
      <div class="specimen__row">
        <TriStateSwitch value="excluded" sizeRole="chrome" ariaLabel="Compact chrome switch" />
        <TriStateSwitch value="included" sizeRole="control" ariaLabel="Compact control switch" />
        <TriStateSwitch value="default" sizeRole="prominent" ariaLabel="Compact prominent switch" />
      </div>
    </UiPresentationProvider>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <TriStateSwitch value="included" disabled ariaLabel="Disabled switch" />
  </SpecimenGroup>

  <SpecimenGroup label="Custom semantic colors">
    <TriStateSwitch
      value={filter}
      excludedColor="#ef4444"
      defaultColor="#64748b"
      includedColor="#22c55e"
      ariaLabel="Filter mode with custom semantic colors"
      on:valueChange={(e) => (filter = e.detail.value)}
    />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }
</style>
