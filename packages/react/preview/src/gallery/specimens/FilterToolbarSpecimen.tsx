import { useState, type CSSProperties } from "react";
import { FilterToolbar, Select, TextInput, Button, IconButton, type SelectOption } from "@poodle/react";
import { SpecimenLayout } from "../SpecimenLayout";
import { SpecimenGroup } from "../SpecimenGroup";

const statusOptions: SelectOption[] = [
  { value: "all", label: "All statuses" },
  { value: "active", label: "Active" },
  { value: "archived", label: "Archived" },
  { value: "draft", label: "Draft" },
];

const typeOptions: SelectOption[] = [
  { value: "all", label: "All types" },
  { value: "document", label: "Document" },
  { value: "spreadsheet", label: "Spreadsheet" },
  { value: "presentation", label: "Presentation" },
];

const ownerOptions: SelectOption[] = [
  { value: "all", label: "All owners" },
  { value: "me", label: "Me" },
  { value: "team", label: "My team" },
];

const variantStyle: CSSProperties = {
  width: "min(100%, 64rem)",
};

export function FilterToolbarSpecimen() {
  const [collapsed1, setCollapsed1] = useState(true);
  const [collapsed2, setCollapsed2] = useState(true);

  return (
    <SpecimenLayout
      bareVariants
      sizes={(size) => (
        <div style={variantStyle}>
          <FilterToolbar summaryText={`Toolbar at ${size}`} size={size} ariaLabel={`Filter toolbar at ${size}`}>
            <TextInput id={`size-search-${size}`} type="search" placeholder="Filter projects…" ariaLabel="Search" />
            <Select id={`size-status-${size}`} options={statusOptions} defaultValue="all" ariaLabel="Status" />
            <Select id={`size-type-${size}`} options={typeOptions} defaultValue="all" ariaLabel="Type" />
          </FilterToolbar>
        </div>
      )}
      densities={(density) => (
        <div style={variantStyle}>
          <FilterToolbar
            summaryText={`Toolbar at ${density}`}
            density={density}
            ariaLabel={`Filter toolbar at ${density}`}
          >
            <TextInput
              id={`density-search-${density}`}
              type="search"
              placeholder="Filter projects…"
              ariaLabel="Search"
            />
            <Select id={`density-status-${density}`} options={statusOptions} defaultValue="all" ariaLabel="Status" />
            <Select id={`density-type-${density}`} options={typeOptions} defaultValue="all" ariaLabel="Type" />
          </FilterToolbar>
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Responsive grid layout">
          <FilterToolbar summaryText="Showing 24 of 156 items" ariaLabel="Item filters">
            <TextInput id="filter-search" type="search" placeholder="Search…" ariaLabel="Search items" />
            <Select id="filter-status" options={statusOptions} defaultValue="all" ariaLabel="Status" />
            <Select id="filter-type" options={typeOptions} defaultValue="all" ariaLabel="Type" />
            <Select id="filter-owner" options={ownerOptions} defaultValue="all" ariaLabel="Owner" />
          </FilterToolbar>
        </SpecimenGroup>

        <SpecimenGroup label="Collapsible with actions">
          <FilterToolbar
            collapsible
            collapsed={collapsed1}
            onCollapsedChange={setCollapsed1}
            summaryText="Showing 24 of 156 items"
            ariaLabel="Collapsible filters"
            actions={<IconButton icon="refresh-cw" sizeRole="chrome" ariaLabel="Refresh" />}
          >
            <TextInput id="col-search" type="search" placeholder="Search…" ariaLabel="Search" />
            <Select id="col-status" options={statusOptions} defaultValue="all" ariaLabel="Status" />
            <Select id="col-type" options={typeOptions} defaultValue="all" ariaLabel="Type" />
          </FilterToolbar>
        </SpecimenGroup>

        <SpecimenGroup label="Explicit collapsed state">
          <FilterToolbar
            collapsible
            collapsed={collapsed2}
            onCollapsedChange={setCollapsed2}
            summaryText="3 filters active"
            ariaLabel="Collapsed filters"
            actions={<IconButton icon="refresh-cw" sizeRole="chrome" ariaLabel="Refresh" />}
          >
            <TextInput id="col2-search" type="search" placeholder="Search…" ariaLabel="Search" />
            <Select id="col2-status" options={statusOptions} defaultValue="active" ariaLabel="Status" />
          </FilterToolbar>
        </SpecimenGroup>

        <SpecimenGroup label="With secondary slot">
          <FilterToolbar
            ariaLabel="Project filters"
            columns={3}
            secondary={
              <Button variant="secondary" sizeRole="chrome">
                Reset all
              </Button>
            }
          >
            <TextInput id="proj-search" type="search" placeholder="Filter projects…" ariaLabel="Filter" />
            <Select id="proj-status" options={statusOptions} defaultValue="all" ariaLabel="Status" />
            <Select id="proj-type" options={typeOptions} defaultValue="all" ariaLabel="Type" />
          </FilterToolbar>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
