# formatFileSize

Status: contract
Updated: 2026-04-08

## 1. Purpose

- Component name: `formatFileSize`
- Layer: `foundation`
- Summary: a presentational byte-size formatter for UI copy
- In scope: compact byte-size labels for limits, metadata, subtitles, and
  validation messages
- Out of scope: file validation, file acceptance, upload orchestration, or
  backend storage policy

## 2. Signature

```ts
formatFileSize(bytes: number | null | undefined): string
```

## 3. Behavior

- `0` becomes `0 B`
- `null`, `undefined`, or `NaN` become `""`
- values under `1024` stay in bytes
- larger values scale to `KB`, `MB`, or `GB`
- values above bytes use one decimal place and trim trailing `.0`

## 4. Example

```ts
import { formatFileSize } from "@poodle/svelte-primitives";

formatFileSize(500); // "500 B"
formatFileSize(1024); // "1 KB"
formatFileSize(1536000); // "1.5 MB"
```

## 5. Accessibility

- accessibility-neutral utility with no DOM output
- intended to support concise, human-readable size copy in visible UI
