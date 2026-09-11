/**
 * Explicit CodeMirror language adapter for the CodeEditor, published only
 * through the `./editor/codemirror` subpath.
 *
 * This is the one supported way to construct a Poodle language registry: lazy
 * loaders typed to resolve a CodeMirror `LanguageSupport`. The registry handed
 * to the component stays opaque — arbitrary editor extensions, themes,
 * keymaps, plugins, and DOM hooks have no path through this subpath, and a
 * loader that resolves anything other than a language extension is refused
 * before it can reach an editor.
 *
 * Keep the substrate import type-only at the surface level: importing this
 * module must not eagerly load any grammar package. Consumers name their
 * grammars inside their own loaders.
 */

import type { LanguageSupport } from "@codemirror/language";
import {
  createCodeEditorLanguageRegistry as createOpaqueRegistry,
} from "@inflatable-cookie/poodle-core";
import type {
  CodeEditorLanguageId,
  CodeEditorLanguageRegistry,
} from "@inflatable-cookie/poodle-core";

/** Lazy loader resolving one CodeMirror language support instance. */
export type CodeEditorLanguageLoader = () => Promise<LanguageSupport>;

/** Registry construction input: id/loader pairs or a plain record. */
export type CodeEditorLanguageRegistryInput =
  | Iterable<readonly [CodeEditorLanguageId, CodeEditorLanguageLoader]>
  | Record<string, CodeEditorLanguageLoader>;
function isLanguageSupport(value: unknown): value is LanguageSupport {
  return typeof value === "object" && value !== null && "extension" in value;
}

function guarded(loader: CodeEditorLanguageLoader): () => Promise<unknown> {
  return async () => {
    const resolved = await loader();
    if (!isLanguageSupport(resolved)) {
      throw new Error(
        "code-editor: language loader must resolve a CodeMirror LanguageSupport.",
      );
    }
    return resolved;
  };
}

/**
 * Build an opaque CodeEditor language registry from consumer-owned lazy
 * CodeMirror language loaders. Shares the framework-neutral validation and
 * memoization semantics with every other Poodle adapter: empty ids,
 * non-function loaders, duplicate ids, and a built-in `plain-text` entry
 * throw here; unknown ids and rejected loads fail closed at load time.
 */
export function createCodeEditorLanguageRegistry(
  input: CodeEditorLanguageRegistryInput,
): CodeEditorLanguageRegistry {
  const pairs: [CodeEditorLanguageId, () => Promise<unknown>][] =
    Symbol.iterator in input
      ? [...input].map(
          ([language, loader]): [CodeEditorLanguageId, () => Promise<unknown>] => [
            language,
            guarded(loader),
          ],
        )
      : Object.entries(input).map(
          ([language, loader]): [CodeEditorLanguageId, () => Promise<unknown>] => [
            language,
            guarded(loader),
          ],
        );
  return createOpaqueRegistry(pairs);
}
