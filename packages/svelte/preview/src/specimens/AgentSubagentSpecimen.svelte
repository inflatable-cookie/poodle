<script lang="ts">
  import { AgentSubagent, type AgentSubagentItem } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const item = (overrides: Partial<AgentSubagentItem>): AgentSubagentItem => ({
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
</script>

<SpecimenLayout>
  <SpecimenGroup
    label="Running"
    description="The child works alongside the turn: identity and status in the header, a dots spinner and a live one-line activity beneath."
  >
    <AgentSubagent item={item({})} {detailLines} />
  </SpecimenGroup>

  <SpecimenGroup
    label="Waiting"
    description="A paused child is not actively working, so there is no spinner — only the activity line."
  >
    <AgentSubagent item={item({ status: "waiting", activityLine: "Waiting for the operator's decision" })} />
  </SpecimenGroup>

  <SpecimenGroup
    label="Completed"
    description="A settled child shows what it accomplished — the summary replaces the activity line, and nothing spins."
  >
    <AgentSubagent item={item({ status: "completed", summary: "Found the drift: three vectors were stale" })} />
  </SpecimenGroup>

  <SpecimenGroup
    label="Failed"
    description="The failure carries its own colour, and the summary says what went wrong."
  >
    <AgentSubagent item={item({ status: "failed", summary: "The parser crate failed to build: 3 errors in lexer.rs" })} />
  </SpecimenGroup>

  <SpecimenGroup
    label="Unknown"
    description="The provider supplied no portable status, so the badge reads literally 'Unknown' — never prettified, and no spinner."
  >
    <AgentSubagent item={item({ status: "unknown", activityLine: undefined })} />
  </SpecimenGroup>

  <SpecimenGroup
    label="Expanded"
    description="The disclosure reveals the child's recent activity lines — a simple block list for v1."
  >
    <AgentSubagent item={item({})} {detailLines} expanded />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <AgentSubagent item={item({ status: "completed" })} {size} />
  {/snippet}

  {#snippet densities(density)}
    <AgentSubagent item={item({ status: "running" })} {density} />
  {/snippet}
</SpecimenLayout>
