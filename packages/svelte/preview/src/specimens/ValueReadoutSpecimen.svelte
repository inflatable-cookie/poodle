<script lang="ts">
  import { ValueReadout } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  const examples = [
    ["Number", 12.345, { type: "number", decimals: 2 }],
    ["dB", -12.4, { type: "db", decimals: 1 }],
    ["Hz / kHz", 12_500, { type: "hz", decimals: 2 }],
    ["Percent", 0.625, { type: "percent", decimals: 1 }],
    ["Ratio", 4, { type: "ratio", decimals: 2 }],
    ["Milliseconds", 1250, { type: "milliseconds", decimals: 2 }],
    ["Note name", 60, { type: "note" }],
    ["Semitones", -7, { type: "semitones", decimals: 1 }],
  ] as const;
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  {#each examples as [label, value, format]}
    <SpecimenGroup label="{label}"><ValueReadout {value} min={-20_000} max={20_000} {format} ariaLabel={label} /></SpecimenGroup>
  {/each}
  <SpecimenGroup label="Negative / boundary / disabled"><div class="row"><ValueReadout value={-1} min={-1} max={1} /><ValueReadout value={1} min={-1} max={1} /><ValueReadout value={0} disabled ariaLabel="Disabled readout" /></div></SpecimenGroup>
</div>
  {#snippet sizes(size)}<ValueReadout value={-12.4} format={{ type: "db" }} {size} ariaLabel={`Readout ${size} size`} />{/snippet}
  {#snippet densities(density)}<ValueReadout value={-12.4} format={{ type: "db" }} {density} ariaLabel={`Readout ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page { display: grid; gap: 1rem; }
.row { display: flex; gap: .75rem; flex-wrap: wrap; }</style>
