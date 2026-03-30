<script lang="ts">
  import { PageHeader } from "@poodle/svelte-composites";
  import { Button, Eyebrow, MetaBar, Pill, TimeAgo } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Basic</Eyebrow>
    <PageHeader title="Components" subtitle="Browse and manage your component library." />
  </div>

  <div class="specimen__group">
    <Eyebrow>With back link</Eyebrow>
    <PageHeader
      title="Media Library"
      subtitle="Browse, review, and manage uploaded files."
      backHref="/dashboard"
      backLabel="Back to dashboard"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>With eyebrow and actions</Eyebrow>
    <PageHeader title="Button" eyebrow="Primitive" subtitle="Primary interactive control for triggering actions.">
      {#snippet actions()}
        <Button variant="secondary">View source</Button>
        <Button>Edit</Button>
      {/snippet}
    </PageHeader>
  </div>

  <div class="specimen__group">
    <Eyebrow>With count</Eyebrow>
    <PageHeader title="Users" count={128} backHref="/dashboard" backLabel="Back to dashboard" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Section and banner</Eyebrow>
    <PageHeader
      section="Scheduled Task"
      title="Nightly Sync"
      backHref="/system/tasks"
      backLabel="Back to tasks"
      backIsContextual={true}
      bannerMessage="This task is currently paused."
      bannerTone="warning"
    >
      {#snippet actions()}
        <Button variant="secondary">Run now</Button>
        <Button>Edit</Button>
      {/snippet}
    </PageHeader>
  </div>

  <div class="specimen__group">
    <Eyebrow>With MetaBar</Eyebrow>
    <PageHeader title="Nightly Sync" section="Scheduled Task" backHref="/system/tasks" backLabel="Tasks">
      {#snippet meta()}
        <MetaBar>
          <Pill tone="success" appearance="badge">Active</Pill>
          <span class="meta-text">Every 6 hours</span>
          <span class="meta-text">Last run <TimeAgo datetime="2026-03-30T08:15:00Z" /></span>
        </MetaBar>
      {/snippet}
      {#snippet actions()}
        <Button variant="secondary">Run now</Button>
        <Button>Edit schedule</Button>
      {/snippet}
    </PageHeader>
  </div>

  <div class="specimen__group">
    <Eyebrow>Title only</Eyebrow>
    <PageHeader title="Settings" />
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

  .meta-text {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
  }
</style>
