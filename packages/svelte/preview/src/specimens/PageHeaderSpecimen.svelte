<script lang="ts">
  import { PageHeader } from "@inflatable-cookie/poodle-svelte";
  import { IconButton, MetaBar, Pill, TimeAgo } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let navAction = $state("");
  let hierarchyAction = $state("");
  let statusAction = $state("");
  let metaAction = $state("");
</script>

<SpecimenLayout>
  {#snippet children()}
    <div class="poodle-specimen">
      <SpecimenGroup label="Page title and summary">
        <PageHeader title="Components" subtitle="Browse and manage your component library." />
        <PageHeader title="Settings" />
      </SpecimenGroup>

      <SpecimenGroup label="Navigation and actions">
        <PageHeader
          title="Media Library"
          subtitle="Browse, review, and manage uploaded files."
          backHref="/dashboard"
          backLabel="Dashboard"
        >
          {#snippet actions()}
            <IconButton
              icon="upload"
              ariaLabel="Upload"
              variant="secondary"
              onClick={() => (navAction = "Upload")}
            />
            <IconButton
              icon="settings"
              ariaLabel="Settings"
              variant="secondary"
              onClick={() => (navAction = "Settings")}
            />
          {/snippet}
        </PageHeader>
        <PageHeader
          title="Cash flow forecasts"
          section="Module"
          subtitle="Manage content and ordering for this module."
          backHref="/learning/pathways"
          backLabel="Pathways"
        >
          {#snippet breadcrumbs()}
            <nav class="poodle-breadcrumbs" aria-label="Breadcrumb">
              <a href="/learning/pathways">Pathways</a>
              <span class="poodle-breadcrumbs__chevron" aria-hidden="true">›</span>
              <a href="/learning/pathways/foundation">Foundation</a>
              <span class="poodle-breadcrumbs__chevron" aria-hidden="true">›</span>
              <span>Module</span>
            </nav>
          {/snippet}
          {#snippet actions()}
            <IconButton
              icon="upload"
              ariaLabel="Upload"
              variant="secondary"
              onClick={() => (navAction = "Upload module")}
            />
            <IconButton
              icon="settings"
              ariaLabel="Settings"
              variant="secondary"
              onClick={() => (navAction = "Settings module")}
            />
          {/snippet}
        </PageHeader>
        {#if navAction}
          <p class="poodle-specimen__hint">
            Last action: <strong>{navAction}</strong>
          </p>
        {/if}
      </SpecimenGroup>

      <SpecimenGroup label="Hierarchy and count">
        <PageHeader title="Button" eyebrow="Primitive" subtitle="Primary interactive control for triggering actions.">
          {#snippet actions()}
            <IconButton
              icon="code"
              ariaLabel="View source"
              variant="secondary"
              onClick={() => (hierarchyAction = "View source")}
            />
            <IconButton
              icon="pencil"
              ariaLabel="Edit"
              variant="secondary"
              onClick={() => (hierarchyAction = "Edit")}
            />
          {/snippet}
        </PageHeader>
        <PageHeader title="Users" count={128} backHref="/dashboard" backLabel="Dashboard" />
        {#if hierarchyAction}
          <p class="poodle-specimen__hint">
            Last action: <strong>{hierarchyAction}</strong>
          </p>
        {/if}
      </SpecimenGroup>

      <SpecimenGroup label="Contextual status">
        <PageHeader
          section="Scheduled Task"
          title="Nightly Sync"
          backHref="/system/tasks"
          backLabel="Tasks"
          backIsContextual={true}
          bannerMessage="This task is currently paused."
          bannerTone="warning"
        >
          {#snippet actions()}
            <IconButton
              icon="play"
              ariaLabel="Run now"
              variant="secondary"
              onClick={() => (statusAction = "Run now")}
            />
            <IconButton
              icon="pencil"
              ariaLabel="Edit"
              variant="secondary"
              onClick={() => (statusAction = "Edit task")}
            />
          {/snippet}
        </PageHeader>
        {#if statusAction}
          <p class="poodle-specimen__hint">
            Last action: <strong>{statusAction}</strong>
          </p>
        {/if}
      </SpecimenGroup>

      <SpecimenGroup label="Operational metadata">
        <PageHeader title="Nightly Sync" section="Scheduled Task" backHref="/system/tasks" backLabel="Tasks">
          {#snippet meta()}
            <MetaBar>
              <Pill tone="success" appearance="badge">Active</Pill>
              <span class="poodle-meta-text">Every 6 hours</span>
              <span class="poodle-meta-text">Last run <TimeAgo datetime="2026-03-30T08:15:00Z" /></span>
            </MetaBar>
          {/snippet}
          {#snippet actions()}
            <IconButton
              icon="play"
              ariaLabel="Run now"
              variant="secondary"
              onClick={() => (metaAction = "Run now")}
            />
            <IconButton
              icon="calendar"
              ariaLabel="Edit schedule"
              variant="secondary"
              onClick={() => (metaAction = "Edit schedule")}
            />
          {/snippet}
        </PageHeader>
        {#if metaAction}
          <p class="poodle-specimen__hint">
            Last action: <strong>{metaAction}</strong>
          </p>
        {/if}
      </SpecimenGroup>
    </div>
  {/snippet}

  {#snippet sizes(size)}
    <SpecimenGroup label={size.toUpperCase()}>
      <PageHeader
        title="Media Library"
        subtitle="Browse, review, and manage uploaded files."
        backHref="/dashboard"
        backLabel="Dashboard"
        {size}
      >
        {#snippet actions()}
          <IconButton icon="upload" ariaLabel="Upload" variant="secondary" />
          <IconButton icon="settings" ariaLabel="Settings" variant="secondary" />
        {/snippet}
      </PageHeader>
    </SpecimenGroup>
  {/snippet}

  {#snippet densities(density)}
    <SpecimenGroup label={density.toUpperCase()}>
      <PageHeader
        title="Media Library"
        subtitle="Browse, review, and manage uploaded files."
        backHref="/dashboard"
        backLabel="Dashboard"
        {density}
      >
        {#snippet actions()}
          <IconButton icon="upload" ariaLabel="Upload" variant="secondary" />
          <IconButton icon="settings" ariaLabel="Settings" variant="secondary" />
        {/snippet}
      </PageHeader>
    </SpecimenGroup>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-meta-text {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
  }

  .poodle-breadcrumbs {
    display: inline-flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.375rem;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
  }

  .poodle-breadcrumbs a {
    color: inherit;
    text-decoration: none;
  }

  .poodle-breadcrumbs a:hover {
    color: var(--poodle-color-text-primary);
  }

  .poodle-breadcrumbs__chevron {
    opacity: 0.7;
  }

  .poodle-specimen__hint {
    margin: 0.5rem 0 0;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
