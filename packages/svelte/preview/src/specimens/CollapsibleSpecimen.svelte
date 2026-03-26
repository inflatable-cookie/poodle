<script lang="ts">
  import { Collapsible, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;
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
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <Collapsible title="Collapsible at {size}" {size}>
          <p>Content at <strong>{size}</strong> size.</p>
        </Collapsible>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <Collapsible title="Collapsible at {density} density" {density}>
          <p>Content at <strong>{density}</strong> density.</p>
        </Collapsible>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <Collapsible title="Locked section" description="Requires admin access." disabled>
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

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
