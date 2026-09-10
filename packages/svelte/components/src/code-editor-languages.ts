/**
 * Admitted language support for the CodeEditor engine.
 * TypeScript reuses the JavaScript language package in TypeScript parser mode;
 * TOML and shell use audited legacy stream parsers. There is no silent
 * fallback: unknown languages throw in `languageFor`.
 *
 * Dynamic `import()` is load-bearing here, not convenience: each admitted
 * language stays in its own distribution chunk so the base `./editor` entry
 * never pays for unrequested grammars. Specifiers are literals, so the
 * bundler still resolves and fails on every one at build time.
 */

import type { Extension } from "@codemirror/state";
import type { CodeEditorLanguage, CodeEditorPerformanceMode } from "@inflatable-cookie/poodle-core";

export async function languageFor(
  language: CodeEditorLanguage,
  performanceMode: CodeEditorPerformanceMode,
): Promise<Extension> {
  if (performanceMode === "plain" || language === "plain-text") return [];
  switch (language) {
    case "markdown": {
      const { markdown } = await import("@codemirror/lang-markdown");
      return markdown();
    }
    case "json": {
      const { json } = await import("@codemirror/lang-json");
      return json();
    }
    case "yaml": {
      const { yaml } = await import("@codemirror/lang-yaml");
      return yaml();
    }
    case "toml": {
      const { StreamLanguage } = await import("@codemirror/language");
      const { toml } = await import("@codemirror/legacy-modes/mode/toml");
      return StreamLanguage.define(toml);
    }
    case "javascript": {
      const { javascript } = await import("@codemirror/lang-javascript");
      return javascript();
    }
    case "typescript": {
      const { javascript } = await import("@codemirror/lang-javascript");
      return javascript({ typescript: true });
    }
    case "html": {
      const { html } = await import("@codemirror/lang-html");
      return html();
    }
    case "css": {
      const { css } = await import("@codemirror/lang-css");
      return css();
    }
    case "rust": {
      const { rust } = await import("@codemirror/lang-rust");
      return rust();
    }
    case "shell": {
      const { StreamLanguage } = await import("@codemirror/language");
      const { shell } = await import("@codemirror/legacy-modes/mode/shell");
      return StreamLanguage.define(shell);
    }
    default: {
      const exhaustive: never = language;
      throw new Error(`code-editor: unsupported language "${String(exhaustive)}".`);
    }
  }
}
