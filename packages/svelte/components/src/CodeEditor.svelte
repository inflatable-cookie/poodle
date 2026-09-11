<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/code-editor.css";
  import {
    installInputModality,
    isCodeEditorValueAdmissible,
  } from "@inflatable-cookie/poodle-core";
  import type {
    CodeEditorChange,
    CodeEditorDiagnostic,
    CodeEditorLanguageId,
    CodeEditorLanguageRegistry,
    CodeEditorPerformanceMode,
    CodeEditorTabBehavior,
  } from "@inflatable-cookie/poodle-core";
  import type { ControlDensity } from "./types";
  import { onDestroy, onMount } from "svelte";

  import {
    assertAdmittedLanguage,
    createCodeEditorEngine,
  } from "./code-editor-engine";
  import type { CodeEditorActiveDiagnostic, CodeEditorEngine } from "./code-editor-engine";
  import { getUiPresentation } from "./presentation";

  /**
   * Web-admitted controlled code and plain-text editing surface over
   * CodeMirror 6. The engine stays private: no CodeMirror type crosses this
   * API, and the component is reached only through `./editor`.
   */
  interface Props {
    value: string;
    language?: CodeEditorLanguageId;
    languageRegistry?: CodeEditorLanguageRegistry | null;
    lineNumbers?: boolean;
    searchable?: boolean;
    diagnostics?: CodeEditorDiagnostic[];
    readOnly?: boolean;
    disabled?: boolean;
    placeholder?: string;
    ariaLabel?: string;
    wrapLines?: boolean;
    tabSize?: number;
    tabBehavior?: CodeEditorTabBehavior;
    performanceMode?: CodeEditorPerformanceMode;
    density?: ControlDensity | null;
    onChange?: ((change: CodeEditorChange) => void) | null;
  }

  let {
    value,
    language = "plain-text",
    languageRegistry = null,
    lineNumbers = true,
    searchable = true,
    diagnostics = [],
    readOnly = false,
    disabled = false,
    placeholder = "",
    ariaLabel = "Code editor",
    wrapLines = false,
    tabSize = 2,
    tabBehavior = "focus",
    performanceMode = "full",
    density = null,
    onChange = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  // `language` is validated against `languageRegistry` when the engine mounts
  // and on every language update; asserting here would capture only the
  // initial prop values.
  /** Imperative escape hatch: focus the editing surface. Documented as a method, not a prop. */
  export function focus(): void {
    (hostElement?.querySelector(".cm-content") as HTMLElement | null)?.focus();
  }

  let hostElement: HTMLDivElement | null = $state(null);
  let engine: CodeEditorEngine | null = null;
  let activeDiagnostic: CodeEditorActiveDiagnostic | null = $state(null);
  let messageId = `poodle-code-editor-${Math.random().toString(36).slice(2)}`;

  // Live prop snapshot. Reads of $props bindings stay current, so the
  // post-creation flush below picks up host updates that landed while the
  // engine was still loading.
  function currentOptions() {
    return {
      value,
      language,
      languageRegistry,
      lineNumbers,
      searchable,
      readOnly,
      disabled,
      placeholder,
      ariaLabel,
      wrapLines,
      tabSize,
      tabBehavior,
      performanceMode,
      diagnostics,
    };
  }

  onMount(() => {
    assertAdmittedLanguage(language, languageRegistry);
    installInputModality();
    if (!isCodeEditorValueAdmissible(value)) {
      console.warn(
        "code-editor: value exceeds the 2 MiB envelope; hosts must refuse larger sources before mounting",
      );
    }
    let cancelled = false;
    createCodeEditorEngine(
      hostElement as HTMLDivElement,
      currentOptions(),
      {
        onChange: (change) => onChange?.(change),
        onActiveDiagnostic: (active) => {
          activeDiagnostic = active;
        },
      },
    ).then((created) => {
      if (cancelled) {
        created.destroy();
        return;
      }
      engine = created;
      // Host updates may have landed while the engine was loading; the sync
      // effect possibly ran with no engine to receive them. Flush the live
      // props so the host value stays authoritative.
      void engine.update(currentOptions());
    });
    return () => {
      cancelled = true;
    };
  });

  onDestroy(() => {
    engine?.destroy();
    engine = null;
  });

  $effect(() => {
    // Read every prop unconditionally before touching `engine`: `engine` is
    // null until the async mount resolves, and an `engine?.update(...)`
    // short-circuit would subscribe to nothing, leaving this effect dead.
    const next = currentOptions();
    void engine?.update(next);
  });
</script>

<div
  class="poodle-code-editor"
  data-density={resolvedDensity}
  data-disabled={disabled || undefined}
  data-readonly={readOnly || undefined}
>
  <div
    bind:this={hostElement}
    class="poodle-code-editor__viewport"
    aria-describedby={activeDiagnostic ? messageId : undefined}
  ></div>
  {#if activeDiagnostic}
    {@const active = activeDiagnostic}
    <div
      id={messageId}
      class="poodle-code-editor__diagnostic-message"
      data-severity={active.diagnostic.severity}
      role="status"
    >
      {active.diagnostic.severity} {active.line}:{active.column} — {active.diagnostic.message}
    </div>
  {/if}
</div>
