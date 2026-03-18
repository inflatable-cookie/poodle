<script lang="ts">
  import { Tabs, Eyebrow, type TabItem } from "@pug/svelte-primitives";

  const basicTabs: TabItem[] = [
    { value: "overview", label: "Overview" },
    { value: "features", label: "Features" },
    { value: "pricing", label: "Pricing" },
    { value: "faq", label: "FAQ", isDisabled: true },
  ];

  const iconTabs: TabItem[] = [
    { value: "home", label: "Home", icon: "house" },
    { value: "settings", label: "Settings", icon: "settings" },
    { value: "users", label: "Users", icon: "users" },
  ];

  const closableTabs: TabItem[] = [
    { value: "index.ts", label: "index.ts" },
    { value: "App.svelte", label: "App.svelte", isClosable: true },
    { value: "utils.ts", label: "utils.ts", isClosable: true },
    { value: "types.ts", label: "types.ts", isClosable: true },
  ];

  const stripTabs: TabItem[] = [
    { value: "editor", label: "Editor", icon: "code" },
    { value: "preview", label: "Preview", icon: "eye" },
    { value: "terminal", label: "Terminal", icon: "terminal", isClosable: true },
    { value: "output", label: "Output", icon: "file-text", isClosable: true },
  ];

  const panelTabs: TabItem[] = [
    { value: "explorer", label: "Explorer", icon: "folder", isClosable: true },
    { value: "search", label: "Search", icon: "search", isClosable: true },
    { value: "git", label: "Source Control", icon: "layers", isClosable: true },
    { value: "debug", label: "Debug", icon: "terminal", isClosable: true },
  ];

  let lastClosed = "";
  let lastReorder = "";
  let panelCollapsed = false;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Underline variant (default, with panel)</Eyebrow>
    <Tabs
      items={basicTabs}
      defaultValue="overview"
      ariaLabel="Section tabs"
      let:activeValue
    >
      <p>Active tab: <strong>{activeValue}</strong></p>
    </Tabs>
  </div>

  <div class="specimen__group">
    <Eyebrow>Card variant (closable, reorderable)</Eyebrow>
    <Tabs
      items={closableTabs}
      variant="card"
      defaultValue="App.svelte"
      isReorderable
      ariaLabel="Open files"
      on:close={(e) => (lastClosed = e.detail.value)}
      on:reorder={(e) => (lastReorder = e.detail.items.join(", "))}
    />
    {#if lastClosed}
      <p>Closed: <strong>{lastClosed}</strong></p>
    {/if}
    {#if lastReorder}
      <p>Reordered: <strong>{lastReorder}</strong></p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>Pill variant (with icons)</Eyebrow>
    <Tabs
      items={iconTabs}
      variant="pill"
      defaultValue="home"
      ariaLabel="Navigation"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Underline (with icons, no panel)</Eyebrow>
    <Tabs
      items={iconTabs}
      defaultValue="home"
      ariaLabel="Icon tabs"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Strip variant (full-width bar with icons, closable, reorderable)</Eyebrow>
    <div class="specimen__frame">
      <Tabs
        items={stripTabs}
        variant="strip"
        defaultValue="editor"
        isReorderable
        ariaLabel="Workspace surfaces"
        on:close={(e) => (lastClosed = e.detail.value)}
        on:reorder={(e) => (lastReorder = e.detail.items.join(", "))}
      />
      <div class="specimen__surface-body">
        <p>Surface content area</p>
      </div>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Strip variant — vertical (icon-only, collapsed panel)</Eyebrow>
    <div class="specimen__frame specimen__frame--row">
      <Tabs
        items={panelTabs}
        variant="strip"
        orientation="vertical"
        defaultValue="explorer"
        ariaLabel="Side panel tabs"
      />
      <div class="specimen__surface-body specimen__surface-body--fill">
        <p>Panel content</p>
      </div>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Strip variant — collapse toggle (click to toggle orientation)</Eyebrow>
    <div class="specimen__frame specimen__frame--row">
      {#if !panelCollapsed}
        <div class="specimen__panel-expanded">
          <Tabs
            items={panelTabs}
            variant="strip"
            orientation="horizontal"
            defaultValue="explorer"
            isReorderable
            ariaLabel="Side panel tabs"
            on:close={(e) => (lastClosed = e.detail.value)}
          />
          <div class="specimen__surface-body specimen__surface-body--fill">
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
        class="specimen__collapse-btn"
        on:click={() => (panelCollapsed = !panelCollapsed)}
      >
        {panelCollapsed ? "→" : "←"}
      </button>
    </div>
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__group :global(p) {
    margin: 0;
    font-size: 0.875rem;
    color: var(--pug-color-text-secondary);
  }

  .specimen__frame {
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    overflow: hidden;
  }

  .specimen__surface-body {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 6rem;
    color: var(--pug-color-text-muted);
    font-size: 0.8125rem;
    background: var(--pug-color-background-panel);
  }

  .specimen__surface-body--fill {
    flex: 1;
    height: auto;
    min-height: 8rem;
  }

  .specimen__frame--row {
    display: flex;
    flex-direction: row;
  }

  .specimen__panel-expanded {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
  }

  .specimen__collapse-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    min-height: 0;
    padding: 0;
    border: 0;
    border-left: 0.0625rem solid var(--pug-color-border-subtle);
    background: var(--pug-color-background-surface);
    color: var(--pug-color-text-muted);
    cursor: pointer;
    font-size: 0.75rem;
  }

  .specimen__collapse-btn:hover {
    background: var(--pug-color-surface-hover);
    color: var(--pug-color-text-primary);
  }
</style>
