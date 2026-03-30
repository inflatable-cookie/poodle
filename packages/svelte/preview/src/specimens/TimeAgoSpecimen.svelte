<script lang="ts">
  import { TimeAgo, Eyebrow } from "@poodle/svelte-primitives";

  const now = Date.now();
  const twoMinutesAgo = new Date(now - 2 * 60 * 1000);
  const threeHoursAgo = new Date(now - 3 * 60 * 60 * 1000);
  const twoDaysAgo = new Date(now - 2 * 24 * 60 * 60 * 1000);
  const inFiveMinutes = new Date(now + 5 * 60 * 1000);
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Recent timestamps</Eyebrow>
    <div class="specimen__row">
      <span class="specimen__label">2 minutes ago:</span>
      <TimeAgo datetime={twoMinutesAgo} />
    </div>
    <div class="specimen__row">
      <span class="specimen__label">3 hours ago:</span>
      <TimeAgo datetime={threeHoursAgo} />
    </div>
    <div class="specimen__row">
      <span class="specimen__label">2 days ago:</span>
      <TimeAgo datetime={twoDaysAgo} />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Future timestamp</Eyebrow>
    <div class="specimen__row">
      <span class="specimen__label">In 5 minutes:</span>
      <TimeAgo datetime={inFiveMinutes} />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Long format</Eyebrow>
    <div class="specimen__row">
      <span class="specimen__label">2 minutes ago:</span>
      <TimeAgo datetime={twoMinutesAgo} short={false} />
    </div>
    <div class="specimen__row">
      <span class="specimen__label">2 days ago:</span>
      <TimeAgo datetime={twoDaysAgo} short={false} />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Static (live updates off)</Eyebrow>
    <TimeAgo datetime={twoMinutesAgo} live={false} />
  </div>

  <div class="specimen__group">
    <Eyebrow>From ISO string</Eyebrow>
    <TimeAgo datetime="2026-03-14T00:00:00Z" />
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

  .specimen__row {
    display: flex;
    align-items: baseline;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
    min-width: 8rem;
  }
</style>
