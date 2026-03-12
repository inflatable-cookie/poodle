import { aliases, tokens } from "../../../tokens/artifacts/ts/index";

export type UnderlayBridgeToken = {
  underlayVar: string;
  pugTokenPath: string;
  cssVar: string;
  note?: string;
};

// This map stays intentionally small in g01.013. It proves bridge ownership
// without turning the bridge into a second semantic token source.
export const underlayTokenMap: UnderlayBridgeToken[] = [
  {
    underlayVar: "--underlay-color-canvas",
    pugTokenPath: "semantic.color.background.canvas",
    cssVar: "--pug-semantic-color-background-canvas",
  },
  {
    underlayVar: "--underlay-color-surface",
    pugTokenPath: "semantic.color.background.surface",
    cssVar: "--pug-semantic-color-background-surface",
  },
  {
    underlayVar: "--underlay-color-panel",
    pugTokenPath: "semantic.color.background.panel",
    cssVar: "--pug-semantic-color-background-panel",
  },
  {
    underlayVar: "--underlay-color-text-primary",
    pugTokenPath: "semantic.color.text.primary",
    cssVar: "--pug-semantic-color-text-primary",
  },
  {
    underlayVar: "--underlay-color-text-secondary",
    pugTokenPath: "semantic.color.text.secondary",
    cssVar: "--pug-semantic-color-text-secondary",
  },
  {
    underlayVar: "--underlay-color-border-default",
    pugTokenPath: "semantic.color.border.default",
    cssVar: "--pug-semantic-color-border-default",
  },
  {
    underlayVar: "--underlay-color-accent",
    pugTokenPath: "semantic.color.accent.base",
    cssVar: "--pug-semantic-color-accent-base",
    note: "Bridge-local alias only. Canonical accent meaning still lives in Pug.",
  },
  {
    underlayVar: "--underlay-space-panel-x",
    pugTokenPath: "semantic.space.panel.x",
    cssVar: "--pug-semantic-space-panel-x",
  },
  {
    underlayVar: "--underlay-space-panel-y",
    pugTokenPath: "semantic.space.panel.y",
    cssVar: "--pug-semantic-space-panel-y",
  },
  {
    underlayVar: "--underlay-size-control-height",
    pugTokenPath: "semantic.size.control.height",
    cssVar: "--pug-semantic-size-control-height",
  },
  {
    underlayVar: "--underlay-size-panel-header",
    pugTokenPath: "semantic.size.panel.header",
    cssVar: "--pug-semantic-size-panel-header",
  },
];

export const underlayBridgeAliasNotes = aliases;
export const canonicalTokenFamilies = Object.keys(tokens.semantic);
