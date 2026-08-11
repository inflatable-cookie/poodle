<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/mod-matrix-grid.css";
  import "@inflatable-cookie/poodle-core/styles/slider.css";
  import { createModMatrixContext, formatAudioValue, modMatrixTransition, modMatrixVisualState, type ModMatrixCell, type ModMatrixContext, type ModMatrixEffect, type ModMatrixHeader } from "@inflatable-cookie/poodle-core";
  import ModMatrixVisual from "./audio/ModMatrixVisual.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";
  interface Props { size?: ControlSize | null; sizeRole?: SemanticControlSizeRole; density?: ControlDensity | null; sources?: ModMatrixHeader[]; destinations?: ModMatrixHeader[]; cells?: ModMatrixCell[]; step?: number; disabled?: boolean; ariaLabel?: string | null; onCellChange?: (cell: ModMatrixCell) => void; onCellCommit?: (cell: ModMatrixCell) => void; onGestureBegin?: () => void; onGestureEnd?: () => void; }
  let { size = null, sizeRole = "control", density = null, sources = [], destinations = [], cells = $bindable([]), step = .01, disabled = false, ariaLabel = "Modulation matrix", onCellChange, onCellCommit, onGestureBegin, onGestureEnd }: Props = $props();
  const uiPresentation = getUiPresentation(); const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole)); const resolvedDensity = $derived(density ?? $uiPresentation.density);
  let root: HTMLDivElement;
  let machine = $state(createModMatrixContext());
  let dragPointer: number | null = null;
  let dragTarget: HTMLElement | null = null;
  const context = $derived<ModMatrixContext>(createModMatrixContext({ ...machine, sources, destinations, cells, step, disabled }));
  const visualState = $derived(modMatrixVisualState(context));
  function run(effects: ModMatrixEffect[], next: ModMatrixContext) { for (const effect of effects) { if (effect.type === "emitCellChange" || effect.type === "emitCellCommit") { cells = next.cells; effect.type === "emitCellChange" ? onCellChange?.(effect.cell) : onCellCommit?.(effect.cell); } else if (effect.type === "beginGesture") onGestureBegin?.(); else onGestureEnd?.(); } }
  function send(event: Parameters<typeof modMatrixTransition>[1]) { const result = modMatrixTransition(context, event); machine = result.context; run(result.effects, result.context); }
  function pointerDown(event: PointerEvent) {
    if (event.button !== 0 || disabled) return;
    const target = (event.target as Element).closest<HTMLElement>(".poodle-mod-matrix-grid-visual__cell");
    if (!target || !root.contains(target)) return;
    const row = sources.findIndex((source) => source.id === target.dataset.sourceId);
    const column = destinations.findIndex((destination) => destination.id === target.dataset.destinationId);
    if (row < 0 || column < 0) return;
    event.preventDefault(); dragPointer = event.pointerId; dragTarget = target; root.setPointerCapture(event.pointerId);
    const rect = target.getBoundingClientRect();
    send({ type: "DRAG_BEGIN", row, column, amountNorm: Math.min(Math.max((event.clientX - rect.left) / Math.max(rect.width, 1), 0), 1), fine: event.shiftKey });
    root.querySelectorAll<HTMLButtonElement>(".poodle-mod-matrix-grid__control")[row * destinations.length + column]?.focus();
  }
  function pointerMove(event: PointerEvent) {
    if (dragPointer !== event.pointerId) return;
    if (!dragTarget) return;
    const rect = dragTarget.getBoundingClientRect();
    send({ type: "DRAG_MOVE", amountNorm: Math.min(Math.max((event.clientX - rect.left) / Math.max(rect.width, 1), 0), 1), fine: event.shiftKey });
  }
  function pointerEnd(event: PointerEvent) { if (dragPointer === event.pointerId) { dragPointer = null; dragTarget = null; send({ type: "DRAG_END" }); } }
  function key(event: KeyboardEvent) { if (event.key.startsWith("Arrow")) { event.preventDefault(); send({ type: "MOVE_FOCUS", rows: event.key === "ArrowDown" ? 1 : event.key === "ArrowUp" ? -1 : 0, columns: event.key === "ArrowRight" ? 1 : event.key === "ArrowLeft" ? -1 : 0 }); } else if (event.key === "Home" || event.key === "End") { event.preventDefault(); send({ type: "BOUND_FOCUS", bound: event.ctrlKey ? (event.key === "Home" ? "grid-start" : "grid-end") : (event.key === "Home" ? "row-start" : "row-end") }); } else if (event.key === " ") { event.preventDefault(); send({ type: "TOGGLE_FOCUSED" }); } else if (event.key === "PageUp" || event.key === "PageDown") { event.preventDefault(); send({ type: "NUDGE_FOCUSED", direction: event.key === "PageUp" ? 1 : -1, fine: event.shiftKey }); } }
</script>
<div bind:this={root} class="poodle-mod-matrix-grid" role="group" aria-label={ariaLabel ?? undefined} aria-disabled={disabled} data-scope="mod-matrix-grid" data-part="root" data-size={resolvedSize} data-density={resolvedDensity} style={`--poodle-mod-columns:${destinations.length}`} onpointerdown={pointerDown} onpointermove={pointerMove} onpointerup={pointerEnd} onpointercancel={pointerEnd}>
  <ModMatrixVisual {visualState} />
  <div class="poodle-mod-matrix-grid__controls" role="grid" tabindex="-1" aria-label={ariaLabel ?? undefined} onkeydown={key}>
    {#each sources as source, row (source.id)}
      <div class="poodle-mod-matrix-grid__row" role="row">
        {#each visualState.cells.filter((cell) => cell.sourceId === source.id) as cell, column (`${cell.sourceId}:${cell.destinationId}`)}
          <button class="poodle-mod-matrix-grid__control" type="button" role="gridcell" disabled={disabled} aria-label={`${source.label} to ${destinations[column]?.label ?? cell.destinationId}, ${cell.enabled ? "enabled" : "disabled"}, ${formatAudioValue(cell.amount, { type: "number", decimals: 2 })}, range ${formatAudioValue(cell.parameters.min, { type: "number", decimals: 2 })} to ${formatAudioValue(cell.parameters.max, { type: "number", decimals: 2 })}`} aria-selected={cell.enabled} tabindex={cell.focused || (row === 0 && column === 0 && !visualState.focus) ? 0 : -1} onfocus={() => send({ type: "FOCUS_CELL", row, column })}></button>
        {/each}
      </div>
    {/each}
  </div>
</div>
