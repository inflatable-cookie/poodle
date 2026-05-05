<script lang="ts">
  import { ToggleGroup, type ToggleGroupOption } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const viewOptions: ToggleGroupOption[] = [
    { value: "grid", label: "Grid" },
    { value: "list", label: "List" },
    { value: "board", label: "Board" },
  ];

  const alignOptions: ToggleGroupOption[] = [
    { value: "left", label: "Left" },
    { value: "center", label: "Center" },
    { value: "right", label: "Right" },
    { value: "justify", label: "Justify" },
  ];

  const tagOptions: ToggleGroupOption[] = [
    { value: "design", label: "Design" },
    { value: "engineering", label: "Engineering" },
    { value: "docs", label: "Docs" },
  ];

  let view = "grid";
  let tags: string[] = ["design", "docs"];
</script>

<SpecimenLayout>
  <SpecimenGroup label="Single selection">
    <ToggleGroup
      options={viewOptions}
      value={view}
      ariaLabel="View mode"
      on:valueChange={(e) => (view = e.detail.value as string)}
    />
    <p class="poodle-specimen__hint">View: <strong>{view}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Four options">
    <ToggleGroup
      options={alignOptions}
      defaultValue="left"
      ariaLabel="Text alignment"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Multiple selection">
    <ToggleGroup
      options={tagOptions}
      value={tags}
      selectionMode="multiple"
      ariaLabel="Filter tags"
      on:valueChange={(e) => (tags = e.detail.value as string[])}
    />
    <p class="poodle-specimen__hint">Selected: <strong>{tags.join(", ") || "none"}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <ToggleGroup
      options={viewOptions}
      defaultValue="list"
      disabled
      ariaLabel="Disabled toggle group"
    />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <ToggleGroup options={viewOptions} defaultValue="grid" {size} ariaLabel="{size} toggle group" />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <ToggleGroup options={viewOptions} defaultValue="grid" {density} ariaLabel="{density} toggle group" />
  </svelte:fragment>
</SpecimenLayout>

<style>
  .poodle-specimen__hint {
    margin: 0;
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
