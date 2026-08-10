<script lang="ts">
  import { Keyboard } from "@inflatable-cookie/poodle-svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  let held = $state<number[]>([60, 64, 67]);
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  <section><h3>Horizontal input / local chord</h3><Keyboard externalHeldNotes={held} onNoteOn={(note) => held = [...new Set([...held, note])]} onNoteOff={(note) => held = held.filter((value) => value !== note)} ariaLabel="Playable keyboard" /></section>
  <section><h3>Vertical piano-roll gutter</h3><Keyboard orientation="vertical" firstNote={48} lastNote={60} externalHeldNotes={[52, 55]} ariaLabel="Pitch gutter" /></section>
  <section><h3>Velocity depth</h3><Keyboard firstNote={60} lastNote={72} ariaLabel="Velocity keyboard" /></section>
  <section><h3>Computer keys / octave shift</h3><Keyboard firstNote={60} lastNote={84} octaveShift={1} ariaLabel="Computer-key octave" /></section>
  <section><h3>External playback highlight</h3><Keyboard externalHeldNotes={[61, 65, 68]} ariaLabel="Host playback" /></section>
  <section><h3>Disabled</h3><Keyboard disabled externalHeldNotes={[60, 64]} ariaLabel="Disabled keyboard" /></section>
</div>
  {#snippet sizes(size)}<Keyboard firstNote={60} lastNote={72} externalHeldNotes={[60, 64]} {size} ariaLabel={`Keyboard ${size} size`} />{/snippet}
  {#snippet densities(density)}<Keyboard firstNote={60} lastNote={72} externalHeldNotes={[60, 64]} {density} ariaLabel={`Keyboard ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page{display:grid;gap:1.5rem}section{display:grid;gap:.75rem}h3{margin:0;color:var(--poodle-color-text-secondary);font-size:.75rem}</style>
