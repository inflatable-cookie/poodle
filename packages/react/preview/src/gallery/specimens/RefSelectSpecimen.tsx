import { useState } from "react";
import { RefSelect, type RefOption } from "@poodle/react";
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
      <SpecimenGroup label="Refs with the checked-out branch marked (live value)">
        <RefSelect refs={refs} value={value} onChange={setValue} currentRef="main" />
        <p>
          Selected: <code style={code}>{value}</code> — the marker stays on{" "}
          <code style={code}>main</code>
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="Host-driven search (searchValue supplied, host filters)">
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
      </SpecimenGroup>

      <SpecimenGroup label="Loading more refs">
        <RefSelect refs={refs} value="main" currentRef="main" loading />
      </SpecimenGroup>

      <SpecimenGroup label="No matches (host-driven, empty list)">
        <RefSelect refs={[]} searchValue="nothing-matches" currentRef="main" />
      </SpecimenGroup>

      <SpecimenGroup label="Search hidden (short lists don't need it)">
        <RefSelect refs={refs.slice(0, 3)} value="main" currentRef="main" searchable={false} />
      </SpecimenGroup>

      <SpecimenGroup label="Outlined trigger">
        <RefSelect refs={refs} value="main" currentRef="main" variant="outlined" />
      </SpecimenGroup>

      <SpecimenGroup label="Subdued (as embedded in the AgentChatInput footer)">
        <RefSelect refs={refs} value="main" currentRef="main" emphasis="subdued" />
      </SpecimenGroup>

      <SpecimenGroup label="No selection">
        <RefSelect refs={refs} value="" currentRef="main" />
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <RefSelect refs={refs} value="main" currentRef="main" disabled />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
