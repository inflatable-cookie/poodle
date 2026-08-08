<script lang="ts">
  import { StatusBar } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default (no chrome)">
    <StatusBar>
      {#snippet leading()}
        <span class="poodle-status-item">main</span>
        <span class="poodle-status-item">0 errors</span>
      {/snippet}
      {#snippet trailing()}
        <span class="poodle-status-item">Ln 42, Col 18</span>
        <span class="poodle-status-item">UTF-8</span>
        <span class="poodle-status-item">TypeScript</span>
      {/snippet}
    </StatusBar>
  </SpecimenGroup>

  <SpecimenGroup label="With chrome">
    <StatusBar chrome>
      {#snippet leading()}
        <span class="poodle-status-item">main</span>
        <span class="poodle-status-item">0 errors</span>
      {/snippet}
      {#snippet trailing()}
        <span class="poodle-status-item">Ln 42, Col 18</span>
        <span class="poodle-status-item">UTF-8</span>
        <span class="poodle-status-item">TypeScript</span>
      {/snippet}
    </StatusBar>
  </SpecimenGroup>

  <SpecimenGroup label="Summary only">
    <StatusBar summary="3 items selected" />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <div class="poodle-variant-block">
      <StatusBar chrome summary="Status bar" {size}>
        {#snippet trailing()}
          <span class="poodle-status-item">UTF-8</span>
          <span class="poodle-status-item">TypeScript</span>
        {/snippet}
      </StatusBar>
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-variant-block">
      <StatusBar chrome summary="Status bar" {density}>
        {#snippet trailing()}
          <span class="poodle-status-item">UTF-8</span>
          <span class="poodle-status-item">TypeScript</span>
        {/snippet}
      </StatusBar>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-status-item {
    font-size: inherit;
    color: var(--poodle-color-text-secondary);
    padding: 0 0.375rem;
  }

  .poodle-variant-block {
    width: 100%;
  }
</style>
