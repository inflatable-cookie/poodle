import { useState } from "react";
import { RefSelect, type RefOption } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

// Host vocabulary: Poodle knows the shape of a ref, never git itself.
const refs: RefOption[] = [
  { value: "main", label: "main", kind: "branch", description: "a1b2c3d", group: "Branches" },
  {
    value: "tree-component",
    label: "tree-component",
    kind: "branch",
    description: "9f0e1d2",
    group: "Branches",
  },
  {
    value: "agent-composer",
    label: "agent-composer",
    kind: "branch",
    description: "4c5b6a7",
    group: "Branches",
  },
  { value: "v1.4.0", label: "v1.4.0", kind: "tag", group: "Tags" },
  { value: "v1.3.2", label: "v1.3.2", kind: "tag", group: "Tags" },
  {
    value: "e3f4a5b",
    label: "e3f4a5b",
    kind: "commit",
    description: "Fix the failing parity gate",
    group: "Recent commits",
  },
];

const code = { fontFamily: "var(--poodle-typography-code-family)", fontSize: "0.75rem" } as const;
const stack = { display: "flex", flexDirection: "column", gap: "0.75rem" } as const;

export function RefSelectSpecimen() {
  const [value, setValue] = useState("tree-component");
  const [hostQuery, setHostQuery] = useState("comp");
  const [sizeValue, setSizeValue] = useState("main");
  const [densityValue, setDensityValue] = useState("main");

  // A host-driven search filters upstream; the component renders what it is given.
  const hostFiltered = refs.filter((option) =>
    option.label.toLowerCase().includes(hostQuery.toLowerCase()),
  );

  return (
    <SpecimenLayout
      sizes={(size) => (
        <RefSelect
          refs={refs}
          size={size}
          value={sizeValue}
          onChange={setSizeValue}
          currentRef="main"
        />
      )}
      densities={(density) => (
        <RefSelect
          refs={refs}
          density={density}
          value={densityValue}
          onChange={setDensityValue}
          currentRef="main"
        />
      )}
    >
      <SpecimenGroup label="Branch and tag selection">
        <RefSelect refs={refs} value={value} onChange={setValue} currentRef="main" />
        <p>
          Selected: <code style={code}>{value}</code> — the marker stays on{" "}
          <code style={code}>main</code>
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="Search and no matches">
        <div style={stack}>
          <RefSelect
            refs={hostFiltered}
            value="tree-component"
            currentRef="main"
            searchValue={hostQuery}
            onSearchChange={setHostQuery}
          />
          <p>
            Query: <code style={code}>{hostQuery}</code> → {hostFiltered.length} ref(s) passed in
          </p>
          <RefSelect refs={[]} searchValue="nothing-matches" currentRef="main" />
          <p>Host search with an empty list shows no matches.</p>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Loading and short-list search">
        <div style={stack}>
          <RefSelect refs={refs} value="main" currentRef="main" loading />
          <RefSelect refs={refs.slice(0, 3)} value="main" currentRef="main" searchable={false} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Trigger presentation">
        <div style={stack}>
          <RefSelect refs={refs} value="main" currentRef="main" variant="outlined" />
          <RefSelect refs={refs} value="main" currentRef="main" emphasis="subdued" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Selection states">
        <div style={stack}>
          <RefSelect refs={refs} value="" currentRef="main" />
          <RefSelect refs={refs} value="main" currentRef="main" disabled />
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
