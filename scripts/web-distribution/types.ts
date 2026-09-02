export type JsExportMap = {
  types: string;
  import: string;
  default: string;
};

export type LibraryEntry = {
  name: string;
  source: string;
  outputExt: ".js" | ".mjs";
};

export type AssetCopy = {
  from: string;
  to: string;
};

export type ReceiptTools = {
  svelte: string;
  typescript: string;
  vite: string;
};

export type ReceiptOutput = {
  path: string;
  sha256: string;
};

export type BuildReceipt = {
  cssPolicy: string;
  inputs: string[];
  lanes: string[];
  markdownPolicy: string;
  outputs: ReceiptOutput[];
  package: string;
  schemaVersion: 1;
  sourceCommit: string;
  sourceMaps: false;
  tools: ReceiptTools;
  version: string;
};

export type PackageBuildSpec = {
  packageDir: string;
  packageName: string;
  version: string;
  lanes: ["single"] | ["client", "server"];
  cssPolicy: string;
  markdownPolicy: string;
  entries: LibraryEntry[];
  assets: AssetCopy[];
  declarationTsconfig: string;
  extraDeclarationCopies?: AssetCopy[];
  forbiddenModules: string[];
};

export type BuiltPackage = {
  packageDir: string;
  distDir: string;
  receipt: BuildReceipt;
  receiptPath: string;
};
