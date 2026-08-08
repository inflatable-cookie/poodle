import {
  Eyebrow,
  Grid,
  Stack,
  Surface,
  Table,
  Tabs,
  TextInput,
  type TabItem,
  type TableColumn,
  type TableRow,
} from "@inflatable-cookie/poodle-react";

type PanelId = "token-summary-section" | "token-inspector";

export interface TokenToolsPanelProps {
  activePanelId?: PanelId;
  keySemanticTokens?: Array<{ path: string; value: string }>;
  filteredTokens?: Array<{ path: string; value: string }>;
  matchingTokenCount?: number;
  inspectorQuery?: string;
  onSelectPanel?: (panelId: PanelId) => void;
  onQueryChange?: (value: string) => void;
  onQueryClear?: () => void;
}

const tabItems: TabItem[] = [
  { value: "token-summary-section", label: "Runtime values" },
  { value: "token-inspector", label: "Inspector" },
];

const inspectorColumns: TableColumn[] = [
  { id: "path", label: "Path", isRowHeader: true },
  { id: "value", label: "Value" },
];

function isColorToken(path: string): boolean {
  return path.includes(".color.") || path.startsWith("color.");
}

export function TokenToolsPanel({
  activePanelId = "token-summary-section",
  keySemanticTokens = [],
  filteredTokens = [],
  matchingTokenCount = 0,
  inspectorQuery = "",
  onSelectPanel = () => {},
  onQueryChange = () => {},
  onQueryClear = () => {},
}: TokenToolsPanelProps) {
  const inspectorRows: TableRow[] = filteredTokens.map((token, i) => ({
    id: `token-${i}`,
    cells: { path: token.path, value: token.value },
  }));

  return (
    <Stack gap="lg" asRole="region" ariaLabel="Token tools">
      <Stack gap="sm">
        <Eyebrow>Token tools</Eyebrow>
        <h2 className="poodle-heading">Runtime values and emitted-token inspection</h2>
      </Stack>

      <Stack gap="md">
        <span className="poodle-hint">@inflatable-cookie/poodle-core/tokens</span>
        <span className="poodle-path">packages/tokens/artifacts/css/</span>
        <span className="poodle-path">packages/tokens/artifacts/ts/</span>
      </Stack>

      <Tabs
        value={activePanelId}
        items={tabItems}
        ariaLabel="Token tools"
        onValueChange={(value) => onSelectPanel(value as PanelId)}
      >
        {(activeValue) =>
          activeValue === "token-summary-section" ? (
            <Grid columns="repeat(auto-fit, minmax(14rem, 1fr))" gap="md">
              {keySemanticTokens.map((token) => (
                <Surface key={token.path} tone="panel" border="subtle" padding="md">
                  <Stack gap="md">
                    <span className="poodle-path">{token.path}</span>
                    <span className="poodle-value-row">
                      {isColorToken(token.path) ? (
                        <span className="poodle-swatch" style={{ background: token.value }} />
                      ) : null}
                      <strong className="poodle-value">{token.value}</strong>
                    </span>
                  </Stack>
                </Surface>
              ))}
            </Grid>
          ) : (
            <Stack gap="md">
              <div className="poodle-search-input">
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
              <p className="poodle-inspector-count">{matchingTokenCount} semantic tokens shown</p>
              <Table
                columns={inspectorColumns}
                rows={inspectorRows}
                ariaLabel="Semantic token inspector"
                emptyMessage="No tokens match the current filter."
              />
            </Stack>
          )
        }
      </Tabs>
    </Stack>
  );
}
