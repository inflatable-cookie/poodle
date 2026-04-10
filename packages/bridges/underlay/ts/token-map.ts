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
    poodleTokenPath: "color.background.canvas",
    cssVar: "--poodle-color-background-canvas",
  },
  {
    underlayVar: "--underlay-color-surface",
    poodleTokenPath: "color.background.surface",
    cssVar: "--poodle-color-background-surface",
  },
  {
    underlayVar: "--underlay-color-panel",
    poodleTokenPath: "color.background.panel",
    cssVar: "--poodle-color-background-panel",
  },
  {
    underlayVar: "--underlay-color-text-primary",
    poodleTokenPath: "color.text.primary",
    cssVar: "--poodle-color-text-primary",
  },
  {
    underlayVar: "--underlay-color-text-secondary",
    poodleTokenPath: "color.text.secondary",
    cssVar: "--poodle-color-text-secondary",
  },
  {
    underlayVar: "--underlay-color-border-default",
    poodleTokenPath: "color.border.default",
    cssVar: "--poodle-color-border-default",
  },
  {
    underlayVar: "--underlay-color-accent",
    poodleTokenPath: "color.accent.base",
    cssVar: "--poodle-color-accent-base",
    note: "Bridge-local alias only. Canonical accent meaning still lives in Poodle.",
  },
  {
    underlayVar: "--underlay-space-panel-x",
    poodleTokenPath: "space.panel.x",
    cssVar: "--poodle-space-panel-x",
  },
  {
    underlayVar: "--underlay-space-panel-y",
    poodleTokenPath: "space.panel.y",
    cssVar: "--poodle-space-panel-y",
  },
  {
    underlayVar: "--underlay-size-control-height",
    poodleTokenPath: "size.control.height",
    cssVar: "--poodle-size-control-height",
  },
  {
    underlayVar: "--underlay-size-panel-header",
    poodleTokenPath: "size.panel.header",
    cssVar: "--poodle-size-panel-header",
  },
];

export const canonicalTokenFamilies = Object.keys(tokens.semantic);
