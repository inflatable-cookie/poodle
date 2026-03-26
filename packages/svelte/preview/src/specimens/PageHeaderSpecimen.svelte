<script lang="ts">
  import { PageHeader } from "@poodle/svelte-composites";
  import { Button, Eyebrow } from "@poodle/svelte-primitives";
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
      <svelte:fragment slot="actions">
        <Button variant="secondary">View source</Button>
        <Button>Edit</Button>
      </svelte:fragment>
    </PageHeader>
  </div>

  <div class="specimen__group">
    <Eyebrow>With count</Eyebrow>
    <PageHeader title="Users" count={128} backHref="/dashboard" backLabel="Back to dashboard" />
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
</style>
