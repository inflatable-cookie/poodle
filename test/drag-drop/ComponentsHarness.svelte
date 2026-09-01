<script lang="ts">
  import { default as BlockEditor } from "../../packages/svelte/components/src/BlockEditor.svelte";
  import { default as EditableList } from "../../packages/svelte/components/src/EditableList.svelte";
  import { default as DragDropProvider } from "../../packages/svelte/components/src/DragDropProvider.svelte";
  import { default as ModelCatalogueEditor } from "../../packages/svelte/components/src/ModelCatalogueEditor.svelte";
  import { default as OrderBy } from "../../packages/svelte/components/src/OrderBy.svelte";
  import type { EditorBlock, OrderByValue } from "../../packages/svelte/components/src/types";
  import type { ModelCatalogueItem } from "@inflatable-cookie/poodle-core";

  function model(id: string, label: string): ModelCatalogueItem {
    return {
      id,
      label,
      providerLabel: null,
      description: null,
      badges: [],
      visible: true,
      isDisabled: false,
    };
  }

  // Deliberately colliding ids across two instances under ONE provider.
  let catalogueA = $state([model("alpha", "Alpha"), model("beta", "Beta"), model("gamma", "Gamma")]);
  let catalogueB = $state([model("alpha", "Alpha"), model("beta", "Beta"), model("gamma", "Gamma")]);
  let orderA = $state<string[]>([]);
  let orderB = $state<string[]>([]);
  let orderACount = $state(0);
  let hides = $state<string[]>([]);
  let pendingA = $state(false);
  let dragEnabledC = $state(true);
  let catalogueC = $state([
    model("alpha", "Alpha"),
    { ...model("beta", "Beta"), isDisabled: true },
    model("gamma", "Gamma"),
  ]);
  let orderC = $state<string[]>([]);
  let orderCCount = $state(0);

  let blocks = $state<EditorBlock[]>([
    { id: "b1", type: "paragraph", content: "one", data: {} },
    { id: "b2", type: "paragraph", content: "two", data: {} },
    { id: "b3", type: "paragraph", content: "three", data: {} },
  ]);
  let blockCount = $state(0);

  let sort = $state<OrderByValue>([
    { key: "title", direction: "asc" },
    { key: "updated", direction: "desc" },
    { key: "size", direction: "asc" },
  ]);
  let sortCount = $state(0);

  let rows = $state([
    { id: "r1", label: "One" },
    { id: "r2", label: "Two" },
    { id: "r3", label: "Three" },
  ]);
  let rowCount = $state(0);

  function applyOrder(items: ModelCatalogueItem[], ids: string[]): ModelCatalogueItem[] {
    return ids.map((id) => items.find((item) => item.id === id)!).filter(Boolean);
  }

  $effect(() => {
    (window as unknown as Record<string, unknown>).__svelteFixture = {
      removeCatalogueItem(id: string) {
        catalogueA = catalogueA.filter((item) => item.id !== id);
      },
      replaceCatalogue(ids: string[]) {
        catalogueA = ids.map((id) => catalogueA.find((item) => item.id === id)!).filter(Boolean);
      },
      lockCatalogue() {
        pendingA = true;
      },
      disableDragC() {
        dragEnabledC = false;
      },
      replaceBlocks(ids: string[]) {
        blocks = ids.map((id) => blocks.find((entry) => entry.id === id)!).filter(Boolean);
      },
    };
  });
</script>

<div id="svelte-root">
  <div
    id="svelte-probe"
    data-order-a={orderA.join(",")}
    data-order-b={orderB.join(",")}
    data-order-a-count={orderACount}
    data-hides={hides.join(",")}
    data-blocks={blocks.map((entry) => entry.id).join(",")}
    data-blocks-count={blockCount}
    data-sort={sort.map((entry) => entry.key).join(",")}
    data-sort-count={sortCount}
    data-rows={rows.map((row) => row.id).join(",")}
    data-rows-count={rowCount}
    data-order-c={orderC.join(",")}
    data-order-c-count={orderCCount}
  ></div>

  <!-- One provider, two catalogues, the same three model ids. -->
  <DragDropProvider>
    <div id="svelte-mce-a">
      <ModelCatalogueEditor
        items={catalogueA}
        isPending={pendingA}
        onOrderChange={(ids) => {
          orderA = ids;
          orderACount += 1;
          catalogueA = applyOrder(catalogueA, ids);
        }}
        onVisibilityChange={(change) => (hides = [...hides, `${change.id}:${change.visible}`])}
      />
    </div>
    <div id="svelte-mce-c">
      <ModelCatalogueEditor
        items={catalogueC}
        isDragEnabled={dragEnabledC}
        onOrderChange={(ids) => {
          orderC = ids;
          orderCCount += 1;
          catalogueC = applyOrder(catalogueC, ids);
        }}
      />
    </div>
    <div id="svelte-mce-b">
      <ModelCatalogueEditor items={catalogueB} onOrderChange={(ids) => (orderB = ids)} />
    </div>
  </DragDropProvider>

  <div id="svelte-blocks">
    <BlockEditor
      {blocks}
      blockTypes={[{ type: "paragraph", label: "Paragraph" }]}
      onChange={(next) => {
        blocks = next;
        blockCount += 1;
      }}
    />
  </div>

  <!-- An EditableList owns its own provider; it is the web keyboard-pickup route. -->
  <div id="svelte-list">
    <EditableList
      bind:items={rows}
      ariaLabel="Rows"
      reorderable
      onReorder={() => (rowCount += 1)}
    />
  </div>

  <!-- OrderBy inside an ambient provider: its panel is portalled out of that
       provider's root, so it must still reorder on its own controller. -->
  <DragDropProvider>
  <div id="svelte-order">
    <OrderBy
      fields={[
        { key: "title", label: "Title" },
        { key: "updated", label: "Updated" },
        { key: "size", label: "Size" },
      ]}
      bind:value={sort}
      onChange={(next) => {
        sort = next;
        sortCount += 1;
      }}
    />
  </div>
  </DragDropProvider>
</div>
