import {
  AgentSubagent,
  type AgentSubagentItem,
} from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const item = (
  overrides: Partial<AgentSubagentItem> = {},
): AgentSubagentItem => ({
  id: "child-1",
  label: "Scout",
  status: "running",
  activityLine: "Searching the parser crate for the drift",
  summary: "Found the drift: three vectors were stale",
  ...overrides,
});

const detailLines = [
  "Searching packages/contracts/headless/vectors for stale fixtures",
  "Matched 41 of 44 vectors against the TS core",
  "Diffing the three misses against the Rust mirror",
];

export function AgentSubagentSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => (
        <AgentSubagent item={item({ status: "completed" })} size={size} />
      )}
      densities={(density) => <AgentSubagent item={item()} density={density} />}
    >
      <SpecimenGroup label="Running">
        <AgentSubagent item={item()} detailLines={detailLines} />
      </SpecimenGroup>
      <SpecimenGroup label="Waiting">
        <AgentSubagent
          item={item({
            status: "waiting",
            activityLine: "Waiting for the operator's decision",
          })}
        />
      </SpecimenGroup>
      <SpecimenGroup label="Completed">
        <AgentSubagent item={item({ status: "completed" })} />
      </SpecimenGroup>
      <SpecimenGroup label="Failed">
        <AgentSubagent
          item={item({
            status: "failed",
            summary: "The parser crate failed to build: 3 errors in lexer.rs",
          })}
        />
      </SpecimenGroup>
      <SpecimenGroup label="Unknown">
        <AgentSubagent
          item={item({ status: "unknown", activityLine: undefined })}
        />
      </SpecimenGroup>
      <SpecimenGroup label="Expanded">
        <AgentSubagent item={item()} detailLines={detailLines} expanded />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
