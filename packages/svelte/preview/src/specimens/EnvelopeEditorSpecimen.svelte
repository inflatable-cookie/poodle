<script lang="ts">
  import type { EnvelopePoint } from "@inflatable-cookie/poodle-core";
  import { EnvelopeEditor } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  const adsr: EnvelopePoint[] = [{ id: "start", x: 0, y: 0, curve: 0.35 }, { id: "attack", x: 0.18, y: 1, curve: -0.25 }, { id: "decay", x: 0.48, y: 0.62, curve: 0.2 }, { id: "sustain", x: 0.82, y: 0.62, curve: 0 }, { id: "end", x: 1, y: 0, curve: 0 }];
  const curved: EnvelopePoint[] = [{ id: "a", x: 0, y: 0, curve: 0.7 }, { id: "b", x: 0.5, y: 1, curve: -0.7 }, { id: "c", x: 1, y: 0, curve: 0 }];
  const flat: EnvelopePoint[] = [{ id: "a", x: 0, y: 0.5, curve: 0.8 }, { id: "b", x: 1, y: 0.5, curve: 0 }];
  let editable = $state(structuredClone(adsr));
  const snap = (point: { x: number; y: number }) => ({ x: Math.round(point.x * 20) / 20, y: Math.round(point.y * 20) / 20 });
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  <SpecimenGroup label="ADSR-like default"><EnvelopeEditor bind:points={editable} ariaLabel="ADSR envelope" /></SpecimenGroup>
  <SpecimenGroup label="Positive / negative curves"><EnvelopeEditor points={curved} ariaLabel="Curved envelope" /></SpecimenGroup>
  <SpecimenGroup label="Selected / dragging, add / remove"><p>Drag points; double-click the surface to add and a point to remove.</p><EnvelopeEditor points={structuredClone(adsr)} ariaLabel="Editable envelope" /></SpecimenGroup>
  <SpecimenGroup label="Snapped movement"><EnvelopeEditor points={structuredClone(adsr)} snapPoint={snap} ariaLabel="Snapped envelope" /></SpecimenGroup>
  <SpecimenGroup label="Keyboard and curve nudges"><p>Focus a point; use arrows, Shift+arrows, and Page Up/Down.</p><EnvelopeEditor points={structuredClone(adsr)} step={0.05} ariaLabel="Keyboard envelope" /></SpecimenGroup>
  <SpecimenGroup label="Flat-segment regression"><EnvelopeEditor points={flat} ariaLabel="Flat envelope" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><EnvelopeEditor points={adsr} disabled ariaLabel="Disabled envelope" /></SpecimenGroup>
</div>
  {#snippet sizes(size)}<EnvelopeEditor points={adsr} {size} ariaLabel={`Envelope ${size} size`} />{/snippet}
  {#snippet densities(density)}<EnvelopeEditor points={adsr} {density} ariaLabel={`Envelope ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page { display: grid; gap: 1.5rem; }
h3, p { margin: 0; }
p { color: var(--poodle-color-text-secondary); font-size: .75rem; }</style>
