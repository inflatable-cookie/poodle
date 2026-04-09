<script lang="ts">
  import { StatusBar } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Default">
    <div class="specimen__frame">
      <StatusBar summary="Ready">
        <svelte:fragment slot="leading">
          <span class="specimen__status-item">main</span>
          <span class="specimen__status-item">0 errors</span>
        </svelte:fragment>
        <svelte:fragment slot="trailing">
          <span class="specimen__status-item">Ln 42, Col 18</span>
          <span class="specimen__status-item">UTF-8</span>
          <span class="specimen__status-item">TypeScript</span>
        </svelte:fragment>
      </StatusBar>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Summary only">
    <div class="specimen__frame">
      <StatusBar summary="3 items selected" />
    </div>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__frame {
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    overflow: hidden;
  }

  .specimen__status-item {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
    padding: 0 0.375rem;
  }
</style>
