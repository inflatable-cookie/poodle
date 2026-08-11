import type { CSSProperties } from "react";
import { AppHeader, Button, IconButton, Menubar, Tabs } from "@inflatable-cookie/poodle-react";
import type { MenubarItem, TabsItem } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

// Destination tabs in the centre region (mirrors soundcheck's centred
// header: symmetric side columns keep the middle truly centred).
const centerTabs: TabsItem[] = [
  { value: "editor", label: "Editor", icon: "code" },
  { value: "preview", label: "Preview", icon: "eye" },
  { value: "terminal", label: "Terminal", icon: "terminal" },
];

const menuItems: MenubarItem[] = [
  {
    value: "file",
    label: "File",
    items: [
      { value: "new", label: "New File", shortcutLabel: "Ctrl+N" },
      { value: "open", label: "Open...", shortcutLabel: "Ctrl+O" },
      { value: "save", label: "Save", shortcutLabel: "Ctrl+S" },
      { value: "sep1", label: "", kind: "separator" },
      { value: "exit", label: "Exit" },
    ],
  },
  {
    value: "edit",
    label: "Edit",
    items: [
      { value: "undo", label: "Undo", shortcutLabel: "Ctrl+Z" },
      { value: "redo", label: "Redo", shortcutLabel: "Ctrl+Shift+Z" },
      { value: "sep1", label: "", kind: "separator" },
      { value: "cut", label: "Cut", shortcutLabel: "Ctrl+X" },
      { value: "copy", label: "Copy", shortcutLabel: "Ctrl+C" },
      { value: "paste", label: "Paste", shortcutLabel: "Ctrl+V" },
    ],
  },
  {
    value: "view",
    label: "View",
    items: [
      { value: "sidebar", label: "Toggle Sidebar", shortcutLabel: "Ctrl+B" },
      { value: "terminal", label: "Toggle Terminal", shortcutLabel: "Ctrl+`" },
      { value: "sep1", label: "", kind: "separator" },
      { value: "zoom-in", label: "Zoom In", shortcutLabel: "Ctrl+=" },
      { value: "zoom-out", label: "Zoom Out", shortcutLabel: "Ctrl+-" },
    ],
  },
  {
    value: "help",
    label: "Help",
    items: [
      { value: "docs", label: "Documentation" },
      { value: "about", label: "About" },
    ],
  },
];

const appShellStyle: CSSProperties = {
  border: "0.0625rem solid var(--poodle-color-border-subtle)",
  borderRadius: "var(--poodle-radius-surface)",
  overflow: "hidden",
};

const appBodyStyle: CSSProperties = {
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  height: "8rem",
  color: "var(--poodle-color-text-muted)",
  fontSize: "0.8125rem",
  background: "var(--poodle-color-background-panel)",
};

const logoStyle: CSSProperties = {
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "center",
  width: "1.5rem",
  height: "1.5rem",
  borderRadius: "0.25rem",
  background: "var(--poodle-color-accent-base)",
  color: "#fff",
  fontSize: "0.75rem",
  fontWeight: 700,
};

const variantBlockStyle: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "0.5rem",
  width: "min(40rem, 100%)",
};

const variantLabelStyle: CSSProperties = {
  color: "var(--poodle-color-text-muted)",
  fontSize: "0.75rem",
  fontWeight: 700,
  letterSpacing: "0.16em",
  textTransform: "uppercase",
};

// Narrow-width posture: a ≤45rem frame shows the centred header reflowed
// to one row (the reflow itself is viewport-driven, see the contract).
const narrowFrameStyle: CSSProperties = {
  width: "min(40rem, 100%)",
};

const actionButtons = (
  <>
    <Button sizeRole="chrome" variant="ghost">New</Button>
    <Button sizeRole="chrome" variant="ghost">Open</Button>
  </>
);

const settingsUtility = <IconButton icon="settings" sizeRole="chrome" variant="ghost" ariaLabel="Settings" />;

export function AppHeaderSpecimen() {
  return (
    <SpecimenLayout
      bareVariants
      sizes={(size) => (
        <div style={variantBlockStyle}>
          <div style={variantLabelStyle}>{size.toUpperCase()}</div>
          <AppHeader title="My Application" size={size} actions={actionButtons} utility={settingsUtility} />
        </div>
      )}
      densities={(density) => (
        <div style={variantBlockStyle}>
          <div style={variantLabelStyle}>{density.toUpperCase()}</div>
          <AppHeader title="My Application" density={density} actions={actionButtons} utility={settingsUtility} />
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Full app window header (title + menubar + utility)" bare>
          <div style={appShellStyle}>
            <AppHeader
              title="Poodle Studio"
              actions={
                <div className="poodle-specimen__menubar-inline">
                  <Menubar items={menuItems} ariaLabel="Application menu" />
                </div>
              }
              utility={
                <>
                  <IconButton icon="search" sizeRole="chrome" variant="ghost" ariaLabel="Search" />
                  <IconButton icon="bell" sizeRole="chrome" variant="ghost" ariaLabel="Notifications" />
                  <IconButton icon="settings" sizeRole="chrome" variant="ghost" ariaLabel="Settings" />
                </>
              }
            />
            <div style={appBodyStyle}>
              <p style={{ margin: 0 }}>Application content area</p>
            </div>
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="With title, actions, and utility" bare>
          <AppHeader title="My Application" actions={actionButtons} utility={settingsUtility} />
        </SpecimenGroup>

        <SpecimenGroup label="Title only" bare>
          <AppHeader title="Poodle Workstation" />
        </SpecimenGroup>

        <SpecimenGroup label="Custom identity slot" bare>
          <AppHeader
            identity={
              <>
                <span style={logoStyle}>P</span>
                <strong>Poodle Studio</strong>
              </>
            }
            utility={
              <>
                <IconButton icon="bell" sizeRole="chrome" variant="ghost" ariaLabel="Notifications" />
                <IconButton icon="user" sizeRole="chrome" variant="ghost" ariaLabel="Account" />
              </>
            }
          />
        </SpecimenGroup>

        <SpecimenGroup label="Centred header (destination tabs in the centre)" bare>
          <AppHeader
            title="My Application"
            center={
              <Tabs
                items={centerTabs}
                variant="block"
                activeEdge="outline"
                defaultValue="editor"
                size="sm"
                ariaLabel="Destinations"
              />
            }
            actions={actionButtons}
            utility={settingsUtility}
          />
        </SpecimenGroup>

        <SpecimenGroup label="Centred header at narrow width (≤45rem viewport)" bare>
          <div style={narrowFrameStyle}>
            <AppHeader
              title="My Application"
              center={
                <Tabs
                  items={centerTabs}
                  variant="block"
                  activeEdge="outline"
                  defaultValue="editor"
                  size="sm"
                  ariaLabel="Destinations"
                />
              }
              actions={actionButtons}
              utility={settingsUtility}
            />
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
