import type { CSSProperties } from "react";
import { ActionDiscoveryPanel } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const bareFrame: CSSProperties = {
  width: "min(32rem, 100%)",
  maxWidth: "32rem",
  overflow: "hidden",
  border: 0,
  borderRadius: 0,
  padding: 0,
};
const variantBlock: CSSProperties = { width: "min(32rem, 100%)" };

const groupedItems = [
  { id: "save", title: "Save", shortcut: "Ctrl+S", group: "File" },
  { id: "open", title: "Open File", shortcut: "Ctrl+O", group: "File" },
  { id: "close", title: "Close Tab", shortcut: "Ctrl+W", group: "File" },
  { id: "find", title: "Find in Files", shortcut: "Ctrl+Shift+F", group: "Edit" },
  { id: "replace", title: "Find and Replace", shortcut: "Ctrl+H", group: "Edit" },
  { id: "terminal", title: "Toggle Terminal", shortcut: "Ctrl+`", group: "View" },
  { id: "sidebar", title: "Toggle Sidebar", shortcut: "Ctrl+B", group: "View" },
];

const descriptiveItems = [
  {
    id: "deploy",
    title: "Deploy to Production",
    description: "Push current branch to production environment",
    badge: "Dangerous",
    group: "CI/CD",
  },
  {
    id: "preview",
    title: "Open Preview",
    description: "Launch preview in a new tab",
    shortcut: "Ctrl+Shift+P",
    group: "CI/CD",
  },
  { id: "lint", title: "Run Linter", shortcut: "Ctrl+Shift+L", group: "Tools" },
];

const variantItems = [
  { id: "save", title: "Save", shortcut: "Ctrl+S", group: "File" },
  { id: "deploy", title: "Deploy", badge: "Dangerous", group: "Release" },
];

export function ActionDiscoveryPanelSpecimen() {
  return (
    <SpecimenLayout
      bareVariants
      sizes={(size) => (
        <div style={variantBlock}>
          <SpecimenGroup label={size.toUpperCase()}>
            <div style={bareFrame}>
              <ActionDiscoveryPanel items={variantItems} size={size} ariaLabel={`${size} actions`} />
            </div>
          </SpecimenGroup>
        </div>
      )}
      densities={(density) => (
        <div style={variantBlock}>
          <SpecimenGroup label={density.toUpperCase()}>
            <div style={bareFrame}>
              <ActionDiscoveryPanel items={variantItems} density={density} ariaLabel={`${density} actions`} />
            </div>
          </SpecimenGroup>
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Grouped actions">
          <div style={bareFrame}>
            <ActionDiscoveryPanel items={groupedItems} ariaLabel="Demo actions" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="With descriptions and badges">
          <div style={bareFrame}>
            <ActionDiscoveryPanel items={descriptiveItems} ariaLabel="CI actions" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Empty state">
          <div style={bareFrame}>
            <ActionDiscoveryPanel items={[]} state="empty" ariaLabel="Empty actions" />
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
