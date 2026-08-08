<script lang="ts">
  import { Meter } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout showSizes={true} showDensities={false}>
  {#snippet children()}
    <div class="poodle-specimen">
      <SpecimenGroup label="Default (50%)">
        <Meter value={50} ariaLabel="Storage usage" />
      </SpecimenGroup>

      <SpecimenGroup label="With thresholds">
        <Meter value={82} low={25} high={75} optimum={50} ariaLabel="CPU usage" />
        <p>82% — above high threshold</p>
      </SpecimenGroup>

      <SpecimenGroup label="Low value (optimal range)">
        <Meter value={30} low={25} high={75} optimum={50} ariaLabel="Memory usage" />
        <p>30% — within normal range</p>
      </SpecimenGroup>

      <SpecimenGroup label="Custom range (0–500)">
        <Meter value={350} min={0} max={500} ariaLabel="API calls" />
        <p>350 / 500 API calls used</p>
      </SpecimenGroup>

      <SpecimenGroup label="Ring">
        <div class="poodle-meter-specimen__row">
          <Meter shape="ring" value={38} ariaLabel="Context used" />
          <Meter shape="ring" value={86} high={80} ariaLabel="Context used, above warn threshold" />
          <Meter shape="ring" value={64} showValue size="xl" ariaLabel="Context used" />
        </div>
        <p>38% · 86% (above high) · 64% with readout</p>
      </SpecimenGroup>

      <SpecimenGroup label="Ring sizes">
        <div class="poodle-meter-specimen__row">
          <Meter shape="ring" value={60} size="xs" ariaLabel="Ring xs" />
          <Meter shape="ring" value={60} size="sm" ariaLabel="Ring sm" />
          <Meter shape="ring" value={60} size="md" ariaLabel="Ring md" />
          <Meter shape="ring" value={60} size="lg" ariaLabel="Ring lg" />
          <Meter shape="ring" value={60} size="xl" ariaLabel="Ring xl" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Ring tones">
        <div class="poodle-meter-specimen__row">
          <Meter shape="ring" value={60} tone="success" ariaLabel="Success tone" />
          <Meter shape="ring" value={60} tone="accent" ariaLabel="Accent tone" />
          <Meter shape="ring" value={60} tone="warning" ariaLabel="Warning tone" />
          <Meter shape="ring" value={60} tone="danger" ariaLabel="Danger tone" />
          <Meter shape="ring" value={60} tone="neutral" ariaLabel="Neutral tone" />
        </div>
      </SpecimenGroup>
    </div>
  {/snippet}

  {#snippet sizes(size)}
    <div class="poodle-meter-specimen__variant">
      <Meter value={50} ariaLabel={`Storage usage at ${size}`} {size} />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 20rem;
  }

  .poodle-meter-specimen__row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .poodle-meter-specimen__variant {
    width: min(100%, 20rem);
  }
</style>
