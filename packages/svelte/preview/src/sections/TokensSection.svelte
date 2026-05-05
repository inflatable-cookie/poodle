<script lang="ts">
  import { cssVars } from "@poodle/svelte-tokens";
  import { Surface } from "@poodle/svelte";
  import TokenToolsPanel from "../components/TokenToolsPanel.svelte";

  export let liveTokenValues: Partial<Record<string, string>> = {};

  type SemanticTokenPath = keyof typeof cssVars;

  const semanticPaths = Object.keys(cssVars) as SemanticTokenPath[];
  const keySemanticPaths: SemanticTokenPath[] = [
    "color.background.canvas",
    "color.background.panel",
    "color.background.elevated",
    "color.text.primary",
    "color.text.secondary",
    "color.border.default",
    "color.accent.base",
    "color.status.success",
    "size.control.height",
    "space.control.x",
    "space.control.y",
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

<article class="poodle-tokens-page">
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
  .poodle-tokens-page {
    padding: 1.5rem 2rem;
    max-width: 64rem;
    overflow-y: auto;
  }
</style>
