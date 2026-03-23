<script lang="ts">
  import { cssVars } from "@flint/svelte-tokens";
  import { Surface } from "@flint/svelte-primitives";
  import TokenToolsPanel from "../components/TokenToolsPanel.svelte";

  export let liveTokenValues: Partial<Record<string, string>> = {};

  type SemanticTokenPath = keyof typeof cssVars;

  const semanticPaths = Object.keys(cssVars) as SemanticTokenPath[];
  const keySemanticPaths: SemanticTokenPath[] = [
    "semantic.color.background.canvas",
    "semantic.color.background.panel",
    "semantic.color.background.elevated",
    "semantic.color.text.primary",
    "semantic.color.text.secondary",
    "semantic.color.border.default",
    "semantic.color.accent.base",
    "semantic.color.status.success",
    "semantic.size.control.height",
    "semantic.space.control.x",
    "semantic.space.control.y",
  ];

  let activePanelId: "token-summary-section" | "token-inspector" = "token-summary-section";
  let inspectorQuery = "";

  $: filteredTokens = semanticPaths
    .filter((path) => path.toLowerCase().includes(inspectorQuery.trim().toLowerCase()))
    .map((path) => ({ path, value: (liveTokenValues as Record<string, string>)[path] ?? "" }));

  $: keySemanticTokens = keySemanticPaths.map((path) => ({
    path,
    value: (liveTokenValues as Record<string, string>)[path] ?? "",
  }));

  $: matchingTokenCount = filteredTokens.length;

  function handleQueryChange(event: CustomEvent<{ value: string }>): void {
    inspectorQuery = event.detail.value;
  }

  function handleQueryClear(): void {
    inspectorQuery = "";
  }

  function handleSelectPanel(panelId: "token-summary-section" | "token-inspector"): void {
    activePanelId = panelId;
  }
</script>

<article class="tokens-page">
  <Surface tone="panel" border="subtle" padding="lg">
    <TokenToolsPanel
      {activePanelId}
      {keySemanticTokens}
      {filteredTokens}
      {matchingTokenCount}
      {inspectorQuery}
      onSelectPanel={handleSelectPanel}
      onQueryChange={handleQueryChange}
      onQueryClear={handleQueryClear}
    />
  </Surface>
</article>

<style>
  .tokens-page {
    padding: 1.5rem 2rem;
    max-width: 64rem;
    overflow-y: auto;
  }
</style>
