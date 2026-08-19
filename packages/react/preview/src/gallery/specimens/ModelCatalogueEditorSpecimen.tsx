import { useState, type CSSProperties, type ReactNode } from "react";
import { Button, Icon, ModelCatalogueEditor } from "@inflatable-cookie/poodle-react";
import type { ModelCatalogueItem } from "@inflatable-cookie/poodle-core";
import { MODEL_CATALOGUE_FIXTURES } from "@inflatable-cookie/poodle-core";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const stackStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "2rem" };
const panelStyle: CSSProperties = { width: "min(36rem, 100%)" };
const groupStackStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "1rem" };
const noteStyle: CSSProperties = { margin: "0 0 0.75rem", fontSize: "0.875rem", opacity: 0.75 };

interface InteractiveCatalogueProps {
  customAction?: () => ReactNode;
  leading?: (props: { item: ModelCatalogueItem }) => ReactNode;
  rowMeta?: (props: { item: ModelCatalogueItem }) => ReactNode;
  isDragEnabled?: boolean;
  showMoveActions?: boolean;
}

function InteractiveCatalogue({
  customAction,
  leading,
  rowMeta,
  isDragEnabled = true,
  showMoveActions = true,
}: InteractiveCatalogueProps) {
  const [items, setItems] = useState<ModelCatalogueItem[]>([...MODEL_CATALOGUE_FIXTURES]);

  function applyOrder(orderedIds: string[]): void {
    setItems((current) => {
      const hidden = current.filter((item) => !item.visible);
      const byId = new Map(current.map((item) => [item.id, item]));
      const reordered = orderedIds.map((id) => byId.get(id)!);
      return [...reordered, ...hidden];
    });
  }

  function applyVisibility(change: { id: string; visible: boolean }): void {
    setItems((current) =>
      current.map((item) =>
        item.id === change.id ? { ...item, visible: change.visible } : item,
      ),
    );
  }

  return (
    <ModelCatalogueEditor
      items={items}
      onOrderChange={applyOrder}
      onVisibilityChange={applyVisibility}
      onInfo={() => {}}
      customAction={customAction}
      leading={leading}
      rowMeta={rowMeta}
      isDragEnabled={isDragEnabled}
      showMoveActions={showMoveActions}
    />
  );
}

export function ModelCatalogueEditorSpecimen() {
  return (
    <SpecimenLayout showSizes={false} showDensities={false}>
      <div style={stackStyle}>
        <SpecimenGroup label="Shown and hidden models">
          <p style={noteStyle}>
            Shown models keep source order; hidden ones collapse below. Shared Label
            appears twice — identity is the opaque id, never the display label.
          </p>
          <div style={panelStyle}>
            <InteractiveCatalogue />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Reorder and visibility controls">
          <p style={noteStyle}>
            Pointer drag, keyboard grab, and explicit move buttons are three routes
            to the same reorder. A host may switch either affordance off; hiding and
            restoring stay available.
          </p>
          <div style={groupStackStyle}>
            <div style={panelStyle}>
              <InteractiveCatalogue isDragEnabled={false} />
            </div>
            <div style={panelStyle}>
              <InteractiveCatalogue showMoveActions={false} />
            </div>
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Host mark, actions, and row metadata">
          <div style={panelStyle}>
            <InteractiveCatalogue
              leading={({ item }) => (item.id === "model-gamma" ? <Icon name="star" /> : null)}
              rowMeta={({ item }) => (item.id === "model-gamma" ? "128k context" : null)}
              customAction={() => (
                <Button variant="secondary" size="sm">
                  Add custom model
                </Button>
              )}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Loading and pending">
          <div style={groupStackStyle}>
            <div style={panelStyle}>
              <ModelCatalogueEditor items={[]} state="loading" />
            </div>
            {/* A mutation lock leaves the list readable and every control inert. */}
            <div style={panelStyle}>
              <ModelCatalogueEditor items={MODEL_CATALOGUE_FIXTURES} isPending={true} />
            </div>
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Empty catalogue">
          <div style={panelStyle}>
            <ModelCatalogueEditor items={[]} state="empty" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Unavailable, error, and session-negotiated">
          <div style={groupStackStyle}>
            <div style={panelStyle}>
              <ModelCatalogueEditor items={[]} state="unavailable" />
            </div>
            <div style={panelStyle}>
              <ModelCatalogueEditor items={[]} state="error" />
            </div>
            <div style={panelStyle}>
              <ModelCatalogueEditor items={[]} state="sessionNegotiated" />
            </div>
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
