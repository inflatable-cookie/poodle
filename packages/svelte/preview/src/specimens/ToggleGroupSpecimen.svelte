<script lang="ts">
  import { ToggleGroup, Eyebrow, type ToggleGroupOption } from "@pug/svelte-primitives";

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
  let align = "left";
  let tags: string[] = ["design", "docs"];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Single selection</Eyebrow>
    <ToggleGroup
      options={viewOptions}
      value={view}
      ariaLabel="View mode"
      on:valueChange={(e) => (view = e.detail.value)}
    />
    <p>View: <strong>{view}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Four options</Eyebrow>
    <ToggleGroup
      options={alignOptions}
      value={align}
      ariaLabel="Text alignment"
      on:valueChange={(e) => (align = e.detail.value)}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Multiple selection</Eyebrow>
    <ToggleGroup
      options={tagOptions}
      value={tags}
      selectionMode="multiple"
      ariaLabel="Filter tags"
      on:valueChange={(e) => (tags = e.detail.value)}
    />
    <p>Selected: <strong>{tags.join(", ") || "none"}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <ToggleGroup
      options={viewOptions}
      defaultValue="list"
      isDisabled
      ariaLabel="Disabled toggle group"
    />
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
    color: var(--pug-color-text-secondary);
  }
</style>
