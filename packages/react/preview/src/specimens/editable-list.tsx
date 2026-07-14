import { useState } from "react";
import { EditableList } from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

function EditableListSpecimen() {
  const [items, setItems] = useState([
    { id: "a", label: "Alpha" },
    { id: "b", label: "Beta" },
    { id: "c", label: "Gamma" },
  ]);
  return (
    <SpecimenSection title="EditableList">
      <EditableList
        items={items}
        onChange={setItems}
        editable
        removable
        showWorkflowChrome={false}
        maxItems={6}
      />
      <span data-testid="el-order">{items.map((i) => i.label).join(",")}</span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "editable-list", title: "EditableList", render: () => <EditableListSpecimen /> });
