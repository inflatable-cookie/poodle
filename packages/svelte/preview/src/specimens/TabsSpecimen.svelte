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

  let lastClosed = "";
  let lastReorder = "";
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
</style>
