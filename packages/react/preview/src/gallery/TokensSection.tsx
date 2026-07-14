import { useState } from "react";
import { Surface } from "@poodle/react";
import { cssVars } from "@poodle/svelte-tokens";

import { TokenToolsPanel } from "./TokenToolsPanel";

type PanelId = "token-summary-section" | "token-inspector";

export interface TokensSectionProps {
  liveTokenValues?: Partial<Record<string, string>>;
}

const semanticPaths = Object.keys(cssVars);
const keySemanticPaths = [
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

export function TokensSection({ liveTokenValues = {} }: TokensSectionProps) {
  const [activePanelId, setActivePanelId] = useState<PanelId>("token-summary-section");
  const [inspectorQuery, setInspectorQuery] = useState("");

  const values = liveTokenValues as Record<string, string>;
  const filteredTokens = semanticPaths
    .filter((path) => path.toLowerCase().includes(inspectorQuery.trim().toLowerCase()))
    .map((path) => ({ path, value: values[path] ?? "" }));
  const keySemanticTokens = keySemanticPaths.map((path) => ({ path, value: values[path] ?? "" }));

  return (
    <article className="poodle-tokens-page">
      <Surface tone="panel" border="subtle" padding="lg">
        <TokenToolsPanel
          activePanelId={activePanelId}
          keySemanticTokens={keySemanticTokens}
          filteredTokens={filteredTokens}
          matchingTokenCount={filteredTokens.length}
          inspectorQuery={inspectorQuery}
          onSelectPanel={setActivePanelId}
          onQueryChange={setInspectorQuery}
          onQueryClear={() => setInspectorQuery("")}
        />
      </Surface>
    </article>
  );
}
