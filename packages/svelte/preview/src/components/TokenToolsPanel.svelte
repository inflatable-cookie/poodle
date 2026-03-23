<script lang="ts">
  import { Eyebrow, Grid, Stack, SearchField, Surface, Table, Tabs, type TabItem, type TableColumn, type TableRow } from "@flint/svelte-primitives";

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

  function isColorToken(path: string): boolean {
    return path.includes(".color.");
  }
</script>

<Stack gap="lg" asRole="region" ariaLabel="Token tools">
  <Stack gap="sm">
    <Eyebrow>Token tools</Eyebrow>
    <h2 class="heading">Runtime values and emitted-token inspection</h2>
  </Stack>

  <Stack gap="md">
    <span class="hint">@flint/svelte-tokens</span>
    <span class="path">packages/tokens/artifacts/css/</span>
    <span class="path">packages/tokens/artifacts/ts/</span>
  </Stack>

  <Tabs
    value={activePanelId}
    items={tabItems}
    ariaLabel="Token tools"
    on:valueChange={(event) =>
      onSelectPanel(event.detail.value as "token-summary-section" | "token-inspector")}
    let:activeValue
  >
    {#if activeValue === "token-summary-section"}
      <Grid columns="repeat(auto-fit, minmax(14rem, 1fr))" gap="md">
        {#each keySemanticTokens as token}
          <Surface tone="default" border="subtle" padding="md">
            <Stack gap="md">
              <span class="path">{token.path}</span>
              <span class="value-row">
                {#if isColorToken(token.path)}
                  <span class="swatch" style="background: {token.value};" />
                {/if}
                <strong class="value">{token.value}</strong>
              </span>
            </Stack>
          </Surface>
        {/each}
      </Grid>
    {:else}
      <Stack gap="md">
        <div class="search-field">
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
      </Stack>
    {/if}
  </Tabs>
</Stack>

<style>
  .heading {
    margin: 0;
    font-size: 1.75rem;
    line-height: 1.1;
  }

  .hint {
    padding: 0.25rem 0.625rem;
    border-radius: 999rem;
    background: color-mix(in srgb, var(--flint-color-accent-base) 18%, transparent);
    color: var(--flint-color-text-primary);
    font-size: 0.75rem;
    font-weight: 700;
  }

  .path {
    color: var(--flint-color-text-secondary);
    font-family: var(--flint-typography-code-family);
    font-size: 0.75rem;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .value-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .swatch {
    flex-shrink: 0;
    width: 1.25rem;
    height: 1.25rem;
    border: 0.0625rem solid color-mix(in srgb, var(--flint-color-border-default) 60%, transparent);
    border-radius: var(--flint-radius-control);
  }

  .value {
    font-family: var(--flint-typography-code-family);
    font-size: 0.8125rem;
  }

  .search-field {
    min-width: min(17.5rem, 100%);
    max-width: 26rem;
  }

  .inspector-count {
    margin: 0;
    color: var(--flint-color-text-secondary);
    font-size: 0.8125rem;
  }
</style>
