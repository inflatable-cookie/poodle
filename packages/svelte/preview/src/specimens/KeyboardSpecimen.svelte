<script lang="ts">
  import { Keyboard } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  let held = $state<number[]>([60, 64, 67]);
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  <SpecimenGroup label="Horizontal input / local chord"><Keyboard externalHeldNotes={held} onNoteOn={(note) => held = [...new Set([...held, note])]} onNoteOff={(note) => held = held.filter((value) => value !== note)} ariaLabel="Playable keyboard" /></SpecimenGroup>
  <SpecimenGroup label="Vertical piano-roll gutter"><Keyboard orientation="vertical" firstNote={48} lastNote={60} externalHeldNotes={[52, 55]} ariaLabel="Pitch gutter" /></SpecimenGroup>
  <SpecimenGroup label="Velocity depth"><Keyboard firstNote={60} lastNote={72} ariaLabel="Velocity keyboard" /></SpecimenGroup>
  <SpecimenGroup label="Computer keys / octave shift"><Keyboard firstNote={60} lastNote={84} octaveShift={1} ariaLabel="Computer-key octave" /></SpecimenGroup>
  <SpecimenGroup label="External playback highlight"><Keyboard externalHeldNotes={[61, 65, 68]} ariaLabel="Host playback" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><Keyboard disabled externalHeldNotes={[60, 64]} ariaLabel="Disabled keyboard" /></SpecimenGroup>
</div>
  {#snippet sizes(size)}<Keyboard firstNote={60} lastNote={72} externalHeldNotes={[60, 64]} {size} ariaLabel={`Keyboard ${size} size`} />{/snippet}
  {#snippet densities(density)}<Keyboard firstNote={60} lastNote={72} externalHeldNotes={[60, 64]} {density} ariaLabel={`Keyboard ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page{display:grid;gap:1.5rem}section{display:grid;gap:.75rem}h3{margin:0;color:var(--poodle-color-text-secondary);font-size:.75rem}</style>
