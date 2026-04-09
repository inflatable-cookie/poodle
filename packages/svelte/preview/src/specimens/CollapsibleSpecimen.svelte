<script lang="ts">
  import { Collapsible } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;
</script>

<div class="specimen">
  <SpecimenGroup label="Default (closed)">
    <Collapsible title="Project settings" description="Configure build options and deploy targets.">
      <p>Build target: production</p>
      <p>Output directory: dist/</p>
      <p>Source maps: enabled</p>
    </Collapsible>
  </SpecimenGroup>

  <SpecimenGroup label="Default open">
    <Collapsible title="Advanced options" defaultOpen>
      <p>Cache TTL: 3600s</p>
      <p>Retry count: 3</p>
      <p>Timeout: 30s</p>
    </Collapsible>
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <Collapsible title="Collapsible at {size}" {size}>
          <p>Content at <strong>{size}</strong> size.</p>
        </Collapsible>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <Collapsible title="Collapsible at {density} density" {density}>
          <p>Content at <strong>{density}</strong> density.</p>
        </Collapsible>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <Collapsible title="Locked section" description="Requires admin access." disabled>
      <p>This content is hidden behind a disabled collapsible.</p>
    </Collapsible>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
