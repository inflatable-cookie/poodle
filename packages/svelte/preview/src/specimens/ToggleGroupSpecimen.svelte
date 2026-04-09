<script lang="ts">
  import {
    ToggleGroup,
    Eyebrow,
    UiPresentationProvider,
    type ToggleGroupOption,
  } from "@poodle/svelte-primitives";

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
      on:valueChange={(e) => (view = e.detail.value as string)}
    />
    <p>View: <strong>{view}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Four options</Eyebrow>
    <ToggleGroup
      options={alignOptions}
      value={align}
      ariaLabel="Text alignment"
      on:valueChange={(e) => (align = e.detail.value as string)}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Multiple selection</Eyebrow>
    <ToggleGroup
      options={tagOptions}
      value={tags}
      selectionMode="multiple"
      ariaLabel="Filter tags"
      on:valueChange={(e) => (tags = e.detail.value as string[])}
    />
    <p>Selected: <strong>{tags.join(", ") || "none"}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Semantic role offsets</Eyebrow>
    <UiPresentationProvider density="compact" sizeScale="sm">
      <div class="specimen__row">
        <ToggleGroup options={viewOptions} defaultValue="grid" sizeRole="chrome" ariaLabel="Chrome-sized group" />
        <ToggleGroup options={viewOptions} defaultValue="list" ariaLabel="Control-sized group" />
        <ToggleGroup options={viewOptions} defaultValue="board" sizeRole="prominent" ariaLabel="Prominent group" />
      </div>
    </UiPresentationProvider>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <ToggleGroup
      options={viewOptions}
      defaultValue="list"
      disabled
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
    color: var(--poodle-color-text-secondary);
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }
</style>
