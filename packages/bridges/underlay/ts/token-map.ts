import { tokens } from "../../../tokens/artifacts/ts/index";

export type UnderlayBridgeToken = {
  underlayVar: string;
  poodleTokenPath: string;
  cssVar: string;
  note?: string;
};

// This map stays intentionally small in g01.013. It proves bridge ownership
// without turning the bridge into a second semantic token source.
export const underlayTokenMap: UnderlayBridgeToken[] = [
  {
    underlayVar: "--underlay-color-canvas",
    poodleTokenPath: "semantic.color.background.canvas",
    cssVar: "--poodle-semantic-color-background-canvas",
  },
  {
    underlayVar: "--underlay-color-surface",
    poodleTokenPath: "semantic.color.background.surface",
    cssVar: "--poodle-semantic-color-background-surface",
  },
  {
    underlayVar: "--underlay-color-panel",
    poodleTokenPath: "semantic.color.background.panel",
    cssVar: "--poodle-semantic-color-background-panel",
  },
  {
    underlayVar: "--underlay-color-text-primary",
    poodleTokenPath: "semantic.color.text.primary",
    cssVar: "--poodle-semantic-color-text-primary",
  },
  {
    underlayVar: "--underlay-color-text-secondary",
    poodleTokenPath: "semantic.color.text.secondary",
    cssVar: "--poodle-semantic-color-text-secondary",
  },
  {
    underlayVar: "--underlay-color-border-default",
    poodleTokenPath: "semantic.color.border.default",
    cssVar: "--poodle-semantic-color-border-default",
  },
  {
    underlayVar: "--underlay-color-accent",
    poodleTokenPath: "semantic.color.accent.base",
    cssVar: "--poodle-semantic-color-accent-base",
    note: "Bridge-local alias only. Canonical accent meaning still lives in Poodle.",
  },
  {
    underlayVar: "--underlay-space-panel-x",
    poodleTokenPath: "semantic.space.panel.x",
    cssVar: "--poodle-semantic-space-panel-x",
  },
  {
    underlayVar: "--underlay-space-panel-y",
    poodleTokenPath: "semantic.space.panel.y",
    cssVar: "--poodle-semantic-space-panel-y",
  },
  {
    underlayVar: "--underlay-size-control-height",
    poodleTokenPath: "semantic.size.control.height",
    cssVar: "--poodle-semantic-size-control-height",
  },
  {
    underlayVar: "--underlay-size-panel-header",
    poodleTokenPath: "semantic.size.panel.header",
    cssVar: "--poodle-semantic-size-panel-header",
  },
];

export const canonicalTokenFamilies = Object.keys(tokens.semantic);
