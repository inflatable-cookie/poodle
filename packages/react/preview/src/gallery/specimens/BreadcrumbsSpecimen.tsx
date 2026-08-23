import { useState } from "react";
import { Breadcrumbs, type BreadcrumbItem } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const basicItems: BreadcrumbItem[] = [
  { value: "home", label: "Home" },
  { value: "projects", label: "Projects" },
  { value: "poodle", label: "Poodle", current: true },
];

// Icon presentation is per item: a named glyph beside the label, or a visually
// icon-only root that is still announced as "Home".
const iconItems: BreadcrumbItem[] = [
  { value: "home", label: "Home", icon: "home", iconOnly: true },
  { value: "projects", label: "Projects", icon: "folder" },
  { value: "poodle", label: "Poodle", icon: "package", current: true },
];

const deepItems: BreadcrumbItem[] = [
  { value: "home", label: "Home" },
  { value: "workspace", label: "Workspace" },
  { value: "projects", label: "Projects" },
  { value: "poodle", label: "Poodle Design System" },
  { value: "primitives", label: "Primitives" },
  { value: "button", label: "Button", current: true },
];

export function BreadcrumbsSpecimen() {
  const [lastNav, setLastNav] = useState("");

  return (
    <SpecimenLayout
      sizes={(size) => <Breadcrumbs items={basicItems} size={size} />}
      densities={(density) => <Breadcrumbs items={basicItems} density={density} />}
    >
      <SpecimenGroup label="Basic">
        <Breadcrumbs items={basicItems} onNavigate={(value) => setLastNav(value)} />
        {lastNav ? (
          <p>Navigated to: <strong>{lastNav}</strong></p>
        ) : null}
      </SpecimenGroup>

      <SpecimenGroup label="Icons">
        <Breadcrumbs items={iconItems} />
      </SpecimenGroup>

      <SpecimenGroup label="Deep path">
        <Breadcrumbs items={deepItems} />
      </SpecimenGroup>

      <SpecimenGroup label="Collapsed (max 3 visible)">
        <Breadcrumbs items={deepItems} maxVisibleItems={3} />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
