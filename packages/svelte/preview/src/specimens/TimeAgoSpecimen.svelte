<script lang="ts">
  import { TimeAgo } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const now = Date.now();
  const twoMinutesAgo = new Date(now - 2 * 60 * 1000);
  const threeHoursAgo = new Date(now - 3 * 60 * 60 * 1000);
  const twoDaysAgo = new Date(now - 2 * 24 * 60 * 60 * 1000);
  const inFiveMinutes = new Date(now + 5 * 60 * 1000);
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Recent timestamps">
    <div class="poodle-specimen__row">
      <span class="poodle-specimen__label">2 minutes ago:</span>
      <TimeAgo datetime={twoMinutesAgo} />
    </div>
    <div class="poodle-specimen__row">
      <span class="poodle-specimen__label">3 hours ago:</span>
      <TimeAgo datetime={threeHoursAgo} />
    </div>
    <div class="poodle-specimen__row">
      <span class="poodle-specimen__label">2 days ago:</span>
      <TimeAgo datetime={twoDaysAgo} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Future timestamp">
    <div class="poodle-specimen__row">
      <span class="poodle-specimen__label">In 5 minutes:</span>
      <TimeAgo datetime={inFiveMinutes} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Long format">
    <div class="poodle-specimen__row">
      <span class="poodle-specimen__label">2 minutes ago:</span>
      <TimeAgo datetime={twoMinutesAgo} short={false} />
    </div>
    <div class="poodle-specimen__row">
      <span class="poodle-specimen__label">2 days ago:</span>
      <TimeAgo datetime={twoDaysAgo} short={false} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Static (live updates off)">
    <TimeAgo datetime={twoMinutesAgo} live={false} />
  </SpecimenGroup>

  <SpecimenGroup label="Inherited typography">
    <p class="poodle-specimen__inline-copy">
      Activity finished
      <TimeAgo datetime={twoMinutesAgo} typography="inherit" />
      in running prose.
    </p>
  </SpecimenGroup>

  <SpecimenGroup label="From ISO string">
    <TimeAgo datetime="2026-03-14T00:00:00Z" />
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__row {
    display: flex;
    align-items: baseline;
    gap: 0.5rem;
  }

  .poodle-specimen__label {
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
    min-width: 8rem;
  }

  .poodle-specimen__inline-copy {
    font-size: 1rem;
    line-height: 1.6;
    color: var(--poodle-color-text-primary);
  }
</style>
