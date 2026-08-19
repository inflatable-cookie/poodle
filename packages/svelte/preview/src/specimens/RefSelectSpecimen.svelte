<script lang="ts">
  import { RefSelect, type RefOption } from "@inflatable-cookie/poodle-svelte";
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
  <SpecimenGroup label="Branch and tag selection">
    <RefSelect {refs} bind:value currentRef="main" />
    <p>Selected: <code>{value}</code> — the marker stays on <code>main</code></p>
  </SpecimenGroup>

  <SpecimenGroup label="Search and no matches">
    <div class="poodle-specimen__stack">
      <RefSelect
        refs={hostFiltered}
        value="tree-component"
        currentRef="main"
        searchValue={hostQuery}
        onSearchChange={(next) => (hostQuery = next)}
      />
      <p>Query: <code>{hostQuery}</code> → {hostFiltered.length} ref(s) passed in</p>
      <RefSelect refs={[]} searchValue="nothing-matches" currentRef="main" />
      <p>Host search with an empty list shows no matches.</p>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Loading and short-list search">
    <div class="poodle-specimen__stack">
      <RefSelect {refs} value="main" currentRef="main" loading />
      <RefSelect refs={refs.slice(0, 3)} value="main" currentRef="main" searchable={false} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Trigger presentation">
    <div class="poodle-specimen__stack">
      <RefSelect {refs} value="main" currentRef="main" variant="outlined" />
      <RefSelect {refs} value="main" currentRef="main" emphasis="subdued" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Selection states">
    <div class="poodle-specimen__stack">
      <RefSelect {refs} value="" currentRef="main" />
      <RefSelect {refs} value="main" currentRef="main" disabled />
    </div>
  </SpecimenGroup>

  {#snippet sizes(size)}
    <RefSelect {refs} {size} bind:value={sizeValue} currentRef="main" />
  {/snippet}

  {#snippet densities(density)}
    <RefSelect {refs} {density} bind:value={densityValue} currentRef="main" />
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  code {
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
  }
</style>
