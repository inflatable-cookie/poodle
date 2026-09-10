/**
 * TipTap 3 / ProseMirror engine assembly for the rich-text editor and
 * renderer. Reached only through the dedicated `./rich-text` entry; never the
 * root barrel, `./markdown`, or `./editor`.
 *
 * No top-level DOM access: constructing the TipTap `Editor` happens
 * exclusively in `createRichTextEngine`, which wrappers call after mount.
 * Importing this module (including on the server) loads the engine code but
 * never touches the document.
 *
 * Engine objects (editor instances, nodes, transactions, plugins, commands)
 * stay private: nothing in this module's public surface returns them.
 */

import { Editor, Extension, getSchema } from "@tiptap/core";
import type { Extensions } from "@tiptap/core";
import { DOMSerializer, Node as ProseMirrorNode } from "@tiptap/pm/model";
import type { Schema, Node as PMNode } from "@tiptap/pm/model";
import { Plugin, PluginKey } from "@tiptap/pm/state";
import { Decoration, DecorationSet } from "@tiptap/pm/view";
import type { StepMap } from "@tiptap/pm/transform";

import { Blockquote } from "@tiptap/extension-blockquote";
import { Bold } from "@tiptap/extension-bold";
import { Code } from "@tiptap/extension-code";
import { CodeBlock } from "@tiptap/extension-code-block";
import { Document } from "@tiptap/extension-document";
import { HardBreak } from "@tiptap/extension-hard-break";
import { Heading } from "@tiptap/extension-heading";
import type { Level } from "@tiptap/extension-heading";
import { HorizontalRule } from "@tiptap/extension-horizontal-rule";
import { Image } from "@tiptap/extension-image";
import { Italic } from "@tiptap/extension-italic";
import { Link } from "@tiptap/extension-link";
import { BulletList, ListItem, ListKeymap, OrderedList } from "@tiptap/extension-list";
import { Paragraph } from "@tiptap/extension-paragraph";
import { Strike } from "@tiptap/extension-strike";
import { Table, TableCell, TableHeader, TableRow } from "@tiptap/extension-table";
import { Text } from "@tiptap/extension-text";
import { UndoRedo } from "@tiptap/extensions";

import {
  RICH_TEXT_MAX_BYTES,
  RICH_TEXT_MAX_NODES,
  isRichTextCommand,
  resolveRichTextToolbar,
  validateRichTextFeatures,
  type ProseMirrorDocumentJSON,
  type ProseMirrorNodeJSON,
  type RichTextCommand,
  type RichTextFeature,
} from "@inflatable-cookie/poodle-core";

export type {
  ProseMirrorDocumentJSON,
  ProseMirrorNodeJSON,
  RichTextCommand,
  RichTextFeature,
} from "@inflatable-cookie/poodle-core";

export interface RichTextImageInput {
  src: string;
  alt: string;
  title?: string | null;
}

export interface RichTextEngineOptions {
  value: ProseMirrorDocumentJSON;
  features: readonly RichTextFeature[];
  toolbar: "auto" | readonly RichTextCommand[];
  readOnly: boolean;
  disabled: boolean;
  placeholder: string;
  ariaLabel: string;
  /** Host-owned asset choice for `insert-image`; valid only with `images`. */
  requestImage?: (() => Promise<RichTextImageInput | null>) | null;
}

export interface RichTextCommandState {
  /** Whether the command can run in the current document position. */
  available: boolean;
  /** Whether the command's target mark or node is active at the selection. */
  active: boolean;
}

export interface RichTextToolbarSnapshot {
  commands: readonly RichTextCommand[];
  states: Readonly<Record<RichTextCommand, RichTextCommandState>>;
}

export interface RichTextEngineCallbacks {
  onChange: (document: ProseMirrorDocumentJSON) => void;
  onToolbar: (snapshot: RichTextToolbarSnapshot) => void;
}

export interface RichTextEngine {
  readonly commands: readonly RichTextCommand[];
  commandState: (command: RichTextCommand) => RichTextCommandState;
  runCommand: (command: RichTextCommand) => void;
  /** Editor-owned link affordance state. Never invokes a browser prompt. */
  linkHref: () => string | null;
  applyLink: (href: string) => void;
  removeLink: () => void;
  update: (next: Partial<RichTextEngineOptions>) => void;
  destroy: () => void;
}

/** Fail closed: unknown or duplicate features never mount. */
export function assertAdmittedFeatures(
  features: readonly RichTextFeature[],
): asserts features is readonly RichTextFeature[] {
  const { refused } = validateRichTextFeatures(features);
  if (refused.length > 0) {
    throw new Error(
      `rich-text: unsupported or duplicate feature "${refused[0]}". Poodle admits curated features only and every feature must be unique.`,
    );
  }
}

/** Fail closed: explicit toolbars may only name commands the features provide. */
export function assertAdmittedToolbar(
  toolbar: "auto" | readonly RichTextCommand[],
  features: readonly RichTextFeature[],
  requestImage: RichTextEngineOptions["requestImage"],
): void {
  const { refused } = resolveRichTextToolbar(toolbar, features, requestImage);
  if (refused.length > 0) {
    throw new Error(
      `rich-text: unsupported toolbar command "${refused[0]}". Explicit toolbars may only use commands provided by the enabled features.`,
    );
  }
}

/**
 * Poodle's admitted attribute keys per node type. The ProseMirror schema is
 * the semantic authority, but its permissive defaults (unknown attributes are
 * silently dropped) never apply: controlled documents carrying an unadmitted
 * attribute fail closed.
 */
const ADMITTED_NODE_ATTRS: Readonly<Record<string, readonly string[]>> = {
  doc: [],
  paragraph: [],
  text: [],
  heading: ["level"],
  bulletList: [],
  orderedList: ["start", "type"],
  listItem: [],
  blockquote: [],
  codeBlock: ["language"],
  horizontalRule: [],
  hardBreak: [],
  image: ["src", "alt", "title"],
  table: [],
  tableRow: [],
  tableCell: ["colspan", "rowspan", "colwidth", "align"],
  tableHeader: ["colspan", "rowspan", "colwidth", "align"],
};

const ADMITTED_MARK_ATTRS: Readonly<Record<string, readonly string[]>> = {
  bold: [],
  italic: [],
  strike: [],
  code: [],
  link: ["href", "target", "rel", "class", "title"],
};

/** Heading levels the curated feature set exposes (heading-1 through heading-3). */
const ADMITTED_HEADING_LEVELS: readonly number[] = [1, 2, 3];

/**
 * URL admission posture. Poodle refuses executable URL schemes; consumers own
 * source admission and Content Security Policy.
 */
const ADMITTED_URL_PROTOCOLS = new Set(["http:", "https:", "mailto:"]);

function urlProtocolOf(value: string): string {
  try {
    return new URL(value, "https://poodle.invalid").protocol;
  } catch {
    return "";
  }
}

export function isAdmittedImageUrl(src: string): boolean {
  const protocol = urlProtocolOf(src);
  if (protocol === "data:") {
    return /^data:image\//i.test(src) && !/[\r\n]/.test(src);
  }
  return ADMITTED_URL_PROTOCOLS.has(protocol);
}

export function isAdmittedLinkHref(href: string): boolean {
  if (href.startsWith("#")) return true;
  return ADMITTED_URL_PROTOCOLS.has(urlProtocolOf(href));
}

function invalidNode(type: string): Error {
  return new Error(
    `rich-text: unsupported node type "${type}" for the configured feature set. Controlled documents fail closed and are never stripped.`,
  );
}

function invalidMark(type: string): Error {
  return new Error(
    `rich-text: unsupported mark type "${type}" for the configured feature set. Controlled documents fail closed and are never stripped.`,
  );
}

function assertAdmittedAttrs(
  admitted: readonly string[],
  attrs: unknown,
  what: string,
): void {
  if (attrs === undefined || attrs === null) return;
  if (typeof attrs !== "object" || Array.isArray(attrs)) {
    throw new Error(`rich-text: invalid attributes for ${what}.`);
  }
  for (const key of Object.keys(attrs as Record<string, unknown>)) {
    if (!admitted.includes(key)) {
      throw new Error(
        `rich-text: unsupported attribute "${key}" on ${what}. Controlled documents fail closed and are never stripped.`,
      );
    }
  }
}

/**
 * Exact structural validation against the configured schema, then ProseMirror
 * itself. Unknown nodes, marks, attributes, or invalid content throw before
 * any editor or renderer output exists. Returns nothing; refusal is an error.
 */
export function assertValidRichTextDocument(
  schema: Schema,
  value: ProseMirrorDocumentJSON,
): PMNode {
  if (
    !value ||
    typeof value !== "object" ||
    Array.isArray(value) ||
    value.type !== "doc" ||
    "text" in value
  ) {
    throw new Error(
      'rich-text: value must be a ProseMirror document JSON object with type "doc".',
    );
  }
  assertDocumentNodes(schema, value);
  if (exceedsRichTextEnvelope(value)) {
    throw new Error(
      `rich-text: value exceeds the supported document envelope (${RICH_TEXT_MAX_BYTES} bytes of UTF-8 JSON, ${RICH_TEXT_MAX_NODES} nodes); hosts must refuse larger documents before mounting.`,
    );
  }
  let node: PMNode;
  try {
    node = ProseMirrorNode.fromJSON(schema, value);
  } catch (error) {
    throw new Error(
      `rich-text: value is not valid for the configured schema: ${(error as Error).message}`,
    );
  }
  try {
    node.check();
    node.descendants((descendant) => descendant.check());
  } catch (error) {
    throw new Error(
      `rich-text: value is not valid for the configured schema: ${(error as Error).message}`,
    );
  }
  return node;
}

/** Node-count part of the envelope; the walk runs before schema parsing. */
function exceedsRichTextEnvelope(value: ProseMirrorDocumentJSON): boolean {
  const serialized = JSON.stringify(value);
  return (
    countRichTextNodes(value) > RICH_TEXT_MAX_NODES ||
    new TextEncoder().encode(serialized).length > RICH_TEXT_MAX_BYTES
  );
}

function countRichTextNodes(node: ProseMirrorDocumentJSON): number {
  let count = 1;
  for (const child of node.content ?? []) {
    count += countRichTextNodes(child as ProseMirrorDocumentJSON);
  }
  return count;
}

function assertDocumentNodes(schema: Schema, value: ProseMirrorNodeJSON): void {
  const node = value as ProseMirrorNodeJSON;
  const type = node.type;
  if (type === "text") {
    if (typeof node.text !== "string") {
      throw new Error('rich-text: text node without a "text" string.');
    }
    if (node.content !== undefined) {
      throw new Error('rich-text: text node with a "content" list.');
    }
  } else if (!(type in schema.nodes)) {
    throw invalidNode(type);
  }
  assertAdmittedAttrs(ADMITTED_NODE_ATTRS[type] ?? [], node.attrs, `node "${type}"`);
  for (const mark of node.marks ?? []) {
    const markType = (mark as { type?: unknown })?.type;
    if (!mark || typeof mark !== "object" || typeof markType !== "string") {
      throw new Error("rich-text: value contains a malformed mark.");
    }
    if (!(markType in schema.marks)) throw invalidMark(markType);
    assertAdmittedAttrs(ADMITTED_MARK_ATTRS[markType] ?? [], mark.attrs, `mark "${markType}"`);
    if (markType === "link" && mark.attrs) {
      const href = (mark.attrs as Record<string, unknown>).href;
      if (typeof href !== "string" || !isAdmittedLinkHref(href)) {
        throw new Error(
          `rich-text: unsupported link URL "${String(href)}". Poodle refuses executable URL schemes.`,
        );
      }
    }
  }
  if (type === "image") {
    const attrs = (node.attrs ?? {}) as Record<string, unknown>;
    if (typeof attrs.src !== "string" || !isAdmittedImageUrl(attrs.src)) {
      throw new Error(
        `rich-text: unsupported image URL "${String(attrs.src)}". Poodle refuses executable URL schemes.`,
      );
    }
    if (typeof attrs.alt !== "string") {
      throw new Error(
        'rich-text: image nodes require a string "alt" attribute; an empty value is an explicit decorative image.',
      );
    }
  }
  if (type === "heading") {
    const level = ((node.attrs ?? {}) as Record<string, unknown>).level;
    if (typeof level !== "number" || !ADMITTED_HEADING_LEVELS.includes(level)) {
      throw new Error(
        `rich-text: unsupported heading level ${JSON.stringify(level)}; Poodle admits levels 1 to 3.`,
      );
    }
  }
  for (const child of node.content ?? []) {
    assertDocumentNodes(schema, child as ProseMirrorNodeJSON);
  }
}

function extensionsFor(features: readonly RichTextFeature[]): Extensions {
  const extensions: Extensions = [
    Document,
    Paragraph,
    Text,
    HardBreak,
    UndoRedo,
    Heading.configure({ levels: [...ADMITTED_HEADING_LEVELS] as Level[] }),
  ];
  if (features.includes("formatting")) extensions.push(Bold, Italic, Strike, Code);
  if (features.includes("links")) extensions.push(Link);
  if (features.includes("lists")) extensions.push(BulletList, OrderedList, ListItem, ListKeymap);
  if (features.includes("blockquote")) extensions.push(Blockquote);
  if (features.includes("code-block")) extensions.push(CodeBlock);
  if (features.includes("horizontal-rule")) extensions.push(HorizontalRule);
  if (features.includes("tables")) {
    extensions.push(Table, TableRow, TableCell, TableHeader);
  }
  if (features.includes("images")) extensions.push(Image);
  return extensions;
}

export function createRichTextSchema(features: readonly RichTextFeature[]): Schema {
  assertAdmittedFeatures(features);
  return getSchema(extensionsFor(features));
}

/**
 * Internal placeholder decoration. Shows only for one empty editable
 * document; read-only, disabled, and non-empty documents never show it.
 * The live getters are read on every decoration evaluation so prop updates
 * apply without recreating the engine.
 */
function placeholderExtension(
  placeholder: () => string,
  editable: () => boolean,
): Extension {
  return Extension.create({
    name: "poodleRichTextPlaceholder",
    addProseMirrorPlugins() {
      const key = new PluginKey("poodleRichTextPlaceholder");
      return [
        new Plugin({
          key,
          props: {
            decorations(editorState) {
              const text = placeholder();
              const doc = editorState.doc;
              const isEmptyEditableDocument =
                doc.childCount === 1 &&
                doc.firstChild?.type.name === "paragraph" &&
                doc.firstChild.content.size === 0;
              if (!text || !editable() || !isEmptyEditableDocument) return null;
              return DecorationSet.create(editorState.doc, [
                Decoration.widget(
                  1,
                  () => {
                    const widget = document.createElement("span");
                    widget.className = "poodle-rich-text-editor__placeholder";
                    widget.textContent = text;
                    widget.setAttribute("data-poodle-placeholder", "");
                    return widget;
                  },
                  { side: -1 },
                ),
              ]);
            },
          },
        }),
      ];
    },
  });
}

interface PendingImageRequest {
  requestImage: () => Promise<RichTextImageInput | null>;
  from: number;
  to: number;
  mappings: StepMap[];
  settled: boolean;
}

function assembleExtensions(
  features: readonly RichTextFeature[],
  placeholder: () => string,
  editable: () => boolean,
): Extensions {
  return [...extensionsFor(features), placeholderExtension(placeholder, editable)];
}

const RICH_TEXT_FOCUSABLE_SELECTOR =
  'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';

/**
 * Focus escape target for the latched Escape-then-Tab: the nearest focusable
 * element outside the whole editor composition in document order.
 */
function richTextEscapeFocusTarget(
  host: HTMLElement,
  backwards: boolean,
): HTMLElement | null {
  const candidates = Array.from(
    document.querySelectorAll<HTMLElement>(RICH_TEXT_FOCUSABLE_SELECTOR),
  ).filter((element) => !host.contains(element));
  if (candidates.length === 0) return null;
  if (backwards) {
    const before = candidates.filter(
      (element) => !!(host.compareDocumentPosition(element) & Node.DOCUMENT_POSITION_PRECEDING),
    );
    return before[before.length - 1] ?? candidates[candidates.length - 1] ?? null;
  }
  const after = candidates.filter(
    (element) => !!(host.compareDocumentPosition(element) & Node.DOCUMENT_POSITION_FOLLOWING),
  );
  return after[0] ?? candidates[0] ?? null;
}

function editorAttributes(options: RichTextEngineOptions): Record<string, string> {
  return {
    role: "textbox",
    "aria-multiline": "true",
    "aria-label": options.ariaLabel,
    class: "poodle-rich-text-editor__surface",
  };
}

export function createRichTextEngine(
  host: HTMLElement,
  options: RichTextEngineOptions,
  callbacks: RichTextEngineCallbacks,
): RichTextEngine {
  assertAdmittedFeatures(options.features);
  assertAdmittedToolbar(options.toolbar, options.features, options.requestImage);
  let features = [...options.features];
  let schema = createRichTextSchema(features);
  assertValidRichTextDocument(schema, options.value);

  let state: RichTextEngineOptions = { ...options, features: [...options.features] };
  let destroyed = false;
  let pendingImage: PendingImageRequest | null = null;

  const editable = () => !state.readOnly && !state.disabled;

  const wire = (editor: Editor) => {
    editor.on("transaction", ({ transaction }) => {
      if (pendingImage && transaction.docChanged) {
        for (const map of transaction.mapping.maps) pendingImage.mappings.push(map);
      }
    });
    editor.on("update", ({ editor: updated }) => {
      if (!destroyed) callbacks.onChange(updated.getJSON() as ProseMirrorDocumentJSON);
    });
    editor.on("transaction", () => {
      if (!destroyed) callbacks.onToolbar(snapshot());
    });
  };

  const commands = (): readonly RichTextCommand[] =>
    resolveRichTextToolbar(state.toolbar, state.features, state.requestImage).commands;

  const commandState = (command: RichTextCommand): RichTextCommandState => {
    if (destroyed || !editable() || !isRichTextCommand(command)) {
      return { available: false, active: false };
    }
    const can = editor.can();
    switch (command) {
      case "undo":
        return { available: can.undo(), active: false };
      case "redo":
        return { available: can.redo(), active: false };
      case "bold":
        return { available: can.toggleBold(), active: editor.isActive("bold") };
      case "italic":
        return { available: can.toggleItalic(), active: editor.isActive("italic") };
      case "strike":
        return { available: can.toggleStrike(), active: editor.isActive("strike") };
      case "inline-code":
        return { available: can.toggleCode(), active: editor.isActive("code") };
      case "heading-1":
        return {
          available: can.toggleHeading({ level: 1 }),
          active: editor.isActive("heading", { level: 1 }),
        };
      case "heading-2":
        return {
          available: can.toggleHeading({ level: 2 }),
          active: editor.isActive("heading", { level: 2 }),
        };
      case "heading-3":
        return {
          available: can.toggleHeading({ level: 3 }),
          active: editor.isActive("heading", { level: 3 }),
        };
      case "link":
        return {
          available: can.setLink({ href: "https://poodle.invalid" }),
          active: editor.isActive("link"),
        };
      case "bullet-list":
        return { available: can.toggleBulletList(), active: editor.isActive("bulletList") };
      case "ordered-list":
        return { available: can.toggleOrderedList(), active: editor.isActive("orderedList") };
      case "blockquote":
        return { available: can.toggleBlockquote(), active: editor.isActive("blockquote") };
      case "code-block":
        return { available: can.toggleCodeBlock(), active: editor.isActive("codeBlock") };
      case "horizontal-rule":
        return { available: can.setHorizontalRule(), active: false };
      case "insert-table":
        return {
          available: can.insertTable({ rows: 3, cols: 3, withHeaderRow: true }),
          active: false,
        };
      case "add-row":
        return { available: can.addRowAfter(), active: false };
      case "add-column":
        return { available: can.addColumnAfter(), active: false };
      case "delete-table":
        return { available: can.deleteTable(), active: false };
      case "insert-image":
        return {
          // One request is live at a time; the host choice stays downstream.
          available:
            !pendingImage && features.includes("images") && typeof state.requestImage === "function",
          active: false,
        };
    }
  };

  const snapshot = (): RichTextToolbarSnapshot => {
    const list = commands();
    const states: Record<RichTextCommand, RichTextCommandState> = {} as Record<
      RichTextCommand,
      RichTextCommandState
    >;
    for (const command of list) states[command] = commandState(command);
    return { commands: list, states };
  };

  function run(command: RichTextCommand): void {
    if (destroyed || !editable() || !isRichTextCommand(command)) return;
    switch (command) {
      case "undo":
        editor.chain().focus().undo().run();
        return;
      case "redo":
        editor.chain().focus().redo().run();
        return;
      case "bold":
        editor.chain().focus().toggleBold().run();
        return;
      case "italic":
        editor.chain().focus().toggleItalic().run();
        return;
      case "strike":
        editor.chain().focus().toggleStrike().run();
        return;
      case "inline-code":
        editor.chain().focus().toggleCode().run();
        return;
      case "heading-1":
        editor.chain().focus().toggleHeading({ level: 1 }).run();
        return;
      case "heading-2":
        editor.chain().focus().toggleHeading({ level: 2 }).run();
        return;
      case "heading-3":
        editor.chain().focus().toggleHeading({ level: 3 }).run();
        return;
      case "link":
        // The wrapper opens the editor-owned link affordance for `link`;
        // engine insertion goes through applyLink/removeLink.
        return;
      case "bullet-list":
        editor.chain().focus().toggleBulletList().run();
        return;
      case "ordered-list":
        editor.chain().focus().toggleOrderedList().run();
        return;
      case "blockquote":
        editor.chain().focus().toggleBlockquote().run();
        return;
      case "code-block":
        editor.chain().focus().toggleCodeBlock().run();
        return;
      case "horizontal-rule":
        editor.chain().focus().setHorizontalRule().run();
        return;
      case "insert-table":
        editor.chain().focus().insertTable({ rows: 3, cols: 3, withHeaderRow: true }).run();
        return;
      case "add-row":
        editor.chain().focus().addRowAfter().run();
        return;
      case "add-column":
        editor.chain().focus().addColumnAfter().run();
        return;
      case "delete-table":
        editor.chain().focus().deleteTable().run();
        return;
      case "insert-image":
        requestImageInsertion();
        return;
    }
  }

  /**
   * One host-owned async request retains the insertion selection while the
   * host chooses an asset. Resolving inserts the returned standard image
   * node once, at the retained (position-mapped) selection, only while still
   * editable and images stay enabled. Cancellation, rejection, disable, and
   * unmount change nothing and leave focus recoverable.
   */
  function requestImageInsertion(): void {
    if (destroyed || !editable()) return;
    if (!features.includes("images") || typeof state.requestImage !== "function") return;
    if (pendingImage) return;
    const selection = editor.state.selection;
    const pending: PendingImageRequest = {
      requestImage: state.requestImage,
      from: selection.from,
      to: selection.to,
      mappings: [],
      settled: false,
    };
    pendingImage = pending;
    callbacks.onToolbar(snapshot());
    void pending.requestImage().then(
      (result) => {
        if (pendingImage !== pending || pending.settled) return;
        pendingImage = null;
        pending.settled = true;
        if (result === null) {
          // Cancellation changes nothing and leaves focus recoverable.
          if (!destroyed) callbacks.onToolbar(snapshot());
          return;
        }
        if (destroyed || !editable() || !features.includes("images")) {
          if (!destroyed) callbacks.onToolbar(snapshot());
          return;
        }
        let from = pending.from;
        let to = pending.to;
        for (const map of pending.mappings) {
          const mappedFrom = map.map(from, -1);
          const mappedTo = map.map(to, 1);
          from = mappedFrom;
          to = Math.max(mappedFrom, mappedTo);
        }
        if (from < 0 || to > editor.state.doc.content.size || from > to) return;
        editor
          .chain()
          .insertContentAt({ from, to }, {
            type: "image",
            attrs: { src: result.src, alt: result.alt, title: result.title ?? null },
          })
          .run();
      },
      () => {
        // Rejection changes nothing and leaves focus recoverable.
        if (pendingImage === pending) pendingImage = null;
        pending.settled = true;
        if (!destroyed) callbacks.onToolbar(snapshot());
      },
    );
  }

  let editor = new Editor({
    element: host,
    content: state.value,
    extensions: assembleExtensions(features, () => state.placeholder, editable),
    editable: editable(),
    editorProps: { attributes: editorAttributes(state) },
  });
  wire(editor);

  /**
   * Table-cell navigation follows the ProseMirror model (Tab / Shift-Tab
   * inside tables), but Escape then Tab always escapes: the capture listener
   * below runs before the engine's own key handling, consumes the latched
   * Tab, and moves DOM focus to the next (or previous) focusable element
   * outside the editor composition. Focus is never trapped.
   */
  let escapeLatched = false;
  const hostKeydown = (event: KeyboardEvent): void => {
    if (event.key === "Escape") {
      escapeLatched = true;
      return;
    }
    if (event.key !== "Tab") {
      escapeLatched = false;
      return;
    }
    if (!escapeLatched || event.defaultPrevented) return;
    escapeLatched = false;
    event.preventDefault();
    event.stopImmediatePropagation();
    const target = richTextEscapeFocusTarget(host, event.shiftKey);
    if (target) target.focus();
    else (host.querySelector(".ProseMirror") as HTMLElement | null)?.blur();
  };
  host.addEventListener("keydown", hostKeydown, true);

  // The last accepted host value, not the engine's own serialization:
  // engine-side table normalization is engine state and never re-dispatches
  // a controlled update.
  let lastHostJson = JSON.stringify(state.value);

  // The wrapper's toolbar renders from this initial snapshot; user
  // transactions refresh it. Nothing here is a document change.
  callbacks.onToolbar(snapshot());

  return {
    commands: commands(),
    commandState,
    runCommand: run,
    linkHref: () => {
      if (destroyed || !editor.isActive("link")) return null;
      const href = editor.getAttributes("link").href;
      return typeof href === "string" ? href : null;
    },
    applyLink: (href: string) => {
      if (destroyed || !editable()) return;
      if (!isAdmittedLinkHref(href)) {
        throw new Error(
          `rich-text: unsupported link URL "${href}". Poodle refuses executable URL schemes.`,
        );
      }
      editor.chain().focus().setLink({ href }).run();
    },
    removeLink: () => {
      if (destroyed || !editable()) return;
      editor.chain().focus().unsetLink().run();
    },
    update: (next: Partial<RichTextEngineOptions>) => {
      if (destroyed) return;
      const previous = state;
      const merged: RichTextEngineOptions = { ...state, ...next };
      const featuresChanged =
        next.features !== undefined &&
        (next.features.length !== features.length ||
          next.features.some((feature, index) => feature !== features[index]));
      if (next.features && featuresChanged) {
        // Schema reconfiguration validates the next value against the next
        // schema before replacing engine state. A valid change emits no
        // callback; an invalid change leaves the prior editor intact and
        // reports a development error.
        try {
          assertAdmittedFeatures(next.features);
          assertAdmittedToolbar(merged.toolbar, next.features, merged.requestImage);
          const nextValue = next.value ?? previous.value;
          assertValidRichTextDocument(createRichTextSchema(next.features), nextValue);
        } catch (error) {
          console.error("rich-text: feature reconfiguration refused.", error);
          return;
        }
        editor.destroy();
        features = [...next.features];
        state = merged;
        const replacement = new Editor({
          element: host,
          content: state.value,
          extensions: assembleExtensions(features, () => state.placeholder, editable),
          editable: editable(),
          editorProps: { attributes: editorAttributes(state) },
        });
        wire(replacement);
        editor = replacement;
        lastHostJson = JSON.stringify(state.value);
        callbacks.onToolbar(snapshot());
        return;
      }
      state = merged;
      const editableChanged =
        (next.readOnly !== undefined && next.readOnly !== previous.readOnly) ||
        (next.disabled !== undefined && next.disabled !== previous.disabled);
      if (editableChanged) {
        editor.setEditable(editable(), false);
        callbacks.onToolbar(snapshot());
      }
      if (next.value !== undefined) {
        // Host values are compared against the last accepted host value, not
        // the engine's normalized serialization: table normalization is
        // engine state and must never re-dispatch a controlled update.
        const nextJson = JSON.stringify(state.value);
        if (nextJson !== lastHostJson) {
          // The host value is authoritative; a host revert rejects the edit
          // without a callback echo.
          try {
            assertValidRichTextDocument(schema, state.value);
          } catch (error) {
            console.error("rich-text: controlled update refused.", error);
            state = previous;
            callbacks.onToolbar(snapshot());
            return;
          }
          editor.commands.setContent(state.value, { emitUpdate: false });
          lastHostJson = nextJson;
        }
      }
      if (
        next.placeholder !== undefined &&
        next.placeholder !== previous.placeholder &&
        editor.isEmpty
      ) {
        editor.view.dispatch(editor.state.tr.setMeta("poodle-rich-text-placeholder", true));
      }
    },
    destroy: () => {
      if (destroyed) return;
      destroyed = true;
      pendingImage = null;
      host.removeEventListener("keydown", hostKeydown, true);
      editor.destroy();
    },
  };
}

/**
 * Renderer support: validate the document against the configured schema and
 * serialize the same semantic structure the editor would produce. No editor,
 * no history, no selection state, and no HTML conversion: this is the
 * schema's own DOM serializer. Never called on the server (the wrapper renders
 * the document only after client mount).
 */
export function renderRichTextDocument(
  target: HTMLElement,
  value: ProseMirrorDocumentJSON,
  features: readonly RichTextFeature[],
): void {
  assertAdmittedFeatures(features);
  const schema = createRichTextSchema(features);
  const node = assertValidRichTextDocument(schema, value);
  const fragment = DOMSerializer.fromSchema(schema).serializeFragment(node.content);
  target.replaceChildren(fragment);
}
