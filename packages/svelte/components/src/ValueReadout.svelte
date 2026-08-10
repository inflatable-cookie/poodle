<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/value-readout.css";
  import {
    formatAudioValue, valueReadoutVisualState,
    type AudioValueFormat, type AudioValueLaw,
  } from "@inflatable-cookie/poodle-core";
  import ValueVisual from "./audio/ValueVisual.svelte";

  let {
    value = 0, min = 0, max = 1, law = { type: "linear" },
    format = { type: "number", decimals: 2 }, disabled = false, ariaLabel = null,
  }: {
    value?: number; min?: number; max?: number; law?: AudioValueLaw;
    format?: AudioValueFormat; disabled?: boolean; ariaLabel?: string | null;
  } = $props();

  const visualState = $derived(valueReadoutVisualState(value, min, max, law, !disabled));
  const text = $derived(formatAudioValue(value, format));
</script>

<output class="poodle-value-readout" aria-label={ariaLabel ?? undefined} data-disabled={disabled} data-scope="value-readout" data-part="root">
  <ValueVisual {visualState} {text} kind="readout" />
</output>
