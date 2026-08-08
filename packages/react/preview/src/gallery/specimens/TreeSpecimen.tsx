import { useState, type CSSProperties } from "react";
import { Tree, ContextMenu, type TreeNode, type MenuItem } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const frameStyle: CSSProperties = {
  width: "18rem",
  minHeight: "14rem",
  border: "0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 60%, transparent)",
  borderRadius: "var(--poodle-radius-control)",
  overflow: "auto",
};

// The virtualized tree owns its own scroll viewport.
const virtualFrameStyle: CSSProperties = { ...frameStyle, minHeight: 0, overflow: "hidden" };

const fileTree: TreeNode[] = [
  {
    value: "src",
    label: "src",
    icon: "folder",
    children: [
      {
        value: "src/components",
        label: "components",
        icon: "folder",
        children: [
          { value: "src/components/Button.svelte", label: "Button.svelte", icon: "file" },
          { value: "src/components/Tree.svelte", label: "Tree.svelte", icon: "file" },
        ],
      },
      // Empty-but-branch folder (lazy / no children yet).
      { value: "src/lib", label: "lib", icon: "folder", isBranch: true },
      { value: "src/index.ts", label: "index.ts", icon: "file" },
    ],
  },
  { value: "package.json", label: "package.json", icon: "file" },
  { value: "README.md", label: "README.md", icon: "file" },
  {
    value: "node_modules",
    label: "node_modules",
    icon: "folder",
    isBranch: true,
    isDisabled: true,
  },
];

// Large tree for virtual scrolling: 60 folders × 20 files = 1260 rows.
const bigTree: TreeNode[] = Array.from({ length: 60 }, (_, i) => ({
  value: `folder-${i}`,
  label: `Folder ${i}`,
  icon: "folder",
  children: Array.from({ length: 20 }, (_, j) => ({
    value: `folder-${i}/file-${j}`,
    label: `file-${j}.ts`,
    icon: "file",
  })),
}));

const menuItems: MenuItem[] = [
  { value: "rename", label: "Rename" },
  { value: "delete", label: "Delete", tone: "danger" },
];

function findNode(list: TreeNode[], value: string): TreeNode | null {
  for (const node of list) {
    if (node.value === value) return node;
    const found = node.children ? findNode(node.children, value) : null;
    if (found) return found;
  }
  return null;
}

function removeNode(list: TreeNode[], value: string): TreeNode[] {
  return list
    .filter((n) => n.value !== value)
    .map((n) => (n.children ? { ...n, children: removeNode(n.children, value) } : n));
}

type Pos = "before" | "after" | "inside";

function subtreeContains(list: TreeNode[], value: string): boolean {
  return list.some((n) => n.value === value || subtreeContains(n.children ?? [], value));
}

function reorderNodes(nodes: TreeNode[], from: string, to: string, position: Pos): TreeNode[] {
  if (from === to) return nodes;
  const fromNode = findNode(nodes, from);
  if (!fromNode || subtreeContains(fromNode.children ?? [], to)) return nodes;
  let moved: TreeNode | null = null;
  const strip = (list: TreeNode[]): TreeNode[] => {
    const out: TreeNode[] = [];
    for (const n of list) {
      if (n.value === from) {
        moved = n;
        continue;
      }
      out.push(n.children ? { ...n, children: strip(n.children) } : n);
    }
    return out;
  };
  const without = strip(nodes);
  if (!moved) return nodes;
  const insert = (list: TreeNode[]): TreeNode[] => {
    if (position === "inside") {
      return list.map((n) =>
        n.value === to
          ? { ...n, children: [...(n.children ?? []), moved!] }
          : n.children
            ? { ...n, children: insert(n.children) }
            : n,
      );
    }
    const out: TreeNode[] = [];
    for (const n of list) {
      if (n.value === to && position === "before") out.push(moved!);
      out.push(n.children ? { ...n, children: insert(n.children) } : n);
      if (n.value === to && position === "after") out.push(moved!);
    }
    return out;
  };
  return insert(without);
}

export function TreeSpecimen() {
  // Interactive single-select explorer.
  const [selected, setSelected] = useState<string[]>(["src/components/Tree.svelte"]);
  const [expanded, setExpanded] = useState<string[]>(["src", "src/components"]);

  // Interactive multi-select explorer.
  const [multiSelected, setMultiSelected] = useState<string[]>([
    "src/components/Button.svelte",
    "src/components/Tree.svelte",
  ]);
  const [multiExpanded, setMultiExpanded] = useState<string[]>(["src", "src/components"]);

  // Checkbox cascade selection.
  const [checked, setChecked] = useState<string[]>(["src/components/Button.svelte"]);

  // Lazy / async children loading.
  const [lazyNodes, setLazyNodes] = useState<TreeNode[]>([
    { value: "remote", label: "remote", icon: "folder", isBranch: true },
    { value: "cache", label: "cache", icon: "folder", isBranch: true },
  ]);
  const [lazyExpanded, setLazyExpanded] = useState<string[]>([]);
  const [lazyLoading, setLazyLoading] = useState<string[]>([]);

  const [bigExpanded, setBigExpanded] = useState<string[]>(() => bigTree.map((n) => n.value));

  // Inline rename + right-click context menu.
  const [renameNodes, setRenameNodes] = useState<TreeNode[]>([
    {
      value: "docs",
      label: "docs",
      icon: "folder",
      children: [
        { value: "docs/intro.md", label: "intro.md", icon: "file" },
        { value: "docs/guide.md", label: "guide.md", icon: "file" },
      ],
    },
    { value: "notes.txt", label: "notes.txt", icon: "file" },
  ]);
  const [renameExpanded, setRenameExpanded] = useState<string[]>(["docs"]);
  const [editing, setEditing] = useState<string | null>(null);
  const [menuOpen, setMenuOpen] = useState(false);
  const [menuAnchor, setMenuAnchor] = useState<{ x: number; y: number } | null>(null);
  const [menuTarget, setMenuTarget] = useState<string | null>(null);

  function loadChildren(value: string): void {
    if (lazyLoading.includes(value)) return;
    setLazyLoading([...lazyLoading, value]);
    // Simulate an async fetch.
    setTimeout(() => {
      setLazyNodes((prev) => {
        const node = findNode(prev, value);
        if (node) {
          node.children = [
            { value: `${value}/alpha.ts`, label: "alpha.ts", icon: "file" },
            { value: `${value}/beta.ts`, label: "beta.ts", icon: "file" },
            { value: `${value}/nested`, label: "nested", icon: "folder", isBranch: true },
          ];
        }
        return [...prev];
      });
      setLazyLoading((prev) => prev.filter((v) => v !== value));
    }, 900);
  }

  function onRename(value: string, text: string): void {
    setRenameNodes((prev) => {
      const node = findNode(prev, value);
      if (node) node.label = text;
      return [...prev];
    });
  }

  function onReorderNodes(from: string, to: string, position: Pos): void {
    setRenameNodes((prev) => reorderNodes(prev, from, to, position));
  }

  function openMenu(value: string, x: number, y: number): void {
    setMenuTarget(value);
    setMenuAnchor({ x, y });
    setMenuOpen(true);
  }

  function onMenuAction(action: string): void {
    if (action === "rename" && menuTarget) setEditing(menuTarget);
    else if (action === "delete" && menuTarget) {
      const target = menuTarget;
      setRenameNodes((prev) => removeNode(prev, target));
    }
    setMenuOpen(false);
  }

  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={frameStyle}>
          <Tree
            ariaLabel={`${size} tree`}
            nodes={fileTree}
            defaultExpandedValues={["src", "src/components"]}
            selectedValues={["src/components/Tree.svelte"]}
            size={size}
          />
        </div>
      )}
      densities={(density) => (
        <div style={frameStyle}>
          <Tree
            ariaLabel={`${density} tree`}
            nodes={fileTree}
            defaultExpandedValues={["src", "src/components"]}
            selectedValues={["src/components/Tree.svelte"]}
            density={density}
          />
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="File explorer">
          <div style={frameStyle}>
            <Tree
              ariaLabel="Project files"
              nodes={fileTree}
              selectedValues={selected}
              onSelectionChange={setSelected}
              expandedValues={expanded}
              onExpandedChange={setExpanded}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Multi-select (Ctrl/Cmd + click, Shift + click)">
          <div style={frameStyle}>
            <Tree
              ariaLabel="Multi-select files"
              nodes={fileTree}
              selectedValues={multiSelected}
              onSelectionChange={setMultiSelected}
              expandedValues={multiExpanded}
              onExpandedChange={setMultiExpanded}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="No guides, no icons">
          <div style={frameStyle}>
            <Tree
              ariaLabel="Plain tree"
              nodes={fileTree}
              showGuides={false}
              showIcons={false}
              defaultExpandedValues={["src", "src/components"]}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label={`Checkbox cascade (${checked.length} checked)`}>
          <div style={frameStyle}>
            <Tree
              ariaLabel="Checkbox tree"
              nodes={fileTree}
              showCheckboxes
              checkedValues={checked}
              onCheckedChange={setChecked}
              defaultExpandedValues={["src", "src/components"]}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Lazy / async children (expand a folder)">
          <div style={frameStyle}>
            <Tree
              ariaLabel="Lazy tree"
              nodes={lazyNodes}
              expandedValues={lazyExpanded}
              onExpandedChange={setLazyExpanded}
              loadingValues={lazyLoading}
              onLoadChildren={loadChildren}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Virtualized — 1260 rows, only the window renders">
          <div style={virtualFrameStyle}>
            <Tree
              ariaLabel="Virtual tree"
              nodes={bigTree}
              virtualized
              virtualHeight={320}
              expandedValues={bigExpanded}
              onExpandedChange={setBigExpanded}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Rename (F2) · right-click menu · drag or Alt+↑/↓ to reorder">
          <div style={frameStyle}>
            <Tree
              ariaLabel="Rename tree"
              nodes={renameNodes}
              expandedValues={renameExpanded}
              onExpandedChange={setRenameExpanded}
              editingValue={editing}
              onEditingChange={setEditing}
              reorderable
              onRenameCommit={onRename}
              onContextMenu={openMenu}
              onReorder={onReorderNodes}
            />
          </div>
          <ContextMenu
            open={menuOpen}
            onOpenChange={setMenuOpen}
            anchorPoint={menuAnchor}
            items={menuItems}
            ariaLabel="Tree node actions"
            onAction={onMenuAction}
          />
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
