import { useState } from "react";
import {
  RelationPicker,
  type PickerItem,
  type DrillDownConfig,
  type DrillDownItem,
} from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const items: PickerItem[] = [
  { id: "btn", label: "Button", description: "Primary interactive control", meta: "Primitive" },
  { id: "card", label: "Card", description: "Contained surface for grouped content", meta: "Composite" },
  { id: "dialog", label: "Dialog", description: "Modal overlay for confirmations", meta: "Primitive" },
  { id: "table", label: "Table", description: "Static data table", meta: "Primitive" },
  { id: "tabs", label: "Tabs", description: "Tabbed interface with panel switching", meta: "Primitive" },
  { id: "form", label: "FormActions", description: "Action row for form controls", meta: "Primitive" },
];

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
    const finalItems = drillItems[subId] ?? [];
    if (!_query.trim()) return finalItems;
    return finalItems.filter((item) => item.label.toLowerCase().includes(_query.trim().toLowerCase()));
  },
};

const pStyle = { margin: 0, fontSize: "0.875rem", color: "var(--poodle-color-text-secondary)" };
const variantStyle = { width: "min(100%, 28rem)" };

export function RelationPickerSpecimen() {
  const [selected, setSelected] = useState<string[]>(["btn", "card"]);
  const [drillSelected, setDrillSelected] = useState<string[]>([]);
  const [sizeSelected, setSizeSelected] = useState<string[]>(["btn"]);
  const [densitySelected, setDensitySelected] = useState<string[]>(["btn"]);

  return (
    <SpecimenLayout
      bareVariants
      sizes={(size) => (
        <div style={variantStyle}>
          <RelationPicker
            title="Select components"
            description="Choose related components."
            items={items}
            selectedIds={sizeSelected}
            selectionMode="multiple"
            size={size}
            onSelectionChange={(nextSelected) => setSizeSelected(nextSelected)}
          />
        </div>
      )}
      densities={(density) => (
        <div style={variantStyle}>
          <RelationPicker
            title="Select components"
            description="Choose related components."
            items={items}
            selectedIds={densitySelected}
            selectionMode="multiple"
            density={density}
            onSelectionChange={(nextSelected) => setDensitySelected(nextSelected)}
          />
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Multiple selection" bare>
          <RelationPicker
            title="Select components"
            description="Choose related components."
            items={items}
            selectedIds={selected}
            selectionMode="multiple"
            onSelectionChange={(nextSelected) => setSelected(nextSelected)}
          />
          <p style={pStyle}>Selected: <strong>{selected.join(", ")}</strong></p>
        </SpecimenGroup>

        <SpecimenGroup label="Single selection" bare>
          <RelationPicker title="Choose a parent" items={items} selectionMode="single" />
        </SpecimenGroup>

        <SpecimenGroup label="Drill-down (Category → Subcategory → Items)" bare>
          <RelationPicker
            title="Browse components"
            description="Navigate categories to find components."
            items={[]}
            selectedIds={drillSelected}
            selectionMode="multiple"
            drillDown={drillDown}
            onSelectionChange={(nextSelected) => setDrillSelected(nextSelected)}
          />
          {drillSelected.length > 0 ? (
            <p style={pStyle}>Selected: <strong>{drillSelected.join(", ")}</strong></p>
          ) : null}
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
