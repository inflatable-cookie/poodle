import { useState, type CSSProperties } from "react";
import { SidebarNav, type SidebarNavGroup } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const frameStyle: CSSProperties = {
  width: "16rem",
  minHeight: "20rem",
  borderRight: "0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 60%, transparent)",
  overflow: "auto",
};

const catalogueGroups: SidebarNavGroup[] = [
  {
    id: "catalogue",
    items: [
      { value: "button", label: "Button" },
      { value: "dock-region", label: "DockRegion" },
      { value: "split-view", label: "SplitView" },
      { value: "tabs", label: "Tabs" },
    ],
  },
];

const harnessGroups: SidebarNavGroup[] = [
  {
    id: "commands",
    label: "Commands",
    items: [{ value: "shared-commands", label: "Shared commands" }],
  },
  {
    id: "runtime",
    label: "Runtime",
    items: [
      { value: "device-monitor", label: "Device + monitor control" },
      { value: "pulse-runtime-foundation", label: "Pulse runtime foundation" },
      { value: "support-history", label: "Support + historical observability" },
    ],
  },
  {
    id: "shell",
    label: "Shell",
    items: [{ value: "shell-kernel", label: "Shell kernel" }],
  },
];

const libraryGroups: SidebarNavGroup[] = [
  {
    id: "library",
    label: "Library",
    items: [
      { value: "videos", label: "Videos", endLabel: "198" },
      { value: "audio", label: "Audio", endLabel: "42" },
      { value: "images", label: "Images", endLabel: "1,204" },
      { value: "archive", label: "Archive", endLabel: "0", disabled: true },
    ],
  },
];

const savedViewMenu = [
  { value: "rename", label: "Rename" },
  { value: "sep", label: "", kind: "separator" as const },
  { value: "delete", label: "Delete", tone: "danger" as const },
];

const savedViewGroups: SidebarNavGroup[] = [
  {
    id: "saved",
    label: "Saved views",
    items: [
      {
        value: "q4",
        label: "Q4 close",
        contextMenuItems: savedViewMenu,
        contextMenuAriaLabel: "Q4 close actions",
      },
      { value: "cash", label: "Cash flow", contextMenuItems: savedViewMenu },
      { value: "all", label: "All records" },
    ],
  },
];

export function SidebarNavSpecimen() {
  const [catalogueValue, setCatalogueValue] = useState("dock-region");
  const [harnessValue, setHarnessValue] = useState("pulse-runtime-foundation");
  const [libraryValue, setLibraryValue] = useState("videos");
  const [savedViewValue, setSavedViewValue] = useState("q4");

  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={frameStyle}>
          <SidebarNav
            ariaLabel={`${size} sidebar navigation`}
            groups={harnessGroups}
            value={harnessValue}
            size={size}
          />
        </div>
      )}
      densities={(density) => (
        <div style={frameStyle}>
          <SidebarNav
            ariaLabel={`${density} sidebar navigation`}
            groups={harnessGroups}
            value={harnessValue}
            density={density}
          />
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Single-group catalogue">
          <div style={frameStyle}>
            <SidebarNav
              ariaLabel="Catalogue navigation"
              groups={catalogueGroups}
              value={catalogueValue}
              onValueChange={(value) => setCatalogueValue(value)}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Grouped verification nav">
          <div style={frameStyle}>
            <SidebarNav
              ariaLabel="Verification navigation"
              groups={harnessGroups}
              value={harnessValue}
              onValueChange={(value) => setHarnessValue(value)}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Library counts (endLabel)">
          <div style={frameStyle}>
            <SidebarNav
              ariaLabel="Library navigation"
              groups={libraryGroups}
              value={libraryValue}
              onValueChange={(value) => setLibraryValue(value)}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Saved views (context menu)">
          <div style={frameStyle}>
            <SidebarNav
              ariaLabel="Saved views"
              groups={savedViewGroups}
              value={savedViewValue}
              onValueChange={(value) => setSavedViewValue(value)}
            />
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
