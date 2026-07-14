import { useState } from "react";
import { EditableList, type EditableListItemLike } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const specimenItems: EditableListItemLike[] = [
  { id: "1", label: "svelte" },
  { id: "2", label: "typescript" },
  { id: "3", label: "design-system" },
];

export function EditableListSpecimen() {
  const [tags, setTags] = useState<EditableListItemLike[]>([
    { id: "1", label: "svelte" },
    { id: "2", label: "typescript" },
    { id: "3", label: "design-system" },
  ]);

  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={{ width: "min(100%, 26rem)" }}>
          <EditableList
            items={specimenItems}
            editable
            ariaLabel={`Editable list at ${size}`}
            addPlaceholder="Add a tag…"
            addLabel="Add"
            size={size}
          />
        </div>
      )}
      densities={(density) => (
        <div style={{ width: "min(100%, 26rem)" }}>
          <EditableList
            items={specimenItems}
            editable
            ariaLabel={`Editable list at ${density} density`}
            addPlaceholder="Add a tag…"
            addLabel="Add"
            density={density}
          />
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Editable + reorderable">
          <EditableList
            items={tags}
            onChange={setTags}
            editable
            ariaLabel="Tags"
            addPlaceholder="Add a tag…"
            addLabel="Add"
          />
        </SpecimenGroup>

        <SpecimenGroup label="With max items (5)">
          <EditableList
            items={[
              { id: "a", label: "Item A" },
              { id: "b", label: "Item B" },
            ]}
            editable
            maxItems={5}
            ariaLabel="Limited list"
            addPlaceholder="Add item…"
          />
        </SpecimenGroup>

        <SpecimenGroup label="Removable only (no reorder, no add)">
          <EditableList
            items={[
              { id: "x", label: "First item" },
              { id: "y", label: "Second item" },
            ]}
            reorderable={false}
            removable
            ariaLabel="Static list"
          />
        </SpecimenGroup>

        <SpecimenGroup label="Disabled">
          <EditableList items={tags} editable disabled ariaLabel="Disabled list" />
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
