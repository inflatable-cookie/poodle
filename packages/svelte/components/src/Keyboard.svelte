<script lang="ts">
  import { untrack } from "svelte";
  import "@inflatable-cookie/poodle-core/styles/keyboard.css";
  import {
    createKeyboardContext, formatAudioValue, keyboardHitTest, keyboardTransition,
    keyboardVelocityAtPoint, keyboardVisualState, type KeyboardContext,
    type KeyboardEffect, type KeyboardOrientation,
  } from "@inflatable-cookie/poodle-core";
  import KeyboardVisual from "./audio/KeyboardVisual.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    size?: ControlSize | null; sizeRole?: SemanticControlSizeRole; density?: ControlDensity | null;
    firstNote?: number; lastNote?: number; orientation?: KeyboardOrientation; octaveShift?: number;
    computerBaseNote?: number; computerKeyMap?: Record<string, number>; externalHeldNotes?: number[];
    disabled?: boolean; ariaLabel?: string | null;
    onNoteOn?: (note: number, velocity: number) => void; onNoteOff?: (note: number) => void;
  }
  let { size = null, sizeRole = "control", density = null, firstNote = 48, lastNote = 72,
    orientation = "horizontal", octaveShift = 0, computerBaseNote = 60, computerKeyMap,
    externalHeldNotes = [], disabled = false, ariaLabel = "Keyboard", onNoteOn, onNoteOff }: Props = $props();
  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  let root: HTMLDivElement;
  let machine = $state(createKeyboardContext());
  let activePointer: number | null = null;
  let synced: { firstNote: number; lastNote: number; octaveShift: number; disabled: boolean } | null = null;
  const context = $derived<KeyboardContext>(createKeyboardContext({ ...machine, firstNote, lastNote, orientation, octaveShift, computerBaseNote, computerKeyMap: computerKeyMap ?? machine.computerKeyMap, externalHeldNotes, disabled }));
  const visualState = $derived(keyboardVisualState(context));

  function run(effects: KeyboardEffect[]) { for (const effect of effects) effect.type === "noteOn" ? onNoteOn?.(effect.note, effect.velocity) : onNoteOff?.(effect.note); }
  function send(event: Parameters<typeof keyboardTransition>[1]) { const result = keyboardTransition(context, event); machine = result.context; run(result.effects); }
  $effect(() => {
    const next = { firstNote, lastNote, octaveShift, disabled };
    if (!synced) { synced = next; return; }
    const prior = synced;
    if (next.firstNote === prior.firstNote && next.lastNote === prior.lastNote && next.octaveShift === prior.octaveShift && next.disabled === prior.disabled) return;
    untrack(() => {
      let current = context; const effects: KeyboardEffect[] = [];
      if (next.firstNote !== prior.firstNote || next.lastNote !== prior.lastNote) { const result = keyboardTransition(current, { type: "SET_RANGE", firstNote: next.firstNote, lastNote: next.lastNote }); current = result.context; effects.push(...result.effects); }
      if (next.octaveShift !== prior.octaveShift) { const result = keyboardTransition(current, { type: "SET_OCTAVE_SHIFT", value: next.octaveShift }); current = result.context; effects.push(...result.effects); }
      if (next.disabled !== prior.disabled) { const result = keyboardTransition(current, { type: "SET_DISABLED", value: next.disabled }); current = result.context; effects.push(...result.effects); }
      machine = current; synced = next; if (next.disabled) activePointer = null; run(effects);
    });
  });
  function pointerDown(event: PointerEvent) {
    if (event.button !== 0 || disabled) return;
    const note = keyboardHitTest(context, { x: event.clientX, y: event.clientY }, root.getBoundingClientRect());
    if (note === null) return;
    event.preventDefault(); activePointer = event.pointerId; root.setPointerCapture(event.pointerId);
    send({ type: "PRESS", inputId: `pointer:${event.pointerId}`, note, velocity: keyboardVelocityAtPoint({ x: event.clientX, y: event.clientY }, root.getBoundingClientRect(), orientation) });
  }
  function pointerEnd(event: PointerEvent) { if (activePointer === event.pointerId) { activePointer = null; send({ type: "RELEASE", inputId: `pointer:${event.pointerId}` }); } }
  function keydown(event: KeyboardEvent) { if (event.key.toLowerCase() in context.computerKeyMap) { event.preventDefault(); send({ type: "COMPUTER_KEY_DOWN", key: event.key, repeat: event.repeat }); } }
  function keyup(event: KeyboardEvent) { if (event.key.toLowerCase() in context.computerKeyMap) { event.preventDefault(); send({ type: "COMPUTER_KEY_UP", key: event.key }); } }
  function noteKey(event: KeyboardEvent, note: number) {
    if (event.key === "ArrowRight" || event.key === "ArrowUp" || event.key === "ArrowLeft" || event.key === "ArrowDown") { event.preventDefault(); send({ type: "MOVE_FOCUS", direction: event.key === "ArrowRight" || event.key === "ArrowUp" ? 1 : -1 }); }
    else if ((event.key === " " || event.key === "Enter") && !event.repeat) { event.preventDefault(); send({ type: "PRESS", inputId: `a11y:${note}`, note, velocity: 100 }); }
  }
</script>

<div bind:this={root} class="poodle-keyboard" role="toolbar" tabindex="-1" aria-orientation={orientation} aria-label={ariaLabel ?? undefined} aria-disabled={disabled} data-scope="keyboard" data-part="root" data-size={resolvedSize} data-density={resolvedDensity} data-orientation={orientation} onpointerdown={pointerDown} onpointerup={pointerEnd} onpointercancel={pointerEnd} onkeydown={keydown} onkeyup={keyup}>
  <KeyboardVisual {visualState} />
  {#each visualState.keys as key (key.note)}
    <button class="poodle-keyboard__key-control" type="button" disabled={disabled} aria-label={formatAudioValue(key.note, { type: "note" })} aria-pressed={key.held || key.externallyHeld} tabindex={key.focused || (visualState.keys[0]?.note === key.note && visualState.keys.every((candidate) => !candidate.focused)) ? 0 : -1} onfocus={() => send({ type: "FOCUS_NOTE", note: key.note })} onkeydown={(event) => noteKey(event, key.note)} onkeyup={(event) => { if (event.key === " " || event.key === "Enter") send({ type: "RELEASE", inputId: `a11y:${key.note}` }); }}></button>
  {/each}
</div>
