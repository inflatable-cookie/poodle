<script lang="ts">
  import { PageHeader } from "@poodle/svelte";
  import { IconButton, MetaBar, Pill, TimeAgo } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Basic">
    <PageHeader title="Components" subtitle="Browse and manage your component library." />
  </SpecimenGroup>

  <SpecimenGroup label="With back link and actions">
    <PageHeader
      title="Media Library"
      subtitle="Browse, review, and manage uploaded files."
      backHref="/dashboard"
      backLabel="Dashboard"
    >
      {#snippet actions()}
        <IconButton icon="upload" ariaLabel="Upload" variant="secondary" />
        <IconButton icon="settings" ariaLabel="Settings" variant="secondary" />
      {/snippet}
    </PageHeader>
  </SpecimenGroup>

  <SpecimenGroup label="With eyebrow and actions">
    <PageHeader title="Button" eyebrow="Primitive" subtitle="Primary interactive control for triggering actions.">
      {#snippet actions()}
        <IconButton icon="code" ariaLabel="View source" variant="secondary" />
        <IconButton icon="pencil" ariaLabel="Edit" variant="secondary" />
      {/snippet}
    </PageHeader>
  </SpecimenGroup>

  <SpecimenGroup label="With count">
    <PageHeader title="Users" count={128} backHref="/dashboard" backLabel="Dashboard" />
  </SpecimenGroup>

  <SpecimenGroup label="Section and banner">
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
        <IconButton icon="play" ariaLabel="Run now" variant="secondary" />
        <IconButton icon="pencil" ariaLabel="Edit" variant="secondary" />
      {/snippet}
    </PageHeader>
  </SpecimenGroup>

  <SpecimenGroup label="With MetaBar">
    <PageHeader title="Nightly Sync" section="Scheduled Task" backHref="/system/tasks" backLabel="Tasks">
      {#snippet meta()}
        <MetaBar>
          <Pill tone="success" appearance="badge">Active</Pill>
          <span class="poodle-meta-text">Every 6 hours</span>
          <span class="poodle-meta-text">Last run <TimeAgo datetime="2026-03-30T08:15:00Z" /></span>
        </MetaBar>
      {/snippet}
      {#snippet actions()}
        <IconButton icon="play" ariaLabel="Run now" variant="secondary" />
        <IconButton icon="calendar" ariaLabel="Edit schedule" variant="secondary" />
      {/snippet}
    </PageHeader>
  </SpecimenGroup>

  <SpecimenGroup label="Title only">
    <PageHeader title="Settings" />
  </SpecimenGroup>
</div>

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
</style>
