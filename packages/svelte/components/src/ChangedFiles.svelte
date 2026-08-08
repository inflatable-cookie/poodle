<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/changed-files.css";

  import {
    buildChangedFileTree,
    changedFileScopes,
    changedFilesTotals,
    type ChangedFileNode,
  } from "@inflatable-cookie/poodle-core";

  import Button from "./Button.svelte";
  import Icon from "./Icon.svelte";
  import Tree from "./Tree.svelte";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation";

  import type {
    ChangedFile,
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
    id: string;
    files?: ChangedFile[];
    expanded?: boolean;
    expandedPaths?: string[];
    chipLimit?: number;
    showOpenDiff?: boolean;
    openDiffLabel?: string;
    showFilesLabel?: string;
    hideFilesLabel?: string;
    countLabel?: (count: number) => string;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onToggle?: ((id: string) => void) | undefined;
    onOpenDiff?: ((id: string) => void) | undefined;
    onFileSelect?: ((path: string) => void) | undefined;
    onExpandedPathsChange?: ((paths: string[]) => void) | undefined;
  }

  let {
    id,
    files = [],
    expanded = $bindable<boolean>(false),
    expandedPaths = $bindable<string[]>([]),
    chipLimit = 3,
    showOpenDiff = true,
    openDiffLabel = "Open diff",
    showFilesLabel = "Show files",
    hideFilesLabel = "Hide files",
    countLabel = (count: number) => `${count} changed files`,
    size = null,
    sizeRole = "control",
    density = null,
    onToggle = undefined,
    onOpenDiff = undefined,
    onFileSelect = undefined,
    onExpandedPathsChange = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const glyphSize = $derived(resolveSupportingVisualSize(resolvedSize));

  const totals = $derived(changedFilesTotals(files));
  const scopes = $derived(changedFileScopes(files));
  const tree = $derived(buildChangedFileTree(files));
  const visibleChips = $derived(files.slice(0, chipLimit));

  /** Counts are colour-coded, and colour alone is not a signal. */
  const headerName = $derived(
    `${countLabel(totals.fileCount)}, ${totals.additions} added, ${totals.deletions} removed`,
  );

  const treeNodes = $derived(toTreeNodes(tree));

  function toTreeNodes(nodes: ChangedFileNode[]): any[] {
    return nodes.map((node) => ({
      value: node.path,
      label: node.label,
      isBranch: node.isDirectory,
      children: node.children.length > 0 ? toTreeNodes(node.children) : undefined,
      meta: { additions: node.additions, deletions: node.deletions },
    }));
  }

  function toggle(): void {
    expanded = !expanded;
    onToggle?.(id);
  }
</script>

<!-- An empty card renders nothing rather than an empty state. A turn that
     changed no files should not have a box saying so — the absence is the
     message. -->
{#if files.length > 0}
  <div
    class="poodle-changed-files"
    data-expanded={String(expanded)}
    data-file-count={totals.fileCount}
    data-size={resolvedSize}
    data-density={resolvedDensity}
  >
    <div class="poodle-changed-files__header">
      <button
        type="button"
        class="poodle-changed-files__toggle"
        aria-expanded={expanded}
        aria-controls={`${id}-files`}
        aria-label={headerName}
        onclick={toggle}
      >
        <span class="poodle-changed-files__toggle-icon"><Icon name="chevron-right" size={glyphSize} /></span>
        <span class="poodle-changed-files__count">{countLabel(totals.fileCount)}</span>
        <span class="poodle-changed-files__additions">+{totals.additions}</span>
        <span class="poodle-changed-files__deletions">−{totals.deletions}</span>
      </button>

      <!-- The same action as the chevron, duplicated visually. Only one carries
           the accessible name: two controls announcing the same thing is a
           worse outcome than one. -->
      <button type="button" class="poodle-changed-files__files-toggle" aria-hidden="true" tabindex="-1" onclick={toggle}>
        {expanded ? hideFilesLabel : showFilesLabel}
      </button>

      {#if showOpenDiff}
        <div class="poodle-changed-files__actions">
          <Button
            variant="secondary"
            size={resolvedSize}
            icon="diff"
            onClick={() => onOpenDiff?.(id)}
          >
            {openDiffLabel}
          </Button>
        </div>
      {/if}
    </div>

    {#if expanded}
      <div class="poodle-changed-files__tree" id={`${id}-files`}>
        <Tree
          nodes={treeNodes}
          expandedValues={expandedPaths}
          collapseTwistyWhenFlat
          size={resolvedSize}
          density={resolvedDensity}
          onExpandedChange={(paths: string[]) => {
            expandedPaths = paths;
            onExpandedPathsChange?.(paths);
          }}
          onActivate={(value: string) => onFileSelect?.(value)}
        />
      </div>
    {:else}
      <div class="poodle-changed-files__summary" id={`${id}-files`}>
        <div class="poodle-changed-files__scopes">
          {#each scopes as scope (scope.name)}
            <span>
              <span class="poodle-changed-files__scope-name">{scope.name}</span>
              {scope.fileCount} {scope.fileCount === 1 ? "file" : "files"}
            </span>
          {/each}
        </div>

        {#each visibleChips as file (file.path)}
          <button
            type="button"
            class="poodle-changed-files__chip"
            onclick={() => onFileSelect?.(file.path)}
          >
            <Icon name="file" size={glyphSize} />
            <span class="poodle-changed-files__chip-label">{file.path.split("/").pop()}</span>
          </button>
        {/each}

        {#if files.length > visibleChips.length}
          <button type="button" class="poodle-changed-files__more" onclick={toggle}>
            Show all {totals.fileCount} files
          </button>
        {/if}
      </div>
    {/if}
  </div>
{/if}
