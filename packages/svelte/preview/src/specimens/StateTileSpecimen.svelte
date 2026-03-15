<script lang="ts">
  import { StateTile } from "@pug/svelte-composites";
  import { Eyebrow } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Basic tiles</Eyebrow>
    <div class="specimen__grid">
      <StateTile label="Components" value="85" />
      <StateTile label="Coverage" value="94%" />
      <StateTile label="Open issues" value="12" />
      <StateTile label="Build time" value="1.8s" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With trend indicators</Eyebrow>
    <div class="specimen__grid">
      <StateTile label="Active users" value="2,847" trend="up" trendLabel="+12.3%" />
      <StateTile label="Error rate" value="0.04%" trend="down" trendLabel="-8%" />
      <StateTile label="Latency" value="42ms" trend="flat" trendLabel="No change" />
      <StateTile label="Revenue" value="$14.2k" trend="up" trendLabel="+3.1%" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With sparklines</Eyebrow>
    <div class="specimen__grid">
      <StateTile
        label="Requests/min"
        value="1,204"
        trend="up"
        trendLabel="+5%"
        sparklineData={[800, 920, 850, 1100, 980, 1050, 1204]}
      />
      <StateTile
        label="CPU usage"
        value="62%"
        trend="down"
        trendLabel="-4%"
        sparklineData={[75, 72, 68, 70, 65, 63, 62]}
      />
      <StateTile
        label="Memory"
        value="4.2 GB"
        sparklineData={[3.8, 3.9, 4.0, 4.1, 4.0, 4.1, 4.2]}
      />
    </div>
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(10rem, 1fr));
    gap: 0.75rem;
  }
</style>
