<script lang="ts">
  import { onMount, onDestroy } from "svelte";

  export let datetime: Date | string | number;
  export let live = true;
  export let interval = 30_000;
  export let ariaLabel: string | null = null;

  let now = Date.now();
  let timer: ReturnType<typeof setInterval> | null = null;

  $: timestamp = toTimestamp(datetime);
  $: relativeText = formatRelative(timestamp, now);
  $: absoluteText = formatAbsolute(timestamp);

  function toTimestamp(dt: Date | string | number): number {
    if (dt instanceof Date) return dt.getTime();
    if (typeof dt === "number") return dt;
    return new Date(dt).getTime();
  }

  function formatRelative(ts: number, currentNow: number): string {
    const diff = currentNow - ts;
    const absDiff = Math.abs(diff);
    const isFuture = diff < 0;
    const prefix = isFuture ? "in " : "";
    const suffix = isFuture ? "" : " ago";

    if (absDiff < 5_000) return "just now";
    if (absDiff < 60_000) {
      const seconds = Math.floor(absDiff / 1_000);
      return `${prefix}${seconds}s${suffix}`;
    }
    if (absDiff < 3_600_000) {
      const minutes = Math.floor(absDiff / 60_000);
      return `${prefix}${minutes}m${suffix}`;
    }
    if (absDiff < 86_400_000) {
      const hours = Math.floor(absDiff / 3_600_000);
      return `${prefix}${hours}h${suffix}`;
    }
    if (absDiff < 2_592_000_000) {
      const days = Math.floor(absDiff / 86_400_000);
      return `${prefix}${days}d${suffix}`;
    }
    if (absDiff < 31_536_000_000) {
      const months = Math.floor(absDiff / 2_592_000_000);
      return `${prefix}${months}mo${suffix}`;
    }
    const years = Math.floor(absDiff / 31_536_000_000);
    return `${prefix}${years}y${suffix}`;
  }

  function formatAbsolute(ts: number): string {
    const d = new Date(ts);
    return d.toLocaleString(undefined, {
      year: "numeric",
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  onMount(() => {
    if (live) {
      timer = setInterval(() => {
        now = Date.now();
      }, interval);
    }
  });

  onDestroy(() => {
    if (timer !== null) {
      clearInterval(timer);
    }
  });
</script>

<time
  class="time-ago"
  datetime={new Date(timestamp).toISOString()}
  title={absoluteText}
  aria-label={ariaLabel ?? `${relativeText} (${absoluteText})`}
>
  {relativeText}
</time>

<style>
  .time-ago {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    font-variant-numeric: tabular-nums;
    cursor: default;
  }
</style>
