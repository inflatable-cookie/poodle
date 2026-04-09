<script lang="ts">
  import { SegmentedControl, type SegmentedControlOption, type ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const viewOptions: SegmentedControlOption[] = [
    { value: "grid", label: "Grid" },
    { value: "list", label: "List" },
    { value: "table", label: "Table" },
  ];

  const statusOptions: SegmentedControlOption[] = [
    { value: "all", label: "All" },
    { value: "active", label: "Active" },
    { value: "archived", label: "Archived" },
    { value: "draft", label: "Draft", disabled: true },
  ];

  let view = "grid";
</script>

<div class="specimen">
  <SpecimenGroup label="Default">
    <SegmentedControl
      options={viewOptions}
      value={view}
      ariaLabel="View mode"
      on:valueChange={(e) => (view = e.detail.value)}
    />
    <p>View: <strong>{view}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="With disabled option">
    <SegmentedControl options={statusOptions} defaultValue="all" ariaLabel="Status filter" />
  </SpecimenGroup>

  <SpecimenGroup label="Content Fit">
    <SegmentedControl
      options={[
        { value: "1h", label: "1h", title: "Last 1 hour" },
        { value: "6h", label: "6h", title: "Last 6 hours" },
        { value: "24h", label: "24h", title: "Last 24 hours" }
      ]}
      defaultValue="24h"
      ariaLabel="Timeline window"
      size="xs"
      equalWidth={false}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <SegmentedControl options={viewOptions} defaultValue="grid" ariaLabel="{density} view mode" {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Fully disabled">
    <SegmentedControl options={viewOptions} defaultValue="list" ariaLabel="Disabled control" disabled />
  </SpecimenGroup>
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
    gap: 0.5rem;
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

</style>
