<script lang="ts">
  import { Collapsible, Eyebrow } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default (closed)</Eyebrow>
    <Collapsible title="Project settings" description="Configure build options and deploy targets.">
      <p>Build target: production</p>
      <p>Output directory: dist/</p>
      <p>Source maps: enabled</p>
    </Collapsible>
  </div>

  <div class="specimen__group">
    <Eyebrow>Default open</Eyebrow>
    <Collapsible title="Advanced options" defaultOpen>
      <p>Cache TTL: 3600s</p>
      <p>Retry count: 3</p>
      <p>Timeout: 30s</p>
    </Collapsible>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <Collapsible title="Locked section" description="Requires admin access." isDisabled>
      <p>This content is hidden behind a disabled collapsible.</p>
    </Collapsible>
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

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
