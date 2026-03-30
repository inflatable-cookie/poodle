<script lang="ts">
  import { SegmentedControl, Eyebrow, type SegmentedControlOption } from "@poodle/svelte-primitives";

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
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <SegmentedControl
      options={viewOptions}
      value={view}
      ariaLabel="View mode"
      on:valueChange={(e) => (view = e.detail.value)}
    />
    <p>View: <strong>{view}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>With disabled option</Eyebrow>
    <SegmentedControl options={statusOptions} defaultValue="all" ariaLabel="Status filter" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Content Fit</Eyebrow>
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
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <SegmentedControl options={viewOptions} defaultValue="grid" ariaLabel="{density} view mode" {density} />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Fully disabled</Eyebrow>
    <SegmentedControl options={viewOptions} defaultValue="list" ariaLabel="Disabled control" disabled />
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
    gap: 0.5rem;
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

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
