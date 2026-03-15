<script lang="ts">
  import { Eyebrow, SearchField, Table, Tabs, type TabItem, type TableColumn, type TableRow } from "@pug/svelte-primitives";

  export let activePanelId: "token-summary-section" | "token-inspector" = "token-summary-section";
  export let keySemanticTokens: Array<{ path: string; value: string }> = [];
  export let filteredTokens: Array<{ path: string; value: string }> = [];
  export let matchingTokenCount = 0;
  export let inspectorQuery = "";
  export let onSelectPanel: (panelId: "token-summary-section" | "token-inspector") => void = () => {};
  export let onQueryChange: (event: CustomEvent<{ value: string }>) => void = () => {};
  export let onQueryClear: () => void = () => {};

  const tabItems: TabItem[] = [
    { value: "token-summary-section", label: "Runtime values" },
    { value: "token-inspector", label: "Inspector" },
  ];

  const inspectorColumns: TableColumn[] = [
    { id: "path", label: "Path", isRowHeader: true },
    { id: "value", label: "Value" },
  ];

  $: inspectorRows = filteredTokens.map((token, i): TableRow => ({
    id: `token-${i}`,
    cells: { path: token.path, value: token.value },
  }));
</script>

<section class="panel token-tools-shell" aria-labelledby="token-tools-heading">
  <div class="section-header">
    <div>
      <Eyebrow>Token tools</Eyebrow>
      <h2 id="token-tools-heading">Runtime values and emitted-token inspection</h2>
    </div>
  </div>
  <div class="section-meta-bar">
    <span class="command-shortcut-hint">@pug/svelte-tokens</span>
    <span class="token-path">packages/tokens/artifacts/css/</span>
    <span class="token-path">packages/tokens/artifacts/ts/</span>
  </div>

  <Tabs
    value={activePanelId}
    items={tabItems}
    ariaLabel="Token tools"
    on:valueChange={(event) =>
      onSelectPanel(event.detail.value as "token-summary-section" | "token-inspector")}
    let:activeValue
  >
    {#if activeValue === "token-summary-section"}
      <div class="summary-grid">
        {#each keySemanticTokens as token}
          <article class="summary-tile">
            <span class="token-path">{token.path}</span>
            <strong>{token.value}</strong>
          </article>
        {/each}
      </div>
    {:else}
      <div class="token-tools-inspector">
        <div class="filter-field token-tools-inspector__search">
          <SearchField
            id="token-inspector-query"
            value={inspectorQuery}
            placeholder="Filter tokens by path"
            ariaLabel="Filter semantic tokens"
            on:valueChange={onQueryChange}
            on:clear={onQueryClear}
          />
        </div>
        <p class="inspector-count">{matchingTokenCount} semantic tokens shown</p>
        <Table
          columns={inspectorColumns}
          rows={inspectorRows}
          ariaLabel="Semantic token inspector"
          emptyMessage="No tokens match the current filter."
        />
      </div>
    {/if}
  </Tabs>
</section>
