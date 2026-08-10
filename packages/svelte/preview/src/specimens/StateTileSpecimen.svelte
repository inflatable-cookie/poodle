<script lang="ts">
  import { StateTile } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="States">
    <div class="poodle-specimen__grid">
      <StateTile label="Queued jobs" value="14" />
      <StateTile label="Success rate" value="99.8%" trend="up" trendLabel="Up 1.2%" />
      <StateTile label="Errors" value="3" trend="down" trendLabel="Down 4" />
      <StateTile label="Capacity" value="72%" trend="steady" hasSparkline>
        {#snippet sparkline()}<span aria-label="Stable over seven days">▁▂▂▃▂▃▃</span>{/snippet}
      </StateTile>
    </div>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen__grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(11rem, 1fr)); gap: 0.75rem; }
</style>
