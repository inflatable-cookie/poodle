<script lang="ts">
  import { cssVars } from "@inflatable-cookie/poodle-core/tokens";
  import { Surface } from "@inflatable-cookie/poodle-svelte";
  import TokenToolsPanel from "../components/TokenToolsPanel.svelte";
  let { liveTokenValues = {} }: { liveTokenValues?: Partial<Record<string, string>> } = $props();
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

  let activePanelId: "token-summary-section" | "token-inspector" = $state("token-summary-section");
  let inspectorQuery = $state("");

  let filteredTokens = $derived(
    semanticPaths
      .filter((path) => path.toLowerCase().includes(inspectorQuery.trim().toLowerCase()))
      .map((path) => ({ path, value: (liveTokenValues as Record<string, string>)[path] ?? "" })),
  );

  let keySemanticTokens = $derived(
    keySemanticPaths.map((path) => ({
      path,
      value: (liveTokenValues as Record<string, string>)[path] ?? "",
    })),
  );

  let matchingTokenCount = $derived(filteredTokens.length);
  function handleQueryChange(value: string): void {
    inspectorQuery = value;
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
