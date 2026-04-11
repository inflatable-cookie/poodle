<script lang="ts">
  import { StatusBar } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout bareVariants>
  <SpecimenGroup bare label="Default (no chrome)">
    <StatusBar>
      <svelte:fragment slot="leading">
        <span class="status-item">main</span>
        <span class="status-item">0 errors</span>
      </svelte:fragment>
      <svelte:fragment slot="trailing">
        <span class="status-item">Ln 42, Col 18</span>
        <span class="status-item">UTF-8</span>
        <span class="status-item">TypeScript</span>
      </svelte:fragment>
    </StatusBar>
  </SpecimenGroup>

  <SpecimenGroup bare label="With chrome">
    <StatusBar chrome>
      <svelte:fragment slot="leading">
        <span class="status-item">main</span>
        <span class="status-item">0 errors</span>
      </svelte:fragment>
      <svelte:fragment slot="trailing">
        <span class="status-item">Ln 42, Col 18</span>
        <span class="status-item">UTF-8</span>
        <span class="status-item">TypeScript</span>
      </svelte:fragment>
    </StatusBar>
  </SpecimenGroup>

  <SpecimenGroup bare label="Summary only">
    <StatusBar summary="3 items selected" />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <StatusBar summary="Status at {size}" {size}>
      <svelte:fragment slot="trailing">
        <span class="status-item">UTF-8</span>
        <span class="status-item">TypeScript</span>
      </svelte:fragment>
    </StatusBar>
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <StatusBar summary="Status at {density}" {density}>
      <svelte:fragment slot="trailing">
        <span class="status-item">UTF-8</span>
        <span class="status-item">TypeScript</span>
      </svelte:fragment>
    </StatusBar>
  </svelte:fragment>
</SpecimenLayout>

<style>
  .status-item {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
    padding: 0 0.375rem;
  }
</style>
