import { useState, type CSSProperties } from "react";
import { Button, ModelCatalogueEditor } from "@inflatable-cookie/poodle-react";
import type { ModelCatalogueItem } from "@inflatable-cookie/poodle-core";
import { MODEL_CATALOGUE_FIXTURES } from "@inflatable-cookie/poodle-core";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const stackStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "2rem" };
const panelStyle: CSSProperties = { width: "min(36rem, 100%)" };
const noteStyle: CSSProperties = { margin: 0, fontSize: "0.875rem", opacity: 0.75 };

function ReorderableCatalogue() {
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
    />
  );
}

export function ModelCatalogueEditorSpecimen() {
  return (
    <SpecimenLayout showSizes={false} showDensities={false}>
      <div style={stackStyle}>
        <SpecimenGroup label="Shown and hidden models">
          <div style={panelStyle}>
            <ModelCatalogueEditor items={MODEL_CATALOGUE_FIXTURES} onInfo={() => {}} />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Reorder-capable list">
          <div style={panelStyle}>
            <ReorderableCatalogue />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Duplicate display labels">
          <p style={noteStyle}>
            Shared Label appears twice with distinct opaque ids in the fixtures above.
          </p>
        </SpecimenGroup>

        <SpecimenGroup label="Custom action">
          <div style={panelStyle}>
            <ModelCatalogueEditor
              items={MODEL_CATALOGUE_FIXTURES}
              customAction={() => (
                <Button variant="secondary" size="sm">
                  Add custom model
                </Button>
              )}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Loading">
          <div style={panelStyle}>
            <ModelCatalogueEditor items={[]} state="loading" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Unavailable">
          <div style={panelStyle}>
            <ModelCatalogueEditor items={[]} state="unavailable" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Empty">
          <div style={panelStyle}>
            <ModelCatalogueEditor items={[]} state="empty" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Error">
          <div style={panelStyle}>
            <ModelCatalogueEditor items={[]} state="error" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Session negotiated">
          <div style={panelStyle}>
            <ModelCatalogueEditor items={[]} state="sessionNegotiated" />
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
