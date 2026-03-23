<script lang="ts">
  import { SelectionSummary } from "@poodle/svelte-composites";
  import { Eyebrow } from "@poodle/svelte-primitives";

  let items = [
    { id: "1", label: "Button" },
    { id: "2", label: "Card" },
    { id: "3", label: "Dialog" },
    { id: "4", label: "Table" },
    { id: "5", label: "Tabs" },
  ];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Multiple items selected</Eyebrow>
    <SelectionSummary
      {items}
      selectionMode="multiple"
      on:remove={(e) => (items = items.filter((i) => i.id !== e.detail.id))}
      on:clear={() => (items = [])}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Single item</Eyebrow>
    <SelectionSummary items={[{ id: "1", label: "Primary button" }]} selectionMode="single" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Truncated (max 3 visible)</Eyebrow>
    <SelectionSummary
      items={[
        { id: "a", label: "Alpha" },
        { id: "b", label: "Beta" },
        { id: "c", label: "Gamma" },
        { id: "d", label: "Delta" },
        { id: "e", label: "Epsilon" },
        { id: "f", label: "Zeta" },
      ]}
      maxVisibleItems={3}
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
</style>
