# formatDisplayDate

> **Implementation note**: utility function contract — no Svelte component expected. These are pure JS/TS formatting helpers imported directly.

Status: contract
Updated: 2026-04-08

## 1. Purpose

- Component name: `formatDisplayDate`, `formatDisplayDateTime`
- Layer: `foundation`
- Summary: presentational timestamp formatters for common UI display copy
- In scope: compact date-only and datetime labels for tables, detail panels,
  and admin metadata views
- Out of scope: calendar-specific ISO parsing, relative time rendering,
  date-range wording, or locale/timezone application policy

## 2. Signatures

```ts
formatDisplayDate(value: Date | string | number | null | undefined, locale?: string): string
formatDisplayDateTime(value: Date | string | number | null | undefined, locale?: string): string
```

## 3. Behavior

- accept `Date`, ISO-like strings, timestamps, `null`, and `undefined`
- invalid or missing values return `""`
- `formatDisplayDate` uses the platform locale date presentation
- `formatDisplayDateTime` uses the platform locale date+time presentation

## 4. Example

```ts
import { formatDisplayDate, formatDisplayDateTime } from "@poodle/svelte-primitives";

formatDisplayDate("2026-04-08T10:15:00Z");
formatDisplayDateTime("2026-04-08T10:15:00Z");
```

## 5. Accessibility

- accessibility-neutral utilities with no DOM output
- intended to support readable date labels in visible UI copy
