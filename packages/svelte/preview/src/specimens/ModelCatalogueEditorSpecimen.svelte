<script lang="ts">
  import { Button, ModelCatalogueEditor } from "@inflatable-cookie/poodle-svelte";
  import type { ModelCatalogueItem } from "@inflatable-cookie/poodle-core";
  import { MODEL_CATALOGUE_FIXTURES } from "@inflatable-cookie/poodle-core";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let items = $state<ModelCatalogueItem[]>([...MODEL_CATALOGUE_FIXTURES]);

  function applyOrder(orderedIds: string[]): void {
    const hidden = items.filter((item) => !item.visible);
    const byId = new Map(items.map((item) => [item.id, item]));
    const reordered = orderedIds.map((id) => byId.get(id)!);
    items = [...reordered, ...hidden];
  }

  function applyVisibility(change: { id: string; visible: boolean }): void {
    items = items.map((item) =>
      item.id === change.id ? { ...item, visible: change.visible } : item,
    );
  }
</script>

<SpecimenLayout showSizes={false} showDensities={false}>
  {#snippet children()}
    <div class="poodle-model-catalogue-editor-specimen">
      <SpecimenGroup label="Shown and hidden models">
        <div class="poodle-model-catalogue-editor-specimen__panel">
          <ModelCatalogueEditor
            items={MODEL_CATALOGUE_FIXTURES}
            onInfo={() => {}}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Reorder-capable list">
        <div class="poodle-model-catalogue-editor-specimen__panel">
          <ModelCatalogueEditor
            {items}
            onOrderChange={applyOrder}
            onVisibilityChange={applyVisibility}
            onInfo={() => {}}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Duplicate display labels">
        <p class="poodle-model-catalogue-editor-specimen__note">
          Shared Label appears twice with distinct opaque ids in the fixtures above.
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="Custom action">
        <div class="poodle-model-catalogue-editor-specimen__panel">
          <ModelCatalogueEditor items={MODEL_CATALOGUE_FIXTURES}>
            {#snippet customAction()}
              <Button variant="secondary" size="sm">Add custom model</Button>
            {/snippet}
          </ModelCatalogueEditor>
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
