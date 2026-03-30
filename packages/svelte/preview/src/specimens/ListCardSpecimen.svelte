<script lang="ts">
  import { ContextMenu, Eyebrow, Icon, IconButton, ListCard, ListCardCounter, Pill } from "@poodle/svelte-primitives";
  import { folder, layers, grid2x2 } from "@poodle/icons-lucide";

  let lastClick = "";
  let selectedCard = false;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Interactive list cards</Eyebrow>
    <div class="specimen__stack">
      <ListCard
        title="design-system-v2.figma"
        subtitle="Updated by Clay · 2h ago"
        meta="14.2 MB"
        interactive
        on:click={() => (lastClick = "design-system-v2.figma")}
      >
        <svelte:fragment slot="leading"><Icon icon={folder} /></svelte:fragment>
      </ListCard>
      <ListCard
        title="component-specs.pdf"
        subtitle="Shared with team · Yesterday"
        meta="2.8 MB"
        interactive
        on:click={() => (lastClick = "component-specs.pdf")}
      >
        <svelte:fragment slot="leading"><Icon name="file-text" /></svelte:fragment>
      </ListCard>
      <ListCard
        title="brand-assets.zip"
        subtitle="Archived"
        meta="48 MB"
        disabled
      >
        <svelte:fragment slot="leading"><Icon icon={layers} /></svelte:fragment>
      </ListCard>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Link roots and explicit actions</Eyebrow>
    <div class="specimen__stack">
      <ListCard
        title="Billing settings"
        subtitle="Manage invoices and payment methods"
        href="#billing"
      >
        <svelte:fragment slot="leading"><Icon name="credit-card" /></svelte:fragment>
        <svelte:fragment slot="badges">
          <Pill appearance="badge" tone="neutral">2</Pill>
        </svelte:fragment>
      </ListCard>
      <ListCard
        title="Project Alpha"
        subtitle="Replaces the old media-trigger pattern with explicit actions"
        interactive
        on:click={() => (lastClick = "Project Alpha")}
      >
        <svelte:fragment slot="leading"><Icon icon={folder} /></svelte:fragment>
        <svelte:fragment slot="actions">
          <IconButton icon="ellipsis" ariaLabel="Project actions" variant="ghost" />
        </svelte:fragment>
      </ListCard>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Selectable cards</Eyebrow>
    <div class="specimen__stack">
      <ListCard
        title="Selected row"
        subtitle="Batch-selection ready"
        selectable
        selected={selectedCard}
        on:selectedChange={(event) => (selectedCard = event.detail.selected)}
      >
        <svelte:fragment slot="leading"><Icon name="check-square" /></svelte:fragment>
        <svelte:fragment slot="badges">
          <Pill tone="positive">Chosen</Pill>
        </svelte:fragment>
      </ListCard>
      <ListCard
        title="Compact reorder item"
        layout="compact"
        showReorderHandle
      >
        <svelte:fragment slot="leading"><Icon name="grip" /></svelte:fragment>
        <svelte:fragment slot="badges">
          <Pill appearance="badge" tone="neutral">Draft</Pill>
        </svelte:fragment>
      </ListCard>
      <ListCard
        title="Reordered item"
        subtitle="Move within section"
        layout="compact"
        showReorderHandle
      >
        <svelte:fragment slot="title">
          <span>Week 2</span>
          <span style="color: var(--poodle-color-text-secondary);"> → Week 1: Cash Flow</span>
        </svelte:fragment>
        <svelte:fragment slot="badges">
          <Pill tone="accent">Move</Pill>
        </svelte:fragment>
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
        interactive
        on:click={() => (lastClick = "hero-banner.png")}
      >
        <svelte:fragment slot="leading"><Icon name="image" /></svelte:fragment>
      </ListCard>
      <ListCard
        title="onboarding-flow.mp4"
        subtitle="Screen recording · Today"
        meta="128 MB"
        leadingShape="rounded-square"
        interactive
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
        interactive
        on:click={() => (lastClick = "API integration guide")}
      >
        <svelte:fragment slot="leading"><Icon name="file-text" /></svelte:fragment>
        <svelte:fragment slot="badges">
          <Pill tone="accent">New</Pill>
        </svelte:fragment>
      </ListCard>
      <ListCard
        title="Q4 planning deck"
        subtitle="Shared with leadership"
        interactive
        on:click={() => (lastClick = "Q4 planning deck")}
      >
        <svelte:fragment slot="leading"><Icon icon={layers} /></svelte:fragment>
        <svelte:fragment slot="badges">
          <Pill appearance="badge" tone="neutral">3</Pill>
          <Pill tone="caution">Review</Pill>
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
        interactive
        on:click={() => (lastClick = "Design system")}
      >
        <svelte:fragment slot="leading"><Icon icon={folder} /></svelte:fragment>
        <svelte:fragment slot="badges">
          <Pill tone="positive">Active</Pill>
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
        interactive
        on:click={() => (lastClick = "Brand guidelines")}
      >
        <svelte:fragment slot="leading"><Icon icon={folder} /></svelte:fragment>
        <svelte:fragment slot="footer">
          <ListCardCounter icon="file-text" count={6} tooltip="6 documents" />
          <ListCardCounter icon="image" count={42} tooltip="42 images" />
        </svelte:fragment>
      </ListCard>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Solid fill with accent colors</Eyebrow>
    <div class="specimen__stack">
      <ListCard
        title="Design tokens"
        subtitle="Foundation layer"
        leadingShape="rounded-square"
        leadingFill="solid"
        accentColor="#6366f1"
        interactive
        on:click={() => (lastClick = "Design tokens")}
      >
        <svelte:fragment slot="leading"><Icon icon={layers} /></svelte:fragment>
      </ListCard>
      <ListCard
        title="Components"
        subtitle="Primitives and composites"
        leadingShape="rounded-square"
        leadingFill="solid"
        accentColor="#ec4899"
        interactive
        on:click={() => (lastClick = "Components")}
      >
        <svelte:fragment slot="leading"><Icon icon={grid2x2} /></svelte:fragment>
      </ListCard>
      <ListCard
        title="Documentation"
        subtitle="Contracts and guides"
        leadingShape="rounded-square"
        leadingFill="solid"
        accentColor="#14b8a6"
        interactive
        on:click={() => (lastClick = "Documentation")}
      >
        <svelte:fragment slot="leading"><Icon name="file-text" /></svelte:fragment>
      </ListCard>
      <ListCard
        title="Default accent (no custom color)"
        subtitle="Uses theme accent"
        leadingShape="rounded-square"
        leadingFill="solid"
        interactive
        on:click={() => (lastClick = "Default accent")}
      >
        <svelte:fragment slot="leading"><Icon icon={folder} /></svelte:fragment>
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
          interactive
          on:click={() => (lastClick = "Card clicked")}
        >
          <svelte:fragment slot="leading"><Icon name="file-text" /></svelte:fragment>
        </ListCard>
      </ContextMenu>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Not live (dashed border, interactive)</Eyebrow>
    <div class="specimen__stack">
      <ListCard
        title="Unpublished draft"
        subtitle="Created yesterday · not yet deployed"
        meta="Draft"
        interactive
        notLive
        on:click={() => (lastClick = "Unpublished draft")}
      >
        <svelte:fragment slot="leading"><Icon name="file-text" /></svelte:fragment>
      </ListCard>
      <ListCard
        title="Staging environment"
        subtitle="Pending approval"
        interactive
        notLive
        leadingShape="rounded-square"
      >
        <svelte:fragment slot="leading"><Icon icon={layers} /></svelte:fragment>
        <svelte:fragment slot="badges">
          <Pill tone="caution" size="md">Pending</Pill>
        </svelte:fragment>
      </ListCard>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Corner sash badges</Eyebrow>
    <div class="specimen__stack">
      <ListCard
        title="Free tier plan"
        subtitle="No credit card required"
        sash="Free"
        interactive
        on:click={() => (lastClick = "Free tier plan")}
      >
        <svelte:fragment slot="leading"><Icon icon={layers} /></svelte:fragment>
      </ListCard>
      <ListCard
        title="Premium integration"
        subtitle="Unlocks advanced features"
        sash="New"
        sashColor="#6366f1"
        leadingShape="rounded-square"
        leadingFill="solid"
        accentColor="#6366f1"
        interactive
        on:click={() => (lastClick = "Premium integration")}
      >
        <svelte:fragment slot="leading"><Icon icon={grid2x2} /></svelte:fragment>
      </ListCard>
      <ListCard
        title="Legacy connector"
        subtitle="Deprecated — migrate by Q2"
        sash="EOL"
        sashColor="#ef4444"
        interactive
        on:click={() => (lastClick = "Legacy connector")}
      >
        <svelte:fragment slot="leading"><Icon name="file-text" /></svelte:fragment>
      </ListCard>
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
