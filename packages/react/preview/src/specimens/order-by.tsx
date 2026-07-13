import { useState } from "react";
import { OrderBy, type OrderByValue } from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

function OrderBySpecimen() {
  const [sort, setSort] = useState<OrderByValue>([{ key: "name", direction: "asc" }]);
  return (
    <SpecimenSection title="OrderBy">
      <OrderBy
        value={sort}
        onChange={setSort}
        fields={[
          { key: "name", label: "Name" },
          { key: "created", label: "Created", defaultDirection: "desc" },
          { key: "size", label: "Size" },
        ]}
      />
      <span data-testid="orderby-value">{sort.map((s) => `${s.key}:${s.direction}`).join(",") || "none"}</span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "order-by", title: "OrderBy", render: () => <OrderBySpecimen /> });
