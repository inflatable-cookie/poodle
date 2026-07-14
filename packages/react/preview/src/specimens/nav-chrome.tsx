import { useState } from "react";
import {
  AppHeader,
  Breadcrumbs,
  Button,
  IconButton,
  NavigationMenu,
  SidebarNav,
  type BreadcrumbItem,
  type NavigationMenuItem,
  type SidebarNavGroup,
} from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

const sidebarGroups: SidebarNavGroup[] = [
  {
    id: "workspace",
    label: "Workspace",
    items: [
      { value: "overview", label: "Overview" },
      { value: "projects", label: "Projects", href: "#projects" },
      { value: "archive", label: "Archive", disabled: true },
    ],
  },
  {
    id: "settings",
    label: "Settings",
    items: [
      { value: "members", label: "Members" },
      { value: "billing", label: "Billing" },
    ],
  },
];

const breadcrumbItems: BreadcrumbItem[] = [
  { value: "home", label: "Home", href: "#home" },
  { value: "library", label: "Library" },
  { value: "components", label: "Components" },
  { value: "poodle", label: "Poodle" },
  { value: "breadcrumbs", label: "Breadcrumbs", current: true },
];

const navMenuItems: NavigationMenuItem[] = [
  { value: "products", label: "Products", description: "Product lineup" },
  { value: "solutions", label: "Solutions" },
  { value: "legacy", label: "Legacy", disabled: true },
  { value: "resources", label: "Resources" },
];

function NavChromeDemo() {
  const [lastEvent, setLastEvent] = useState("");
  const [sidebarValue, setSidebarValue] = useState<string>("overview");

  return (
    <>
      <SpecimenSection title="AppHeader">
        <AppHeader
          title="Poodle Studio"
          subtitle="Design system workbench"
          actions={<Button size="sm">New file</Button>}
          utility={<IconButton icon="settings" ariaLabel="Settings" variant="ghost" />}
        />
      </SpecimenSection>

      <SpecimenSection title="SidebarNav">
        <div style={{ maxWidth: "16rem" }}>
          <SidebarNav
            groups={sidebarGroups}
            value={sidebarValue}
            ariaLabel="Workspace navigation"
            onValueChange={(value) => {
              setSidebarValue(value);
              setLastEvent(`sidebar:${value}`);
            }}
          />
        </div>
      </SpecimenSection>

      <SpecimenSection title="Breadcrumbs">
        <Breadcrumbs items={breadcrumbItems} onNavigate={(value) => setLastEvent(`crumb:${value}`)} />
        <Breadcrumbs
          items={breadcrumbItems}
          maxVisibleItems={3}
          ariaLabel="Collapsed breadcrumb"
          onNavigate={(value) => setLastEvent(`crumb:${value}`)}
        />
      </SpecimenSection>

      <SpecimenSection title="NavigationMenu">
        <NavigationMenu
          items={navMenuItems}
          ariaLabel="Primary"
          onValueChange={(value) => setLastEvent(`navmenu:${value ?? "closed"}`)}
        >
          {(value, item) => (
            <div style={{ padding: "0.5rem" }} data-testid="nav-panel">
              <strong>{item?.label}</strong>
              <p style={{ margin: 0 }}>{item?.description ?? `Panel for ${value}`}</p>
            </div>
          )}
        </NavigationMenu>
      </SpecimenSection>

      {lastEvent ? (
        <SpecimenSection title="Last event">
          <p data-testid="last-event">{lastEvent}</p>
        </SpecimenSection>
      ) : null}
    </>
  );
}

registerSpecimen({
  slug: "nav-chrome",
  title: "AppHeader / SidebarNav / Breadcrumbs / NavigationMenu",
  render: () => <NavChromeDemo />,
});
