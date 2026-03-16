<script lang="ts">
  import { ContextMenu, Eyebrow, Icon, ListCard, ListCardCounter, Pill, Badge } from "@pug/svelte-primitives";

  let lastClick = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Interactive list cards</Eyebrow>
    <div class="specimen__stack">
      <ListCard
        title="design-system-v2.figma"
        subtitle="Updated by Clay · 2h ago"
        meta="14.2 MB"
        isInteractive
        on:click={() => (lastClick = "design-system-v2.figma")}
      >
        <svelte:fragment slot="leading"><Icon name="folder" /></svelte:fragment>
      </ListCard>
      <ListCard
        title="component-specs.pdf"
        subtitle="Shared with team · Yesterday"
        meta="2.8 MB"
        isInteractive
        on:click={() => (lastClick = "component-specs.pdf")}
      >
        <svelte:fragment slot="leading"><Icon name="file-text" /></svelte:fragment>
      </ListCard>
      <ListCard
        title="brand-assets.zip"
        subtitle="Archived"
        meta="48 MB"
        isDisabled
      >
        <svelte:fragment slot="leading"><Icon name="layers" /></svelte:fragment>
      </ListCard>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Rounded-square leading (thumbnails)</Eyebrow>
    <div class="specimen__stack">
      <ListCard
        title="hero-banner.png"
        subtitle="Uploaded by Jamie · 4h ago"
        meta="3.1 MB"
        leadingShape="rounded-square"
        isInteractive
        on:click={() => (lastClick = "hero-banner.png")}
      >
        <svelte:fragment slot="leading"><Icon name="image" /></svelte:fragment>
      </ListCard>
      <ListCard
        title="onboarding-flow.mp4"
        subtitle="Screen recording · Today"
        meta="128 MB"
        leadingShape="rounded-square"
        isInteractive
        on:click={() => (lastClick = "onboarding-flow.mp4")}
      >
        <svelte:fragment slot="leading"><Icon name="play" /></svelte:fragment>
      </ListCard>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With badges</Eyebrow>
    <div class="specimen__stack">
      <ListCard
        title="API integration guide"
        subtitle="Updated yesterday"
        meta="Draft"
        isInteractive
        on:click={() => (lastClick = "API integration guide")}
      >
        <svelte:fragment slot="leading"><Icon name="file-text" /></svelte:fragment>
        <svelte:fragment slot="badges">
          <Pill tone="accent" size="xs">New</Pill>
        </svelte:fragment>
      </ListCard>
      <ListCard
        title="Q4 planning deck"
        subtitle="Shared with leadership"
        isInteractive
        on:click={() => (lastClick = "Q4 planning deck")}
      >
        <svelte:fragment slot="leading"><Icon name="layers" /></svelte:fragment>
        <svelte:fragment slot="badges">
          <Badge variant="muted">3</Badge>
          <Pill tone="caution" size="xs">Review</Pill>
        </svelte:fragment>
      </ListCard>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With footer counters</Eyebrow>
    <div class="specimen__stack">
      <ListCard
        title="Design system"
        subtitle="12 contributors"
        leadingShape="rounded-square"
        isInteractive
        on:click={() => (lastClick = "Design system")}
      >
        <svelte:fragment slot="leading"><Icon name="folder" /></svelte:fragment>
        <svelte:fragment slot="badges">
          <Pill tone="positive" size="xs">Active</Pill>
        </svelte:fragment>
        <svelte:fragment slot="footer">
          <ListCardCounter icon="file-text" count={24} tooltip="24 documents" />
          <ListCardCounter icon="image" count={8} tooltip="8 images" />
          <ListCardCounter icon="layers" count={3} tooltip="3 sub-folders" href="#sub-folders" />
        </svelte:fragment>
      </ListCard>
      <ListCard
        title="Brand guidelines"
        subtitle="Last updated 2 weeks ago"
        leadingShape="rounded-square"
        isInteractive
        on:click={() => (lastClick = "Brand guidelines")}
      >
        <svelte:fragment slot="leading"><Icon name="folder" /></svelte:fragment>
        <svelte:fragment slot="footer">
          <ListCardCounter icon="file-text" count={6} tooltip="6 documents" />
          <ListCardCounter icon="image" count={42} tooltip="42 images" />
        </svelte:fragment>
      </ListCard>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With context menu</Eyebrow>
    <div class="specimen__stack">
      <ContextMenu
        items={[
          { label: "Open", value: "open" },
          { label: "Rename", value: "rename" },
          { label: "Duplicate", value: "duplicate" },
          { kind: "separator", label: "", value: "sep-1" },
          { label: "Delete", value: "delete" },
        ]}
        on:action={(e) => (lastClick = `Action: ${e.detail.value}`)}
      >
        <ListCard
          title="Right-click for actions"
          subtitle="Context menu on the whole card"
          meta="12 KB"
          isInteractive
          on:click={() => (lastClick = "Card clicked")}
        >
          <svelte:fragment slot="leading"><Icon name="file-text" /></svelte:fragment>
        </ListCard>
      </ContextMenu>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Static list card</Eyebrow>
    <ListCard title="Read-only item" subtitle="No click handler" />
  </div>

  {#if lastClick}
    <div class="specimen__group">
      <Eyebrow>Last click</Eyebrow>
      <p>{lastClick}</p>
    </div>
  {/if}
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1.5rem; }
  .specimen__group { display: flex; flex-direction: column; gap: 0.5rem; }
  .specimen__stack { display: flex; flex-direction: column; gap: 0.25rem; }
  p { margin: 0; }
</style>
