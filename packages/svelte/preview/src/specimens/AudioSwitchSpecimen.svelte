<script lang="ts">
  import { AudioSwitch } from "@inflatable-cookie/poodle-svelte";
  let latch = $state(0);
  let momentary = $state(0);
  let mode = $state(1);
</script>

<div class="specimen">
  <label>Latch <AudioSwitch bind:state={latch} ariaLabel="Power" /></label>
  <label>Momentary <AudioSwitch mode="momentary" bind:state={momentary} lampOn={momentary > 0} ariaLabel="Trigger" /></label>
  <label>Multi <AudioSwitch mode="multi" bind:state={mode} stateCount={3} stateLabels={["Low", "Mid", "High"]} lampOn={mode === 2} ariaLabel="Range" /></label>
</div>
<style>.specimen { display: flex; gap: 1.5rem; } label { display: grid; gap: 0.5rem; color: var(--poodle-color-text-secondary); }</style>
