<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/audio-switch.css";
  import {
    audioSwitchTransition, audioSwitchVisualState, createAudioSwitchContext,
    type AudioSwitchContext, type AudioSwitchEffect, type AudioSwitchMode,
  } from "@inflatable-cookie/poodle-core";
  import AudioSwitchVisual from "./audio/AudioSwitchVisual.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    size?: ControlSize | null; sizeRole?: SemanticControlSizeRole; density?: ControlDensity | null;
    mode?: AudioSwitchMode; state?: number; stateCount?: number; lampOn?: boolean | null;
    stateLabels?: string[]; disabled?: boolean; ariaLabel?: string | null;
    onStateChange?: (state: number) => void; onStateCommit?: (state: number) => void;
  }

  let {
    size = null, sizeRole = "control", density = null,
    mode = "latch", state: currentState = $bindable(0), stateCount = 2, lampOn = null,
    stateLabels = [], disabled = false, ariaLabel = "Audio switch", onStateChange, onStateCommit,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  let machine = $state(createAudioSwitchContext());
  let activePointer: number | null = null;
  const normalizedStateCount = $derived(Math.max(Math.floor(Number.isFinite(stateCount) ? stateCount : 2), 2));
  const normalizedState = $derived(Math.min(Math.max(Math.round(Number.isFinite(currentState) ? currentState : 0), 0), normalizedStateCount - 1));
  const context = $derived<AudioSwitchContext>({ ...machine, mode, state: normalizedState, stateCount: normalizedStateCount, lampOn, disabled });
  const visualState = $derived(audioSwitchVisualState(context));
  const stateText = $derived(stateLabels[normalizedState] ?? `State ${normalizedState + 1} of ${normalizedStateCount}`);
  const accessibleLabel = $derived(mode === "multi" ? `${ariaLabel ?? "Audio switch"}, ${stateText}` : ariaLabel ?? "Audio switch");

  function runEffects(effects: AudioSwitchEffect[]): void {
    for (const effect of effects) {
      if (effect.type === "emitStateChange") { currentState = effect.state; onStateChange?.(effect.state); }
      else { currentState = effect.state; onStateCommit?.(effect.state); }
    }
  }

  function send(event: Parameters<typeof audioSwitchTransition>[1]): void {
    const result = audioSwitchTransition(context, event); machine = result.context; runEffects(result.effects);
  }

  function pointerDown(event: PointerEvent): void {
    if (event.button !== 0 || disabled) return;
    event.preventDefault(); activePointer = event.pointerId; event.currentTarget.setPointerCapture(event.pointerId); send({ type: "PRESS" });
  }

  function pointerUp(event: PointerEvent): void {
    if (activePointer !== event.pointerId) return;
    activePointer = null; send({ type: "RELEASE" });
  }

  function keydown(event: KeyboardEvent): void {
    if (event.key !== " " && event.key !== "Enter") return;
    event.preventDefault(); send({ type: "PRESS" });
  }

  function keyup(event: KeyboardEvent): void {
    if (event.key !== " " && event.key !== "Enter") return;
    event.preventDefault(); send({ type: "RELEASE" });
  }
</script>

<button
  type="button"
  class="poodle-audio-switch"
  aria-label={accessibleLabel}
  aria-pressed={mode === "multi" ? undefined : normalizedState > 0}
  disabled={disabled}
  data-scope="audio-switch"
  data-part="root"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-mode={mode}
  onpointerdown={pointerDown}
  onpointerup={pointerUp}
  onpointercancel={(event) => { if (activePointer === event.pointerId) { activePointer = null; send({ type: "CANCEL" }); } }}
  onmouseenter={() => send({ type: "HOVER", value: true })}
  onmouseleave={() => send({ type: "HOVER", value: false })}
  onfocus={() => send({ type: "FOCUS", value: true })}
  onblur={() => { send({ type: "FOCUS", value: false }); send({ type: "CANCEL" }); }}
  onkeydown={keydown}
  onkeyup={keyup}
>
  <AudioSwitchVisual {visualState} />
</button>
