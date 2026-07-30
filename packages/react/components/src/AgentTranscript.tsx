import { useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";

import "@poodle/styles/agent-transcript.css";

import {
  groupTranscriptItems,
  isPinnedToBottom,
  transcriptWindow,
  type TranscriptBlock,
} from "@poodle/headless";

import { AgentMessage } from "./AgentMessage";
import { AgentQuestionRecord } from "./AgentQuestionRecord";
import { ChangedFiles } from "./ChangedFiles";
import { EmptyState } from "./EmptyState";
import { Icon } from "./Icon";
import { Spinner } from "./Spinner";
import { ToolCallGroup } from "./ToolCallGroup";
import { resolveSemanticControlSize, resolveSupportingVisualSize, useUiPresentation } from "./presentation";
import type {
  ControlDensity,
  ControlSize,
  SemanticControlSizeRole,
  TranscriptItem,
} from "./types";

export interface AgentTranscriptProps {
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
  onToolRunToggle?: (id: string) => void;
  onToolCallToggle?: (id: string) => void;
  onChangedFilesToggle?: (id: string) => void;
  onOpenDiff?: (id: string) => void;
  onFileSelect?: (path: string) => void;
  onScrollStateChange?: (pinned: boolean) => void;
}

export function AgentTranscript({
  items = [],
  virtualized = true,
  estimatedBlockHeight = 120,
  overscan = 3,
  autoScroll = true,
  pinThreshold = 32,
  jumpLabel = "Jump to latest",
  ariaLabel = "Conversation",
  emptyLabel = "No messages yet",
  expandedToolRuns,
  expandedToolCalls,
  expandedChangedFiles,
  size = null,
  sizeRole = "control",
  density = null,
  onToolRunToggle,
  onToolCallToggle,
  onChangedFilesToggle,
  onOpenDiff,
  onFileSelect,
  onScrollStateChange,
}: AgentTranscriptProps) {
  const presentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(presentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? presentation.density;
  const glyphSize = resolveSupportingVisualSize(resolvedSize);

  /** Derived, not stored: a stored copy would drift from `items` on append. */
  const blocks = useMemo(() => groupTranscriptItems(items), [items]);
  const renderedBlocks = useMemo(() => blocks.filter((block) => block.kind !== "activity"), [blocks]);
  const activity = useMemo(
    () => [...items].reverse().find((item) => item.kind === "activity") ?? null,
    [items],
  );

  const viewportRef = useRef<HTMLDivElement | null>(null);
  const [pinned, setPinned] = useState(true);
  const [scrollTop, setScrollTop] = useState(0);
  const [viewportHeight, setViewportHeight] = useState(0);

  /**
   * Measured heights keyed by block id, not by position.
   *
   * Keying by index is wrong as soon as the window moves: the index a block had
   * when its observer was created is not the index it has now, so heights get
   * written against whichever block currently occupies that slot. Ids are
   * stable for the life of a block, so a measurement can only ever land on the
   * block it came from.
   */
  const [heightById, setHeightById] = useState<Record<string, number>>({});

  const [uncontrolledRuns, setUncontrolledRuns] = useState<string[]>([]);
  const [uncontrolledCalls, setUncontrolledCalls] = useState<string[]>([]);
  const [uncontrolledChanged, setUncontrolledChanged] = useState<string[]>([]);
  const openRuns = expandedToolRuns ?? uncontrolledRuns;
  const openCalls = expandedToolCalls ?? uncontrolledCalls;
  const openChanged = expandedChangedFiles ?? uncontrolledChanged;

  const isEmpty = items.length === 0;
  const showsJump = !pinned && !isEmpty;

  const windowRange = virtualized
    ? transcriptWindow(
        renderedBlocks.map((block) => heightById[block.id] ?? 0),
        estimatedBlockHeight,
        scrollTop,
        viewportHeight,
        overscan,
      )
    : null;

  const visibleBlocks = windowRange
    ? renderedBlocks.slice(windowRange.startIndex, windowRange.endIndex)
    : renderedBlocks;

  const handleScroll = () => {
    const element = viewportRef.current;
    if (!element) return;

    setScrollTop(element.scrollTop);
    setViewportHeight(element.clientHeight);

    const nextPinned = isPinnedToBottom(
      element.scrollTop,
      element.scrollHeight,
      element.clientHeight,
      pinThreshold,
    );

    if (nextPinned !== pinned) {
      setPinned(nextPinned);
      onScrollStateChange?.(nextPinned);
    }
  };

  const jumpToLatest = () => {
    const element = viewportRef.current;
    if (!element) return;
    element.scrollTop = element.scrollHeight;
    setPinned(true);
    onScrollStateChange?.(true);
  };

  /**
   * Following is applied after the append is laid out, not by clamping
   * `scrollTop` continuously. The distinction matters while a message streams:
   * its block grows every frame, and a continuous clamp fights the reader's own
   * wheel events instead of losing to them.
   */
  useLayoutEffect(() => {
    const element = viewportRef.current;
    if (!autoScroll || !pinned || !element) return;
    element.scrollTop = element.scrollHeight;
  }, [items.length, autoScroll, pinned]);

  /**
   * The viewport's own size.
   *
   * Without this the height stays 0 until the first scroll event, so the very
   * first window is one overscan tall and a transcript that opens without being
   * scrolled renders almost nothing.
   */
  useEffect(() => {
    const element = viewportRef.current;
    if (!element || typeof ResizeObserver === "undefined") return;

    const sync = () => {
      setViewportHeight(element.clientHeight);
      setScrollTop(element.scrollTop);
    };

    sync();
    const observer = new ResizeObserver(sync);
    observer.observe(element);
    return () => observer.disconnect();
  }, []);

  /**
   * Block measurement.
   *
   * Measurements are coalesced into one frame, never dropped. An earlier
   * version held a re-entrancy guard borrowed from the Tabs shed ladder and
   * discarded anything that arrived while it was up — but a `ResizeObserver`
   * does not fire again for a block whose size has not changed, so those
   * heights were lost for good. The window then sized itself from stale
   * estimates and stopped short of filling the viewport.
   *
   * No guard is needed here, unlike in Tabs. There, measuring changed the thing
   * being measured. Here a block's height depends on its content and width,
   * never on which window is rendered, so heights → window → more blocks →
   * heights converges: each block measures once and then stays put.
   */
  const pendingHeights = useRef(new Map<string, number>());
  const flushQueued = useRef(false);

  const queueHeight = useCallback((id: string, height: number) => {
    pendingHeights.current.set(id, height);
    if (flushQueued.current) return;

    flushQueued.current = true;
    requestAnimationFrame(() => {
      flushQueued.current = false;
      const pending = pendingHeights.current;
      pendingHeights.current = new Map();

      setHeightById((current) => {
        let changed = false;
        const next = { ...current };
        for (const [key, value] of pending) {
          if (next[key] !== value) {
            next[key] = value;
            changed = true;
          }
        }
        return changed ? next : current;
      });
    });
  }, []);

  const measureRef = useCallback(
    (id: string) => (element: HTMLDivElement | null) => {
      if (!element || typeof ResizeObserver === "undefined") return;

      const record = () => {
        const height = element.getBoundingClientRect().height;
        if (height > 0) queueHeight(id, height);
      };

      // Measure immediately as well as on resize: a block that never changes
      // size after mount produces no further observer callbacks, and without a
      // first reading it would keep its estimate forever.
      record();

      const observer = new ResizeObserver(record);
      observer.observe(element);
      return () => observer.disconnect();
    },
    [queueHeight],
  );

  const toggleIn = (list: string[], id: string): string[] =>
    list.includes(id) ? list.filter((value) => value !== id) : [...list, id];

  const renderBlock = (block: TranscriptBlock) => {
    if (block.kind === "message") {
      return (
        <AgentMessage
          markdown={block.markdown}
          role={block.role}
          isStreaming={block.isStreaming ?? false}
          size={resolvedSize}
          density={resolvedDensity}
        />
      );
    }
    if (block.kind === "tool-run") {
      return (
        <ToolCallGroup
          id={block.id}
          calls={block.calls}
          expanded={openRuns.includes(block.id)}
          expandedCalls={openCalls}
          size={resolvedSize}
          density={resolvedDensity}
          onToggle={(id) => {
            if (expandedToolRuns === undefined) setUncontrolledRuns((list) => toggleIn(list, id));
            onToolRunToggle?.(id);
          }}
          onCallToggle={(id) => {
            if (expandedToolCalls === undefined) setUncontrolledCalls((list) => toggleIn(list, id));
            onToolCallToggle?.(id);
          }}
        />
      );
    }
    if (block.kind === "answered-question") {
      return (
        <AgentQuestionRecord
          question={block.question}
          answer={block.answer}
          size={resolvedSize}
          density={resolvedDensity}
        />
      );
    }
    if (block.kind === "changed-files") {
      return (
        <ChangedFiles
          id={block.id}
          files={block.files}
          expanded={openChanged.includes(block.id)}
          size={resolvedSize}
          density={resolvedDensity}
          onToggle={(id) => {
            if (expandedChangedFiles === undefined) setUncontrolledChanged((list) => toggleIn(list, id));
            onChangedFilesToggle?.(id);
          }}
          onOpenDiff={onOpenDiff}
          onFileSelect={onFileSelect}
        />
      );
    }
    return null;
  };

  return (
    <div
      className="poodle-agent-transcript"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-empty={isEmpty ? "true" : undefined}
      data-pinned={String(pinned)}
      data-virtualized={String(virtualized)}
    >
      {/* `log` is the role for append-only output, and `polite` means a
          finished response is announced without interrupting the reader. */}
      <div
        className="poodle-agent-transcript__viewport"
        role="log"
        aria-live="polite"
        aria-label={ariaLabel}
        ref={viewportRef}
        onScroll={handleScroll}
      >
        {isEmpty ? (
          <div className="poodle-agent-transcript__empty">
            <EmptyState title={emptyLabel} />
          </div>
        ) : windowRange ? (
          <div className="poodle-agent-transcript__runway" style={{ height: `${windowRange.totalHeight}px` }}>
            <div
              className="poodle-agent-transcript__slice"
              style={{ transform: `translateY(${windowRange.offsetY}px)` }}
            >
              {visibleBlocks.map((block) => (
                <div
                  key={block.id}
                  className="poodle-agent-transcript__block"
                  data-kind={block.kind}
                  ref={measureRef(block.id)}
                >
                  {renderBlock(block)}
                </div>
              ))}
            </div>
          </div>
        ) : (
          <div className="poodle-agent-transcript__blocks">
            {renderedBlocks.map((block) => (
              <div key={block.id} className="poodle-agent-transcript__block" data-kind={block.kind}>
                {renderBlock(block)}
              </div>
            ))}
          </div>
        )}

        {activity ? (
          <div className="poodle-agent-transcript__activity">
            <Spinner variant="dots" size={glyphSize} tone="current" />
            <span className="poodle-agent-transcript__activity-label">{activity.label}</span>
          </div>
        ) : null}
      </div>

      {showsJump ? (
        <button type="button" className="poodle-agent-transcript__jump" onClick={jumpToLatest}>
          <Icon name="arrow-down" size={glyphSize} />
          <span>{jumpLabel}</span>
        </button>
      ) : null}
    </div>
  );
}
