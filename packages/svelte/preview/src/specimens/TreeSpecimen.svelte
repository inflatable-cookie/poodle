<script lang="ts">
  import { Tree, ContextMenu } from "@poodle/svelte";
  import type { TreeNode, MenuItem } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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

  // Interactive single-select explorer.
  let selected = $state<string[]>(["src/components/Tree.svelte"]);
  let expanded = $state<string[]>(["src", "src/components"]);

  // Interactive multi-select explorer.
  let multiSelected = $state<string[]>([
    "src/components/Button.svelte",
    "src/components/Tree.svelte",
  ]);
  let multiExpanded = $state<string[]>(["src", "src/components"]);

  // Checkbox cascade selection.
  let checked = $state<string[]>(["src/components/Button.svelte"]);

  // Lazy / async children loading.
  let lazyNodes = $state<TreeNode[]>([
    { value: "remote", label: "remote", icon: "folder", isBranch: true },
    { value: "cache", label: "cache", icon: "folder", isBranch: true },
  ]);
  let lazyExpanded = $state<string[]>([]);
  let lazyLoading = $state<string[]>([]);

  function findNode(list: TreeNode[], value: string): TreeNode | null {
    for (const node of list) {
      if (node.value === value) return node;
      const found = node.children ? findNode(node.children, value) : null;
      if (found) return found;
    }
    return null;
  }

  function loadChildren(value: string): void {
    if (lazyLoading.includes(value)) return;
    lazyLoading = [...lazyLoading, value];
    // Simulate an async fetch.
    setTimeout(() => {
      const node = findNode(lazyNodes, value);
      if (node) {
        node.children = [
          { value: `${value}/alpha.ts`, label: "alpha.ts", icon: "file" },
          { value: `${value}/beta.ts`, label: "beta.ts", icon: "file" },
          { value: `${value}/nested`, label: "nested", icon: "folder", isBranch: true },
        ];
      }
      lazyNodes = [...lazyNodes];
      lazyLoading = lazyLoading.filter((v) => v !== value);
    }, 900);
  }

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
  let bigExpanded = $state<string[]>(bigTree.map((n) => n.value));

  // Inline rename + right-click context menu.
  let renameNodes = $state<TreeNode[]>([
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
  let renameExpanded = $state<string[]>(["docs"]);
  let editing = $state<string | null>(null);
  let menuOpen = $state(false);
  let menuAnchor = $state<{ x: number; y: number } | null>(null);
  let menuTarget = $state<string | null>(null);
  const menuItems: MenuItem[] = [
    { value: "rename", label: "Rename" },
    { value: "delete", label: "Delete", tone: "danger" },
  ];

  function removeNode(list: TreeNode[], value: string): TreeNode[] {
    return list
      .filter((n) => n.value !== value)
      .map((n) => (n.children ? { ...n, children: removeNode(n.children, value) } : n));
  }
  function onRename(value: string, text: string): void {
    const node = findNode(renameNodes, value);
    if (node) node.label = text;
    renameNodes = [...renameNodes];
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
  function onReorderNodes(from: string, to: string, position: Pos): void {
    renameNodes = reorderNodes(renameNodes, from, to, position);
  }
  function openMenu(value: string, x: number, y: number): void {
    menuTarget = value;
    menuAnchor = { x, y };
    menuOpen = true;
  }
  function onMenuAction(action: string): void {
    if (action === "rename" && menuTarget) editing = menuTarget;
    else if (action === "delete" && menuTarget) renameNodes = removeNode(renameNodes, menuTarget);
    menuOpen = false;
  }
</script>

<SpecimenLayout>
  <div class="poodle-specimen">
    <SpecimenGroup label="File explorer">
      <div class="poodle-specimen__frame">
        <Tree
          ariaLabel="Project files"
          nodes={fileTree}
          bind:selectedValues={selected}
          bind:expandedValues={expanded}
        />
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Multi-select (Ctrl/Cmd + click, Shift + click)">
      <div class="poodle-specimen__frame">
        <Tree
          ariaLabel="Multi-select files"
          nodes={fileTree}
          bind:selectedValues={multiSelected}
          bind:expandedValues={multiExpanded}
        />
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="No guides, no icons">
      <div class="poodle-specimen__frame">
        <Tree
          ariaLabel="Plain tree"
          nodes={fileTree}
          showGuides={false}
          showIcons={false}
          defaultExpandedValues={["src", "src/components"]}
        />
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Checkbox cascade ({checked.length} checked)">
      <div class="poodle-specimen__frame">
        <Tree
          ariaLabel="Checkbox tree"
          nodes={fileTree}
          showCheckboxes
          bind:checkedValues={checked}
          defaultExpandedValues={["src", "src/components"]}
        />
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Lazy / async children (expand a folder)">
      <div class="poodle-specimen__frame">
        <Tree
          ariaLabel="Lazy tree"
          nodes={lazyNodes}
          bind:expandedValues={lazyExpanded}
          loadingValues={lazyLoading}
          onLoadChildren={loadChildren}
        />
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Virtualized — 1260 rows, only the window renders">
      <div class="poodle-specimen__frame poodle-specimen__frame--virtual">
        <Tree
          ariaLabel="Virtual tree"
          nodes={bigTree}
          virtualized
          virtualHeight={320}
          bind:expandedValues={bigExpanded}
        />
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Rename (F2) · right-click menu · drag or Alt+↑/↓ to reorder">
      <div class="poodle-specimen__frame">
        <Tree
          ariaLabel="Rename tree"
          nodes={renameNodes}
          bind:expandedValues={renameExpanded}
          bind:editingValue={editing}
          reorderable
          onRenameCommit={onRename}
          onContextMenu={openMenu}
          onReorder={onReorderNodes}
        />
      </div>
      <ContextMenu
        bind:open={menuOpen}
        anchorPoint={menuAnchor}
        items={menuItems}
        ariaLabel="Tree node actions"
        onAction={onMenuAction}
      />
    </SpecimenGroup>
  </div>

  {#snippet sizes(size)}
    <div class="poodle-specimen__frame">
      <Tree
        ariaLabel={`${size} tree`}
        nodes={fileTree}
        defaultExpandedValues={["src", "src/components"]}
        selectedValues={["src/components/Tree.svelte"]}
        {size}
      />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-specimen__frame">
      <Tree
        ariaLabel={`${density} tree`}
        nodes={fileTree}
        defaultExpandedValues={["src", "src/components"]}
        selectedValues={["src/components/Tree.svelte"]}
        {density}
      />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: grid;
    gap: 1rem;
  }

  .poodle-specimen__frame {
    width: 18rem;
    min-height: 14rem;
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 60%, transparent);
    border-radius: var(--poodle-radius-control);
    overflow: auto;
  }

  /* The virtualized tree owns its own scroll viewport. */
  .poodle-specimen__frame--virtual {
    min-height: 0;
    overflow: hidden;
  }
</style>
