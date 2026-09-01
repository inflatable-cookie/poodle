/**
 * g16.028 — the drag-and-drop programme's absence inventory.
 *
 * The certification claim is a negative one: after the migration waves, no
 * programme component owns an HTML drag lifecycle or a component-local drag
 * index any more, and every one of them reaches the common substrate instead.
 * A negative claim needs an executable proof, or it decays the first time
 * somebody plants the old path back.
 *
 * Run with `effigy drift:drag-inventory`.
 */
import fs from "node:fs";
import path from "node:path";

const ROOT = path.resolve(import.meta.dir, "..");

/**
 * The seven programme components, with the exact surfaces the inventory reads.
 *
 * `parts` are the framework sub-components a composite renders its rows
 * through; a lifecycle moved one file down is not a lifecycle removed.
 */
const PROGRAMME_COMPONENTS = [
  { name: "Tabs", svelte: "Tabs.svelte", react: "Tabs.tsx", rust: "tabs.rs", parts: ["tabs-parts"] },
  {
    name: "EditableList",
    svelte: "EditableList.svelte",
    react: "EditableList.tsx",
    rust: "editable_list.rs",
    parts: ["editable-list"],
  },
  { name: "Tree", svelte: "Tree.svelte", react: "Tree.tsx", rust: "tree.rs", parts: ["tree-item"] },
  {
    name: "DockRegion",
    svelte: "DockRegion.svelte",
    react: "DockRegion.tsx",
    rust: "dock_region.rs",
    parts: [],
  },
  {
    name: "ModelCatalogueEditor",
    svelte: "ModelCatalogueEditor.svelte",
    react: "ModelCatalogueEditor.tsx",
    rust: "model_catalogue_editor.rs",
    parts: ["model-catalogue-editor"],
  },
  { name: "OrderBy", svelte: "OrderBy.svelte", react: "OrderBy.tsx", rust: "order_by.rs", parts: ["order-by"] },
  {
    name: "BlockEditor",
    svelte: "BlockEditor.svelte",
    react: "BlockEditor.tsx",
    rust: "block_editor.rs",
    parts: ["block-editor"],
  },
] as const;

/**
 * Tokens that only exist in an HTML drag lifecycle or a component-local drag
 * session. Each names the exact thing the substrate replaced, so a failure
 * reads as a diagnosis rather than a lint hit.
 */
const BANNED_TOKENS: Array<{ pattern: RegExp; why: string }> = [
  { pattern: /\bdraggable\b/, why: "HTML `draggable` attribute; the substrate arms its own gesture" },
  { pattern: /\bdataTransfer\b/i, why: "`DataTransfer` payload; subjects are opaque and never serialized by a component" },
  { pattern: /\beffectAllowed\b/, why: "HTML drag operation negotiation; `allowedOperations` is the contract" },
  { pattern: /\bdropEffect\b/, why: "HTML drag operation negotiation; the session carries the operation" },
  { pattern: /\bondrag(start|end|over|enter|leave)\b/, why: "DOM drag event handler; the substrate owns the sensor" },
  { pattern: /\bondrop\b/, why: "DOM drop handler; a drop is a registered target's commit" },
  { pattern: /\bonDrag(Over|Enter|Leave)\b/, why: "React DOM drag handler; the substrate owns the sensor" },
  {
    pattern: /\b(dragIndex|dragOverIndex|dragSourceIndex|draggingId|dropTargetId)\b/,
    why: "component-local drag index/state; posture is read from the session snapshot",
  },
];

/**
 * Drag-shaped paths that are deliberately retained, each with a non-payload
 * reason grounded in its contract. An entry that stops matching is reported
 * too: a stale exemption is the same drift as a missing one.
 */
const RETAINED: Array<{ file: string; token: string; reason: string }> = [
  {
    file: "packages/core/src/dom/cross-window-data-transfer.ts",
    token: "dataTransfer",
    reason:
      "spec 069 §Cross-Window Host Bridge: the browser's own transport for a drag that leaves the window. Substrate-owned, not a component lifecycle.",
  },
  {
    file: "packages/core/src/dom/inbound-file-data-transfer.ts",
    token: "dataTransfer",
    reason:
      "spec 069 §Inbound Files: files arrive from outside the application through the platform's own drag transport. Substrate-owned.",
  },
  {
    file: "packages/core/src/dom/drag-drop-controller.ts",
    token: "draggable",
    reason:
      "spec 069 §Native DataTransfer Adapter: the controller arms `draggable` on a bridged source only, and disarms it on every terminal.",
  },
];

/**
 * Every framework surface the substrate must be reached through.
 *
 * Exact import specifiers, not substrings: `"./drag-drop-anything"` contains
 * `"./drag-drop"`, so a loose marker would accept a module that does not
 * exist.
 */
const SUBSTRATE_MARKERS = {
  svelte: ['from "./drag-drop-context"', 'from "./drag-drop"', 'from "../drag-drop"'],
  react: ['from "./drag-drop"', 'from "../drag-drop"'],
  rust: ["use crate::drag_drop::", "crate::drag_drop::"],
};

function read(relativePath: string): string {
  return fs.readFileSync(path.join(ROOT, relativePath), "utf8");
}

function exists(relativePath: string): boolean {
  return fs.existsSync(path.join(ROOT, relativePath));
}

function sourcesIn(relativeDirectory: string): string[] {
  const directory = path.join(ROOT, relativeDirectory);
  if (!fs.existsSync(directory)) return [];
  return fs
    .readdirSync(directory)
    .filter((entry) => entry.endsWith(".svelte") || entry.endsWith(".tsx") || entry.endsWith(".ts"))
    .map((entry) => `${relativeDirectory}/${entry}`)
    .sort();
}

function scan(relativePath: string, failures: string[]): void {
  const source = read(relativePath);
  const lines = source.split("\n");
  for (const { pattern, why } of BANNED_TOKENS) {
    lines.forEach((line, index) => {
      // A comment may name the thing that was removed; the inventory is about
      // code, and a rule that could not survive its own explanation would be
      // one nobody writes down.
      const stripped = line.replace(/\/\/.*$/, "").replace(/<!--[\s\S]*?-->/g, "");
      if (pattern.test(stripped)) {
        failures.push(`${relativePath}:${index + 1}: ${why}\n    ${line.trim()}`);
      }
    });
  }
}

export function runInventory(): string[] {
  const failures: string[] = [];

  for (const component of PROGRAMME_COMPONENTS) {
    const svelte = `packages/svelte/components/src/${component.svelte}`;
    const react = `packages/react/components/src/${component.react}`;
    const rust = `packages/render/src/${component.rust}`;
    for (const file of [svelte, react, rust]) {
      if (!exists(file)) {
        failures.push(`${component.name}: expected surface is missing: ${file}`);
        continue;
      }
    }

    const parts = component.parts.flatMap((part) => [
      ...sourcesIn(`packages/svelte/components/src/${part}`),
      ...sourcesIn(`packages/react/components/src/${part}`),
    ]);
    for (const file of [svelte, react, ...parts]) {
      if (exists(file)) scan(file, failures);
    }

    // Presence is the other half: a component with no HTML drag left and no
    // substrate registration would pass a pure absence check while reordering
    // nothing at all.
    const svelteSources = [svelte, ...parts.filter((file) => file.includes("/svelte/"))]
      .filter(exists)
      .map(read)
      .join("\n");
    if (!SUBSTRATE_MARKERS.svelte.some((marker) => svelteSources.includes(marker))) {
      failures.push(`${component.name}: the Svelte surface reaches no drag substrate module`);
    }
    const reactSources = [react, ...parts.filter((file) => file.includes("/react/"))]
      .filter(exists)
      .map(read)
      .join("\n");
    if (!SUBSTRATE_MARKERS.react.some((marker) => reactSources.includes(marker))) {
      failures.push(`${component.name}: the React surface reaches no drag substrate module`);
    }
    if (exists(rust)) {
      const rustSource = read(rust);
      if (!SUBSTRATE_MARKERS.rust.some((marker) => rustSource.includes(marker))) {
        failures.push(`${component.name}: the shared Rust surface reaches no drag substrate module`);
      }
    }
  }

  for (const entry of RETAINED) {
    if (!exists(entry.file)) {
      failures.push(`retained exemption points at a file that is gone: ${entry.file}`);
      continue;
    }
    if (!read(entry.file).includes(entry.token)) {
      failures.push(
        `retained exemption is stale: ${entry.file} no longer contains \`${entry.token}\`. Remove the entry.`,
      );
    }
  }

  return failures;
}

if (import.meta.main) {
  const failures = runInventory();
  if (failures.length > 0) {
    console.error("drag inventory failed:\n");
    for (const failure of failures) console.error(`  ${failure}`);
    console.error(
      `\n${failures.length} finding(s). Every programme component must reach the common substrate and own no HTML drag lifecycle.`,
    );
    process.exit(1);
  }
  console.log(
    `drag inventory: ${PROGRAMME_COMPONENTS.length} programme components on the common substrate; ${RETAINED.length} retained substrate-owned transports, each with a contract reason.`,
  );
}
