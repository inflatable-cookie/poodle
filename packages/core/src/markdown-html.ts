/**
 * Deterministic, DOM-free HTML sanitization for rendered Markdown.
 *
 * `marked` does not sanitize, and a caller-supplied `renderHtml` can return
 * arbitrary markup. Both the Markdown editor preview and the standalone
 * MarkdownRenderer must therefore run the same closed policy before their
 * output reaches `{@html}` / `dangerouslySetInnerHTML`.
 *
 * Why a hand-written tokenizer instead of a browser sanitizer or a dependency:
 * - `@inflatable-cookie/poodle-core` has no runtime dependencies and must keep
 *   none; adding DOMPurify/jsdom would either require a browser global (not
 *   SSR-deterministic) or drag a DOM implementation into every consumer.
 * - The contract requires *identical* server and browser output, which rules
 *   out anything that depends on `document`, `template`, or `innerHTML`.
 * - The admitted surface is small (semantic Markdown prose), so an allowlist
 *   tokenizer is auditable in full.
 *
 * A shallow tag-stripping regex is not a sanitizer: it cannot see encoded
 * entities, quoted `>` characters, namespace prefixes, or raw-text elements.
 * This module walks the string with an explicit tokenizer instead, decoding
 * entities exactly once (matching browser attribute semantics), then
 * re-encoding everything it emits. Anything it does not fully understand is
 * emitted as escaped text, never as markup.
 *
 * Policy: `safe` is the default everywhere. `trusted` is an explicit,
 * caller-owned bypass and is not implemented here — callers return the parser
 * output unchanged and own source provenance and CSP.
 */

export type MarkdownHtmlPolicy = "safe" | "trusted";

/** Elements that survive the safe policy: the admitted Markdown prose surface. */
const ALLOWED_ELEMENTS: ReadonlySet<string> = new Set([
  "a",
  "abbr",
  "b",
  "blockquote",
  "br",
  "caption",
  "cite",
  "code",
  "dd",
  "del",
  "dfn",
  "dl",
  "dt",
  "em",
  "figcaption",
  "figure",
  "h1",
  "h2",
  "h3",
  "h4",
  "h5",
  "h6",
  "hr",
  "i",
  "img",
  "input",
  "ins",
  "kbd",
  "li",
  "mark",
  "ol",
  "p",
  "pre",
  "q",
  "s",
  "samp",
  "small",
  "span",
  "strong",
  "sub",
  "sup",
  "table",
  "tbody",
  "td",
  "tfoot",
  "th",
  "thead",
  "time",
  "tr",
  "u",
  "ul",
  "var",
]);

/** Void elements never carry a closing tag in emitted output. */
const VOID_ELEMENTS: ReadonlySet<string> = new Set([
  "br",
  "hr",
  "img",
  "input",
]);

/**
 * Elements dropped together with everything up to their closing tag. These are
 * raw-text or foreign-content elements where "drop the tag, keep the text"
 * would either leak script source into the document or re-parse inner markup
 * under a different namespace.
 */
const DROP_CONTENT_ELEMENTS: ReadonlySet<string> = new Set([
  "applet",
  "base",
  "embed",
  "frame",
  "frameset",
  "iframe",
  "link",
  "math",
  "meta",
  "noembed",
  "noframes",
  "noscript",
  "object",
  "optgroup",
  "option",
  "param",
  "plaintext",
  "script",
  "select",
  "source",
  "style",
  "svg",
  "template",
  "textarea",
  "title",
  "track",
  "xmp",
]);

/**
 * URL schemes allowed on `href`/`src`. Everything else — `javascript:`,
 * `data:`, `vbscript:`, `file:` — is refused and the attribute dropped.
 */
const ALLOWED_URL_SCHEMES: ReadonlySet<string> = new Set([
  "http",
  "https",
  "mailto",
  "sms",
  "tel",
]);

type AttributeKind =
  | "align"
  | "boolean"
  | "integer"
  | "language-class"
  | "safe-text"
  | "url";

/** Attributes read on every allowed element. */
const GLOBAL_ATTRIBUTES: Readonly<Record<string, AttributeKind>> = {
  title: "safe-text",
};

/** Per-element attribute allowlists. Anything absent is dropped. */
const ELEMENT_ATTRIBUTES: Readonly<Record<string, Readonly<Record<string, AttributeKind>>>> = {
  a: { href: "url", title: "safe-text" },
  img: { alt: "safe-text", src: "url", title: "safe-text" },
  ol: { start: "integer" },
  td: { align: "align" },
  th: { align: "align" },
  code: { class: "language-class" },
  pre: { class: "language-class" },
};

/**
 * Entities decoded before a value is inspected. Browser parsers decode in one
 * pass, so this decoder does too; unrecognised names stay literal, and every
 * emitted value is re-encoded so the browser cannot re-interpret it.
 */
const NAMED_ENTITIES: Readonly<Record<string, string>> = {
  amp: "&",
  apos: "'",
  colon: ":",
  gt: ">",
  lt: "<",
  nbsp: "\u00a0",
  newline: "\n",
  quot: '"',
  tab: "\t",
};

/** The five XML entities browsers also accept without a trailing semicolon. */
const LEGACY_NAMED_ENTITIES: ReadonlySet<string> = new Set([
  "amp",
  "apos",
  "gt",
  "lt",
  "quot",
]);

const ENTITY_PATTERN = /&(?:#([0-9]+)|#[xX]([0-9a-fA-F]+)|([a-zA-Z][a-zA-Z0-9]*))(;?)/g;
const ASCII_WHITESPACE_OR_CONTROL = /[\u0000-\u0020\u007f]/g;
const ATTRIBUTE_NAME_PATTERN = /^[a-zA-Z_:][-a-zA-Z0-9_:.]*$/;
const LANGUAGE_CLASS_PATTERN = /^language-[A-Za-z0-9_+#.-]+$/;
const ALIGN_VALUES: ReadonlySet<string> = new Set(["center", "justify", "left", "right"]);

export function decodeHtmlEntities(value: string): string {
  return value.replace(
    ENTITY_PATTERN,
    (match, decimals: string | undefined, hex: string | undefined, name: string | undefined, semicolon: string) => {
      if (decimals !== undefined || hex !== undefined) {
        const codePoint = decimals !== undefined ? Number.parseInt(decimals, 10) : Number.parseInt(hex ?? "", 16);
        if (
          !Number.isFinite(codePoint) ||
          codePoint < 0 ||
          codePoint > 0x10ffff ||
          (codePoint >= 0xd800 && codePoint <= 0xdfff)
        ) {
          return match;
        }
        return String.fromCodePoint(codePoint);
      }
      const key = (name ?? "").toLowerCase();
      const replacement = NAMED_ENTITIES[key];
      if (replacement === undefined) return match;
      if (semicolon === ";" || LEGACY_NAMED_ENTITIES.has(key)) return replacement;
      return match;
    },
  );
}

function escapeText(value: string): string {
  return value.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}

function escapeAttribute(value: string): string {
  return escapeText(value).replaceAll('"', "&quot;");
}

/** Terminators that end an unquoted attribute name while scanning a tag. */
function isAttributeNameTerminator(character: string): boolean {
  return (
    character === "" ||
    character === "\u0000" ||
    character === '"' ||
    character === "'" ||
    character === "<" ||
    character === ">" ||
    character === "/" ||
    character === "=" ||
    /\s/.test(character)
  );
}

type ParsedAttribute = { name: string; value: string | null };

type ParsedTag = {
  name: string;
  closing: boolean;
  selfClosing: boolean;
  attributes: ParsedAttribute[];
  end: number;
};

function readElementName(source: string, from: number): { name: string; end: number } | null {
  const match = /^[a-zA-Z][a-zA-Z0-9:-]*/.exec(source.slice(from));
  if (!match) return null;
  return { name: match[0], end: from + match[0].length };
}

/**
 * Parse one tag starting at `source[start] === "<"`.
 *
 * Returns `null` for anything it cannot finish reading (unterminated tag,
 * unterminated quoted value, a stray `<` inside the tag) plus for tags whose
 * name is not a plausible element name. Callers emit the `<` as escaped text
 * and continue, so malformed markup degrades to visible text instead of markup.
 */
function parseTag(source: string, start: number): ParsedTag | null {
  let index = start + 1;
  let closing = false;

  if (source[index] === "/") {
    closing = true;
    index += 1;
  }

  const name = readElementName(source, index);
  if (!name) return null;
  index = name.end;

  const attributes: ParsedAttribute[] = [];
  let selfClosing = false;

  while (index < source.length) {
    while (index < source.length && /\s/.test(source[index] ?? "")) index += 1;
    if (index >= source.length) return null;

    const character = source[index] ?? "";

    if (character === ">") {
      return { name: name.name, closing, selfClosing, attributes, end: index + 1 };
    }

    if (character === "/") {
      if (source[index + 1] === ">") {
        selfClosing = true;
        return { name: name.name, closing, selfClosing, attributes, end: index + 2 };
      }
      index += 1;
      continue;
    }

    // A `<` inside a tag means the document is malformed. Abort the tag; the
    // caller re-scans from that `<` as a fresh tag or escapes it as text.
    if (character === "<") return null;

    let attributeName = "";
    while (index < source.length) {
      const next = source[index] ?? "";
      if (isAttributeNameTerminator(next)) break;
      attributeName += next;
      index += 1;
    }
    if (attributeName === "") {
      // A bare quote or `=` with no name: skip the byte and keep scanning.
      index += 1;
      continue;
    }

    while (index < source.length && /\s/.test(source[index] ?? "")) index += 1;

    let value: string | null = null;
    if (source[index] === "=") {
      index += 1;
      while (index < source.length && /\s/.test(source[index] ?? "")) index += 1;
      const quote = source[index];
      if (quote === '"' || quote === "'") {
        index += 1;
        const valueStart = index;
        while (index < source.length && source[index] !== quote) index += 1;
        if (index >= source.length) return null;
        value = source.slice(valueStart, index);
        index += 1;
      } else {
        const valueStart = index;
        while (index < source.length && !/[\s>]/.test(source[index] ?? "")) index += 1;
        value = source.slice(valueStart, index);
      }
    }

    attributes.push({ name: attributeName, value });
  }

  return null;
}

/** Case-insensitive scan for the closing tag of a dropped-content element. */
function skipDroppedContent(source: string, elementName: string, from: number): number {
  const haystack = source.toLowerCase();
  const needle = `</${elementName}`;
  let cursor = haystack.indexOf(needle, from);
  while (cursor !== -1) {
    const after = haystack[cursor + needle.length];
    if (after === undefined || /[\s>/]/.test(after)) {
      const end = source.indexOf(">", cursor);
      return end === -1 ? source.length : end + 1;
    }
    cursor = haystack.indexOf(needle, cursor + needle.length);
  }
  return source.length;
}

function sanitizeUrl(value: string): string | null {
  // Browsers ignore ASCII whitespace and controls inside URLs before scheme
  // detection ("java\tscript:" still executes), so remove them from the value
  // we both inspect and emit rather than only from the inspection copy.
  const cleaned = value.replace(ASCII_WHITESPACE_OR_CONTROL, "");
  const scheme = /^([a-z][a-z0-9+.-]*):/i.exec(cleaned);
  if (scheme && !ALLOWED_URL_SCHEMES.has(scheme[1]!.toLowerCase())) return null;
  return cleaned;
}

function sanitizeLanguageClass(value: string): string | null {
  const tokens = value
    .split(/\s+/)
    .filter((token) => LANGUAGE_CLASS_PATTERN.test(token));
  return tokens.length > 0 ? tokens.join(" ") : null;
}

function renderAttributes(tag: ParsedTag): string {
  const elementAllowlist = ELEMENT_ATTRIBUTES[tag.name] ?? {};
  let rendered = "";

  for (const attribute of tag.attributes) {
    const name = attribute.name.toLowerCase();
    const kind = elementAllowlist[name] ?? GLOBAL_ATTRIBUTES[name];
    if (!kind) continue;

    // Boolean markup only: any value is discarded and re-emitted as present.
    if (kind === "boolean") {
      rendered += ` ${name}=""`;
      continue;
    }

    const raw = decodeHtmlEntities(attribute.value ?? "");

    if (kind === "url") {
      const url = sanitizeUrl(raw);
      if (url === null) continue;
      rendered += ` ${name}="${escapeAttribute(url)}"`;
      continue;
    }

    if (kind === "integer") {
      if (!/^-?[0-9]+$/.test(raw.trim())) continue;
      rendered += ` ${name}="${escapeAttribute(raw.trim())}"`;
      continue;
    }

    if (kind === "align") {
      const align = raw.trim().toLowerCase();
      if (!ALIGN_VALUES.has(align)) continue;
      rendered += ` ${name}="${align}"`;
      continue;
    }

    if (kind === "language-class") {
      const language = sanitizeLanguageClass(raw);
      if (language === null) continue;
      rendered += ` ${name}="${escapeAttribute(language)}"`;
      continue;
    }

    rendered += ` ${name}="${escapeAttribute(raw)}"`;
  }

  return rendered;
}

function sanitizeInputElement(tag: ParsedTag): string {
  // Only Markdown task-list checkboxes are admitted, and only with the fixed
  // presentation they arrive with: no name, value, form, id, or autofocus.
  // Attribute order is preserved so the emitted markup is deterministic and
  // matches the parser's own output.
  const admitted: Array<{ name: string; value: string }> = [];
  let type: string | null = null;

  for (const attribute of tag.attributes) {
    const name = attribute.name.toLowerCase();
    if (name === "type") {
      type = decodeHtmlEntities(attribute.value ?? "").trim().toLowerCase();
      admitted.push({ name: "type", value: type });
    } else if (name === "checked") {
      admitted.push({ name: "checked", value: "" });
    } else if (name === "disabled") {
      admitted.push({ name: "disabled", value: "" });
    }
  }

  if (type !== "checkbox") return "";
  const attributes = admitted
    .map((attribute) => ` ${attribute.name}="${escapeAttribute(attribute.value)}"`)
    .join("");
  return `<input${attributes}>`;
}

function renderOpenTag(tag: ParsedTag): string {
  if (tag.name === "input") return sanitizeInputElement(tag);
  return `<${tag.name}${renderAttributes(tag)}>`;
}

/**
 * Sanitize one HTML string under the safe policy.
 *
 * Pure and deterministic: no DOM, no globals, no environment detection. The
 * same input always produces the same output in SSR and browser builds.
 */
export function sanitizeMarkdownHtml(html: string): string {
  let output = "";
  let index = 0;
  const length = html.length;

  while (index < length) {
    const tagStart = html.indexOf("<", index);
    if (tagStart === -1) {
      output += escapeText(decodeHtmlEntities(html.slice(index)));
      break;
    }
    if (tagStart > index) {
      output += escapeText(decodeHtmlEntities(html.slice(index, tagStart)));
    }
    index = tagStart;

    if (html.startsWith("<!--", index)) {
      const end = html.indexOf("-->", index + 4);
      index = end === -1 ? length : end + 3;
      continue;
    }
    if (html.startsWith("<!", index) || html.startsWith("<?", index)) {
      const end = html.indexOf(">", index + 2);
      index = end === -1 ? length : end + 1;
      continue;
    }

    const tag = parseTag(html, index);
    if (tag === null) {
      output += "&lt;";
      index += 1;
      continue;
    }
    index = tag.end;

    if (tag.closing) {
      if (ALLOWED_ELEMENTS.has(tag.name) && !VOID_ELEMENTS.has(tag.name)) {
        output += `</${tag.name}>`;
      }
      continue;
    }

    if (DROP_CONTENT_ELEMENTS.has(tag.name)) {
      if (!tag.selfClosing) index = skipDroppedContent(html, tag.name, index);
      continue;
    }

    if (!ALLOWED_ELEMENTS.has(tag.name)) continue;
    output += renderOpenTag(tag);
  }

  return output;
}
