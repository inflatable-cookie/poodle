/**
 * RichTextEditor / RichTextRenderer shared TypeScript core (engine-free).
 * Contract: docs/contracts/components/rich-text-editor.md
 *
 * Structural ProseMirror JSON carrier types, the curated feature registry, the
 * curated toolbar command registry, and the shared document envelope. No
 * TipTap or ProseMirror import may appear here: this module ships through the
 * root barrel and must stay free of the rich-text engine. Engine assembly
 * (schema, validation, editor lifecycle) lives adapter-side and is reached
 * only through the dedicated `./rich-text` entries.
 *
 * ProseMirror document JSON and schema semantics are the public authority.
 * These TypeScript interfaces type ProseMirror's serialized node shape; they do
 * not define a competing Poodle schema.
 */

export interface ProseMirrorMarkJSON {
  type: string;
  attrs?: Record<string, unknown>;
}

export interface ProseMirrorNodeJSON {
  type: string;
  attrs?: Record<string, unknown>;
  content?: ProseMirrorNodeJSON[];
  marks?: ProseMirrorMarkJSON[];
  text?: string;
}

export type ProseMirrorDocumentJSON = ProseMirrorNodeJSON & { type: "doc" };

export interface RichTextImageInput {
  src: string;
  alt: string;
  title?: string | null;
}

export type RichTextFeature =
  | "formatting"
  | "headings"
  | "links"
  | "lists"
  | "blockquote"
  | "code-block"
  | "horizontal-rule"
  | "tables"
  | "images";

export type RichTextCommand =
  | "undo"
  | "redo"
  | "bold"
  | "italic"
  | "strike"
  | "inline-code"
  | "heading-1"
  | "heading-2"
  | "heading-3"
  | "heading-4"
  | "heading-5"
  | "heading-6"
  | "link"
  | "bullet-list"
  | "ordered-list"
  | "blockquote"
  | "code-block"
  | "horizontal-rule"
  | "insert-table"
  | "add-row"
  | "add-column"
  | "delete-table"
  | "insert-image";

/** Closed admitted feature set. Unknown strings never silently fall back. */
export const RICH_TEXT_FEATURES: readonly RichTextFeature[] = [
  "formatting",
  "headings",
  "links",
  "lists",
  "blockquote",
  "code-block",
  "horizontal-rule",
  "tables",
  "images",
] as const;

export function isRichTextFeature(value: string): value is RichTextFeature {
  return (RICH_TEXT_FEATURES as readonly string[]).includes(value);
}

export const RICH_TEXT_COMMANDS: readonly RichTextCommand[] = [
  "undo",
  "redo",
  "bold",
  "italic",
  "strike",
  "inline-code",
  "heading-1",
  "heading-2",
  "heading-3",
  "heading-4",
  "heading-5",
  "heading-6",
  "link",
  "bullet-list",
  "ordered-list",
  "blockquote",
  "code-block",
  "horizontal-rule",
  "insert-table",
  "add-row",
  "add-column",
  "delete-table",
  "insert-image",
] as const;

export function isRichTextCommand(value: string): value is RichTextCommand {
  return (RICH_TEXT_COMMANDS as readonly string[]).includes(value);
}

/**
 * The standard feature set enables every listed feature except images.
 * Projects opt into images explicitly; embeds are not a v1 feature.
 */
export const RICH_TEXT_STANDARD_FEATURES: readonly RichTextFeature[] = [
  "formatting",
  "headings",
  "links",
  "lists",
  "blockquote",
  "code-block",
  "horizontal-rule",
  "tables",
] as const;

/**
 * Curated feature-to-command ownership. Undo and redo are editor history
 * commands and are always admitted; they belong to no feature module.
 */
export const RICH_TEXT_FEATURE_COMMANDS: Readonly<
  Record<RichTextFeature, readonly RichTextCommand[]>
> = {
  formatting: ["bold", "italic", "strike", "inline-code"],
  headings: ["heading-1", "heading-2", "heading-3", "heading-4", "heading-5", "heading-6"],
  links: ["link"],
  lists: ["bullet-list", "ordered-list"],
  blockquote: ["blockquote"],
  "code-block": ["code-block"],
  "horizontal-rule": ["horizontal-rule"],
  tables: ["insert-table", "add-row", "add-column", "delete-table"],
  images: ["insert-image"],
} as const;

/** Commands admitted regardless of the enabled feature set. */
export const RICH_TEXT_BASE_COMMANDS: readonly RichTextCommand[] = ["undo", "redo"] as const;

export interface RichTextFeatureValidation {
  /** Unique admitted features in given order. */
  features: RichTextFeature[];
  /** Unknown or duplicate identifiers, in the order first refused. */
  refused: string[];
}

/**
 * Validate a host feature list. Unknown or duplicate features fail closed;
 * Poodle never silently dedupes or drops them. Order has no semantic meaning.
 */
export function validateRichTextFeatures(
  features: readonly RichTextFeature[],
): RichTextFeatureValidation {
  const seen = new Set<string>();
  const admitted: RichTextFeature[] = [];
  const refused: string[] = [];
  for (const feature of features) {
    if (!isRichTextFeature(feature)) {
      refused.push(String(feature));
      continue;
    }
    if (seen.has(feature)) {
      refused.push(feature);
      continue;
    }
    seen.add(feature);
    admitted.push(feature);
  }
  return { features: admitted, refused };
}

/**
 * Commands a feature set admits (feature-provided commands plus the always
 * admitted history commands). `insertImageCommandAdmitted` is handled by the
 * caller because the automatic toolbar also requires a present `requestImage`.
 */
export function richTextAdmittedCommands(
  features: readonly RichTextFeature[],
): readonly RichTextCommand[] {
  const commands = new Set<RichTextCommand>(RICH_TEXT_BASE_COMMANDS);
  for (const feature of features) {
    for (const command of RICH_TEXT_FEATURE_COMMANDS[feature]) commands.add(command);
  }
  return [...commands];
}

export interface RichTextToolbarValidation {
  /** Unique admitted commands, first-occurrence order. */
  commands: RichTextCommand[];
  /** Commands the enabled features do not provide, in given order. */
  refused: string[];
}

/**
 * Validate an explicit toolbar list. Unknown commands fail closed; commands
 * whose owning feature is disabled fail closed. Duplicates are deduped because
 * a duplicated button is harmless while an unknown one is not.
 */
export function validateRichTextToolbar(
  toolbar: readonly RichTextCommand[],
  features: readonly RichTextFeature[],
): RichTextToolbarValidation {
  const admitted = new Set(richTextAdmittedCommands(features));
  const seen = new Set<RichTextCommand>();
  const commands: RichTextCommand[] = [];
  const refused: string[] = [];
  for (const command of toolbar) {
    if (!isRichTextCommand(command) || !admitted.has(command)) {
      refused.push(String(command));
      continue;
    }
    if (seen.has(command)) continue;
    seen.add(command);
    commands.push(command);
  }
  return { commands, refused };
}

/** Automatic toolbar order: history, formatting, headings, link, lists,
 * block, table, image — a fixed, deterministic projection of the features. */
const RICH_TEXT_AUTO_TOOLBAR_ORDER: readonly RichTextCommand[] = [
  "undo",
  "redo",
  "bold",
  "italic",
  "strike",
  "inline-code",
  "heading-1",
  "heading-2",
  "heading-3",
  "heading-4",
  "heading-5",
  "heading-6",
  "link",
  "bullet-list",
  "ordered-list",
  "blockquote",
  "code-block",
  "horizontal-rule",
  "insert-table",
  "add-row",
  "add-column",
  "delete-table",
  "insert-image",
] as const;

export interface RichTextToolbarResult {
  commands: RichTextCommand[];
  refused: string[];
}

/**
 * Resolve the `toolbar` prop against the enabled features. `auto` derives
 * commands from the features (history always admitted; `insert-image` only
 * when images are enabled and a host `requestImage` is present). An explicit
 * list may only use commands the enabled features provide.
 */
export function resolveRichTextToolbar(
  toolbar: "auto" | readonly RichTextCommand[],
  features: readonly RichTextFeature[],
  requestImage: unknown,
): RichTextToolbarResult {
  if (toolbar === "auto") {
    const admitted = new Set(richTextAdmittedCommands(features));
    const commands = RICH_TEXT_AUTO_TOOLBAR_ORDER.filter((command) => {
      if (command === "insert-image") {
        return features.includes("images") && typeof requestImage === "function";
      }
      return admitted.has(command);
    });
    return { commands, refused: [] };
  }
  const resolved = validateRichTextToolbar(toolbar, features);
  return { commands: resolved.commands, refused: resolved.refused };
}

/** Semantic toolbar clusters. Consecutive commands sharing a group render as
 * one intact control cluster that wraps as a unit at constrained widths. */
export type RichTextCommandGroup =
  | "history"
  | "inline"
  | "headings"
  | "link"
  | "lists"
  | "block"
  | "table"
  | "media";

export interface RichTextCommandPresentation {
  /** Toolbar control label; the accessible name and tooltip text. */
  label: string;
  /** Semantic cluster the command belongs to. */
  group: RichTextCommandGroup;
  /** Poodle default icon-set name for the control. */
  icon: string;
  /** Short typographic glyph rendered instead of the icon when more
   *  legible (H1, H2, H3); null renders the icon. */
  glyph: string | null;
  /** Whether the command renders with a pressed state. */
  toggle: boolean;
  /** Destructive commands render distinguishably without changing semantics. */
  destructive: boolean;
}

/**
 * The one shared command-presentation map: label, group, icon/glyph, toggle
 * posture, and destructive tone. Svelte and React compose their toolbars from
 * this record and must not maintain independent visual vocabularies. Icon
 * names resolve against Poodle's default Lucide set.
 */
export const RICH_TEXT_COMMAND_PRESENTATION: Readonly<
  Record<RichTextCommand, RichTextCommandPresentation>
> = {
  undo: { label: "Undo", group: "history", icon: "undo", glyph: null, toggle: false, destructive: false },
  redo: { label: "Redo", group: "history", icon: "redo", glyph: null, toggle: false, destructive: false },
  bold: { label: "Bold", group: "inline", icon: "bold", glyph: null, toggle: true, destructive: false },
  italic: { label: "Italic", group: "inline", icon: "italic", glyph: null, toggle: true, destructive: false },
  strike: { label: "Strikethrough", group: "inline", icon: "strikethrough", glyph: null, toggle: true, destructive: false },
  "inline-code": { label: "Inline code", group: "inline", icon: "code", glyph: null, toggle: true, destructive: false },
  "heading-1": { label: "Heading 1", group: "headings", icon: "heading", glyph: "H1", toggle: false, destructive: false },
  "heading-2": { label: "Heading 2", group: "headings", icon: "heading", glyph: "H2", toggle: false, destructive: false },
  "heading-3": { label: "Heading 3", group: "headings", icon: "heading", glyph: "H3", toggle: false, destructive: false },
  "heading-4": { label: "Heading 4", group: "headings", icon: "heading", glyph: "H4", toggle: false, destructive: false },
  "heading-5": { label: "Heading 5", group: "headings", icon: "heading", glyph: "H5", toggle: false, destructive: false },
  "heading-6": { label: "Heading 6", group: "headings", icon: "heading", glyph: "H6", toggle: false, destructive: false },
  link: { label: "Link", group: "link", icon: "link", glyph: null, toggle: true, destructive: false },
  "bullet-list": { label: "Bulleted list", group: "lists", icon: "list", glyph: null, toggle: true, destructive: false },
  "ordered-list": { label: "Numbered list", group: "lists", icon: "list-ordered", glyph: null, toggle: true, destructive: false },
  blockquote: { label: "Block quote", group: "block", icon: "quote", glyph: null, toggle: true, destructive: false },
  "code-block": { label: "Code block", group: "block", icon: "square-code", glyph: null, toggle: true, destructive: false },
  "horizontal-rule": { label: "Horizontal rule", group: "block", icon: "minus", glyph: null, toggle: false, destructive: false },
  "insert-table": { label: "Insert table", group: "table", icon: "table", glyph: null, toggle: false, destructive: false },
  "add-row": { label: "Add table row", group: "table", icon: "between-horizontal-start", glyph: null, toggle: false, destructive: false },
  "add-column": { label: "Add table column", group: "table", icon: "between-vertical-start", glyph: null, toggle: false, destructive: false },
  "delete-table": { label: "Delete table", group: "table", icon: "trash-2", glyph: null, toggle: false, destructive: true },
  "insert-image": { label: "Insert image", group: "media", icon: "image", glyph: null, toggle: false, destructive: false },
} as const;

/** Human labels for the curated commands. Toolbar controls carry these names. */
export const RICH_TEXT_COMMAND_LABELS: Readonly<Record<RichTextCommand, string>> = Object.fromEntries(
  RICH_TEXT_COMMANDS.map((command) => [command, RICH_TEXT_COMMAND_PRESENTATION[command].label]),
) as Readonly<Record<RichTextCommand, string>>;

/** Commands rendered as pressed-state toggles rather than plain actions. */
export const RICH_TEXT_TOGGLE_COMMANDS: readonly RichTextCommand[] = RICH_TEXT_COMMANDS.filter(
  (command) => RICH_TEXT_COMMAND_PRESENTATION[command].toggle,
);

/** Human group names, one per semantic toolbar cluster. */
export const RICH_TEXT_COMMAND_GROUP_LABELS: Readonly<Record<RichTextCommandGroup, string>> = {
  history: "History",
  inline: "Text formatting",
  headings: "Headings",
  link: "Links",
  lists: "Lists",
  block: "Blocks",
  table: "Tables",
  media: "Media",
} as const;

/**
 * Heading levels the curated headings module admits, in document order. The
 * schema, validation, editor commands, renderer, and selector option projection
 * all read this one list; there is no second level authority.
 */
export const RICH_TEXT_HEADING_LEVELS: readonly number[] = [1, 2, 3, 4, 5, 6] as const;

/** The document level a granular heading command selects, or null. */
export function richTextHeadingCommandLevel(command: RichTextCommand): number | null {
  const match = /^heading-([1-6])$/.exec(command);
  return match ? Number(match[1]) : null;
}

/** The granular heading command for a level, or null when out of range. */
export function richTextHeadingCommand(level: number): RichTextCommand | null {
  return RICH_TEXT_HEADING_LEVELS.includes(level)
    ? (`heading-${level}` as RichTextCommand)
    : null;
}

/** Whether a command names one configured heading level. */
export function isRichTextHeadingCommand(command: string): boolean {
  return isRichTextCommand(command) && richTextHeadingCommandLevel(command) !== null;
}

/**
 * Intrinsic text-mode state of the toolbar heading selector. `normal` covers a
 * paragraph and any other non-heading text block, `heading` names one active
 * level, and `mixed` covers a selection whose blocks span different modes.
 */
export type RichTextHeadingMode =
  | { kind: "normal" }
  | { kind: "heading"; level: number }
  | { kind: "mixed" };

/** Select value meaning "Normal text"; selector chrome, never a public command. */
export const RICH_TEXT_HEADING_NORMAL_VALUE = "normal" as const;
/** Select value meaning "the blocks span different text modes". */
export const RICH_TEXT_HEADING_MIXED_VALUE = "mixed" as const;

/** Human labels for the selector's intrinsic Normal and Mixed states. */
export const RICH_TEXT_HEADING_NORMAL_LABEL = "Normal text";
export const RICH_TEXT_HEADING_MIXED_LABEL = "Mixed";

/** Accessible name of the toolbar heading selector as a whole. */
export const RICH_TEXT_HEADING_SELECT_LABEL = "Text mode";

/** The trigger label for a resolved heading mode. */
export function richTextHeadingModeLabel(mode: RichTextHeadingMode): string {
  if (mode.kind === "mixed") return RICH_TEXT_HEADING_MIXED_LABEL;
  if (mode.kind === "normal") return RICH_TEXT_HEADING_NORMAL_LABEL;
  const command = richTextHeadingCommand(mode.level);
  return command ? RICH_TEXT_COMMAND_LABELS[command] : RICH_TEXT_HEADING_NORMAL_LABEL;
}

/** One rendered option of the heading selector. */
export interface RichTextHeadingOption {
  /** Select value: `normal` or the granular heading command. */
  value: string;
  /** Plain accessible label; the menu previews type scale around it. */
  label: string;
  /** Document level to preview, or null for Normal text. */
  level: number | null;
}

/**
 * Normal text plus the configured heading levels, in configured order. The
 * selector never invents levels: only commands the resolved toolbar admits
 * become options.
 */
export function richTextHeadingOptions(
  headingCommands: readonly RichTextCommand[],
): RichTextHeadingOption[] {
  const options: RichTextHeadingOption[] = [
    { value: RICH_TEXT_HEADING_NORMAL_VALUE, label: RICH_TEXT_HEADING_NORMAL_LABEL, level: null },
  ];
  for (const command of headingCommands) {
    const level = richTextHeadingCommandLevel(command);
    if (level === null) continue;
    options.push({ value: command, label: RICH_TEXT_COMMAND_LABELS[command], level });
  }
  return options;
}

/** A rendered toolbar control for one non-heading command. */
export interface RichTextToolbarCommandItem {
  kind: "command";
  command: RichTextCommand;
}

/**
 * The one resolved heading control: Normal text, the active heading, and Mixed
 * are intrinsic selector states, while `commands` are exactly the admitted
 * granular heading commands in resolved order.
 */
export interface RichTextToolbarHeadingSelectItem {
  kind: "heading-select";
  commands: readonly RichTextCommand[];
  levels: readonly number[];
}

export type RichTextToolbarItem =
  | RichTextToolbarCommandItem
  | RichTextToolbarHeadingSelectItem;

/**
 * Project resolved toolbar commands into render items. Every admitted heading
 * command collapses into one heading selector at the first heading command's
 * position; every other command keeps its resolved order. This is presentation
 * only: the public toolbar configuration stays granular and no composite
 * command exists.
 */
export function projectRichTextToolbar(
  commands: readonly RichTextCommand[],
): RichTextToolbarItem[] {
  const items: RichTextToolbarItem[] = [];
  let headingCommands: RichTextCommand[] | null = null;
  let headingLevels: number[] | null = null;
  for (const command of commands) {
    const level = richTextHeadingCommandLevel(command);
    if (level === null) {
      items.push({ kind: "command", command });
      continue;
    }
    if (headingCommands === null || headingLevels === null) {
      headingCommands = [];
      headingLevels = [];
      items.push({ kind: "heading-select", commands: headingCommands, levels: headingLevels });
    }
    headingCommands.push(command);
    headingLevels.push(level);
  }
  return items;
}

/**
 * Supported document envelope: at most 2 MiB of UTF-8 JSON and at most 10,000
 * nodes. Consumers refuse larger input before mount; Poodle never loads files.
 */
export const RICH_TEXT_MAX_BYTES = 2 * 1024 * 1024;
export const RICH_TEXT_MAX_NODES = 10_000;

export function richTextSerializedByteLength(value: ProseMirrorDocumentJSON): number {
  return new TextEncoder().encode(JSON.stringify(value)).length;
}

export function countRichTextNodes(value: unknown): number {
  let count = 0;
  const visit = (node: unknown): void => {
    if (!node || typeof node !== "object") return;
    count += 1;
    const record = node as ProseMirrorNodeJSON;
    if (Array.isArray(record.marks)) {
      for (const mark of record.marks) visit(mark);
    }
    if (Array.isArray(record.content)) {
      for (const child of record.content) visit(child);
    }
  };
  visit(value);
  return count;
}

export function isRichTextDocumentAdmissible(value: ProseMirrorDocumentJSON): boolean {
  return (
    richTextSerializedByteLength(value) <= RICH_TEXT_MAX_BYTES &&
    countRichTextNodes(value) <= RICH_TEXT_MAX_NODES
  );
}
