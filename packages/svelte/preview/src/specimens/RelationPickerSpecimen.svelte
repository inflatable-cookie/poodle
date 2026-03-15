<script lang="ts">
  import { RelationPicker, type PickerItem } from "@pug/svelte-composites";
  import { Eyebrow } from "@pug/svelte-primitives";

  const items: PickerItem[] = [
    { id: "btn", label: "Button", description: "Primary interactive control", meta: "Primitive" },
    { id: "card", label: "Card", description: "Contained surface for grouped content", meta: "Composite" },
    { id: "dialog", label: "Dialog", description: "Modal overlay for confirmations", meta: "Primitive" },
    { id: "table", label: "Table", description: "Static data table", meta: "Primitive" },
    { id: "tabs", label: "Tabs", description: "Tabbed interface with panel switching", meta: "Primitive" },
    { id: "form", label: "FormActions", description: "Action row for form controls", meta: "Primitive" },
  ];

  let selected: string[] = ["btn", "card"];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Multiple selection</Eyebrow>
    <div class="picker-container">
      <RelationPicker
        title="Select components"
        description="Choose related components."
        {items}
        selectedIds={selected}
        selectionMode="multiple"
        on:selectionChange={(e) => (selected = e.detail.selectedIds)}
      />
    </div>
    <p>Selected: <strong>{selected.join(", ")}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Single selection</Eyebrow>
    <div class="picker-container">
      <RelationPicker
        title="Choose a parent"
        {items}
        selectionMode="single"
      />
    </div>
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

  .picker-container {
    height: 18rem;
    border: 1px solid var(--pug-color-border-default);
    border-radius: 4px;
    overflow: hidden;
  }

  p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--pug-color-text-secondary);
  }
</style>
