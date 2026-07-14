import { useState } from "react";
import { OrderBy, type OrderByValue, type SortField } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const fields: SortField[] = [
  { key: "title", label: "Title" },
  { key: "kind", label: "Kind" },
  { key: "updatedAt", label: "Updated", defaultDirection: "desc" },
  { key: "createdAt", label: "Created", defaultDirection: "desc" },
  { key: "visibility", label: "Visibility", disabled: true },
];

export function OrderBySpecimen() {
  const [value, setValue] = useState<OrderByValue>([
    { key: "updatedAt", direction: "desc" },
    { key: "title", direction: "asc" },
  ]);
  const [sizeValue, setSizeValue] = useState<OrderByValue>([{ key: "title", direction: "asc" }]);
  const [densityValue, setDensityValue] = useState<OrderByValue>([{ key: "title", direction: "asc" }]);

  return (
    <SpecimenLayout
      sizes={(size) => <OrderBy fields={fields} size={size} value={sizeValue} onChange={setSizeValue} />}
      densities={(density) => (
        <OrderBy fields={fields} density={density} value={densityValue} onChange={setDensityValue} />
      )}
    >
      <SpecimenGroup label="Multi-field sort builder">
        <OrderBy fields={fields} value={value} onChange={setValue} compact />
        <pre style={{ margin: 0, fontSize: "0.75rem" }}>{JSON.stringify(value, null, 2)}</pre>
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <OrderBy fields={fields} value={[{ key: "title", direction: "asc" }]} disabled />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
