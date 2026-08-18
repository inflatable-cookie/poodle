<script lang="ts">
  import { Tabs, type TabItem } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const sectionTabs: TabItem[] = [
    { value: "details", label: "Details" },
    { value: "usage", label: "Usage", count: 12, separator: true },
    { value: "versions", label: "Versions", count: 3 },
    { value: "archive", label: "Archive", disabled: true },
  ];

  const surfaceTabs: TabItem[] = [
    { value: "editor", label: "Editor", icon: "code" },
    { value: "preview", label: "Preview", icon: "eye" },
    { value: "terminal", label: "Terminal", icon: "terminal", closable: true },
    { value: "output", label: "Output", icon: "file-text", closable: true },
  ];

  const fileTabs: TabItem[] = [
    { value: "index.ts", label: "index.ts" },
    { value: "App.svelte", label: "App.svelte", closable: true },
    { value: "utils.ts", label: "utils.ts", closable: true },
    { value: "types.ts", label: "types.ts", closable: true },
  ];

  const panelTabs: TabItem[] = [
    { value: "explorer", label: "Explorer", icon: "folder" },
    { value: "search", label: "Search", icon: "search" },
    { value: "git", label: "Source Control", icon: "layers" },
    { value: "debug", label: "Debug", icon: "terminal" },
  ];

  // Four tabs with icons and counts — the shape that collapsed far too early.
  const shedItems: TabItem[] = [
    { value: "screens", label: "Screens", icon: "monitor", count: 12 },
    { value: "components", label: "Components", icon: "box", count: 12 },
    { value: "assets", label: "Assets", icon: "image", count: 375 },
    { value: "info", label: "Info", icon: "info" },
  ];

  let lastClosed = $state("");
  let lastReorder = $state("");
</script>

<SpecimenLayout>
  <div class="poodle-specimen">
    <SpecimenGroup label="Tabs over a panel — counts, a separator, and one disabled tab">
      <Tabs
        items={sectionTabs}
        defaultValue="details"
        bordered
        historyKey="tab"
        ariaLabel="Detail sections"
      >
        {#snippet children(activeValue)}
          <p>Active tab: <strong>{activeValue}</strong></p>
        {/snippet}
      </Tabs>

      <!-- `bordered` is the difference between tabs that sit above content and
           tabs that sit flush in a titlebar or toolbar. -->
      <p class="poodle-specimen__note">Without <code>bordered</code>, for titlebars and toolbars where the tabs are not above content:</p>
      <Tabs
        items={sectionTabs}
        defaultValue="details"
        bordered={false}
        ariaLabel="Flush section tabs"
      />
    </SpecimenGroup>

    <SpecimenGroup label="Variants — card, pill, and block">
      <Tabs items={sectionTabs} variant="card" defaultValue="details" ariaLabel="Card tabs" />
      <Tabs items={sectionTabs} variant="pill" defaultValue="details" ariaLabel="Pill tabs" />
      <div class="poodle-specimen__frame">
        <Tabs items={surfaceTabs} variant="block" defaultValue="editor" ariaLabel="Block tabs" />
      </div>
    </SpecimenGroup>

    <!-- activeEdge and activeFill are variant-agnostic, so one variant is
         enough to teach them. Showing the full product was six groups. -->
    <SpecimenGroup label="Marking the active tab — an edge, a fill, or both">
      <Tabs items={sectionTabs} variant="pill" activeEdge="outline" defaultValue="details" ariaLabel="Outlined tabs" />
      <Tabs items={sectionTabs} variant="pill" activeFill="solid" defaultValue="details" ariaLabel="Solid tabs" />
      <div class="poodle-specimen__frame">
        <Tabs
          items={surfaceTabs}
          variant="block"
          activeEdge="underline"
          activeFill="none"
          defaultValue="editor"
          ariaLabel="Underlined tabs"
        />
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Editable tabs — close one, or drag to reorder">
      <Tabs
        items={fileTabs}
        variant="card"
        defaultValue="App.svelte"
        reorderable
        ariaLabel="Open files"
        onClose={(value) => (lastClosed = value)}
        onReorder={(items) => (lastReorder = items.join(", "))}
      />
      {#if lastClosed}
        <p class="poodle-specimen__note">Closed: <strong>{lastClosed}</strong></p>
      {/if}
      {#if lastReorder}
        <p class="poodle-specimen__note">Reordered: <strong>{lastReorder}</strong></p>
      {/if}
    </SpecimenGroup>

    <SpecimenGroup label="When the row runs out of space — drag the right edge">
      <!-- Figmatic's case: a pane whose width the operator drags. Rather than
           one threshold into a menu, the strip gives up icons, then counts,
           then collapses — each at the width where it actually stops fitting,
           so label length and count magnitude move the points on their own. -->
      <div class="poodle-specimen__resizable">
        <Tabs
          items={shedItems}
          overflowStrategy="shed"
          collapseWhenOverflow
          ariaLabel="Graded overflow"
        />
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Vertical — a side panel's tab rail">
      <div class="poodle-specimen__frame poodle-specimen__frame--row">
        <Tabs
          items={panelTabs}
          variant="block"
          activeEdge="underline"
          orientation="vertical"
          defaultValue="explorer"
          ariaLabel="Side panel tabs"
        />
        <div class="poodle-specimen__surface-body">
          <p>Panel content</p>
        </div>
      </div>
    </SpecimenGroup>
  </div>

  {#snippet sizes(size)}
    <div class="poodle-specimen__axis">
      <Tabs items={sectionTabs} variant="card" defaultValue="details" ariaLabel={`${size} tabs`} {size} />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-specimen__axis">
      <Tabs items={sectionTabs} variant="card" defaultValue="details" ariaLabel={`${density} tabs`} {density} />
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

  .poodle-specimen__frame--row {
    display: flex;
    flex-direction: row;
  }

  .poodle-specimen__axis {
    width: min(100%, 28rem);
  }

  .poodle-specimen__resizable {
    resize: horizontal;
    overflow: auto;
    min-width: 12rem;
    max-width: 48rem;
    width: min(34rem, 100%);
    border: 0.0625rem dashed var(--poodle-color-border-subtle);
    padding: 0.5rem;
  }

  .poodle-specimen__surface-body {
    display: flex;
    flex: 1;
    align-items: center;
    justify-content: center;
    min-height: 8rem;
    color: var(--poodle-color-text-muted);
    font-size: 0.8125rem;
    background: var(--poodle-color-background-panel);
  }

  .poodle-specimen__note {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
  }
</style>
