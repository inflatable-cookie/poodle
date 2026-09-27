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

export function SidebarNavSpecimen() {
  const [catalogueValue, setCatalogueValue] = useState("dock-region");
  const [harnessValue, setHarnessValue] = useState("pulse-runtime-foundation");
  const [libraryValue, setLibraryValue] = useState("videos");

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
      </div>
    </SpecimenLayout>
  );
}
