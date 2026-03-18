<script lang="ts">
  import { Eyebrow } from "@pug/svelte-primitives";
  import { WorkspaceShell } from "@pug/svelte-workstation";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default shell</Eyebrow>
    <div class="specimen__frame">
      <WorkspaceShell>
        <div slot="appHeader" class="specimen__slot">App Header</div>
        <div slot="projectHeader" class="specimen__slot">Project Header</div>
        <div slot="surfaceTabs" class="specimen__slot">Surface Tabs</div>
        <div class="specimen__slot">Center content</div>
        <div slot="statusBar" class="specimen__slot">Status Bar</div>
      </WorkspaceShell>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Loading state</Eyebrow>
    <div class="specimen__frame">
      <WorkspaceShell state="loading" stateTitle="Loading workspace" stateMessage="Please wait..." />
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
    height: 16rem;
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    overflow: hidden;
  }

  .specimen__slot {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.5rem;
    font-size: 0.75rem;
    color: var(--pug-color-text-muted);
    border: 0.0625rem dashed var(--pug-color-border-subtle);
    min-height: 2rem;
  }
</style>
