<script lang="ts">
  import { StatusBar } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default (no chrome)">
    <StatusBar>
      <svelte:fragment slot="leading">
        <span class="poodle-status-item">main</span>
        <span class="poodle-status-item">0 errors</span>
      </svelte:fragment>
      <svelte:fragment slot="trailing">
        <span class="poodle-status-item">Ln 42, Col 18</span>
        <span class="poodle-status-item">UTF-8</span>
        <span class="poodle-status-item">TypeScript</span>
      </svelte:fragment>
    </StatusBar>
  </SpecimenGroup>

  <SpecimenGroup label="With chrome">
    <StatusBar chrome>
      <svelte:fragment slot="leading">
        <span class="poodle-status-item">main</span>
        <span class="poodle-status-item">0 errors</span>
      </svelte:fragment>
      <svelte:fragment slot="trailing">
        <span class="poodle-status-item">Ln 42, Col 18</span>
        <span class="poodle-status-item">UTF-8</span>
        <span class="poodle-status-item">TypeScript</span>
      </svelte:fragment>
    </StatusBar>
  </SpecimenGroup>

  <SpecimenGroup label="Summary only">
    <StatusBar summary="3 items selected" />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <div class="poodle-variant-block">
      <StatusBar chrome summary="Status bar" {size}>
        <svelte:fragment slot="trailing">
          <span class="poodle-status-item">UTF-8</span>
          <span class="poodle-status-item">TypeScript</span>
        </svelte:fragment>
      </StatusBar>
    </div>
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <div class="poodle-variant-block">
      <StatusBar chrome summary="Status bar" {density}>
        <svelte:fragment slot="trailing">
          <span class="poodle-status-item">UTF-8</span>
          <span class="poodle-status-item">TypeScript</span>
        </svelte:fragment>
      </StatusBar>
    </div>
  </svelte:fragment>
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
