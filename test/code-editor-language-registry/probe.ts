/**
 * Headless Chromium + WebKit proof of the g18.012 CodeEditor language
 * registry, the g18.021 token-bound presentation plumbing, and the g18.023
 * dual syntax palettes. jsdom covers the semantic suite; this probe proves
 * the parts only a real browser can: real dynamic ESM grammar loading,
 * controlled switching without remount, focus/undo survival across switches,
 * fail-closed rejected-load behavior in a live page, real mount-time
 * unknown-id refusal through each framework's boundary mechanism, and — for
 * g18.023 — representative TypeScript and JSON resolving to dedicated
 * `color.syntax.*` roles (never UI status/accent tokens), multi-hue
 * perceptual variety, ordinary identifiers on primary text, invalid syntax
 * with a non-colour cue, AA contrast for every role against every named
 * theme's actual editor panel, overlays that never repaint token text, and
 * live theme switches that restyle the same mounted editor.
 *
 *   bun test/code-editor-language-registry/probe.ts --browser=chromium
 *   bun test/code-editor-language-registry/probe.ts --browser=webkit
 */

import { chromium, webkit, type Browser, type BrowserType, type Page } from "playwright";
import { fileURLToPath } from "node:url";

const browserFlag = process.argv.find((arg) => arg.startsWith("--browser="))?.slice("--browser=".length);
const engines: Array<[string, BrowserType]> = (
  [
    ["chromium", chromium],
    ["webkit", webkit],
  ] as Array<[string, BrowserType]>
).filter(([name]) => !browserFlag || browserFlag === name);

if (engines.length === 0) {
  throw new Error(`Unknown --browser=${browserFlag}`);
}

const fixtureRoot = fileURLToPath(new URL(".", import.meta.url));
const repoRoot = fileURLToPath(new URL("../..", import.meta.url));
const viteBin = fileURLToPath(
  new URL("../../packages/svelte/preview/node_modules/vite/bin/vite.js", import.meta.url),
);
const port = 4199;
const url = `http://127.0.0.1:${port}/`;

/**
 * g18.023: the representative TypeScript sample. It must expose at least five
 * distinct chromatic syntax roles through the real grammar while leaving
 * ordinary identifier uses (`entry` at its use site, `string`, `boolean`) on
 * primary text. Keep byte-identical with the harness samples.
 */
const DOC = `// ledger
type Ledger = { owner: string; balance: number };
const answer = 42;
const label = "hello";
export function audit(entry: Ledger): boolean {
  return entry.balance > 0;
}
`;
const INVALID_DOC = `${DOC}### oops ###\nconst broken = ;\n`;

/**
 * The designed dark primitives behind the semantic roles (g18.023). Eclipse
 * inherits the dark base unchanged, so its computed roles must be exactly
 * these; iceberg selects the light ramp, so its keyword must be exactly the
 * light primitive.
 */
const DARK_RAMP = {
  comment: "#9db2c6",
  keyword: "#c9b7fd",
  string: "#97d9a2",
  literal: "#f4c37a",
  type: "#7fd6dc",
  callable: "#8fb8ff",
  property: "#f0a7c6",
  operator: "#aebccf",
  punctuation: "#98adbe",
  invalid: "#ff8f86",
} as const;
const LIGHT_KEYWORD = "#6c2fd2";

const SYNTAX_ROLES = [
  "comment",
  "keyword",
  "string",
  "literal",
  "type",
  "callable",
  "property",
  "operator",
  "punctuation",
  "invalid",
] as const;
type SyntaxRole = (typeof SYNTAX_ROLES)[number];

/** Chromatic roles: the perceptual-variety floor is measured across these. */
const CHROMATIC_ROLES: SyntaxRole[] = ["keyword", "string", "literal", "type", "callable", "property"];

/** Light themes select the light primitive ramp; the rest inherit dark. */
const LIGHT_THEMES = new Set(["iceberg", "clay", "meadow"]);

/** Every named Poodle theme; the sweep proves AA contrast for each panel. */
const ALL_THEMES = [
  "iceberg",
  "eclipse",
  "graphite",
  "midnight",
  "nord",
  "rose",
  "forest",
  "solarized",
  "hornet",
  "cobalt",
  "clay",
  "meadow",
] as const;

let failures = 0;

function check(label: string, ok: boolean, detail = ""): void {
  if (!ok) failures += 1;
  console.log(`${ok ? "  ok  " : "  FAIL"}  ${label}${detail ? `  — ${detail}` : ""}`);
}

async function waitForServer(timeoutMs = 30_000): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      const res = await fetch(url, { signal: AbortSignal.timeout(1000) });
      if (res.ok) return;
    } catch {
      await new Promise((resolve) => setTimeout(resolve, 150));
    }
  }
  throw new Error(`code-editor language-registry fixture on :${port} did not start`);
}

const child = Bun.spawn(
  [
    "bun",
    viteBin,
    "--config",
    `${fixtureRoot}/vite.config.ts`,
    "--port",
    String(port),
    "--strictPort",
    "--host",
    "127.0.0.1",
  ],
  {
    cwd: repoRoot,
    stdout: "inherit",
    stderr: "inherit",
  },
);

await waitForServer();

async function settle(page: Page): Promise<void> {
  await page.evaluate(
    () =>
      new Promise<void>((resolve) => {
        requestAnimationFrame(() => {
          window.setTimeout(resolve, 200);
        });
      }),
  );
}

type Counters = { typescript: number; json: number; broken: number };

/**
 * g18.023: computed reference colours for the semantic syntax roles the
 * private highlight style consumes. Captured live so theme switches are
 * compared against the variables the mounted editor actually resolves.
 */
async function roleColors(page: Page): Promise<Record<SyntaxRole, string>> {
  return page.evaluate((roles) => {
    const probe = document.createElement("span");
    probe.setAttribute(
      "style",
      "position:absolute;visibility:hidden;pointer-events:none",
    );
    document.body.appendChild(probe);
    const out: Partial<Record<SyntaxRole, string>> = {};
    for (const role of roles) {
      probe.style.color = `var(--poodle-color-syntax-${role})`;
      out[role as SyntaxRole] = getComputedStyle(probe).color;
    }
    probe.remove();
    return out as Record<SyntaxRole, string>;
  }, [...SYNTAX_ROLES]);
}

function hexToRgbString(hex: string): string {
  const n = parseInt(hex.slice(1), 16);
  return `rgb(${(n >> 16) & 255}, ${(n >> 8) & 255}, ${n & 255})`;
}

/** Alpha of a computed rgba()/color(srgb ... / a) value; 1 when opaque. */
function overlayAlpha(background: string): number {
  const comma = background.match(/,\s*([\d.]+)\s*\)$/);
  if (comma) return parseFloat(comma[1]);
  const slash = background.match(/\/\s*([\d.]+)\s*\)$/);
  if (slash) return parseFloat(slash[1]);
  return 1;
}

type StyledSpan = {
  text: string;
  color: string;
  invalid: boolean;
  line: number;
};

/** Every span inside the editor content that carries non-default colour. */
async function styledSpans(page: Page, framework: string): Promise<StyledSpan[]> {
  return page.evaluate((fw) => {
    const frame = document.querySelector(`[data-framework="${fw}"] [data-part='main-editor']`);
    const content = frame?.querySelector(".cm-content");
    if (!(content instanceof HTMLElement)) throw new Error(`missing ${fw} editor content`);
    const base = getComputedStyle(content).color;
    const lines = [...content.querySelectorAll(":scope > .cm-line")];
    return [...content.querySelectorAll("span")].flatMap((span) => {
      const color = getComputedStyle(span).color;
      const invalid = span.classList.contains("poodle-code-editor__syntax-invalid");
      if (color === base && !invalid) return [];
      const line = lines.findIndex((line) => line.contains(span));
      return [{ text: span.textContent ?? "", color, invalid, line }];
    });
  }, framework);
}

/**
 * The styled spans of one rendered editor line, by its exact text. Used to
 * prove the use-site of an ordinary identifier carries no syntax span.
 */
async function styledSpansOnLine(
  page: Page,
  framework: string,
  lineText: string,
): Promise<string[]> {
  return page.evaluate(
    ([fw, needle]) => {
      const content = document.querySelector(
        `[data-framework="${fw}"] [data-part='main-editor'] .cm-content`,
      );
      if (!(content instanceof HTMLElement)) throw new Error(`missing ${fw} editor content`);
      const base = getComputedStyle(content).color;
      const line = [...content.querySelectorAll(":scope > .cm-line")].find(
        (candidate) => (candidate.textContent ?? "") === needle,
      );
      if (!(line instanceof HTMLElement)) throw new Error(`missing line ${JSON.stringify(needle)}`);
      return [...line.querySelectorAll("span")]
        .filter((span) => getComputedStyle(span).color !== base)
        .map((span) => span.textContent ?? "");
    },
    [framework, lineText] as const,
  );
}

/** The resolved background colour of the mounted editor panel. */
async function panelColor(page: Page, framework: string): Promise<string> {
  return page.evaluate((fw) => {
    const editor = document.querySelector(
      `[data-framework="${fw}"] [data-part='main-editor'] .poodle-code-editor`,
    );
    if (!(editor instanceof HTMLElement)) throw new Error(`missing ${fw} editor panel`);
    return getComputedStyle(editor).backgroundColor;
  }, framework);
}

/**
 * In-page colour parsing + WCAG contrast. Handles rgb()/rgba(), hex, oklch()
 * (the contrast-axis rendering), and color(srgb ...). Throws on anything else
 * so an engine serialization change fails loudly instead of silently passing.
 */
const CONTRAST_HELPERS = `
  function parseComponent(value, scale) {
    if (value.endsWith('%')) return (parseFloat(value) / 100) * scale;
    return parseFloat(value);
  }
  function parseColor(raw) {
    const value = raw.trim();
    let m = value.match(/^rgba?\\(([\\d.%,\\s\\/]+)\\)$/i);
    if (m) {
      const parts = m[1].split(/[\\s,\\/]+/).filter(Boolean);
      if (parts.length >= 3) {
        const rgba = parts.map((p, i) => i < 3 ? Math.round(parseComponent(p, 255)) : parseFloat(p));
        return { r: rgba[0], g: rgba[1], b: rgba[2], a: rgba.length > 3 ? rgba[3] : 1 };
      }
    }
    m = value.match(/^#([0-9a-f]{6})$/i);
    if (m) {
      const n = parseInt(m[1], 16);
      return { r: (n >> 16) & 255, g: (n >> 8) & 255, b: n & 255, a: 1 };
    }
    m = value.match(/^oklch\\(([\\d.%]+)\\s+([\\d.%]+)\\s+([\\d.]+(?:deg|rad)?)(?:\\s*\\/\\s*([\\d.%]+))?\\)$/i);
    if (m) {
      const L = m[1].endsWith('%') ? parseFloat(m[1]) / 100 : parseFloat(m[1]);
      const C = parseFloat(m[2]);
      const H = parseFloat(m[3]) * (m[3].includes('deg') ? 1 : m[3].includes('rad') ? 180 / Math.PI : 1);
      const h = H * Math.PI / 180;
      const l_ = L + 0.3963377774 * C * Math.cos(h) + 0.2158037573 * C * Math.sin(h);
      const m_ = L - 0.1055613458 * C * Math.cos(h) - 0.0638541728 * C * Math.sin(h);
      const s_ = L - 0.0894841775 * C * Math.cos(h) - 1.2914855480 * C * Math.sin(h);
      const l = l_ * l_ * l_, m2 = m_ * m_ * m_, s = s_ * s_ * s_;
      let r =  4.0767416621 * l - 3.3077115913 * m2 + 0.2309699292 * s;
      let g = -1.2684380046 * l + 2.6097574011 * m2 - 0.3413193965 * s;
      let b = -0.0041960863 * l - 0.7034186147 * m2 + 1.7076147010 * s;
      const enc = (v) => v <= 0.0031308 ? 12.92 * v : 1.055 * Math.pow(v, 1 / 2.4) - 0.055;
      return {
        r: Math.round(Math.min(255, Math.max(0, enc(r) * 255))),
        g: Math.round(Math.min(255, Math.max(0, enc(g) * 255))),
        b: Math.round(Math.min(255, Math.max(0, enc(b) * 255))),
        a: m[4] === undefined ? 1 : m[4].endsWith('%') ? parseFloat(m[4]) / 100 : parseFloat(m[4]),
      };
    }
    m = value.match(/^color\\(srgb\\s+([\\d.%]+)\\s+([\\d.%]+)\\s+([\\d.%]+)(?:\\s*\\/\\s*([\\d.%]+))?\\)$/i);
    if (m) {
      const enc = (v) => v <= 0.0031308 ? 12.92 * v : 1.055 * Math.pow(v, 1 / 2.4) - 0.055;
      const part = (p) => p.endsWith('%') ? parseFloat(p) / 100 : parseFloat(p);
      return {
        r: Math.round(Math.min(255, Math.max(0, enc(part(m[1])) * 255))),
        g: Math.round(Math.min(255, Math.max(0, enc(part(m[2])) * 255))),
        b: Math.round(Math.min(255, Math.max(0, enc(part(m[3])) * 255))),
        a: m[4] === undefined ? 1 : m[4].endsWith('%') ? parseFloat(m[4]) / 100 : parseFloat(m[4]),
      };
    }
    throw new Error('unparseable computed color: ' + value);
  }
  function luminance({ r, g, b }) {
    const lin = (c) => { const s = c / 255; return s <= 0.04045 ? s / 12.92 : Math.pow((s + 0.055) / 1.055, 2.4); };
    return 0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b);
  }
  function wcagContrast(fgRaw, bgRaw) {
    const fg = parseColor(fgRaw);
    const bg = parseColor(bgRaw);
    if (bg.a < 1) throw new Error('opaque background expected: ' + bgRaw);
    const la = luminance(fg), lb = luminance(bg);
    const hi = Math.max(la, lb), lo = Math.min(la, lb);
    return (hi + 0.05) / (lo + 0.05);
  }
`;

/** Per-role WCAG contrast of the syntax roles against the editor panel. */
async function roleContrasts(
  page: Page,
  framework: string,
): Promise<{ contrasts: Record<SyntaxRole, number>; panel: string }> {
  const [roles, panel] = await Promise.all([
    roleColors(page),
    panelColor(page, framework),
  ]);
  const contrasts = await page.evaluate(
    ({ roles, panel, helpers }) => {
      const wcag = new Function(`${helpers}; return wcagContrast;`)();
      const out: Partial<Record<SyntaxRole, number>> = {};
      for (const [role, color] of Object.entries(roles)) {
        out[role as SyntaxRole] = wcag(color, panel);
      }
      return out;
    },
    { roles, panel, helpers: CONTRAST_HELPERS },
  );
  return { contrasts: contrasts as Record<SyntaxRole, number>, panel };
}

async function clickPart(page: Page, framework: string, part: string): Promise<void> {
  await page.evaluate(
    ([fw, id]) => {
      const button = document.querySelector(`[data-framework="${fw}"] [data-part='${id}']`);
      if (!(button instanceof HTMLElement)) throw new Error(`missing ${fw} ${id} button`);
      button.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    },
    [framework, part],
  );
}

async function setTheme(page: Page, theme: string): Promise<void> {
  await page.evaluate((name) => {
    document.documentElement.dataset.theme = name;
  }, theme);
}

async function counters(page: Page, framework: string): Promise<Counters> {
  return page.evaluate((fw) => {
    const all = (window as unknown as { __registryCounters: Record<string, Counters> }).__registryCounters;
    return { ...all[fw] };
  }, framework);
}

async function selectLanguage(page: Page, framework: string, language: string): Promise<void> {
  await clickPart(page, framework, `language-${language}`);
}

async function readEditor(page: Page, framework: string): Promise<{
  doc: string;
  marked: boolean;
  focused: boolean;
}> {
  return page.evaluate((fw) => {
    const section = document.querySelector(`[data-framework="${fw}"]`);
    if (!(section instanceof HTMLElement)) throw new Error(`missing ${fw} section`);
    const frame = section.querySelector("[data-part='main-editor']");
    const editor = frame?.querySelector(".cm-editor");
    const content = frame?.querySelector(".cm-content");
    return {
      doc: content
        ? [...content.querySelectorAll(".cm-line")].map((line) => line.textContent ?? "").join("\n")
        : "",
      marked: editor != null && editor.hasAttribute("data-probe-id"),
      focused: content !== null && document.activeElement === content,
    };
  }, framework);
}

async function markEditor(page: Page, framework: string): Promise<void> {
  await page.evaluate((fw) => {
    const editor = document.querySelector(
      `[data-framework="${fw}"] [data-part='main-editor'] .cm-editor`,
    );
    if (!(editor instanceof HTMLElement)) throw new Error(`missing ${fw} editor`);
    editor.setAttribute("data-probe-id", "mounted");
  }, framework);
}

/** Focus the editor and park the caret at the exact end of the document. */
async function typeText(page: Page, framework: string, text: string): Promise<void> {
  await page.locator(`[data-framework="${framework}"] [data-part='main-editor'] .cm-content`).click();
  await page.keyboard.press("ControlOrMeta+End");
  await page.keyboard.type(text);
  await settle(page);
}

/** Double-click a word to make a real text selection on the mounted editor. */
async function selectWord(page: Page, framework: string, word: string): Promise<void> {
  await page
    .locator(`[data-framework="${framework}"] [data-part='main-editor'] .cm-content`)
    .getByText(word, { exact: true })
    .first()
    .dblclick();
  await settle(page);
}

/** Undo until the document is exactly `doc`, bounded for flake safety. */
async function undoUntil(page: Page, framework: string, doc: string): Promise<boolean> {
  for (let steps = 0; steps < 20; steps += 1) {
    const current = await readEditor(page, framework);
    if (current.doc === doc) return true;
    await page.keyboard.press("ControlOrMeta+z");
    await settle(page);
  }
  return (await readEditor(page, framework)).doc === doc;
}

async function runFramework(page: Page, framework: string, browserName: string): Promise<void> {
  const section = `[data-framework="${framework}"]`;
  await page.locator(`${section} [data-part='main-editor'] .cm-content`).waitFor();

  // Collect unhandled rejections so a rejected load is observable in-page.
  await page.evaluate(() => {
    const w = window as unknown as { __rejections: string[] };
    w.__rejections = [];
    window.addEventListener("unhandledrejection", (event) => {
      w.__rejections.push(
        event.reason instanceof Error ? event.reason.message : String(event.reason),
      );
    });
  });

  await markEditor(page, framework);
  const initial = await readEditor(page, framework);
  const initialCounters = await counters(page, framework);
  check(
    `${browserName} ${framework} initial mount loads exactly the selected language`,
    initial.doc === DOC && initial.marked && initialCounters.typescript === 1 && initialCounters.json === 0 && initialCounters.broken === 0,
    `doc=${JSON.stringify(initial.doc)} counters=${JSON.stringify(initialCounters)}`,
  );

  // 0b. g18.023: representative TypeScript resolves to the dedicated syntax
  //     roles. Every span colour must equal the live --poodle-color-syntax-*
  //     variable for its role — never an accent or UI status token.
  const eclipseRefs = await roleColors(page);
  const tsSpans = await styledSpans(page, framework);
  const tsExpectations: Array<[string, SyntaxRole, RegExp]> = [
    ["ledger comment", "comment", /^\/\/ ledger$/],
    ["type keyword", "keyword", /^type$/],
    ["const keyword", "keyword", /^const$/],
    ["Ledger type", "type", /^Ledger$/],
    ["audit callable", "callable", /^audit$/],
    ["answer callable", "callable", /^answer$/],
    ["42 literal", "literal", /^42$/],
    ["hello string", "string", /^"hello"$/],
    ["balance property", "property", /^balance$/],
    ["compare operator", "operator", /^>$/],
    ["brace punctuation", "punctuation", /^[{(;,)]+$/],
  ];
  for (const [label, role, pattern] of tsExpectations) {
    const span = tsSpans.find((candidate) => pattern.test(candidate.text.trim()));
    check(
      `${browserName} ${framework} typescript ${label} span uses --poodle-color-syntax-${role}`,
      Boolean(span) && span?.color === eclipseRefs[role],
      span ? `color=${span.color} expected=${eclipseRefs[role]}` : "no styled span",
    );
  }
  check(
    `${browserName} ${framework} full typescript renders syntax spans`,
    tsSpans.length >= tsExpectations.length,
    JSON.stringify(tsSpans),
  );

  // The designed dark base is inherited unchanged: eclipse's roles are the
  // dark primitives, resolved through generated CSS.
  for (const role of SYNTAX_ROLES) {
    check(
      `${browserName} ${framework} eclipse ${role} role equals the dark primitive`,
      eclipseRefs[role] === hexToRgbString(DARK_RAMP[role]),
      `color=${eclipseRefs[role]} expected=${hexToRgbString(DARK_RAMP[role])}`,
    );
  }

  // Perceptual variety: at least five distinct chromatic roles where the
  // grammar exposes them — no accent/white field.
  const distinctChromaticTs = new Set(
    CHROMATIC_ROLES.map((role) =>
      tsSpans.some((span) => span.color === eclipseRefs[role]) ? eclipseRefs[role] : null,
    ).filter(Boolean),
  );
  check(
    `${browserName} ${framework} typescript renders at least five distinct chromatic roles`,
    distinctChromaticTs.size >= 5,
    `distinct=${distinctChromaticTs.size} of ${CHROMATIC_ROLES.join(",")}`,
  );

  // Ordinary identifiers stay primary text: on the use-site line the only
  // styled spans are `return`, `balance`, and `>` — `entry` carries none.
  const useSiteSpans = await styledSpansOnLine(page, framework, "  return entry.balance > 0;");
  check(
    `${browserName} ${framework} ordinary identifier use stays unstyled primary text`,
    useSiteSpans.includes("entry") === false &&
      useSiteSpans.some((text) => text.trim() === "balance") &&
      useSiteSpans.some((text) => text.trim() === "return"),
    JSON.stringify(useSiteSpans),
  );

  // 1. Controlled switch to the second consumer language: one additional lazy
  //    load, same editor instance, document preserved.
  await selectLanguage(page, framework, "json");
  await settle(page);
  const jsonState = await readEditor(page, framework);
  const jsonCounters = await counters(page, framework);
  check(
    `${browserName} ${framework} switching loads the second language once on the same editor`,
    jsonState.doc === DOC && jsonState.marked && jsonCounters.typescript === 1 && jsonCounters.json === 1,
    `counters=${JSON.stringify(jsonCounters)} marked=${jsonState.marked}`,
  );

  // 2. Switching back reuses the memoized load: no new loader invocation.
  await selectLanguage(page, framework, "typescript");
  await settle(page);
  const backState = await readEditor(page, framework);
  const backCounters = await counters(page, framework);
  check(
    `${browserName} ${framework} re-selection reuses the memoized load`,
    backState.doc === DOC && backState.marked && backCounters.typescript === 1 && backCounters.json === 1,
    `counters=${JSON.stringify(backCounters)}`,
  );

  // 2b. g18.023: a real JSON sample tokenizes with its own roles. Property
  //     keys read as property, strings as string, numbers/booleans as
  //     literal, and braces/separators as punctuation.
  await clickPart(page, framework, "sample-json");
  await settle(page);
  const jsonSampleSpans = await styledSpans(page, framework);
  const jsonExpectations: Array<[string, SyntaxRole, RegExp]> = [
    ["answer key", "property", /^"answer"$/],
    ["label key", "property", /^"label"$/],
    ["hello string", "string", /^"hello"$/],
    ["42 literal", "literal", /^42$/],
    ["true literal", "literal", /^true$/],
    ["brace punctuation", "punctuation", /^[{}:,]+$/],
  ];
  for (const [label, role, pattern] of jsonExpectations) {
    const span = jsonSampleSpans.find((candidate) => pattern.test(candidate.text.trim()));
    check(
      `${browserName} ${framework} json ${label} span uses --poodle-color-syntax-${role}`,
      Boolean(span) && span?.color === eclipseRefs[role],
      span ? `color=${span.color} expected=${eclipseRefs[role]}` : "no styled span",
    );
  }
  const distinctChromaticJson = new Set(
    CHROMATIC_ROLES.map((role) =>
      jsonSampleSpans.some((span) => span.color === eclipseRefs[role]) ? eclipseRefs[role] : null,
    ).filter(Boolean),
  );
  check(
    `${browserName} ${framework} json renders its chromatic roles distinctly`,
    distinctChromaticJson.size >= 3,
    `distinct=${distinctChromaticJson.size}`,
  );
  const combinedChromatic = new Set([...distinctChromaticTs, ...distinctChromaticJson]);
  check(
    `${browserName} ${framework} ts+json together expose at least five distinct chromatic roles`,
    combinedChromatic.size >= 5,
    `distinct=${combinedChromatic.size}`,
  );
  await clickPart(page, framework, "sample-typescript");
  await settle(page);

  // 3. Plain text is built in: no registry entry, no loader, same editor.
  await selectLanguage(page, framework, "plain-text");
  await settle(page);
  const plainState = await readEditor(page, framework);
  const plainCounters = await counters(page, framework);
  check(
    `${browserName} ${framework} plain-text switches without consulting the registry`,
    plainState.doc === DOC && plainState.marked && plainCounters.typescript === 1 && plainCounters.json === 1 && plainCounters.broken === 0,
    `counters=${JSON.stringify(plainCounters)}`,
  );
  const plainLanguageSpans = await styledSpans(page, framework);
  check(
    `${browserName} ${framework} plain-text renders zero syntax spans`,
    plainLanguageSpans.length === 0,
    JSON.stringify(plainLanguageSpans),
  );

  // 4. Editing survives switching: typed text persists and undo history works
  //    across a plain-text -> typescript -> json round trip.
  await typeText(page, framework, "\nconst live = true;");
  const typed = await readEditor(page, framework);
  check(
    `${browserName} ${framework} typing lands at the end of the plain-text editor`,
    typed.doc === `${DOC}\nconst live = true;`,
    `doc=${JSON.stringify(typed.doc)}`,
  );
  await selectLanguage(page, framework, "typescript");
  await settle(page);
  await selectLanguage(page, framework, "json");
  await settle(page);
  const afterSwitch = await readEditor(page, framework);
  check(
    `${browserName} ${framework} typed text survives two language switches`,
    afterSwitch.doc === `${DOC}\nconst live = true;` && afterSwitch.marked,
    `doc=${JSON.stringify(afterSwitch.doc)}`,
  );
  const undoReached = await undoUntil(page, framework, DOC);
  const afterUndo = await readEditor(page, framework);
  check(
    `${browserName} ${framework} undo history survives language switching`,
    undoReached && afterUndo.doc === DOC && afterUndo.marked,
    `doc=${JSON.stringify(afterUndo.doc)}`,
  );

  // 5. A rejected load fails closed: the rejection surfaces, the previous
  //    language stays active, and nothing remounts.
  const beforeBroken = await counters(page, framework);
  await selectLanguage(page, framework, "broken");
  await settle(page);
  await settle(page);
  const brokenState = await readEditor(page, framework);
  const brokenCounters = await counters(page, framework);
  const rejections = await page.evaluate(() => {
    const w = window as unknown as { __rejections: string[] };
    return [...w.__rejections];
  });
  check(
    `${browserName} ${framework} the rejected load surfaces as an unhandled rejection`,
    rejections.some((message) => message.includes("grammar exploded")),
    `rejections=${JSON.stringify(rejections)}`,
  );
  check(
    `${browserName} ${framework} the failed switch loads only the broken loader and keeps the editor`,
    brokenCounters.broken === beforeBroken.broken + 1 && brokenState.marked && brokenState.doc === DOC,
    `counters=${JSON.stringify(brokenCounters)} marked=${brokenState.marked}`,
  );

  // 6. Recovery: reselecting an admitted id reconfigures through the memoized
  //    registry without remounting.
  await selectLanguage(page, framework, "typescript");
  await settle(page);
  const recovered = await readEditor(page, framework);
  const recoveredCounters = await counters(page, framework);
  check(
    `${browserName} ${framework} the editor recovers through the memoized registry after a failed load`,
    recovered.marked && recoveredCounters.typescript === 1 && recoveredCounters.broken === beforeBroken.broken + 1,
    `counters=${JSON.stringify(recoveredCounters)}`,
  );

  // 7. A mounted unknown id is refused, never silently downgraded to plain
  //    text: the boundary catches the real mount error and no editor mounts.
  await page.evaluate((fw) => {
    const button = document.querySelector(
      `[data-framework="${fw}"] [data-part='mount-invalid']`,
    );
    if (!(button instanceof HTMLElement)) throw new Error(`missing ${fw} mount-invalid button`);
    button.dispatchEvent(new MouseEvent("click", { bubbles: true }));
  }, framework);
  await settle(page);
  const refusal = await page.evaluate((fw) => {
    const scope = document.querySelector(`[data-framework="${fw}"]`);
    if (!(scope instanceof HTMLElement)) throw new Error(`missing ${fw} section`);
    return {
      message: scope.querySelector("[data-part='mount-refusal']")?.textContent ?? "",
      mounted: scope.querySelectorAll("[data-part='refusal-editor'] .cm-editor").length,
    };
  }, framework);
  check(
    `${browserName} ${framework} an unknown id is refused at mount with the exact failure`,
    /unsupported language "cobol"/.test(refusal.message),
    `message=${JSON.stringify(refusal.message)}`,
  );
  check(
    `${browserName} ${framework} the refused mount never presents an editor`,
    refusal.mounted === 0,
    `mounted=${refusal.mounted}`,
  );
  const stillThere = await readEditor(page, framework);
  check(
    `${browserName} ${framework} the refused mount leaves the working editor untouched`,
    stillThere.marked && stillThere.doc === DOC,
    `marked=${stillThere.marked}`,
  );

  // 8. g18.021: full -> plain performance mode drops token presentation on the
  //    same mounted editor without consulting the registry; full restores it.
  await clickPart(page, framework, "mode-plain");
  await settle(page);
  const plainModeState = await readEditor(page, framework);
  const plainModeCounters = await counters(page, framework);
  const plainModeSpans = await styledSpans(page, framework);
  check(
    `${browserName} ${framework} plain performance mode drops syntax spans on the same editor`,
    plainModeState.doc === DOC && plainModeState.marked && plainModeSpans.length === 0,
    `doc=${JSON.stringify(plainModeState.doc)} spans=${JSON.stringify(plainModeSpans)} marked=${plainModeState.marked}`,
  );
  check(
    `${browserName} ${framework} plain performance mode loads no grammar`,
    plainModeCounters.typescript === 1 && plainModeCounters.json === 1,
    `counters=${JSON.stringify(plainModeCounters)}`,
  );
  await clickPart(page, framework, "mode-full");
  await settle(page);
  const restoredModeState = await readEditor(page, framework);
  const restoredModeSpans = await styledSpans(page, framework);
  const restoredKeyword = restoredModeSpans.find((span) => span.text.trim() === "const");
  check(
    `${browserName} ${framework} returning to full mode reinstates token presentation without remounting`,
    restoredModeState.doc === DOC && restoredModeState.marked &&
      Boolean(restoredKeyword) && restoredKeyword?.color === eclipseRefs["keyword"],
    `color=${restoredKeyword?.color} expected=${eclipseRefs["keyword"]} marked=${restoredModeState.marked}`,
  );

  // 9. g18.023: malformed TypeScript plants parser error nodes; they take the
  //    invalid role AND a non-colour wavy-underline cue, then lose both when
  //    the document is repaired. Host diagnostics stay host-owned.
  await clickPart(page, framework, "sample-invalid");
  await settle(page);
  const invalidState = await readEditor(page, framework);
  const invalidRefs = await roleColors(page);
  const invalidSpans = await styledSpans(page, framework);
  const invalidMarks = invalidSpans.filter((span) => span.invalid);
  check(
    `${browserName} ${framework} parser error nodes receive the invalid syntax role`,
    invalidState.marked && invalidMarks.length > 0 &&
      invalidMarks.every((span) => span.color === invalidRefs["invalid"]),
    JSON.stringify(invalidMarks.slice(0, 5)),
  );
  const invalidCue = await page.evaluate(
    ([fw, helpers]) => {
      const wcag = new Function(`${helpers}; return wcagContrast;`)();
      const mark = document.querySelector(
        `[data-framework="${fw}"] [data-part='main-editor'] .poodle-code-editor__syntax-invalid`,
      );
      const panel = document.querySelector(
        `[data-framework="${fw}"] [data-part='main-editor'] .poodle-code-editor`,
      );
      if (!(mark instanceof HTMLElement) || !(panel instanceof HTMLElement)) {
        return { present: false, decoration: null, style: null, contrast: 0 };
      }
      const style = getComputedStyle(mark);
      return {
        present: true,
        decoration: style.textDecorationLine,
        style: style.textDecorationStyle,
        contrast: wcag(style.color, getComputedStyle(panel).backgroundColor),
      };
    },
    [framework, CONTRAST_HELPERS] as const,
  );
  check(
    `${browserName} ${framework} invalid syntax keeps a non-colour wavy cue`,
    invalidCue.present && (invalidCue.decoration ?? "").includes("underline") && invalidCue.style === "wavy",
    JSON.stringify(invalidCue),
  );
  check(
    `${browserName} ${framework} the invalid role itself meets AA on the panel`,
    invalidCue.contrast >= 4.5,
    `contrast=${invalidCue.contrast.toFixed(2)}`,
  );
  await clickPart(page, framework, "sample-typescript");
  await settle(page);
  const cleanedSpans = await styledSpans(page, framework);
  check(
    `${browserName} ${framework} repairing the document removes the invalid marks`,
    cleanedSpans.every((span) => !span.invalid),
    JSON.stringify(cleanedSpans.filter((span) => span.invalid)),
  );

  // 10. g18.023: overlays never erase legibility. A text selection and an
  //     active search match tint the background without repainting token
  //     text; every span keeps its exact role colour under both overlays.
  await selectWord(page, framework, "balance");
  const selectionSpans = await styledSpans(page, framework);
  // The editor keeps the browser's native selection (no drawSelection
  // override), so the overlay is proven from the resolved ::selection style
  // where the engine exposes it, falling back to the shipped component rule.
  const selectionOverlay = await page.evaluate((fw) => {
    const content = document.querySelector(
      `[data-framework="${fw}"] [data-part='main-editor'] .cm-content`,
    );
    const computedSelection = content ? getComputedStyle(content, "::selection").backgroundColor : null;
    let poodleRule: string | null = null;
    for (const sheet of document.styleSheets) {
      let rules: CSSRuleList;
      try {
        rules = sheet.cssRules;
      } catch {
        continue;
      }
      for (const rule of rules) {
        if (
          rule instanceof CSSStyleRule &&
          rule.selectorText?.includes(".cm-selectionBackground") &&
          rule.style.background.includes("transparent")
        ) {
          poodleRule = rule.style.background;
        }
      }
    }
    return { poodleRule, computedSelection };
  }, framework);
  const selectedProperty = selectionSpans.find((span) => span.text.trim() === "balance");
  check(
    `${browserName} ${framework} the selection overlay keeps token text at its role colour`,
    Boolean(selectedProperty) && selectedProperty?.color === eclipseRefs["property"] &&
      selectionSpans.some((span) => span.text.trim() === "const" && span.color === eclipseRefs["keyword"]),
    JSON.stringify(selectionSpans.filter((span) => ["balance", "const"].includes(span.text.trim()))),
  );
  const selectionTranslucent =
    (selectionOverlay.computedSelection !== null &&
      selectionOverlay.computedSelection !== "rgba(0, 0, 0, 0)" &&
      overlayAlpha(selectionOverlay.computedSelection) <= 0.5) ||
    (selectionOverlay.poodleRule !== null &&
      /color-mix.*28%.*transparent/s.test(selectionOverlay.poodleRule));
  check(
    `${browserName} ${framework} the selection overlay is translucent over the panel`,
    selectionTranslucent,
    `computed=${selectionOverlay.computedSelection} rule=${selectionOverlay.poodleRule}`,
  );
  await page.keyboard.press("Escape");
  await page.keyboard.press("ControlOrMeta+f");
  await page.keyboard.type("balance");
  await page.keyboard.press("Enter");
  await settle(page);
  const searchOverlay = await page.evaluate((fw) => {
    const content = document.querySelector(
      `[data-framework="${fw}"] [data-part='main-editor'] .cm-content`,
    );
    const match = content?.querySelector(".cm-searchMatch");
    return match ? getComputedStyle(match).backgroundColor : null;
  }, framework);
  const searchSpans = await styledSpans(page, framework);
  const searchedProperty = searchSpans.find((span) => span.text.trim() === "balance");
  check(
    `${browserName} ${framework} the search-match overlay keeps token text at its role colour`,
    searchOverlay !== null && Boolean(searchedProperty) && searchedProperty?.color === eclipseRefs["property"],
    `background=${searchOverlay} spans=${JSON.stringify(searchSpans.filter((span) => span.text.trim() === "balance"))}`,
  );
  check(
    `${browserName} ${framework} the search-match overlay is translucent over the panel`,
    searchOverlay !== null && overlayAlpha(searchOverlay) <= 0.5,
    `background=${searchOverlay} alpha=${searchOverlay === null ? "-" : overlayAlpha(searchOverlay)}`,
  );
  await page.keyboard.press("Escape");
  await settle(page);

  // 11. g18.023: every named theme's actual panel keeps every syntax role at
  //     AA contrast, and switching themes restyles the same mounted editor
  //     live — no remount, no grammar reload, no document, selection, or
  //     history change.
  await page.evaluate((fw) => {
    const editor = document.querySelector(
      `[data-framework="${fw}"] [data-part='main-editor'] .cm-editor`,
    );
    if (!(editor instanceof HTMLElement)) throw new Error(`missing ${fw} editor`);
    (editor as unknown as { __probeMounted?: boolean }).__probeMounted = true;
  }, framework);
  await selectWord(page, framework, "Ledger");
  const selectionText = await page.evaluate(() => document.getSelection()?.toString() ?? "");
  let previousKeyword: string | null = null;
  let previousWasLight: boolean | null = null;
  for (const theme of ALL_THEMES) {
    await setTheme(page, theme);
    await settle(page);
    const { contrasts, panel } = await roleContrasts(page, framework);
    const worst = SYNTAX_ROLES.reduce(
      (min, role) => (contrasts[role] < contrasts[min] ? role : min),
      "comment" as SyntaxRole,
    );
    check(
      `${browserName} ${framework} ${theme}: every syntax role meets AA on the actual panel`,
      SYNTAX_ROLES.every((role) => contrasts[role] >= 4.5),
      `panel=${panel} worst=${worst} at ${contrasts[worst]?.toFixed(2)}`,
    );
    const refs = await roleColors(page);
    const spans = await styledSpans(page, framework);
    const keyword = spans.find((span) => span.text.trim() === "const");
    const identity = await page.evaluate((fw) => {
      const editor = document.querySelector(
        `[data-framework="${fw}"] [data-part='main-editor'] .cm-editor`,
      );
      return (
        editor !== null &&
        (editor as unknown as { __probeMounted?: boolean }).__probeMounted === true &&
        editor.hasAttribute("data-probe-id")
      );
    }, framework);
    const docStill = await readEditor(page, framework);
    const isLight = LIGHT_THEMES.has(theme);
    check(
      `${browserName} ${framework} ${theme}: theme switch restyles the same mounted editor`,
      identity && docStill.doc === DOC && docStill.marked &&
        Boolean(keyword) && keyword?.color === refs["keyword"],
      `keyword=${keyword?.color} identity=${identity} doc=${docStill.doc === DOC}`,
    );
    if (previousKeyword !== null) {
      if (isLight !== previousWasLight) {
        check(
          `${browserName} ${framework} ${theme}: the palette base actually changed with the theme class`,
          keyword?.color !== previousKeyword,
          `now=${keyword?.color} was=${previousKeyword}`,
        );
      } else {
        check(
          `${browserName} ${framework} ${theme}: roles stay stable within the same base palette`,
          keyword?.color === previousKeyword,
          `now=${keyword?.color} was=${previousKeyword}`,
        );
      }
    }
    previousKeyword = keyword?.color ?? null;
    previousWasLight = isLight;
    check(
      `${browserName} ${framework} ${theme}: selection survives the theme switch`,
      (await page.evaluate(() => document.getSelection()?.toString() ?? "")) === selectionText,
      `selection=${JSON.stringify(selectionText)}`,
    );
  }

  // The light themes select the light primitives through their role-level
  // overrides: iceberg's keyword must be exactly the light primitive.
  await setTheme(page, "iceberg");
  await settle(page);
  const icebergRefs = await roleColors(page);
  check(
    `${browserName} ${framework} iceberg keyword role equals the light primitive`,
    icebergRefs["keyword"] === hexToRgbString(LIGHT_KEYWORD),
    `color=${icebergRefs["keyword"]} expected=${hexToRgbString(LIGHT_KEYWORD)}`,
  );
  check(
    `${browserName} ${framework} iceberg and eclipse palettes are genuinely different bases`,
    SYNTAX_ROLES.every((role) => icebergRefs[role] !== eclipseRefs[role]),
    JSON.stringify(SYNTAX_ROLES.filter((role) => icebergRefs[role] === eclipseRefs[role])),
  );
  await setTheme(page, "eclipse");
  await settle(page);
  const eclipseAgain = await styledSpans(page, framework);
  const eclipseKeyword = eclipseAgain.find((span) => span.text.trim() === "const");
  check(
    `${browserName} ${framework} returning to eclipse restores the original token colours`,
    Boolean(eclipseKeyword) && eclipseKeyword?.color === eclipseRefs["keyword"],
    `color=${eclipseKeyword?.color} expected=${eclipseRefs["keyword"]}`,
  );

  // 12. g18.023: forced colours keep the editor legible — text stays painted
  //     and mounted, and the presentation recovers after the mode ends.
  await page.emulateMedia({ forcedColors: "active" });
  await settle(page);
  const forced = await page.evaluate((fw) => {
    const content = document.querySelector(
      `[data-framework="${fw}"] [data-part='main-editor'] .cm-content`,
    );
    const editor = document.querySelector(
      `[data-framework="${fw}"] [data-part='main-editor'] .cm-editor`,
    );
    if (!(content instanceof HTMLElement) || !(editor instanceof HTMLElement)) {
      return { present: false, mounted: false, color: "", visible: false };
    }
    const style = getComputedStyle(content);
    return {
      present: true,
      mounted: (editor as unknown as { __probeMounted?: boolean }).__probeMounted === true,
      color: style.color,
      visible: style.visibility === "visible" && style.display !== "none",
    };
  }, framework);
  check(
    `${browserName} ${framework} forced colours keep the mounted editor legible`,
    forced.present && forced.mounted && forced.visible && forced.color !== "rgba(0, 0, 0, 0)",
    JSON.stringify(forced),
  );
  await page.emulateMedia({ forcedColors: "none" });
  await settle(page);
  const afterForced = await styledSpans(page, framework);
  const afterForcedKeyword = afterForced.find((span) => span.text.trim() === "const");
  check(
    `${browserName} ${framework} presentation recovers after forced colours end`,
    Boolean(afterForcedKeyword) && afterForcedKeyword?.color === eclipseRefs["keyword"],
    `color=${afterForcedKeyword?.color} expected=${eclipseRefs["keyword"]}`,
  );
}

try {
  for (const [browserName, browserType] of engines) {
    let browser: Browser | undefined;
    try {
      browser = await browserType.launch({ headless: true });
      const page = await browser.newPage({ viewport: { width: 960, height: 720 } });
      await page.goto(url, { waitUntil: "networkidle" });
      await page.waitForSelector('[data-framework="svelte"] [data-part="main-editor"] .poodle-code-editor');
      await page.waitForSelector('[data-framework="react"] [data-part="main-editor"] .poodle-code-editor');
      await runFramework(page, "svelte", browserName);
      await runFramework(page, "react", browserName);
    } finally {
      await browser?.close();
    }
  }
} finally {
  child.kill();
  await child.exited;
}

if (failures > 0) {
  console.error(`\n${failures} check(s) failed`);
  process.exit(1);
}

console.log("\nall CodeEditor language-registry checks passed");
