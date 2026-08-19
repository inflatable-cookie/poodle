<script lang="ts">
  import { Button, Icon, ModelCatalogueEditor } from "@inflatable-cookie/poodle-svelte";
  import { MODEL_CATALOGUE_FIXTURES } from "@inflatable-cookie/poodle-core";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  import ModelCatalogueEditorHarness from "./ModelCatalogueEditorHarness.svelte";
</script>

<SpecimenLayout showSizes={false} showDensities={false}>
  {#snippet children()}
    <div class="poodle-model-catalogue-editor-specimen">
      <SpecimenGroup label="Shown and hidden models">
        <p class="poodle-model-catalogue-editor-specimen__note">
          Shown models keep source order; hidden ones collapse below. Shared Label
          appears twice — identity is the opaque id, never the display label.
        </p>
        <div class="poodle-model-catalogue-editor-specimen__panel">
          <ModelCatalogueEditorHarness />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Reorder and visibility controls">
        <p class="poodle-model-catalogue-editor-specimen__note">
          Pointer drag, keyboard grab, and explicit move buttons are three routes
          to the same reorder. A host may switch either affordance off; hiding and
          restoring stay available.
        </p>
        <div class="poodle-model-catalogue-editor-specimen__stack">
          <div class="poodle-model-catalogue-editor-specimen__panel">
            <ModelCatalogueEditorHarness isDragEnabled={false} />
          </div>
          <div class="poodle-model-catalogue-editor-specimen__panel">
            <ModelCatalogueEditorHarness showMoveActions={false} />
          </div>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Host mark, actions, and row metadata">
        <div class="poodle-model-catalogue-editor-specimen__panel">
          <ModelCatalogueEditorHarness>
            {#snippet leading({ item })}
              {#if item.id === "model-gamma"}
                <Icon name="star" />
              {/if}
            {/snippet}
            {#snippet rowMeta({ item })}
              {#if item.id === "model-gamma"}
                128k context
              {/if}
            {/snippet}
            {#snippet customAction()}
              <Button variant="secondary" size="sm">Add custom model</Button>
            {/snippet}
          </ModelCatalogueEditorHarness>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Loading and pending">
        <div class="poodle-model-catalogue-editor-specimen__stack">
          <div class="poodle-model-catalogue-editor-specimen__panel">
            <ModelCatalogueEditor items={[]} state="loading" />
          </div>
          <!-- A mutation lock leaves the list readable and every control inert. -->
          <div class="poodle-model-catalogue-editor-specimen__panel">
            <ModelCatalogueEditor items={MODEL_CATALOGUE_FIXTURES} isPending={true} />
          </div>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Empty catalogue">
        <div class="poodle-model-catalogue-editor-specimen__panel">
          <ModelCatalogueEditor items={[]} state="empty" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Unavailable, error, and session-negotiated">
        <div class="poodle-model-catalogue-editor-specimen__stack">
          <div class="poodle-model-catalogue-editor-specimen__panel">
            <ModelCatalogueEditor items={[]} state="unavailable" />
          </div>
          <div class="poodle-model-catalogue-editor-specimen__panel">
            <ModelCatalogueEditor items={[]} state="error" />
          </div>
          <div class="poodle-model-catalogue-editor-specimen__panel">
            <ModelCatalogueEditor items={[]} state="sessionNegotiated" />
          </div>
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

  .poodle-model-catalogue-editor-specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-model-catalogue-editor-specimen__note {
    margin: 0 0 0.75rem;
    font-size: 0.875rem;
    opacity: 0.75;
  }
</style>
