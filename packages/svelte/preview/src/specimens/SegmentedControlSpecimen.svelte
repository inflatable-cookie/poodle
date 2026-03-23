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
    { value: "draft", label: "Draft", isDisabled: true },
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
    <Eyebrow>Fully disabled</Eyebrow>
    <SegmentedControl options={viewOptions} defaultValue="list" ariaLabel="Disabled control" isDisabled />
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

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
