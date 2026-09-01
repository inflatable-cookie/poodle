import { useEffect, useState } from "react";
import type { ModelCatalogueItem } from "@inflatable-cookie/poodle-core";

import { BlockEditor } from "../../packages/react/components/src/BlockEditor";
import { DockRegion } from "../../packages/react/components/src/DockRegion";
import { EditableList } from "../../packages/react/components/src/EditableList";
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
  const [dragEnabledC, setDragEnabledC] = useState(true);
  const [catalogueC, setCatalogueC] = useState<ModelCatalogueItem[]>(() => [
    model("alpha", "Alpha"),
    { ...model("beta", "Beta"), isDisabled: true },
    model("gamma", "Gamma"),
  ]);
  const [orderC, setOrderC] = useState<string[]>([]);
  const [orderCCount, setOrderCCount] = useState(0);

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

  const [rows, setRows] = useState([
    { id: "r1", label: "One" },
    { id: "r2", label: "Two" },
    { id: "r3", label: "Three" },
  ]);
  const [rowCount, setRowCount] = useState(0);

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
      disableDragC() {
        setDragEnabledC(false);
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
        data-rows={rows.map((row) => row.id).join(",")}
        data-rows-count={rowCount}
        data-order-c={orderC.join(",")}
        data-order-c-count={orderCCount}
      />

      <div
        id="react-dock"
        style={{
          width: 280,
          height: 160,
          ["--poodle-radius-surface" as string]: "12px",
          ["--poodle-radius-control" as string]: "4px",
          ["--poodle-border-width-focus" as string]: "2px",
          ["--poodle-color-accent-focusRing" as string]: "rgb(0, 0, 255)",
        }}
      >
        <DockRegion
          edge="left"
          items={[
            { value: "explorer", label: "Explorer" },
            { value: "search", label: "Search" },
          ]}
          value="explorer"
        />
      </div>

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
        <div id="react-mce-c">
          <ModelCatalogueEditor
            items={catalogueC}
            isDragEnabled={dragEnabledC}
            onOrderChange={(ids) => {
              setOrderC(ids);
              setOrderCCount((count) => count + 1);
              setCatalogueC((current) => ids.map((id) => current.find((item) => item.id === id)!).filter(Boolean));
            }}
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

      {/* An EditableList owns its own provider; it is the web keyboard-pickup route. */}
      <div id="react-list">
        <EditableList
          items={rows}
          ariaLabel="Rows"
          reorderable
          onReorder={(next) => {
            setRows(next);
            setRowCount((count) => count + 1);
          }}
        />
      </div>

      {/* OrderBy inside an ambient provider: its panel is portalled out of that
          provider's root, so it must still reorder on its own controller. */}
      <DragDropProvider>
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
      </DragDropProvider>
    </div>
  );
}
