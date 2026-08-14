import { useEffect, useState } from "react";

import "@inflatable-cookie/poodle-core/styles/time-ago.css";

import { Tooltip } from "./Tooltip";

export interface TimeAgoProps {
  datetime: Date | string | number;
  live?: boolean;
  interval?: number;
  ariaLabel?: string | null;
  short?: boolean;
  futureFormat?: "in" | "from-now";
  futurePrefix?: string | null;
  pastPrefix?: string | null;
  typography?: "body" | "inherit";
  tooltipFormat?: "full" | "date" | "datetime";
  timezone?: string | null;
}

function toTimestamp(dt: Date | string | number): number {
  if (dt instanceof Date) return dt.getTime();
  if (typeof dt === "number") return dt;
  return new Date(dt).getTime();
}

function formatRelative(
  ts: number,
  currentNow: number,
  shortFormat: boolean,
  futureFormat: "in" | "from-now",
  futurePrefix: string | null,
  pastPrefix: string | null,
): string {
  const diff = currentNow - ts;
  const absDiff = Math.abs(diff);
  const isFuture = diff < 0;
  const tensePrefix = isFuture ? futurePrefix : pastPrefix;
  const tensePrefixText = tensePrefix ? `${tensePrefix} ` : "";
  const directionPrefix = isFuture && futureFormat === "in" ? "in " : "";
  const prefix = `${tensePrefixText}${directionPrefix}`;
  const suffix = isFuture ? (futureFormat === "from-now" ? " from now" : "") : " ago";

  if (absDiff < 5_000) return `${tensePrefixText}${shortFormat ? "now" : "just now"}`;
  if (absDiff < 60_000) {
    const seconds = Math.floor(absDiff / 1_000);
    return shortFormat ? `${prefix}${seconds}s${suffix}` : `${prefix}${seconds} second${seconds === 1 ? "" : "s"}${suffix}`;
  }
  if (absDiff < 3_600_000) {
    const minutes = Math.floor(absDiff / 60_000);
    return shortFormat ? `${prefix}${minutes}m${suffix}` : `${prefix}${minutes} minute${minutes === 1 ? "" : "s"}${suffix}`;
  }
  if (absDiff < 86_400_000) {
    const hours = Math.floor(absDiff / 3_600_000);
    return shortFormat ? `${prefix}${hours}h${suffix}` : `${prefix}${hours} hour${hours === 1 ? "" : "s"}${suffix}`;
  }
  if (absDiff < 2_592_000_000) {
    const days = Math.floor(absDiff / 86_400_000);
    if (!shortFormat && !isFuture && days === 1) return "yesterday";
    return shortFormat ? `${prefix}${days}d${suffix}` : `${prefix}${days} day${days === 1 ? "" : "s"}${suffix}`;
  }
  if (absDiff < 31_536_000_000) {
    const months = Math.floor(absDiff / 2_592_000_000);
    return shortFormat ? `${prefix}${months}mo${suffix}` : `${prefix}${months} month${months === 1 ? "" : "s"}${suffix}`;
  }
  const years = Math.floor(absDiff / 31_536_000_000);
  return shortFormat ? `${prefix}${years}y${suffix}` : `${prefix}${years} year${years === 1 ? "" : "s"}${suffix}`;
}

function formatAbsolute(ts: number, format: "full" | "date" | "datetime", tz: string | null): string {
  const d = new Date(ts);
  const timeZoneOption = tz ? { timeZone: tz } : {};
  try {
    switch (format) {
      case "date":
        return d.toLocaleDateString(undefined, { year: "numeric", month: "long", day: "numeric", ...timeZoneOption });
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

export function TimeAgo({
  datetime,
  live = true,
  interval = 30_000,
  ariaLabel = null,
  short = true,
  futureFormat = "in",
  futurePrefix = null,
  pastPrefix = null,
  typography = "body",
  tooltipFormat = "datetime",
  timezone = null,
}: TimeAgoProps) {
  const [now, setNow] = useState(() => Date.now());

  useEffect(() => {
    if (!live) return;
    setNow(Date.now());
    const timer = setInterval(() => setNow(Date.now()), interval);
    return () => clearInterval(timer);
  }, [live, interval]);

  const timestamp = toTimestamp(datetime);
  const relativeText = formatRelative(timestamp, now, short, futureFormat, futurePrefix, pastPrefix);
  const absoluteText = formatAbsolute(timestamp, tooltipFormat, timezone);

  return (
    <Tooltip content={absoluteText}>
      <time
        className="poodle-time-ago"
        data-typography={typography}
        dateTime={new Date(timestamp).toISOString()}
        aria-label={ariaLabel ?? `${relativeText} (${absoluteText})`}
      >
        {relativeText}
      </time>
    </Tooltip>
  );
}
