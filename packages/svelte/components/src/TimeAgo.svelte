<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import Tooltip from "./Tooltip.svelte";

  export let datetime: Date | string | number;
  export let live = true;
  export let interval = 30_000;
  export let ariaLabel: string | null = null;
  export let short = true;
  export let tooltipFormat: "full" | "date" | "datetime" = "datetime";
  export let timezone: string | null = null;

  let now = Date.now();
  let timer: ReturnType<typeof setInterval> | null = null;

  $: timestamp = toTimestamp(datetime);
  $: relativeText = formatRelative(timestamp, now, short);
  $: absoluteText = formatAbsolute(timestamp, tooltipFormat, timezone);

  function toTimestamp(dt: Date | string | number): number {
    if (dt instanceof Date) return dt.getTime();
    if (typeof dt === "number") return dt;
    return new Date(dt).getTime();
  }

  function formatRelative(ts: number, currentNow: number, shortFormat: boolean): string {
    const diff = currentNow - ts;
    const absDiff = Math.abs(diff);
    const isFuture = diff < 0;
    const prefix = isFuture ? "in " : "";
    const suffix = isFuture ? "" : " ago";

    if (absDiff < 5_000) return shortFormat ? "now" : "just now";
    if (absDiff < 60_000) {
      const seconds = Math.floor(absDiff / 1_000);
      return shortFormat
        ? `${prefix}${seconds}s${suffix}`
        : `${prefix}${seconds} second${seconds === 1 ? "" : "s"}${suffix}`;
    }
    if (absDiff < 3_600_000) {
      const minutes = Math.floor(absDiff / 60_000);
      return shortFormat
        ? `${prefix}${minutes}m${suffix}`
        : `${prefix}${minutes} minute${minutes === 1 ? "" : "s"}${suffix}`;
    }
    if (absDiff < 86_400_000) {
      const hours = Math.floor(absDiff / 3_600_000);
      return shortFormat
        ? `${prefix}${hours}h${suffix}`
        : `${prefix}${hours} hour${hours === 1 ? "" : "s"}${suffix}`;
    }
    if (absDiff < 2_592_000_000) {
      const days = Math.floor(absDiff / 86_400_000);
      if (!shortFormat && !isFuture && days === 1) return "yesterday";
      return shortFormat
        ? `${prefix}${days}d${suffix}`
        : `${prefix}${days} day${days === 1 ? "" : "s"}${suffix}`;
    }
    if (absDiff < 31_536_000_000) {
      const months = Math.floor(absDiff / 2_592_000_000);
      return shortFormat
        ? `${prefix}${months}mo${suffix}`
        : `${prefix}${months} month${months === 1 ? "" : "s"}${suffix}`;
    }
    const years = Math.floor(absDiff / 31_536_000_000);
    return shortFormat
      ? `${prefix}${years}y${suffix}`
      : `${prefix}${years} year${years === 1 ? "" : "s"}${suffix}`;
  }

  function formatAbsolute(
    ts: number,
    format: "full" | "date" | "datetime",
    tz: string | null,
  ): string {
    const d = new Date(ts);
    const timeZoneOption = tz ? { timeZone: tz } : {};

    try {
      switch (format) {
        case "date":
          return d.toLocaleDateString(undefined, {
            year: "numeric",
            month: "long",
            day: "numeric",
            ...timeZoneOption,
          });
        case "full":
          return d.toLocaleString(undefined, {
            year: "numeric",
            month: "long",
            day: "numeric",
            hour: "2-digit",
            minute: "2-digit",
            second: "2-digit",
            timeZoneName: "short",
            ...timeZoneOption,
          });
        case "datetime":
        default:
          return d.toLocaleString(undefined, {
            year: "numeric",
            month: "long",
            day: "numeric",
            hour: "2-digit",
            minute: "2-digit",
            ...timeZoneOption,
          });
      }
    } catch {
      return formatAbsolute(ts, format, null);
    }
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

<Tooltip content={absoluteText}>
  <time
    class="poodle-time-ago"
    datetime={new Date(timestamp).toISOString()}
    aria-label={ariaLabel ?? `${relativeText} (${absoluteText})`}
  >
    {relativeText}
  </time>
</Tooltip>

<style>
  .poodle-time-ago {
    color: var(--poodle-color-text-secondary);
    --poodle-time-ago-underline: color-mix(in srgb, currentColor 32%, transparent);
    --poodle-time-ago-underline-hover: color-mix(in srgb, currentColor 48%, transparent);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    font-variant-numeric: tabular-nums;
    text-decoration-line: underline;
    text-decoration-style: dotted;
    text-decoration-color: var(--poodle-time-ago-underline);
    text-underline-offset: 0.14em;
    cursor: help;
    transition:
      color 120ms ease,
      text-decoration-color 120ms ease;
  }

  .poodle-time-ago:hover,
  .poodle-time-ago:focus-visible {
    color: var(--poodle-color-text-primary);
    text-decoration-color: var(--poodle-time-ago-underline-hover);
  }
</style>
