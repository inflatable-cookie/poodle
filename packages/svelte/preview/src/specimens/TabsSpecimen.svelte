<script lang="ts">
  import { Tabs, type TabItem } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const basicTabs: TabItem[] = [
    { value: "overview", label: "Overview" },
    { value: "features", label: "Features" },
    { value: "pricing", label: "Pricing" },
    { value: "faq", label: "FAQ", disabled: true },
  ];

  const iconTabs: TabItem[] = [
    { value: "home", label: "Home", icon: "house" },
    { value: "settings", label: "Settings", icon: "settings" },
    { value: "users", label: "Users", icon: "users" },
  ];

  const closableTabs: TabItem[] = [
    { value: "index.ts", label: "index.ts" },
    { value: "App.svelte", label: "App.svelte", closable: true },
    { value: "utils.ts", label: "utils.ts", closable: true },
    { value: "types.ts", label: "types.ts", closable: true },
  ];

  const stripTabs: TabItem[] = [
    { value: "editor", label: "Editor", icon: "code" },
    { value: "preview", label: "Preview", icon: "eye" },
    { value: "terminal", label: "Terminal", icon: "terminal", closable: true },
    { value: "output", label: "Output", icon: "file-text", closable: true },
  ];

  const panelTabs: TabItem[] = [
    { value: "explorer", label: "Explorer", icon: "folder", closable: true },
    { value: "search", label: "Search", icon: "search", closable: true },
    { value: "git", label: "Source Control", icon: "layers", closable: true },
    { value: "debug", label: "Debug", icon: "terminal", closable: true },
  ];

  const detailTabs: TabItem[] = [
    { value: "details", label: "Details" },
    { value: "usage", label: "Usage", count: 12, separator: true },
    { value: "versions", label: "Versions", count: 3 },
  ];

  let lastClosed = $state("");
  let lastReorder = $state("");
  let panelCollapsed = $state(false);

  // Four tabs with icons and counts — the shape that collapsed far too early.
  const shedItems = [
    { value: "screens", label: "Screens", icon: "monitor", count: 12 },
    { value: "components", label: "Components", icon: "box", count: 12 },
    { value: "assets", label: "Assets", icon: "image", count: 375 },
    { value: "info", label: "Info", icon: "info" },
  ];
</script>

<SpecimenLayout>
  <div class="poodle-specimen">
    <SpecimenGroup label="Graded overflow (drag the handle)">
      <!-- Figmatic's case: a pane whose width the operator drags. Rather than
           one threshold into a menu, the strip gives up icons, then counts,
           then collapses — each at the width where it actually stops fitting,
           so label length and count magnitude move the points on their own. -->
      <div style="resize:horizontal;overflow:auto;min-width:12rem;max-width:48rem;width:34rem;border:1px dashed var(--poodle-color-border-subtle);padding:0.5rem;">
        <Tabs
          items={shedItems}
          overflowStrategy="shed"
          collapseWhenOverflow
          ariaLabel="Graded overflow"
        />
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Text variant (default, with indicator line)">
      <Tabs
        items={basicTabs}
        defaultValue="overview"
        ariaLabel="Section tabs"
      >
        {#snippet children(activeValue)}
        <p>Active tab: <strong>{activeValue}</strong></p>
        {/snippet}
      </Tabs>
    </SpecimenGroup>

    <SpecimenGroup label="Text variant (no border)">
      <Tabs
        items={basicTabs}
        defaultValue="overview"
        bordered={false}
        ariaLabel="Section tabs without border"
      />
    </SpecimenGroup>

    <SpecimenGroup label="Card variant (closable, reorderable)">
        <Tabs
          items={closableTabs}
          variant="card"
          defaultValue="App.svelte"
          reorderable
          ariaLabel="Open files"
          onClose={(value) => (lastClosed = value)}
          onReorder={(items) => (lastReorder = items.join(", "))}
        />
      {#if lastClosed}
        <p>Closed: <strong>{lastClosed}</strong></p>
      {/if}
      {#if lastReorder}
        <p>Reordered: <strong>{lastReorder}</strong></p>
      {/if}
    </SpecimenGroup>

    <SpecimenGroup label="Block variant (full-width shell tabs with separators)">
      <div class="poodle-specimen__frame">
        <Tabs
          items={stripTabs}
          variant="block"
          defaultValue="editor"
          reorderable
          ariaLabel="Workspace surfaces"
        />
        <div class="poodle-specimen__surface-body">
          <p>Surface content area</p>
        </div>
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Pill variant (with icons)">
      <Tabs
        items={iconTabs}
        variant="pill"
        defaultValue="home"
        ariaLabel="Navigation"
      />
    </SpecimenGroup>

    <SpecimenGroup label="Underline (with icons, no panel)">
      <Tabs
        items={iconTabs}
        defaultValue="home"
        ariaLabel="Icon tabs"
      />
    </SpecimenGroup>

    <SpecimenGroup label="Strip variant (full-width bar with icons, closable, reorderable)">
      <div class="poodle-specimen__frame">
        <Tabs
          items={stripTabs}
          variant="strip"
          defaultValue="editor"
          reorderable
          ariaLabel="Workspace surfaces"
          onClose={(value) => (lastClosed = value)}
          onReorder={(items) => (lastReorder = items.join(", "))}
        />
        <div class="poodle-specimen__surface-body">
          <p>Surface content area</p>
        </div>
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Strip variant — vertical (icon-only, collapsed panel)">
      <div class="poodle-specimen__frame poodle-specimen__frame--row">
        <Tabs
          items={panelTabs}
          variant="strip"
          orientation="vertical"
          defaultValue="explorer"
          ariaLabel="Side panel tabs"
        />
        <div class="poodle-specimen__surface-body poodle-specimen__surface-body--fill">
          <p>Panel content</p>
        </div>
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Strip variant — collapse toggle (click to toggle orientation)">
      <div class="poodle-specimen__frame poodle-specimen__frame--row">
        {#if !panelCollapsed}
          <div class="poodle-specimen__panel-expanded">
            <Tabs
              items={panelTabs}
              variant="strip"
              orientation="horizontal"
              defaultValue="explorer"
              reorderable
              ariaLabel="Side panel tabs"
              onClose={(value) => (lastClosed = value)}
            />
            <div class="poodle-specimen__surface-body poodle-specimen__surface-body--fill">
              <p>Panel body — expanded</p>
            </div>
          </div>
        {:else}
          <Tabs
            items={panelTabs}
            variant="strip"
            orientation="vertical"
            defaultValue="explorer"
            ariaLabel="Side panel tabs"
          />
        {/if}
        <button
          class="poodle-specimen__collapse-btn"
          onclick={() => (panelCollapsed = !panelCollapsed)}
        >
          {panelCollapsed ? "→" : "←"}
        </button>
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Card variant with counts, separators, and URL sync">
      <Tabs
        items={detailTabs}
        variant="card"
        defaultValue="details"
        historyKey="tab"
        ariaLabel="Detail sections"
      >
        {#snippet children(activeValue)}
        <p>Active tab: <strong>{activeValue}</strong></p>
        {/snippet}
      </Tabs>
    </SpecimenGroup>
  </div>

  {#snippet sizes(size)}
    <div class="poodle-specimen__variants-demo">
      <Tabs
        items={detailTabs}
        variant="card"
        defaultValue="details"
        ariaLabel={`${size} tabs`}
        {size}
      />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-specimen__variants-demo">
      <Tabs
        items={detailTabs}
        variant="card"
        defaultValue="details"
        ariaLabel={`${density} tabs`}
        {density}
      />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__frame {
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    overflow: hidden;
  }

  .poodle-specimen__variants-demo {
    width: min(100%, 28rem);
  }

  .poodle-specimen__surface-body {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 6rem;
    color: var(--poodle-color-text-muted);
    font-size: 0.8125rem;
    background: var(--poodle-color-background-panel);
  }

  .poodle-specimen__surface-body--fill {
    flex: 1;
    height: auto;
    min-height: 8rem;
  }

  .poodle-specimen__frame--row {
    display: flex;
    flex-direction: row;
  }

  .poodle-specimen__panel-expanded {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
  }

  .poodle-specimen__collapse-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    min-height: 0;
    padding: 0;
    border: 0;
    border-left: 0.0625rem solid var(--poodle-color-border-subtle);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-muted);
    cursor: pointer;
    font-size: 0.75rem;
  }

  .poodle-specimen__collapse-btn:hover {
    background: var(--poodle-color-surface-hover);
    color: var(--poodle-color-text-primary);
  }
</style>
