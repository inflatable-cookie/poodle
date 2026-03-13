<script lang="ts">
  import { SearchField, Tabs, type TabDefinition } from "@pug/svelte-primitives";

  export let activePanelId: "token-summary-section" | "token-inspector" = "token-summary-section";
  export let keySemanticTokens: Array<{ path: string; value: string }> = [];
  export let filteredTokens: Array<{ path: string; value: string }> = [];
  export let matchingTokenCount = 0;
  export let inspectorQuery = "";
  export let onSelectPanel: (panelId: "token-summary-section" | "token-inspector") => void = () => {};
  export let onQueryChange: (event: CustomEvent<{ value: string }>) => void = () => {};
  export let onQueryClear: () => void = () => {};

  const tabs: TabDefinition[] = [
    { value: "token-summary-section", label: "Runtime values" },
    { value: "token-inspector", label: "Inspector" },
  ];
</script>

<section class="panel token-tools-shell" aria-labelledby="token-tools-heading">
  <div class="section-header">
    <div>
      <p class="eyebrow">Token tools</p>
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
    {tabs}
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
        <div class="token-table-wrap">
          <table>
            <thead>
              <tr>
                <th scope="col">Path</th>
                <th scope="col">Value</th>
              </tr>
            </thead>
            <tbody>
              {#each filteredTokens as token}
                <tr>
                  <td class="token-path">{token.path}</td>
                  <td>{token.value}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {/if}
  </Tabs>
</section>
