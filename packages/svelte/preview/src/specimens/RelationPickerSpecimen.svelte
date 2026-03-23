<script lang="ts">
  import { RelationPicker, type PickerItem, type DrillDownConfig, type DrillDownItem } from "@flint/svelte-composites";
  import { Eyebrow } from "@flint/svelte-primitives";

  const items: PickerItem[] = [
    { id: "btn", label: "Button", description: "Primary interactive control", meta: "Primitive" },
    { id: "card", label: "Card", description: "Contained surface for grouped content", meta: "Composite" },
    { id: "dialog", label: "Dialog", description: "Modal overlay for confirmations", meta: "Primitive" },
    { id: "table", label: "Table", description: "Static data table", meta: "Primitive" },
    { id: "tabs", label: "Tabs", description: "Tabbed interface with panel switching", meta: "Primitive" },
    { id: "form", label: "FormActions", description: "Action row for form controls", meta: "Primitive" },
  ];

  let selected: string[] = ["btn", "card"];

  // Drill-down data
  const categories: DrillDownItem[] = [
    { id: "primitives", label: "Primitives", description: "Foundation-level building blocks", count: 4 },
    { id: "composites", label: "Composites", description: "Higher-level composed components", count: 3 },
    { id: "layout", label: "Layout", description: "Structural layout utilities", count: 2 },
  ];

  const subcategories: Record<string, DrillDownItem[]> = {
    primitives: [
      { id: "controls", label: "Controls", description: "Interactive controls", count: 2 },
      { id: "display", label: "Display", description: "Display components", count: 2 },
    ],
    composites: [
      { id: "data", label: "Data", description: "Data display composites", count: 2 },
      { id: "forms", label: "Forms", description: "Form composites", count: 1 },
    ],
    layout: [
      { id: "containers", label: "Containers", description: "Layout containers", count: 2 },
    ],
  };

  const drillItems: Record<string, PickerItem[]> = {
    controls: [
      { id: "dd-btn", label: "Button", description: "Primary interactive control" },
      { id: "dd-checkbox", label: "Checkbox", description: "Boolean toggle control" },
    ],
    display: [
      { id: "dd-badge", label: "Badge", description: "Status indicator label" },
      { id: "dd-pill", label: "Pill", description: "Removable tag element" },
    ],
    data: [
      { id: "dd-datatable", label: "DataTable", description: "Sortable data grid" },
      { id: "dd-detailsection", label: "DetailSection", description: "Key-value detail view" },
    ],
    forms: [
      { id: "dd-formlayout", label: "FormLayout", description: "Responsive form grid" },
    ],
    containers: [
      { id: "dd-box", label: "Box", description: "Generic layout container" },
      { id: "dd-surface", label: "Surface", description: "Elevated content surface" },
    ],
  };

  const drillDown: DrillDownConfig = {
    levels: [
      {
        key: "category",
        label: "Category",
        items: categories,
        searchPlaceholder: "Search categories...",
      },
      {
        key: "subcategory",
        label: "Subcategory",
        items: (_query, context) => {
          const catId = context.category;
          const subs = subcategories[catId] ?? [];
          if (!_query.trim()) return subs;
          return subs.filter((s) => s.label.toLowerCase().includes(_query.trim().toLowerCase()));
        },
        searchPlaceholder: "Search subcategories...",
      },
    ],
    finalItems: (_query, context) => {
      const subId = context.subcategory;
      const items = drillItems[subId] ?? [];
      if (!_query.trim()) return items;
      return items.filter((i) => i.label.toLowerCase().includes(_query.trim().toLowerCase()));
    },
  };

  let drillSelected: string[] = [];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Multiple selection</Eyebrow>
    <RelationPicker
      title="Select components"
      description="Choose related components."
      {items}
      selectedIds={selected}
      selectionMode="multiple"
      on:selectionChange={(e) => (selected = e.detail.selectedIds)}
    />
    <p>Selected: <strong>{selected.join(", ")}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Single selection</Eyebrow>
    <RelationPicker
      title="Choose a parent"
      {items}
      selectionMode="single"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Drill-down (Category → Subcategory → Items)</Eyebrow>
    <RelationPicker
      title="Browse components"
      description="Navigate categories to find components."
      items={[]}
      selectedIds={drillSelected}
      selectionMode="multiple"
      {drillDown}
      on:selectionChange={(e) => (drillSelected = e.detail.selectedIds)}
    />
    {#if drillSelected.length > 0}
      <p>Selected: <strong>{drillSelected.join(", ")}</strong></p>
    {/if}
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

  p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--flint-color-text-secondary);
  }
</style>
