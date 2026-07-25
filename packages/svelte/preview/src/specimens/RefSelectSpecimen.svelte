<script lang="ts">
  import { RefSelect, type RefOption } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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

  let value = $state("tree-component");
  let hostQuery = $state("comp");
  // A host-driven search filters upstream; the component renders what it is given.
  const hostFiltered = $derived(
    refs.filter((option) => option.label.toLowerCase().includes(hostQuery.toLowerCase())),
  );

  let sizeValue = $state("main");
  let densityValue = $state("main");
</script>

<SpecimenLayout>
  <SpecimenGroup label="Refs with the checked-out branch marked (live value)">
    <RefSelect {refs} bind:value currentRef="main" />
    <p>Selected: <code>{value}</code> — the marker stays on <code>main</code></p>
  </SpecimenGroup>

  <SpecimenGroup label="Host-driven search (searchValue supplied, host filters)">
    <RefSelect
      refs={hostFiltered}
      value="tree-component"
      currentRef="main"
      searchValue={hostQuery}
      onSearchChange={(next) => (hostQuery = next)}
    />
    <p>Query: <code>{hostQuery}</code> → {hostFiltered.length} ref(s) passed in</p>
  </SpecimenGroup>

  <SpecimenGroup label="Loading more refs">
    <RefSelect {refs} value="main" currentRef="main" loading />
  </SpecimenGroup>

  <SpecimenGroup label="No matches (host-driven, empty list)">
    <RefSelect refs={[]} searchValue="nothing-matches" currentRef="main" />
  </SpecimenGroup>

  <SpecimenGroup label="Search hidden (short lists don't need it)">
    <RefSelect refs={refs.slice(0, 3)} value="main" currentRef="main" searchable={false} />
  </SpecimenGroup>

  <SpecimenGroup label="Outlined trigger">
    <RefSelect {refs} value="main" currentRef="main" variant="outlined" />
  </SpecimenGroup>

  <SpecimenGroup label="Subdued (as embedded in the AgentChatInput footer)">
    <RefSelect {refs} value="main" currentRef="main" emphasis="subdued" />
  </SpecimenGroup>

  <SpecimenGroup label="No selection">
    <RefSelect {refs} value="" currentRef="main" />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <RefSelect {refs} value="main" currentRef="main" disabled />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <RefSelect {refs} {size} bind:value={sizeValue} currentRef="main" />
  {/snippet}

  {#snippet densities(density)}
    <RefSelect {refs} {density} bind:value={densityValue} currentRef="main" />
  {/snippet}
</SpecimenLayout>

<style>
  code {
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
  }
</style>
