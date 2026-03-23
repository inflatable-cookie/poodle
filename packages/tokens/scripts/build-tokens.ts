import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

type JsonPrimitive = string | number | boolean | null;
type JsonValue = JsonPrimitive | JsonObject | JsonValue[];
type JsonObject = { [key: string]: JsonValue };

type TokenValue = string | number | boolean;

interface TokenLeaf {
  $type?: string;
  $value: TokenValue;
  $description?: string;
}

interface TokenEntry {
  path: string;
  type: string;
  value: TokenValue;
  description: string;
}

interface ResolvedTokenEntry extends TokenEntry {
  rawValue: TokenValue;
  resolvedValue: TokenValue;
}

interface NamedModeDefinition {
  name: string;
  selector: string;
  description: string;
  entries: ResolvedTokenEntry[];
}

interface ThemeMetadata {
  selector: string;
  description: string;
}

interface AliasMetadata {
  from: string;
  to: string;
  note?: string;
}

interface DeprecationMetadata {
  path: string;
  status: string;
  replacement?: string | null;
  note?: string;
}

interface Manifest {
  name: string;
  version: string;
  canonicalFormat: string;
  artifactBaseline: string;
}

function isJsonObject(value: JsonValue | undefined): value is JsonObject {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function isTokenLeaf(value: JsonValue | undefined): value is TokenLeaf {
  return isJsonObject(value) && "$value" in value;
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const tokensDir = path.resolve(scriptDir, "..");
const schemaDir = path.join(tokensDir, "schema");
const artifactDir = path.join(tokensDir, "artifacts");
const svelteTokensGeneratedDir = path.resolve(tokensDir, "../svelte/tokens/src/generated");

function readJson<T>(filePath: string): T {
  return JSON.parse(fs.readFileSync(filePath, "utf8")) as T;
}

function listJsonFiles(dirPath: string): string[] {
  return fs
    .readdirSync(dirPath, { withFileTypes: true })
    .filter((entry) => entry.isFile() && entry.name.endsWith(".json"))
    .map((entry) => path.join(dirPath, entry.name))
    .sort();
}

function deepMerge(target: JsonObject, source: JsonObject): JsonObject {
  const output: JsonObject = { ...target };

  for (const [key, value] of Object.entries(source)) {
    if (isJsonObject(value) && !("$value" in value)) {
      output[key] = deepMerge((output[key] as JsonObject | undefined) ?? {}, value);
    } else {
      output[key] = value;
    }
  }

  return output;
}

function loadDirectoryObject(relativeDir: string): JsonObject {
  const dirPath = path.join(schemaDir, relativeDir);
  return listJsonFiles(dirPath).reduce<JsonObject>(
    (accumulator, filePath) => deepMerge(accumulator, readJson<JsonObject>(filePath)),
    {},
  );
}

function loadNamedDirectory(relativeDir: string): Record<string, JsonObject> {
  const dirPath = path.join(schemaDir, relativeDir);
  return listJsonFiles(dirPath).reduce<Record<string, JsonObject>>((accumulator, filePath) => {
    const name = path.basename(filePath, ".json");
    accumulator[name] = readJson<JsonObject>(filePath);
    return accumulator;
  }, {});
}

function collectTokenEntries(
  node: JsonObject,
  prefix: string[] = [],
  entries: TokenEntry[] = [],
): TokenEntry[] {
  for (const [key, value] of Object.entries(node)) {
    if (key.startsWith("$")) {
      continue;
    }

    if (isTokenLeaf(value)) {
      entries.push({
        path: [...prefix, key].join("."),
        type: value.$type ?? "unknown",
        value: value.$value,
        description: value.$description ?? "",
      });
      continue;
    }

    if (isJsonObject(value)) {
      collectTokenEntries(value, [...prefix, key], entries);
    }
  }

  return entries;
}

function isReference(value: TokenValue): value is string {
  return typeof value === "string" && /^\{.+\}$/.test(value);
}

function resolveEntries(entries: TokenEntry[]): ResolvedTokenEntry[] {
  const sourceByPath = new Map(entries.map((entry) => [entry.path, entry]));
  const cache = new Map<string, ResolvedTokenEntry>();

  function resolvePath(tokenPath: string, trail: string[] = []): ResolvedTokenEntry {
    const cached = cache.get(tokenPath);
    if (cached) {
      return cached;
    }

    if (trail.includes(tokenPath)) {
      throw new Error(`Circular token reference: ${[...trail, tokenPath].join(" -> ")}`);
    }

    const entry = sourceByPath.get(tokenPath);
    if (!entry) {
      throw new Error(`Unknown token reference: ${tokenPath}`);
    }

    let resolvedValue: TokenValue = entry.value;
    if (isReference(entry.value)) {
      const referencedPath = entry.value.slice(1, -1);
      resolvedValue = resolvePath(referencedPath, [...trail, tokenPath]).resolvedValue;
    }

    const resolvedEntry: ResolvedTokenEntry = {
      ...entry,
      rawValue: entry.value,
      resolvedValue,
    };
    cache.set(tokenPath, resolvedEntry);
    return resolvedEntry;
  }

  return entries.map((entry) => resolvePath(entry.path));
}

function setNested(target: JsonObject, pathParts: string[], value: TokenValue): void {
  let cursor = target;
  for (const part of pathParts.slice(0, -1)) {
    const next = cursor[part];
    if (!isJsonObject(next)) {
      cursor[part] = {};
    }
    cursor = cursor[part] as JsonObject;
  }
  cursor[pathParts.at(-1) ?? ""] = value;
}

function entriesToObject(entries: ResolvedTokenEntry[], stripPrefix: string): JsonObject {
  return entries.reduce<JsonObject>((accumulator, entry) => {
    const pathParts = entry.path.replace(`${stripPrefix}.`, "").split(".");
    setNested(accumulator, pathParts, entry.resolvedValue);
    return accumulator;
  }, {});
}

function cssVarName(tokenPath: string): string {
  const normalized = tokenPath.replace(/^(primitives|semantic)\./, "");
  return `--poodle-${normalized.replace(/\./g, "-")}`;
}

function rustConstName(tokenPath: string, stripPrefix: string): string {
  return tokenPath
    .replace(`${stripPrefix}.`, "")
    .replace(/[^a-zA-Z0-9]+/g, "_")
    .replace(/([a-z])([A-Z])/g, "$1_$2")
    .toUpperCase();
}

function jsString(value: string): string {
  return JSON.stringify(value);
}

function writeFile(relativePath: string, contents: string): void {
  const filePath = path.join(artifactDir, relativePath);
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, contents);
}

function copyDir(sourceDir: string, destinationDir: string): void {
  fs.mkdirSync(destinationDir, { recursive: true });
  for (const entry of fs.readdirSync(sourceDir, { withFileTypes: true })) {
    const sourcePath = path.join(sourceDir, entry.name);
    const destinationPath = path.join(destinationDir, entry.name);
    if (entry.isDirectory()) {
      copyDir(sourcePath, destinationPath);
      continue;
    }
    fs.copyFileSync(sourcePath, destinationPath);
  }
}

function syncSvelteTokenArtifacts(): void {
  fs.rmSync(svelteTokensGeneratedDir, { recursive: true, force: true });
  copyDir(path.join(artifactDir, "ts"), path.join(svelteTokensGeneratedDir, "ts"));
  copyDir(path.join(artifactDir, "css"), path.join(svelteTokensGeneratedDir, "css"));
}

function buildCssBlock(selector: string, entries: ResolvedTokenEntry[]): string {
  const lines = entries.map(
    (entry) => `  ${cssVarName(entry.path)}: ${String(entry.resolvedValue)};`,
  );
  return `${selector} {\n${lines.join("\n")}\n}\n`;
}

function formatHeader(commentPrefix: string): string {
  return `${commentPrefix} Generated by packages/tokens/scripts/build-tokens.ts. Do not edit manually.\n`;
}

const metadata = loadDirectoryObject("metadata");
const themesMetadata = metadata.themes as Record<string, ThemeMetadata>;
const densityMetadata = metadata.density as Record<string, ThemeMetadata>;
const controlSizeMetadata = metadata.controlSize as Record<string, ThemeMetadata>;
const aliasesMetadata = metadata.aliases as AliasMetadata[];
const deprecationsMetadata = metadata.deprecations as DeprecationMetadata[];

const schema = {
  primitives: loadDirectoryObject("primitives"),
  semantic: loadDirectoryObject("semantic"),
  modes: {
    themes: loadNamedDirectory("modes/themes"),
    density: loadNamedDirectory("modes/density"),
    controlSize: loadNamedDirectory("modes/control-size"),
  },
  metadata,
  manifest: readJson<Manifest>(path.join(schemaDir, "manifest.json")),
};

const baseEntries = collectTokenEntries(schema.primitives, ["primitives"]).concat(
  collectTokenEntries(schema.semantic, ["semantic"]),
);
const resolvedBaseEntries = resolveEntries(baseEntries);
const primitiveEntries = resolvedBaseEntries.filter((entry) =>
  entry.path.startsWith("primitives."),
);
const semanticEntries = resolvedBaseEntries.filter((entry) =>
  entry.path.startsWith("semantic."),
);

function resolveModeEntries(modeEntries: TokenEntry[]): ResolvedTokenEntry[] {
  const resolvedEntries = resolveEntries([...resolvedBaseEntries, ...modeEntries]);
  return resolvedEntries.slice(-modeEntries.length);
}

const themeDefinitions = Object.entries(schema.modes.themes).map<NamedModeDefinition>(
  ([name, value]) => {
    const modeEntries = collectTokenEntries(value).map((entry) => ({
      ...entry,
      path: `semantic.${entry.path}`,
    }));

    return {
      name,
      selector: themesMetadata[name].selector,
      description: themesMetadata[name].description,
      entries: resolveModeEntries(modeEntries),
    };
  },
);

const densityDefinitions = Object.entries(schema.modes.density).map<NamedModeDefinition>(
  ([name, value]) => {
    const modeEntries = collectTokenEntries(value).map((entry) => ({
      ...entry,
      path: `semantic.${entry.path}`,
    }));
    return {
      name,
      selector: densityMetadata[name].selector,
      description: densityMetadata[name].description,
      entries: resolveModeEntries(modeEntries),
    };
  },
);

const controlSizeDefinitions = Object.entries(schema.modes.controlSize).map<NamedModeDefinition>(
  ([name, value]) => {
    const modeEntries = collectTokenEntries(value).map((entry) => ({
      ...entry,
      path: `semantic.${entry.path}`,
    }));
    return {
      name,
      selector: controlSizeMetadata[name].selector,
      description: controlSizeMetadata[name].description,
      entries: resolveModeEntries(modeEntries),
    };
  },
);

const cssHeader =
  "/* Generated by packages/tokens/scripts/build-tokens.ts. Do not edit manually. */\n";
writeFile(
  "css/poodle-tokens.css",
  `${cssHeader}${buildCssBlock(":root", [...primitiveEntries, ...semanticEntries])}`,
);

for (const theme of themeDefinitions) {
  writeFile(
    `css/poodle-theme-${theme.name}.css`,
    `${cssHeader}${buildCssBlock(theme.selector, theme.entries)}`,
  );
}

for (const density of densityDefinitions) {
  writeFile(
    `css/poodle-density-${density.name}.css`,
    `${cssHeader}${buildCssBlock(density.selector, density.entries)}`,
  );
}

for (const controlSize of controlSizeDefinitions) {
  writeFile(
    `css/poodle-control-size-${controlSize.name}.css`,
    `${cssHeader}${buildCssBlock(controlSize.selector, controlSize.entries)}`,
  );
}

const tokenPaths = resolvedBaseEntries.map((entry) => entry.path);
const cssVars = Object.fromEntries(
  semanticEntries.map((entry) => [entry.path, cssVarName(entry.path)]),
);

writeFile(
  "ts/index.ts",
  `${formatHeader("//")}
export const tokens = ${JSON.stringify(
    {
      primitives: entriesToObject(primitiveEntries, "primitives"),
      semantic: entriesToObject(semanticEntries, "semantic"),
    },
    null,
    2,
  )} as const;

export const tokenPaths = ${JSON.stringify(tokenPaths, null, 2)} as const;

export const cssVars = ${JSON.stringify(cssVars, null, 2)} as const;

export type TokenPath = (typeof tokenPaths)[number];
export type Tokens = typeof tokens;
`,
);

writeFile(
  "ts/themes.ts",
  `${formatHeader("//")}
export const themes = ${JSON.stringify(
    Object.fromEntries(
      themeDefinitions.map((theme) => [
        theme.name,
        {
          selector: theme.selector,
          description: theme.description,
          overrides: Object.fromEntries(
            theme.entries.map((entry) => [entry.path, entry.resolvedValue]),
          ),
        },
      ]),
    ),
    null,
    2,
  )} as const;

export const densityModes = ${JSON.stringify(
    Object.fromEntries(
      densityDefinitions.map((density) => [
        density.name,
        {
          selector: density.selector,
          description: density.description,
          overrides: Object.fromEntries(
            density.entries.map((entry) => [entry.path, entry.resolvedValue]),
          ),
        },
      ]),
    ),
    null,
    2,
  )} as const;

export const controlSizes = ${JSON.stringify(
    Object.fromEntries(
      controlSizeDefinitions.map((mode) => [
        mode.name,
        {
          selector: mode.selector,
          description: mode.description,
          overrides: Object.fromEntries(
            mode.entries.map((entry) => [entry.path, entry.resolvedValue]),
          ),
        },
      ]),
    ),
    null,
    2,
  )} as const;
`,
);

writeFile(
  "ts/metadata.ts",
  `${formatHeader("//")}
export const manifest = ${JSON.stringify(schema.manifest, null, 2)} as const;

export const aliases = ${JSON.stringify(aliasesMetadata, null, 2)} as const;

export const deprecations = ${JSON.stringify(deprecationsMetadata, null, 2)} as const;
`,
);

syncSvelteTokenArtifacts();

// --- Typed token parsing for multi-renderer consumers ---

const REM_BASE = 16; // 1rem = 16px

interface ParsedColor {
  r: number;
  g: number;
  b: number;
  a: number;
}

function parseHexColor(hex: string): ParsedColor | null {
  const match = hex.match(/^#([0-9a-fA-F]{6})$/);
  if (!match) return null;
  const r = parseInt(match[1].slice(0, 2), 16) / 255;
  const g = parseInt(match[1].slice(2, 4), 16) / 255;
  const b = parseInt(match[1].slice(4, 6), 16) / 255;
  return { r, g, b, a: 1.0 };
}

function parseRgbaColor(value: string): ParsedColor | null {
  const match = value.match(/^rgba\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*,\s*([\d.]+)\s*\)$/);
  if (!match) return null;
  return {
    r: parseInt(match[1]) / 255,
    g: parseInt(match[2]) / 255,
    b: parseInt(match[3]) / 255,
    a: parseFloat(match[4]),
  };
}

function parseColor(value: string): ParsedColor | null {
  return parseHexColor(value) ?? parseRgbaColor(value);
}

function parseRemValue(value: string): number | null {
  const match = value.match(/^([\d.]+)rem$/);
  if (!match) return null;
  return parseFloat(match[1]) * REM_BASE;
}

function parseMsValue(value: string): number | null {
  const match = value.match(/^(\d+)ms$/);
  if (!match) return null;
  return parseFloat(match[1]);
}

function parseNumericValue(value: string): number | null {
  const num = parseFloat(value);
  return isFinite(num) ? num : null;
}

interface ParsedShadow {
  offset_x: number;
  offset_y: number;
  blur: number;
  color: ParsedColor;
}

function parseShadowDimension(token: string): number | null {
  if (token === "0") return 0;
  const rem = parseRemValue(token);
  if (rem !== null) return rem;
  const pxMatch = token.match(/^([\d.]+)px$/);
  if (pxMatch) return parseFloat(pxMatch[1]);
  return null;
}

function parseShadow(value: string): ParsedShadow | null {
  const match = value.match(
    /^([\d.]+(?:rem|px)?)\s+([\d.]+(?:rem|px)?)\s+([\d.]+(?:rem|px)?)\s+(rgba?\([^)]+\)|#[0-9a-fA-F]{6})$/,
  );
  if (!match) return null;
  const color = parseColor(match[4]);
  if (!color) return null;
  const ox = parseShadowDimension(match[1]);
  const oy = parseShadowDimension(match[2]);
  const bl = parseShadowDimension(match[3]);
  if (ox === null || oy === null || bl === null) return null;
  return {
    offset_x: ox,
    offset_y: oy,
    blur: bl,
    color,
  };
}

type TypedValue =
  | { kind: "color"; value: ParsedColor }
  | { kind: "px"; value: number }
  | { kind: "ms"; value: number }
  | { kind: "number"; value: number }
  | { kind: "shadow"; value: ParsedShadow }
  | { kind: "string"; value: string };

function classifyTokenValue(path: string, value: string): TypedValue {
  const color = parseColor(value);
  if (color) return { kind: "color", value: color };

  const shadow = parseShadow(value);
  if (shadow) return { kind: "shadow", value: shadow };

  const rem = parseRemValue(value);
  if (rem !== null) return { kind: "px", value: rem };

  const ms = parseMsValue(value);
  if (ms !== null) return { kind: "ms", value: ms };

  const num = parseNumericValue(value);
  if (num !== null) return { kind: "number", value: num };

  return { kind: "string", value };
}

function formatF32(n: number): string {
  const s = n.toString();
  return s.includes(".") ? s : `${s}.0`;
}

function formatColorConst(name: string, c: ParsedColor): string {
  return `pub const ${name}: ColorValue = ColorValue(${formatF32(c.r)}, ${formatF32(c.g)}, ${formatF32(c.b)}, ${formatF32(c.a)});`;
}

function formatPxConst(name: string, px: number): string {
  return `pub const ${name}: SpaceValue = SpaceValue(${formatF32(px)});`;
}

function formatMsConst(name: string, ms: number): string {
  return `pub const ${name}: DurationValue = DurationValue(${formatF32(ms)});`;
}

function formatNumberConst(name: string, n: number): string {
  return `pub const ${name}: f32 = ${formatF32(n)};`;
}

function formatShadowConst(name: string, s: ParsedShadow): string {
  return `pub const ${name}: ShadowValue = ShadowValue { offset_x: ${formatF32(s.offset_x)}, offset_y: ${formatF32(s.offset_y)}, blur: ${formatF32(s.blur)}, color: ColorValue(${formatF32(s.color.r)}, ${formatF32(s.color.g)}, ${formatF32(s.color.b)}, ${formatF32(s.color.a)}) };`;
}

function buildTypedRustConstants(entries: ResolvedTokenEntry[], stripPrefix: string): string {
  const lines: string[] = [];

  for (const entry of entries) {
    const name = rustConstName(entry.path, stripPrefix);
    const strValue = String(entry.resolvedValue);
    const typed = classifyTokenValue(entry.path, strValue);

    switch (typed.kind) {
      case "color":
        lines.push(formatColorConst(name, typed.value));
        break;
      case "px":
        lines.push(formatPxConst(name, typed.value));
        break;
      case "ms":
        lines.push(formatMsConst(name, typed.value));
        break;
      case "number":
        lines.push(formatNumberConst(name, typed.value));
        break;
      case "shadow":
        lines.push(formatShadowConst(name, typed.value));
        break;
      case "string":
        // String-only tokens (font families, easings) stay as &str in the typed module
        lines.push(
          `pub const ${name}: &str = ${jsString(typed.value)};`,
        );
        break;
    }
  }

  return lines.join("\n");
}

// --- End typed token parsing ---

function buildRustConstants(entries: ResolvedTokenEntry[], stripPrefix: string): string {
  return entries
    .map(
      (entry) =>
        `pub const ${rustConstName(entry.path, stripPrefix)}: &str = ${jsString(String(entry.resolvedValue))};`,
    )
    .join("\n");
}

function buildRustSemanticPathConstants(entries: ResolvedTokenEntry[], stripPrefix: string): string {
  return entries
    .map(
      (entry) =>
        `pub const ${rustConstName(entry.path, stripPrefix)}: &str = ${jsString(entry.path)};`,
    )
    .join("\n");
}

function buildRustDefinitionArray(entries: ResolvedTokenEntry[]): string {
  return entries
    .map(
      (entry) => `    (${jsString(entry.path)}, ${jsString(String(entry.resolvedValue))}),`,
    )
    .join("\n");
}

writeFile(
  "rust/mod.rs",
  `${formatHeader("//")}
pub mod density;
pub mod metadata;
pub mod primitives;
pub mod semantic;
pub mod themes;
pub mod typed;
`,
);

writeFile(
  "rust/primitives.rs",
  `${formatHeader("//")}
${buildRustConstants(primitiveEntries, "primitives")}
`,
);

writeFile(
  "rust/semantic.rs",
  `${formatHeader("//")}
${buildRustSemanticPathConstants(semanticEntries, "semantic")}
`,
);

writeFile(
  "rust/themes.rs",
  `${formatHeader("//")}
#[derive(Debug, Clone, Copy)]
pub struct ThemeDefinition {
    pub name: &'static str,
    pub selector: &'static str,
    pub overrides: &'static [(&'static str, &'static str)],
}

${themeDefinitions
    .map(
      (theme) => `pub const ${theme.name.replace(/[^a-zA-Z0-9]+/g, "_").toUpperCase()}: ThemeDefinition = ThemeDefinition {
    name: ${jsString(theme.name)},
    selector: ${jsString(theme.selector)},
    overrides: &[
${buildRustDefinitionArray(theme.entries)}
    ],
};`,
    )
    .join("\n\n")}
`,
);

writeFile(
  "rust/density.rs",
  `${formatHeader("//")}
#[derive(Debug, Clone, Copy)]
pub struct DensityDefinition {
    pub name: &'static str,
    pub selector: &'static str,
    pub overrides: &'static [(&'static str, &'static str)],
}

#[derive(Debug, Clone, Copy)]
pub struct ControlSizeDefinition {
    pub name: &'static str,
    pub selector: &'static str,
    pub overrides: &'static [(&'static str, &'static str)],
}

${densityDefinitions
    .map(
      (density) => `pub const ${density.name.replace(/[^a-zA-Z0-9]+/g, "_").toUpperCase()}: DensityDefinition = DensityDefinition {
    name: ${jsString(density.name)},
    selector: ${jsString(density.selector)},
    overrides: &[
${buildRustDefinitionArray(density.entries)}
    ],
};`,
    )
    .join("\n\n")}

${controlSizeDefinitions
    .map(
      (mode) => `pub const CONTROL_SIZE_${mode.name.replace(/[^a-zA-Z0-9]+/g, "_").toUpperCase()}: ControlSizeDefinition = ControlSizeDefinition {
    name: ${jsString(mode.name)},
    selector: ${jsString(mode.selector)},
    overrides: &[
${buildRustDefinitionArray(mode.entries)}
    ],
};`,
    )
    .join("\n\n")}
`,
);

writeFile(
  "rust/metadata.rs",
  `${formatHeader("//")}
pub const MANIFEST_NAME: &str = ${jsString(schema.manifest.name)};
pub const MANIFEST_VERSION: &str = ${jsString(schema.manifest.version)};
pub const CANONICAL_FORMAT: &str = ${jsString(schema.manifest.canonicalFormat)};
pub const ARTIFACT_BASELINE: &str = ${jsString(schema.manifest.artifactBaseline)};

pub const ALIASES: &[(&str, &str)] = &[
${aliasesMetadata
    .map((alias) => `    (${jsString(alias.from)}, ${jsString(alias.to)}),`)
    .join("\n")}
];

pub const DEPRECATIONS: &[(&str, &str)] = &[
${deprecationsMetadata
    .map((item) => `    (${jsString(item.path)}, ${jsString(item.status)}),`)
    .join("\n")}
];
`,
);

// --- Typed token artifacts ---

writeFile(
  "rust/typed/mod.rs",
  `${formatHeader("//")}
mod types;
pub mod primitives;
pub mod semantic;

pub use types::{ColorValue, DurationValue, ShadowValue, SpaceValue};
`,
);

writeFile(
  "rust/typed/types.rs",
  `${formatHeader("//")}
/// RGBA color as four f32 values in 0.0\u20131.0 range.
/// Compatible with Jetstream \`Vec4\` and GPUI color types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorValue(pub f32, pub f32, pub f32, pub f32);

impl ColorValue {
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self(r, g, b, a)
    }

    pub const fn as_array(&self) -> [f32; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

/// Dimension in pixels as f32, resolved from rem/px token values.
/// Base size: 16px per rem.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpaceValue(pub f32);

impl SpaceValue {
    pub const fn px(value: f32) -> Self {
        Self(value)
    }

    pub const fn as_f32(&self) -> f32 {
        self.0
    }
}

/// Duration in milliseconds as f32.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DurationValue(pub f32);

impl DurationValue {
    pub const fn ms(value: f32) -> Self {
        Self(value)
    }

    pub const fn as_f32(&self) -> f32 {
        self.0
    }

    pub const fn as_secs(&self) -> f32 {
        self.0 / 1000.0
    }
}

/// Box shadow with offset, blur radius, and color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShadowValue {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub color: ColorValue,
}
`,
);

writeFile(
  "rust/typed/primitives.rs",
  `${formatHeader("//")}
use super::types::{ColorValue, DurationValue, ShadowValue, SpaceValue};

${buildTypedRustConstants(primitiveEntries, "primitives")}
`,
);

writeFile(
  "rust/typed/semantic.rs",
  `${formatHeader("//")}
use super::types::{ColorValue, DurationValue, ShadowValue, SpaceValue};

${buildTypedRustConstants(semanticEntries, "semantic")}
`,
);
