<script lang="ts">
  import { Eyebrow, Button } from "@pug/svelte-primitives";
  import { ProjectHeader } from "@pug/svelte-workstation";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <div class="specimen__frame">
      <ProjectHeader title="My Project" subtitle="~/workspace/my-project" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Dirty state</Eyebrow>
    <div class="specimen__frame">
      <ProjectHeader title="Unsaved Project" isDirty />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With actions</Eyebrow>
    <div class="specimen__frame">
      <ProjectHeader title="My Project" subtitle="v1.2.0">
        <svelte:fragment slot="actions">
          <Button size="sm" variant="ghost">Share</Button>
          <Button size="sm" variant="ghost">Settings</Button>
        </svelte:fragment>
      </ProjectHeader>
    </div>
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

  .specimen__frame {
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    overflow: hidden;
  }
</style>
