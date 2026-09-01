import { useEffect, useState } from "react";
import type { ModelCatalogueItem } from "@inflatable-cookie/poodle-core";

import { BlockEditor } from "../../packages/react/components/src/BlockEditor";
import { DragDropProvider } from "../../packages/react/components/src/drag-drop";
import { ModelCatalogueEditor } from "../../packages/react/components/src/ModelCatalogueEditor";
import { OrderBy } from "../../packages/react/components/src/OrderBy";
import type { EditorBlock, OrderByValue } from "../../packages/react/components/src/types";

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

const initialCatalogue = () => [model("alpha", "Alpha"), model("beta", "Beta"), model("gamma", "Gamma")];

export function ComponentsHarness() {
  // Deliberately colliding ids across two instances under ONE provider.
  const [catalogueA, setCatalogueA] = useState<ModelCatalogueItem[]>(initialCatalogue);
  const [catalogueB, setCatalogueB] = useState<ModelCatalogueItem[]>(initialCatalogue);
  const [orderA, setOrderA] = useState<string[]>([]);
  const [orderB, setOrderB] = useState<string[]>([]);
  const [orderACount, setOrderACount] = useState(0);
  const [hides, setHides] = useState<string[]>([]);
  const [pendingA, setPendingA] = useState(false);

  const [blocks, setBlocks] = useState<EditorBlock[]>([
    { id: "b1", type: "paragraph", content: "one", data: {} },
    { id: "b2", type: "paragraph", content: "two", data: {} },
    { id: "b3", type: "paragraph", content: "three", data: {} },
  ]);
  const [blockCount, setBlockCount] = useState(0);

  const [sort, setSort] = useState<OrderByValue>([
    { key: "title", direction: "asc" },
    { key: "updated", direction: "desc" },
    { key: "size", direction: "asc" },
  ]);
  const [sortCount, setSortCount] = useState(0);

  useEffect(() => {
    (window as unknown as Record<string, unknown>).__reactFixture = {
      removeCatalogueItem(id: string) {
        setCatalogueA((current) => current.filter((item) => item.id !== id));
      },
      replaceCatalogue(ids: string[]) {
        setCatalogueA((current) => ids.map((id) => current.find((item) => item.id === id)!).filter(Boolean));
      },
      lockCatalogue() {
        setPendingA(true);
      },
      replaceBlocks(ids: string[]) {
        setBlocks((current) => ids.map((id) => current.find((entry) => entry.id === id)!).filter(Boolean));
      },
    };
  }, []);

  return (
    <div id="react-root">
      <div
        id="react-probe"
        data-order-a={orderA.join(",")}
        data-order-b={orderB.join(",")}
        data-order-a-count={orderACount}
        data-hides={hides.join(",")}
        data-blocks={blocks.map((entry) => entry.id).join(",")}
        data-blocks-count={blockCount}
        data-sort={sort.map((entry) => entry.key).join(",")}
        data-sort-count={sortCount}
      />

      {/* One provider, two catalogues, the same three model ids. */}
      <DragDropProvider>
        <div id="react-mce-a">
          <ModelCatalogueEditor
            items={catalogueA}
            isPending={pendingA}
            onOrderChange={(ids) => {
              setOrderA(ids);
              setOrderACount((count) => count + 1);
              setCatalogueA((current) => ids.map((id) => current.find((item) => item.id === id)!).filter(Boolean));
            }}
            onVisibilityChange={(change) => setHides((current) => [...current, `${change.id}:${change.visible}`])}
          />
        </div>
        <div id="react-mce-b">
          <ModelCatalogueEditor items={catalogueB} onOrderChange={setOrderB} />
        </div>
      </DragDropProvider>

      <div id="react-blocks">
        <BlockEditor
          blocks={blocks}
          blockTypes={[{ type: "paragraph", label: "Paragraph" }]}
          onChange={(next) => {
            setBlocks(next);
            setBlockCount((count) => count + 1);
          }}
        />
      </div>

      <div id="react-order">
        <OrderBy
          fields={[
            { key: "title", label: "Title" },
            { key: "updated", label: "Updated" },
            { key: "size", label: "Size" },
          ]}
          value={sort}
          onChange={(next) => {
            setSort(next);
            setSortCount((count) => count + 1);
          }}
        />
      </div>
    </div>
  );
}
