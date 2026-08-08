<script lang="ts">
  import { Eyebrow, Grid, Stack, TextInput, Surface, Table, Tabs, type TabItem, type TableColumn, type TableRow } from "@inflatable-cookie/poodle-svelte";
  let {
    activePanelId = "token-summary-section",
    matchingTokenCount = 0,
    inspectorQuery = "",
    onSelectPanel = () => {},
    onQueryChange = () => {},
    onQueryClear = () => {},
    keySemanticTokens = [],
    filteredTokens = [],
  }: {
    activePanelId?: "token-summary-section" | "token-inspector";
    matchingTokenCount?: number;
    inspectorQuery?: string;
    onSelectPanel?: (panelId: "token-summary-section" | "token-inspector") => void;
    onQueryChange?: (value: string) => void;
    onQueryClear?: () => void;
    keySemanticTokens?: Array<{ path: string; value: string }>;
    filteredTokens?: Array<{ path: string; value: string }>;
  } = $props();
  const tabItems: TabItem[] = [
    { value: "token-summary-section", label: "Runtime values" },
    { value: "token-inspector", label: "Inspector" },
  ];

  const inspectorColumns: TableColumn[] = [
    { id: "path", label: "Path", isRowHeader: true },
    { id: "value", label: "Value" },
  ];

  let inspectorRows = $derived(
    filteredTokens.map((token, i): TableRow => ({
      id: `token-${i}`,
      cells: { path: token.path, value: token.value },
    })),
  );

  function isColorToken(path: string): boolean {
    return path.includes(".color.");
  }
</script>

<Stack gap="lg" asRole="region" ariaLabel="Token tools">
  <Stack gap="sm">
    <Eyebrow>Token tools</Eyebrow>
    <h2 class="poodle-heading">Runtime values and emitted-token inspection</h2>
  </Stack>

  <Stack gap="md">
    <span class="poodle-hint">@inflatable-cookie/poodle-core/tokens</span>
    <span class="poodle-path">packages/tokens/artifacts/css/</span>
    <span class="poodle-path">packages/tokens/artifacts/ts/</span>
  </Stack>

  <Tabs
    value={activePanelId}
    items={tabItems}
    ariaLabel="Token tools"
    onValueChange={(value) =>
      onSelectPanel(value as "token-summary-section" | "token-inspector")}
  >
    {#snippet children(activeValue)}
    {#if activeValue === "token-summary-section"}
      <Grid columns="repeat(auto-fit, minmax(14rem, 1fr))" gap="md">
        {#each keySemanticTokens as token}
          <Surface tone="panel" border="subtle" padding="md">
            <Stack gap="md">
              <span class="poodle-path">{token.path}</span>
              <span class="poodle-value-row">
                {#if isColorToken(token.path)}
                  <span class="poodle-swatch" style="background: {token.value};"></span>
                {/if}
                <strong class="poodle-value">{token.value}</strong>
              </span>
            </Stack>
          </Surface>
        {/each}
      </Grid>
    {:else}
      <Stack gap="md">
        <div class="poodle-search-input">
          <TextInput
            id="token-inspector-query"
            type="search"
            value={inspectorQuery}
            placeholder="Filter tokens by path"
            ariaLabel="Filter semantic tokens"
            onValueChange={onQueryChange}
            onClear={onQueryClear}
          />
        </div>
        <p class="poodle-inspector-count">{matchingTokenCount} semantic tokens shown</p>
        <Table
          columns={inspectorColumns}
          rows={inspectorRows}
          ariaLabel="Semantic token inspector"
          emptyMessage="No tokens match the current filter."
        />
      </Stack>
    {/if}
    {/snippet}
  </Tabs>
</Stack>

<style>
  .poodle-heading {
    margin: 0;
    font-size: 1.75rem;
    line-height: 1.1;
  }

  .poodle-hint {
    padding: 0.25rem 0.625rem;
    border-radius: 999rem;
    background: color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent);
    color: var(--poodle-color-text-primary);
    font-size: 0.75rem;
    font-weight: 700;
  }

  .poodle-path {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .poodle-value-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .poodle-swatch {
    flex-shrink: 0;
    width: 1.25rem;
    height: 1.25rem;
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 60%, transparent);
    border-radius: var(--poodle-radius-control);
  }

  .poodle-value {
    font-family: var(--poodle-typography-code-family);
    font-size: 0.8125rem;
  }

  .poodle-search-input {
    min-width: min(17.5rem, 100%);
    max-width: 26rem;
  }

  .poodle-inspector-count {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
  }
</style>
