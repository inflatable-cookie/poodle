<script lang="ts">
  import { PageHeader } from "@flint/svelte-composites";
  import { Button, Eyebrow } from "@flint/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Basic</Eyebrow>
    <PageHeader title="Components" subtitle="Browse and manage your component library." />
  </div>

  <div class="specimen__group">
    <Eyebrow>With eyebrow and actions</Eyebrow>
    <PageHeader title="Button" eyebrow="Primitive" subtitle="Primary interactive control for triggering actions.">
      <svelte:fragment slot="actions">
        <Button variant="secondary" size="sm">View source</Button>
        <Button size="sm">Edit</Button>
      </svelte:fragment>
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
</style>
