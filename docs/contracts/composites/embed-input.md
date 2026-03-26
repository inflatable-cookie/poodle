# EmbedInput

Status: seed contract
Updated: 2026-03-25

## 1. Purpose

- Component name: `EmbedInput`
- Layer: `composites`
- Summary: a text area input that accepts URLs or embed codes, automatically
  detects the embed provider (YouTube, Vimeo, generic URL, iframe), and
  surfaces the parsed result with a status indicator
- In scope: URL and embed code input via TextArea, debounced parsing,
  provider detection (YouTube, Vimeo, generic URL, iframe embed code),
  provider restriction, error and success status display, parsed result
  output
- Out of scope: embed preview/rendering (see EmbedPreview), file upload,
  custom provider plugins, oEmbed API calls

## 2. Anatomy

```text
[Root .embed-input]  <div>
  ├── [TextArea]  TextArea primitive (rows=3)
  └── [Status .embed-input__status]  <div>
        ├── [Error .embed-input__error]  <span> (when error)
        └── [Success]  (when parsed)
              ├── [ProviderPill]  Pill (tone="success", size="xs")
              └── [SuccessText .embed-input__success]  <span>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | flex column container | gap |
| TextArea | yes | TextArea primitive for input | delegates to TextArea contract |
| Status | yes | flex row showing parse result or error | min-height, font-size, gap |
| Error | conditional | error message text | text-danger color |
| ProviderPill | conditional | Pill showing detected provider name | delegates to Pill contract |
| SuccessText | conditional | "Embed detected" confirmation text | text-success color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string` | `""` | no | current input text; bind for two-way |
| `parsed` | `ParsedEmbed \| null` | `null` | no | parsed embed result; bind for two-way output |
| `placeholder` | `string` | `"Paste a URL or embed code..."` | no | placeholder text for the TextArea |
| `parseDebounce` | `number` | `300` | no | debounce delay in milliseconds before parsing |
| `providers` | `string[]` | `[]` | no | allowed provider names; empty array means all providers allowed |
| `disabled` | `boolean` | `false` | no | disables the TextArea input |
| `error` | `string \| null` | `null` | no | external error message; bind for two-way |

### Types

```ts
type ParsedEmbed = {
  provider: string;      // "youtube" | "vimeo" | "generic"
  id: string;            // provider-specific ID or URL
  originalUrl?: string;  // the original URL if applicable
  originalEmbed?: string; // the original iframe embed code if applicable
  width?: number;        // extracted width (if present in embed code)
  height?: number;       // extracted height (if present in embed code)
};
```

### Supported Detection Rules

Both runtimes are expected to resolve `parsed` using the same pattern set:

| Input Pattern | Result |
|---------------|--------|
| `https://youtu.be/{id}` | `provider="youtube"`, `id={id}`, `originalUrl=input` |
| `https://youtube.com/watch?v={id}` | `provider="youtube"`, `id={id}`, `originalUrl=input` |
| `https://youtube.com/embed/{id}` | `provider="youtube"`, `id={id}`, `originalUrl=input` |
| `https://vimeo.com/{digits}` | `provider="vimeo"`, `id={digits}`, `originalUrl=input` |
| valid `http://` or `https://` URL | `provider="generic"`, `id=input`, `originalUrl=input` |
| `<iframe ... src="...">` embed code | `provider="generic"`, `id=src or raw input`, `originalUrl=src when present`, `originalEmbed=input`, `width` / `height` parsed when numeric |

Provider restriction runs after detection. If `providers` is non-empty and the
detected provider is not listed, `parsed` resolves to `null` and `error`
becomes `Provider "{provider}" is not allowed`.

### Slots

None.

### Controlled And Uncontrolled

- `value` supports two-way binding (`bind:value`)
- `parsed` supports two-way binding (`bind:parsed`) — updated after parsing
- `error` supports two-way binding (`bind:error`) — set internally for
  provider restriction violations

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | value is empty or whitespace | status area is empty |
| parsing | value changed, debounce timer running | no dedicated loading surface or spinner; parsing remains silent until a parse result is available |
| success | valid embed detected | Pill showing provider name + "Embed detected" text |
| error | provider not allowed or external error | red error message text |

### Component States

- `parseTimer` (internal): setTimeout handle for debounced parsing
- `parsed` (bindable): result of last successful parse
- `error` (bindable): current error message
- GPUI may derive `parsed` / `error` from the current `value` via the shared
  contract helper instead of relying on renderer-local parsing logic

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `parse` | after debounced parsing completes | `{ parsed: ParsedEmbed \| null, error: string \| null }` | fires on every parse attempt, including failures |
| `change` | input value changes | `{ value: string }` | fires immediately on every input change (before debounce) |

## 6. Accessibility

### Semantics

- TextArea: delegates to TextArea primitive accessibility
- Status messages: visible text only (no live region); error is visually
  distinguished by color
- ProviderPill: decorative indicator

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | focuses the TextArea (standard form navigation) |
| (typing) | triggers debounced parsing |

### Focus And Announcement

- Focus is managed by the TextArea primitive
- No custom focus management in this composite

## 7. Layout

### Sizing

- Root: flex column, gap `0.25rem` (4px)
- Status: flex row, gap `0.375rem` (6px), min-height `1.25rem` (20px),
  font-size `0.75rem` (12px)
- TextArea: 3 rows

### Composition

- Parent expectations: form fields (often wrapped in Field), embed editing UIs
- Child expectations: TextArea primitive, Pill primitive
- Resizing rules: fills parent width; TextArea height determined by rows prop
- Loading guidance: `EmbedInput` does not use `Spinner`; provider parsing and
  validation feedback stay in the compact status row rather than switching to a
  loading-state shell

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| TextArea | (delegates to TextArea) | all TextArea tokens |
| ProviderPill | (delegates to Pill) | Pill tone="success", size="xs" tokens |
| Error | `--poodle-color-text-danger` | error text color (fallback #ef4444) |
| SuccessText | `--poodle-color-text-success` | success text color (fallback #22c55e) |

## 9. Runtime Notes

### Svelte

- Uses `createEventDispatcher` for `parse` and `change` events
- Composes `TextArea` and `Pill` from `@poodle/svelte-primitives`
- Debounce uses `setTimeout`/`clearTimeout` with configurable delay
- Parsing is routed through the exported `resolveEmbedParseState` helper rather
  than being embedded inline inside the component

### GPUI

- Expected crate/module surface: `poodle_gpui::composites::embed_input`
- TextArea and Pill are composed from GPUI primitives
- GPUI consumes the same public `parsed` / `error` contract as Svelte
- The Rust composites contract also exposes a parser helper for deriving
  `parsed` and provider-restriction `error` from `value` and `providers`
  when hosts want contract-backed detection without re-implementing the rules

## 10. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] event names and payloads match
- [ ] ParsedEmbed type is identical
- [ ] provider detection produces same results for same inputs
- [ ] provider restriction logic matches

### Tier 2: Visual Parity

- [ ] status area layout matches (gap, min-height, font-size)
- [ ] error and success colors match
- [ ] Pill appearance matches

### Tier 3: Implementation Freedom

- [ ] debounce mechanism may differ
- [ ] rendering internals stay internal

## 11. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Svelte still owns its debounce/event loop while GPUI is render-only and may pre-resolve parse state before rendering | event ownership differs by runtime, but the visible `parsed` / `error` contract and supported detection rules are aligned | accepted for now | extract a shared TS utility or generator path if runtime-level parser implementation drift becomes costly |

## 12. Specimen Definitions

### URL Or Embed Code Input

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| URL or embed code input | default props, `bind:value`, `bind:parsed`, `placeholder="Paste a YouTube URL, Vimeo link, or embed code..."` | TextArea with placeholder; status area shows parse result when URL is entered |

### With Field Wrapper

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With Field wrapper | wrapped in `Field label="Video embed"`, `placeholder="https://youtube.com/watch?v=..."` | labeled field containing the embed input |

### Restricted Providers

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Restricted providers | `providers={["youtube", "vimeo"]}`, `placeholder="Only YouTube and Vimeo allowed..."` | input that shows error for non-YouTube/Vimeo URLs |

### Detection Matrix

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Detection matrix | helper-driven examples covering YouTube short link, Vimeo link, iframe embed, and restricted generic URL | specimen shows the resolved `parsed` payload or provider restriction error for each canonical supported pattern |

## 13. Approval And Adoption Notes

- Contract status: `seed contract`
- Approvers: pending
- Downstream adopters: content editors, media embed forms, CMS embed fields
- Future follow-up: consider oEmbed API integration for richer metadata;
  consider adding a `validate` callback prop for custom validation;
  consider extracting provider detection into a shared utility
