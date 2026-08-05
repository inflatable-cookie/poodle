<script lang="ts">
  import "@poodle/styles/agent-transcript.css";
  import { tick } from "svelte";

  import {
    groupTranscriptItems,
    isPinnedToBottom,
    transcriptWindow,
    type TranscriptBlock,
  } from "@poodle/headless";

  import AgentMessage from "./AgentMessage.svelte";
  import AgentQuestionRecord from "./AgentQuestionRecord.svelte";
  import ChangedFiles from "./ChangedFiles.svelte";
  import EmptyState from "./EmptyState.svelte";
  import Icon from "./Icon.svelte";
  import Spinner from "./Spinner.svelte";
  import ToolCallGroup from "./ToolCallGroup.svelte";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation.ts";

  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    TranscriptItem,
  } from "./types.ts";

  interface Props {
    items?: TranscriptItem[];
    virtualized?: boolean;
    estimatedBlockHeight?: number;
    overscan?: number;
    autoScroll?: boolean;
    pinThreshold?: number;
    jumpLabel?: string;
    ariaLabel?: string;
    emptyLabel?: string;
    expandedToolRuns?: string[];
    expandedToolCalls?: string[];
    expandedChangedFiles?: string[];
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onToolRunToggle?: ((id: string) => void) | undefined;
    onToolCallToggle?: ((id: string) => void) | undefined;
    onChangedFilesToggle?: ((id: string) => void) | undefined;
    onOpenDiff?: ((id: string) => void) | undefined;
    onFileSelect?: ((path: string) => void) | undefined;
    onScrollStateChange?: ((pinned: boolean) => void) | undefined;
  }

  let {
    items = [],
    virtualized = true,
    estimatedBlockHeight = 120,
    overscan = 3,
    autoScroll = true,
    pinThreshold = 32,
    jumpLabel = "Jump to latest",
    ariaLabel = "Conversation",
    emptyLabel = "No messages yet",
    expandedToolRuns = $bindable<string[]>([]),
    expandedToolCalls = $bindable<string[]>([]),
    expandedChangedFiles = $bindable<string[]>([]),
    size = null,
    sizeRole = "control",
    density = null,
    onToolRunToggle = undefined,
    onToolCallToggle = undefined,
    onChangedFilesToggle = undefined,
    onOpenDiff = undefined,
    onFileSelect = undefined,
    onScrollStateChange = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const glyphSize = $derived(resolveSupportingVisualSize(resolvedSize));

  /** Derived, not stored: a stored copy would drift from `items` on append. */
  const blocks = $derived(groupTranscriptItems(items));
  const renderedBlocks = $derived(blocks.filter((block) => block.kind !== "activity"));
  const activity = $derived(
    [...items].reverse().find((item) => item.kind === "activity") ?? null,
  );

  let viewportElement = $state<HTMLDivElement | null>(null);
  let pinned = $state(true);
  let scrollTop = $state(0);
  let viewportHeight = $state(0);

  /**
   * Measured heights keyed by block id, not by position.
   *
   * Keying by index is wrong as soon as the window moves: the index a block had
   * when its observer was created is not the index it has now, so heights get
   * written against whichever block currently occupies that slot. Ids are
   * stable for the life of a block, so a measurement can only ever land on the
   * block it came from.
   */
  let heightById = $state<Record<string, number>>({});

  const isEmpty = $derived(items.length === 0);
  const showsJump = $derived(!pinned && !isEmpty);

  const windowRange = $derived(
    virtualized
      ? transcriptWindow(
          renderedBlocks.map((block) => heightById[block.id] ?? 0),
          estimatedBlockHeight,
          scrollTop,
          viewportHeight,
          overscan,
        )
      : null,
  );

  const visibleBlocks = $derived(
    windowRange ? renderedBlocks.slice(windowRange.startIndex, windowRange.endIndex) : renderedBlocks,
  );

  function handleScroll(): void {
    if (!viewportElement) return;
    scrollTop = viewportElement.scrollTop;
    viewportHeight = viewportElement.clientHeight;

    const nextPinned = isPinnedToBottom(
      viewportElement.scrollTop,
      viewportElement.scrollHeight,
      viewportElement.clientHeight,
      pinThreshold,
    );

    if (nextPinned !== pinned) {
      pinned = nextPinned;
      onScrollStateChange?.(nextPinned);
    }
  }

  function jumpToLatest(): void {
    if (!viewportElement) return;
    viewportElement.scrollTop = viewportElement.scrollHeight;
    pinned = true;
    onScrollStateChange?.(true);
  }

  /**
   * Following is applied after the append is laid out, not by clamping
   * `scrollTop` continuously. The distinction matters while a message streams:
   * its block grows every frame, and a continuous clamp fights the reader's own
   * wheel events instead of losing to them.
   */
  $effect(() => {
    // Touch the dependency so the effect re-runs when content changes.
    void items.length;
    if (!autoScroll || !pinned || !viewportElement) return;

    tick().then(() => {
      if (!viewportElement) return;
      viewportElement.scrollTop = viewportElement.scrollHeight;
    });
  });

  /**
   * Block measurement.
   *
   * Measurements are coalesced into one frame, never dropped. An earlier
   * version held a re-entrancy guard borrowed from the Tabs shed ladder and
   * discarded anything that arrived while it was up — but a `ResizeObserver`
   * does not fire again for a block whose size has not changed, so those
   * heights were lost for good. The window then sized itself from stale
   * estimates and stopped short of filling the viewport, leaving dead space
   * below the last block.
   *
   * No guard is needed here, unlike in Tabs. There, measuring changed the thing
   * being measured. Here a block's height depends on its content and width,
   * never on which window is rendered, so heights → window → more blocks →
   * heights converges: each block measures once and then stays put.
   */
  let pendingHeights = new Map<string, number>();
  let flushQueued = false;

  function queueHeight(id: string, height: number): void {
    pendingHeights.set(id, height);
    if (flushQueued) return;

    flushQueued = true;
    requestAnimationFrame(() => {
      flushQueued = false;
      const next = { ...heightById };
      let changed = false;

      for (const [key, value] of pendingHeights) {
        if (next[key] !== value) {
          next[key] = value;
          changed = true;
        }
      }
      pendingHeights.clear();

      if (changed) heightById = next;
    });
  }

  function measureBlock(element: HTMLElement, id: string) {
    if (typeof ResizeObserver === "undefined") return;

    const record = () => {
      const height = element.getBoundingClientRect().height;
      if (height > 0) queueHeight(id, height);
    };

    // Measure immediately as well as on resize: a block that never changes size
    // after mount produces no further observer callbacks, and without a first
    // reading it would keep its estimate forever.
    record();

    const observer = new ResizeObserver(record);
    observer.observe(element);
    return { destroy: () => observer.disconnect() };
  }

  /**
   * The viewport's own size.
   *
   * Without this the height stays 0 until the first scroll event, so the very
   * first window is one overscan tall and a transcript that opens without being
   * scrolled renders almost nothing.
   */
  $effect(() => {
    const element = viewportElement;
    if (!element || typeof ResizeObserver === "undefined") return;

    const sync = () => {
      viewportHeight = element.clientHeight;
      scrollTop = element.scrollTop;
    };

    sync();
    const observer = new ResizeObserver(sync);
    observer.observe(element);
    return () => observer.disconnect();
  });

  function toggleIn(list: string[], id: string): string[] {
    return list.includes(id) ? list.filter((value) => value !== id) : [...list, id];
  }
</script>

<div
  class="poodle-agent-transcript"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-empty={isEmpty ? "true" : undefined}
  data-pinned={String(pinned)}
  data-virtualized={String(virtualized)}
>
  <!-- `log` is the role for append-only output, and `polite` means a finished
       response is announced without interrupting whatever the reader is doing. -->
  <div
    class="poodle-agent-transcript__viewport"
    role="log"
    aria-live="polite"
    aria-label={ariaLabel}
    bind:this={viewportElement}
    onscroll={handleScroll}
  >
    {#if isEmpty}
      <div class="poodle-agent-transcript__empty">
        <EmptyState title={emptyLabel} />
      </div>
    {:else if windowRange}
      <div class="poodle-agent-transcript__runway" style={`height: ${windowRange.totalHeight}px`}>
        <div
          class="poodle-agent-transcript__slice"
          style={`transform: translateY(${windowRange.offsetY}px)`}
        >
          {#each visibleBlocks as block (block.id)}
            <div class="poodle-agent-transcript__block" data-kind={block.kind} use:measureBlock={block.id}>
              {@render blockContent(block)}
            </div>
          {/each}
        </div>
      </div>
    {:else}
      <div class="poodle-agent-transcript__blocks">
        {#each renderedBlocks as block (block.id)}
          <div class="poodle-agent-transcript__block" data-kind={block.kind}>
            {@render blockContent(block)}
          </div>
        {/each}
      </div>
    {/if}

    {#if activity}
      <div class="poodle-agent-transcript__activity">
        <Spinner variant="dots" size={glyphSize} tone="current" />
        <span class="poodle-agent-transcript__activity-label">{activity.label}</span>
      </div>
    {/if}
  </div>

  {#if showsJump}
    <button type="button" class="poodle-agent-transcript__jump" onclick={jumpToLatest}>
      <Icon name="arrow-down" size={glyphSize} />
      <span>{jumpLabel}</span>
    </button>
  {/if}
</div>

{#snippet blockContent(block: TranscriptBlock)}
  {#if block.kind === "message"}
    <AgentMessage
      markdown={block.markdown}
      role={block.role}
      isStreaming={block.isStreaming ?? false}
      size={resolvedSize}
      density={resolvedDensity}
    />
  {:else if block.kind === "tool-run"}
    <ToolCallGroup
      id={block.id}
      calls={block.calls}
      expanded={expandedToolRuns.includes(block.id)}
      expandedCalls={expandedToolCalls}
      size={resolvedSize}
      density={resolvedDensity}
      onToggle={(id) => {
        expandedToolRuns = toggleIn(expandedToolRuns, id);
        onToolRunToggle?.(id);
      }}
      onCallToggle={(id) => {
        expandedToolCalls = toggleIn(expandedToolCalls, id);
        onToolCallToggle?.(id);
      }}
    />
  {:else if block.kind === "answered-question"}
    <AgentQuestionRecord
      question={block.question}
      answer={block.answer}
      size={resolvedSize}
      density={resolvedDensity}
    />
  {:else if block.kind === "changed-files"}
    <ChangedFiles
      id={block.id}
      files={block.files}
      expanded={expandedChangedFiles.includes(block.id)}
      size={resolvedSize}
      density={resolvedDensity}
      onToggle={(id) => {
        expandedChangedFiles = toggleIn(expandedChangedFiles, id);
        onChangedFilesToggle?.(id);
      }}
      {onOpenDiff}
      {onFileSelect}
    />
  {/if}
{/snippet}
