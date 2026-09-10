<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/rich-text.css";
  import { RICH_TEXT_STANDARD_FEATURES } from "@inflatable-cookie/poodle-core";
  import type {
    ProseMirrorDocumentJSON,
    RichTextFeature,
  } from "@inflatable-cookie/poodle-core";
  import { onDestroy, onMount } from "svelte";

  import {
    assertAdmittedFeatures,
    assertValidRichTextDocument,
    createRichTextSchema,
    renderRichTextDocument,
  } from "./rich-text-engine";

  /**
   * Read-only rich-text rendering over the same ProseMirror document,
   * feature configuration, schema, and tokens as the editor. No
   * contenteditable node, history, selection state, or editor instance.
   * Reached only through `./rich-text`.
   */
  interface Props {
    value: ProseMirrorDocumentJSON;
    features?: readonly RichTextFeature[];
    ariaLabel?: string | null;
  }

  let { value, features = RICH_TEXT_STANDARD_FEATURES, ariaLabel = null }: Props = $props();

  let contentElement: HTMLDivElement | null = $state(null);

  function render(): void {
    // Fail closed before any output: validation precedes rendering, so
    // invalid documents never produce partial renderer content. This never
    // creates an editor and never touches browser globals.
    const nextValue = value;
    const nextFeatures = features;
    assertAdmittedFeatures(nextFeatures);
    assertValidRichTextDocument(createRichTextSchema(nextFeatures), nextValue);
    const target = contentElement;
    if (!target) return;
    renderRichTextDocument(target, nextValue, nextFeatures);
  }

  onMount(() => {
    render();
  });

  onDestroy(() => {
    contentElement = null;
  });

  $effect(() => {
    // Prop-driven re-renders: the same validation, the same serializer.
    const nextValue = value;
    const nextFeatures = features;
    const target = contentElement;
    if (!target) return;
    renderRichTextDocument(target, nextValue, nextFeatures);
  });
</script>

<div
  class="poodle-rich-text-renderer"
  role={ariaLabel ? "region" : undefined}
  aria-label={ariaLabel ?? undefined}
>
  <div
    bind:this={contentElement}
    class="poodle-rich-text-renderer__content"
    data-poodle-rich-text-rendered=""
  ></div>
</div>
