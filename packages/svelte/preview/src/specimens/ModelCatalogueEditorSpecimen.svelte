<script lang="ts">
  import { Button, ModelCatalogueEditor } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  import ModelCatalogueEditorHarness from "./ModelCatalogueEditorHarness.svelte";
</script>

<SpecimenLayout showSizes={false} showDensities={false}>
  {#snippet children()}
    <div class="poodle-model-catalogue-editor-specimen">
      <SpecimenGroup label="Shown and hidden models">
        <div class="poodle-model-catalogue-editor-specimen__panel">
          <ModelCatalogueEditorHarness />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Reorder-capable list">
        <div class="poodle-model-catalogue-editor-specimen__panel">
          <ModelCatalogueEditorHarness />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Duplicate display labels">
        <p class="poodle-model-catalogue-editor-specimen__note">
          Shared Label appears twice with distinct opaque ids in the fixtures above.
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="Custom action">
        <div class="poodle-model-catalogue-editor-specimen__panel">
          <ModelCatalogueEditorHarness>
            {#snippet customAction()}
              <Button variant="secondary" size="sm">Add custom model</Button>
            {/snippet}
          </ModelCatalogueEditorHarness>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Loading">
        <div class="poodle-model-catalogue-editor-specimen__panel">
          <ModelCatalogueEditor items={[]} state="loading" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Unavailable">
        <div class="poodle-model-catalogue-editor-specimen__panel">
          <ModelCatalogueEditor items={[]} state="unavailable" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Empty">
        <div class="poodle-model-catalogue-editor-specimen__panel">
          <ModelCatalogueEditor items={[]} state="empty" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Error">
        <div class="poodle-model-catalogue-editor-specimen__panel">
          <ModelCatalogueEditor items={[]} state="error" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Session negotiated">
        <div class="poodle-model-catalogue-editor-specimen__panel">
          <ModelCatalogueEditor items={[]} state="sessionNegotiated" />
        </div>
      </SpecimenGroup>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-model-catalogue-editor-specimen {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .poodle-model-catalogue-editor-specimen__panel {
    width: min(36rem, 100%);
  }

  .poodle-model-catalogue-editor-specimen__note {
    margin: 0;
    font-size: 0.875rem;
    opacity: 0.75;
  }
</style>
