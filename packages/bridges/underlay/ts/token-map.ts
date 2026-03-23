import { aliases, tokens } from "../../../tokens/artifacts/ts/index";

export type UnderlayBridgeToken = {
  underlayVar: string;
  flintTokenPath: string;
  cssVar: string;
  note?: string;
};

// This map stays intentionally small in g01.013. It proves bridge ownership
// without turning the bridge into a second semantic token source.
export const underlayTokenMap: UnderlayBridgeToken[] = [
  {
    underlayVar: "--underlay-color-canvas",
    flintTokenPath: "semantic.color.background.canvas",
    cssVar: "--flint-semantic-color-background-canvas",
  },
  {
    underlayVar: "--underlay-color-surface",
    flintTokenPath: "semantic.color.background.surface",
    cssVar: "--flint-semantic-color-background-surface",
  },
  {
    underlayVar: "--underlay-color-panel",
    flintTokenPath: "semantic.color.background.panel",
    cssVar: "--flint-semantic-color-background-panel",
  },
  {
    underlayVar: "--underlay-color-text-primary",
    flintTokenPath: "semantic.color.text.primary",
    cssVar: "--flint-semantic-color-text-primary",
  },
  {
    underlayVar: "--underlay-color-text-secondary",
    flintTokenPath: "semantic.color.text.secondary",
    cssVar: "--flint-semantic-color-text-secondary",
  },
  {
    underlayVar: "--underlay-color-border-default",
    flintTokenPath: "semantic.color.border.default",
    cssVar: "--flint-semantic-color-border-default",
  },
  {
    underlayVar: "--underlay-color-accent",
    flintTokenPath: "semantic.color.accent.base",
    cssVar: "--flint-semantic-color-accent-base",
    note: "Bridge-local alias only. Canonical accent meaning still lives in Flint.",
  },
  {
    underlayVar: "--underlay-space-panel-x",
    flintTokenPath: "semantic.space.panel.x",
    cssVar: "--flint-semantic-space-panel-x",
  },
  {
    underlayVar: "--underlay-space-panel-y",
    flintTokenPath: "semantic.space.panel.y",
    cssVar: "--flint-semantic-space-panel-y",
  },
  {
    underlayVar: "--underlay-size-control-height",
    flintTokenPath: "semantic.size.control.height",
    cssVar: "--flint-semantic-size-control-height",
  },
  {
    underlayVar: "--underlay-size-panel-header",
    flintTokenPath: "semantic.size.panel.header",
    cssVar: "--flint-semantic-size-panel-header",
  },
];

export const underlayBridgeAliasNotes = aliases;
export const canonicalTokenFamilies = Object.keys(tokens.semantic);
