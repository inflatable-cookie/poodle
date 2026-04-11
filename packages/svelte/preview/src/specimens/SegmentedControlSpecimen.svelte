<script lang="ts">
  import { SegmentedControl, type SegmentedControlOption } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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

<SpecimenLayout>
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

  <SpecimenGroup label="Fully disabled">
    <SegmentedControl options={viewOptions} defaultValue="list" ariaLabel="Disabled control" disabled />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <SegmentedControl options={viewOptions} defaultValue="grid" {size} ariaLabel="{size} view mode" />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <SegmentedControl options={viewOptions} defaultValue="grid" {density} ariaLabel="{density} view mode" />
  </svelte:fragment>
</SpecimenLayout>
