<script lang="ts">
  import type { Snippet } from "svelte";
  import { ModelCatalogueEditor } from "@inflatable-cookie/poodle-svelte";
  import type { ModelCatalogueItem } from "@inflatable-cookie/poodle-core";
  import { MODEL_CATALOGUE_FIXTURES } from "@inflatable-cookie/poodle-core";

  interface Props {
    customAction?: Snippet;
  }

  let { customAction }: Props = $props();
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

<ModelCatalogueEditor
  {items}
  {customAction}
  onOrderChange={applyOrder}
  onVisibilityChange={applyVisibility}
  onInfo={() => {}}
/>
