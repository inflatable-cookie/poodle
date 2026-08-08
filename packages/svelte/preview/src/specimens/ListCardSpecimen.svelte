<script lang="ts">
  import { ContextMenu, Icon, IconButton, ListCard, ListCardCounter, Pill } from "@inflatable-cookie/poodle-svelte";
  import { folder, layers, grid2x2 } from "@inflatable-cookie/poodle-icons-lucide";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let lastClick = $state("");
  let selectedCard = $state(false);
</script>

<SpecimenLayout>
  <div class="poodle-specimen">
    <SpecimenGroup label="Interactive list cards">
      <div class="poodle-specimen__stack">
        <ListCard
          title="design-system-v2.figma"
          subtitle="Updated by Clay · 2h ago"
          meta="14.2 MB"
          interactive
          onClick={() => (lastClick = "design-system-v2.figma")}
        >
          {#snippet leading()}
            <Icon icon={folder} />
          {/snippet}
        </ListCard>
        <ListCard
          title="component-specs.pdf"
          subtitle="Shared with team · Yesterday"
          meta="2.8 MB"
          interactive
          onClick={() => (lastClick = "component-specs.pdf")}
        >
          {#snippet leading()}
            <Icon name="file-text" />
          {/snippet}
          {#snippet corner()}
            <Icon name="lock" ariaLabel="Restricted" size="sm" />
          {/snippet}
        </ListCard>
        <ListCard
          title="brand-assets.zip"
          subtitle="Archived"
          meta="48 MB"
          disabled
        >
          {#snippet leading()}
            <Icon icon={layers} />
          {/snippet}
        </ListCard>
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Link roots and explicit actions">
      <div class="poodle-specimen__stack">
        <ListCard
          title="Billing settings"
          subtitle="Manage invoices and payment methods"
          href="#billing"
        >
          {#snippet leading()}
            <Icon name="credit-card" />
          {/snippet}
          {#snippet badges()}
            <Pill appearance="badge" tone="neutral">2</Pill>
          {/snippet}
        </ListCard>
        <ListCard
          title="Project Alpha"
          subtitle="Replaces the old media-trigger pattern with explicit actions"
          interactive
          onClick={() => (lastClick = "Project Alpha")}
        >
          {#snippet leading()}
            <Icon icon={folder} />
          {/snippet}
          {#snippet actions()}
            <IconButton icon="ellipsis" ariaLabel="Project actions" variant="ghost" />
          {/snippet}
        </ListCard>
    </div>
    </SpecimenGroup>

    <SpecimenGroup label="Hierarchy titles">
      <div class="poodle-specimen__stack">
        <ListCard
          title="Cash flow forecasts"
          subtitle="Module content"
          interactive
          onClick={() => (lastClick = "Cash flow forecasts")}
        >
          {#snippet leading()}
            <Icon icon={grid2x2} />
          {/snippet}
          {#snippet titleContent()}
            <span style="color: var(--poodle-color-text-secondary);">Pathway</span>
            <Icon name="chevron-right" size="xs" />
            <span style="color: var(--poodle-color-text-secondary);">Module</span>
            <Icon name="chevron-right" size="xs" />
            <span>Cash flow forecasts</span>
          {/snippet}
          {#snippet metaContent()}
            <span style="color: var(--poodle-color-text-secondary);">Weight 3</span>
          {/snippet}
        </ListCard>
        <ListCard
          title="Week 1: Cash Flow"
          subtitle="Move within section"
          layout="compact"
          showReorderHandle
        >
          {#snippet titleContent()}
            <span style="color: var(--poodle-color-text-secondary);">Module</span>
            <Icon name="chevron-right" size="xs" />
            <span>Week 1: Cash Flow</span>
          {/snippet}
          {#snippet badges()}
            <Pill tone="info">Move</Pill>
          {/snippet}
        </ListCard>
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Highlighted card">
    <div class="poodle-specimen__stack">
      <!-- `highlighted` is a documented state that had no specimen at all, so
           nothing rendered its gradient or inset ring and no gate could see it.
           It is orthogonal to both selection and active. -->
      <ListCard title="Highlighted row" subtitle="Accent gradient and inset ring" interactive highlighted />
      <ListCard title="Highlighted and selected" subtitle="Both at once" interactive selectable selected highlighted />
      <ListCard title="Ordinary row" subtitle="For comparison" interactive />
    </div>
  </SpecimenGroup>

    <SpecimenGroup label="Active card">
    <div class="poodle-specimen__stack">
      <!-- Active is deliberately quieter than Selected: it is always on for one
           card in a list, where selection's full border and fill would shout
           permanently. The third card carries both, since they are orthogonal. -->
      <ListCard title="Acowtancy Build Test" subtitle="Registered 28/07/2026 16:35" interactive active />
      <ListCard title="Acowtancy Build Test" subtitle="Registered 28/07/2026 16:35" interactive />
      <ListCard title="Active and selected" subtitle="Both states at once" interactive selectable selected active />
    </div>
  </SpecimenGroup>

    <SpecimenGroup label="Selectable cards">
    <div class="poodle-specimen__stack">
      <ListCard
        title="Selected row"
        subtitle="Batch-selection ready"
        selectable
        selected={selectedCard}
        onSelectedChange={(selected) => (selectedCard = selected)}
      >
        {#snippet leading()}
          <Icon name="check-square" />
        {/snippet}
        {#snippet badges()}
          <Pill tone="success">Chosen</Pill>
        {/snippet}
      </ListCard>
      <ListCard
        title="Compact reorder item"
        layout="compact"
        showReorderHandle
      >
        {#snippet leading()}
          <Icon name="grip" />
        {/snippet}
        {#snippet badges()}
          <Pill appearance="badge" tone="neutral">Draft</Pill>
        {/snippet}
      </ListCard>
      <ListCard
        title="Reordered item"
        subtitle="Move within section"
        layout="compact"
        showReorderHandle
      >
        {#snippet titleContent()}
          <span style="color: var(--poodle-color-text-secondary);">Module</span>
          <Icon name="chevron-right" size="xs" />
          <span>Week 1: Cash Flow</span>
        {/snippet}
        {#snippet badges()}
          <Pill tone="info">Move</Pill>
        {/snippet}
      </ListCard>
    </div>
    </SpecimenGroup>

    <SpecimenGroup label="Rounded-square leading (thumbnails)">
    <div class="poodle-specimen__stack">
      <ListCard
        title="hero-banner.png"
        subtitle="Uploaded by Jamie · 4h ago"
        meta="3.1 MB"
        leadingShape="rounded-square"
        interactive
        onClick={() => (lastClick = "hero-banner.png")}
      >
        {#snippet leading()}
          <Icon name="image" />
        {/snippet}
      </ListCard>
      <ListCard
        title="onboarding-flow.mp4"
        subtitle="Screen recording · Today"
        meta="128 MB"
        leadingShape="rounded-square"
        interactive
        onClick={() => (lastClick = "onboarding-flow.mp4")}
      >
        {#snippet leading()}
          <Icon name="play" />
        {/snippet}
      </ListCard>
    </div>
    </SpecimenGroup>

    <SpecimenGroup label="Leading size offset">
    <div class="poodle-specimen__stack">
      <ListCard
        title="Default leading"
        subtitle="Matches the card size ladder"
        interactive
      >
        {#snippet leading()}
          <Icon name="file-text" />
        {/snippet}
      </ListCard>
      <ListCard
        title="Offset leading"
        subtitle="Leading block steps up relative to the card size"
        interactive
        leadingSizeOffset={1}
      >
        {#snippet leading()}
          <Icon name="file-text" />
        {/snippet}
      </ListCard>
    </div>
    </SpecimenGroup>

    <SpecimenGroup label="Stacked layout">
    <div class="poodle-specimen__tiles">
      <ListCard
        title="Release notes"
        subtitle="Summarize the latest component changes and known migration edges."
        layout="stacked"
        leadingShape="rounded-square"
        leadingFill="solid"
        accentColor="#6366f1"
        interactive
      >
        {#snippet leading()}
          <Icon icon={layers} />
        {/snippet}
        {#snippet trailing()}
          <Pill tone="success">Published</Pill>
        {/snippet}
      </ListCard>
      <ListCard
        title="Media library"
        subtitle="Review uploads, approve assets, and keep metadata consistent across the library."
        layout="stacked"
        leadingShape="rounded-square"
        interactive
      >
        {#snippet leading()}
          <Icon name="image" />
        {/snippet}
        {#snippet footer()}
          <ListCardCounter icon="image" count={18} tooltip="18 images" />
          <ListCardCounter icon="file-text" count={6} tooltip="6 notes" />
        {/snippet}
        {#snippet trailing()}
          <IconButton icon="ellipsis" ariaLabel="Media library actions" variant="ghost" />
        {/snippet}
      </ListCard>
    </div>
    </SpecimenGroup>

    <SpecimenGroup label="With badges">
    <div class="poodle-specimen__stack">
      <ListCard
        title="API integration guide"
        subtitle="Updated yesterday"
        meta="Draft"
        interactive
        onClick={() => (lastClick = "API integration guide")}
      >
        {#snippet leading()}
          <Icon name="file-text" />
        {/snippet}
        {#snippet badges()}
          <Pill tone="info">New</Pill>
        {/snippet}
      </ListCard>
      <ListCard
        title="Q4 planning deck"
        subtitle="Shared with leadership"
        interactive
        onClick={() => (lastClick = "Q4 planning deck")}
      >
        {#snippet leading()}
          <Icon icon={layers} />
        {/snippet}
        {#snippet badges()}
          <Pill appearance="badge" tone="neutral">3</Pill>
          <Pill tone="warning">Review</Pill>
        {/snippet}
      </ListCard>
    </div>
    </SpecimenGroup>

    <SpecimenGroup label="With footer counters">
    <div class="poodle-specimen__stack">
      <ListCard
        title="Design system"
        subtitle="12 contributors"
        leadingShape="rounded-square"
        interactive
        onClick={() => (lastClick = "Design system")}
      >
        {#snippet leading()}
          <Icon icon={folder} />
        {/snippet}
        {#snippet badges()}
          <Pill tone="success">Active</Pill>
        {/snippet}
        {#snippet footer()}
          <ListCardCounter icon="file-text" count={24} tooltip="24 documents" />
          <ListCardCounter icon="image" count={8} tooltip="8 images" />
          <ListCardCounter icon="layers" count={3} tooltip="3 sub-folders" href="#sub-folders" />
        {/snippet}
      </ListCard>
      <ListCard
        title="Brand guidelines"
        subtitle="Last updated 2 weeks ago"
        leadingShape="rounded-square"
        interactive
        onClick={() => (lastClick = "Brand guidelines")}
      >
        {#snippet leading()}
          <Icon icon={folder} />
        {/snippet}
        {#snippet footer()}
          <ListCardCounter icon="file-text" count={6} tooltip="6 documents" />
          <ListCardCounter icon="image" count={42} tooltip="42 images" />
        {/snippet}
      </ListCard>
    </div>
    </SpecimenGroup>

    <SpecimenGroup label="Inherited footer counters">
    <p class="poodle-specimen__inline-copy">
      Summary:
      <ListCardCounter icon="file-text" count={24} tooltip="24 documents" typography="inherit" />
      and
      <ListCardCounter icon="image" count={8} tooltip="8 images" typography="inherit" />
      published this week.
    </p>
    </SpecimenGroup>

    <SpecimenGroup label="Solid fill with accent colors">
    <div class="poodle-specimen__stack">
      <ListCard
        title="Design tokens"
        subtitle="Foundation layer"
        leadingShape="rounded-square"
        leadingFill="solid"
        accentColor="#6366f1"
        interactive
        onClick={() => (lastClick = "Design tokens")}
      >
        {#snippet leading()}
          <Icon icon={layers} />
        {/snippet}
      </ListCard>
      <ListCard
        title="Components"
        subtitle="Primitives and composites"
        leadingShape="rounded-square"
        leadingFill="solid"
        accentColor="#ec4899"
        interactive
        onClick={() => (lastClick = "Components")}
      >
        {#snippet leading()}
          <Icon icon={grid2x2} />
        {/snippet}
      </ListCard>
      <ListCard
        title="Documentation"
        subtitle="Contracts and guides"
        leadingShape="rounded-square"
        leadingFill="solid"
        accentColor="#14b8a6"
        interactive
        onClick={() => (lastClick = "Documentation")}
      >
        {#snippet leading()}
          <Icon name="file-text" />
        {/snippet}
      </ListCard>
      <ListCard
        title="Default accent (no custom color)"
        subtitle="Uses theme accent"
        leadingShape="rounded-square"
        leadingFill="solid"
        interactive
        onClick={() => (lastClick = "Default accent")}
      >
        {#snippet leading()}
          <Icon icon={folder} />
        {/snippet}
      </ListCard>
    </div>
    </SpecimenGroup>

    <SpecimenGroup label="With built-in context menu">
    <div class="poodle-specimen__stack">
      <ListCard
        title="Right-click for actions"
        subtitle="Context menu on the whole card"
        meta="12 KB"
        interactive
        contextMenuItems={[
          { label: "Open", value: "open" },
          { label: "Rename", value: "rename" },
          { label: "Duplicate", value: "duplicate" },
          { kind: "separator", label: "", value: "sep-1" },
          { label: "Delete", value: "delete" },
        ]}
        contextMenuAriaLabel="File actions"
        onContextAction={(value) => (lastClick = `Action: ${value}`)}
        onClick={() => (lastClick = "Card clicked")}
      >
        {#snippet leading()}
          <Icon name="file-text" />
        {/snippet}
      </ListCard>
    </div>
    </SpecimenGroup>

    <SpecimenGroup label="With wrapped context menu">
    <div class="poodle-specimen__stack">
      <ContextMenu
        items={[
          { label: "Open", value: "open" },
          { label: "Rename", value: "rename" },
          { label: "Duplicate", value: "duplicate" },
          { kind: "separator", label: "", value: "sep-1" },
          { label: "Delete", value: "delete" },
        ]}
        onAction={(value) => (lastClick = `Action: ${value}`)}
      >
        <ListCard
          title="Legacy wrapper path"
          subtitle="Still supported for arbitrary content"
          meta="12 KB"
          interactive
          onClick={() => (lastClick = "Card clicked")}
        >
          {#snippet leading()}
            <Icon name="file-text" />
          {/snippet}
        </ListCard>
      </ContextMenu>
    </div>
    </SpecimenGroup>

    <SpecimenGroup label="Exclusive trailing lane">
    <div class="poodle-specimen__stack">
      <ListCard
        title="Workspace settings"
        subtitle="Shared defaults and access controls"
        meta="Updated 2h ago"
        interactive
      >
        {#snippet leading()}
          <Icon icon={folder} />
        {/snippet}
        {#snippet trailing()}
          <Pill tone="success">Live</Pill>
        {/snippet}
      </ListCard>
    </div>
    </SpecimenGroup>

    <SpecimenGroup label="Not live (dashed border, interactive)">
    <div class="poodle-specimen__stack">
      <ListCard
        title="Unpublished draft"
        subtitle="Created yesterday · not yet deployed"
        meta="Draft"
        interactive
        notLive
        onClick={() => (lastClick = "Unpublished draft")}
      >
        {#snippet leading()}
          <Icon name="file-text" />
        {/snippet}
      </ListCard>
      <ListCard
        title="Staging environment"
        subtitle="Pending approval"
        interactive
        notLive
        leadingShape="rounded-square"
      >
        {#snippet leading()}
          <Icon icon={layers} />
        {/snippet}
        {#snippet badges()}
          <Pill tone="warning" size="md">Pending</Pill>
        {/snippet}
      </ListCard>
    </div>
    </SpecimenGroup>

    <SpecimenGroup label="Corner sash badges">
    <div class="poodle-specimen__stack">
      <ListCard
        title="Free tier plan"
        subtitle="No credit card required"
        sash="Free"
        interactive
        onClick={() => (lastClick = "Free tier plan")}
      >
        {#snippet leading()}
          <Icon icon={layers} />
        {/snippet}
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
        onClick={() => (lastClick = "Premium integration")}
      >
        {#snippet leading()}
          <Icon icon={grid2x2} />
        {/snippet}
      </ListCard>
      <ListCard
        title="Legacy connector"
        subtitle="Deprecated — migrate by Q2"
        sash="EOL"
        sashColor="#ef4444"
        interactive
        onClick={() => (lastClick = "Legacy connector")}
      >
        {#snippet leading()}
          <Icon name="file-text" />
        {/snippet}
      </ListCard>
    </div>
    </SpecimenGroup>

    <SpecimenGroup label="Static list card">
      <ListCard title="Read-only item" subtitle="No click handler" />
    </SpecimenGroup>

    {#if lastClick}
      <SpecimenGroup label="Last click">
        <p>{lastClick}</p>
      </SpecimenGroup>
    {/if}
  </div>

  {#snippet sizes(size)}
    <div class="poodle-list-card-density-demo">
      <ListCard
        title="Workspace settings"
        subtitle="Shared defaults and access controls"
        interactive
        {size}
      >
        {#snippet leading()}
          <Icon icon={folder} />
        {/snippet}
        {#snippet trailing()}
          <Pill tone="success" {size}>Live</Pill>
        {/snippet}
      </ListCard>
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-list-card-density-demo">
      <ListCard
        title="Workspace settings"
        subtitle="Shared defaults and access controls"
        interactive
        {density}
      >
        {#snippet leading()}
          <Icon icon={folder} />
        {/snippet}
        {#snippet trailing()}
          <Pill tone="success">Live</Pill>
        {/snippet}
      </ListCard>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen { display: flex; flex-direction: column; gap: 1rem; }
  .poodle-specimen__stack { display: flex; flex-direction: column; gap: 0.25rem; }
  .poodle-specimen__tiles {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(16rem, 1fr));
    gap: 0.75rem;
  }
  .poodle-specimen__inline-copy { font-size: 1rem; line-height: 1.6; color: var(--poodle-color-text-primary); }
  .poodle-list-card-density-demo { width: min(100%, 26rem); }
  p { margin: 0; }
</style>
