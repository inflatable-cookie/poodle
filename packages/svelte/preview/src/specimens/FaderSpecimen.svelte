<script lang="ts">
  import { Fader, ValueReadout } from "@inflatable-cookie/poodle-svelte";
  let gain = $state(0);
</script>
<div class="poodle-fader-specimen">
  <Fader bind:value={gain} min={-60} max={12} detents={[0]} detentSnap={0.03} defaultValue={0} keyboardStep={1} format={{ type: "db" }} ariaLabel="Channel gain" />
  <Fader value={0.65} orientation="horizontal" detents={[0.5]} ariaLabel="Horizontal fader" />
  <ValueReadout value={gain} min={-60} max={12} format={{ type: "db" }} ariaLabel="Channel gain value" />
</div>
<style>.poodle-fader-specimen { display: flex; align-items: center; gap: 2rem; }</style>
