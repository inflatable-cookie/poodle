<script lang="ts">
  import { cssVars } from "@pug/svelte-tokens";
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
