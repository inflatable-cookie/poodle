# EmbedInput

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `EmbedInput`
- Layer: `composites`
- Summary: a text area input that accepts URLs or embed codes, automatically
  detects the embed provider (YouTube, Vimeo, generic URL, iframe), and
  surfaces the parsed result with a status indicator showing provider name
  and success/error feedback
- In scope: URL and embed code input via TextInput (rows=3), debounced parsing,
  provider detection (YouTube, Vimeo, generic URL, iframe embed code),
  provider restriction, error and success status display, parsed result
  output via two-way binding
- Out of scope: file upload, custom provider plugins

## 2. Anatomy

```text
[Root .embed-input]  <div>
  ├── [TextInput]  TextInput primitive (rows=3)
  └── [Status .embed-input__status]  <div>
        ├── [Error .embed-input__error]  <span> (when error)
        └── [Success]  (when parsed)
              ├── [ProviderPill]  Pill (tone="success", sizeRole="chrome")
              └── [SuccessText .embed-input__success]  <span>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | flex column container | gap |
| TextInput | yes | TextInput primitive for URL/embed code input | delegates to TextInput contract |
| Status | yes | flex row showing parse result or error | min-height, font-size, gap |
| Error | conditional | error message text (when `error` is set) | text-danger color |
| ProviderPill | conditional | Pill showing detected provider name (when `parsed` is set) | delegates to Pill contract (tone="success", sizeRole="chrome") |
| SuccessText | conditional | "Embed detected" confirmation text (when `parsed` is set) | text-success color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string` | `"embed-input"` | no | id attribute for the TextInput |
| `value` | `string` | `""` | no | current input text; supports two-way binding |
| `parsed` | `ParsedEmbed \| null` | `null` | no | parsed embed result; supports two-way binding — updated after parsing |
| `placeholder` | `string` | `"Paste a URL or embed code..."` | no | placeholder text for the TextInput |
| `parseDebounce` | `number` | `300` | no | debounce delay in milliseconds before parsing |
| `providers` | `string[]` | `[]` | no | allowed provider names; empty array means all providers allowed |
| `disabled` | `boolean` | `false` | no | disables the TextInput input |
| `error` | `string \| null` | `null` | no | external error message; supports two-way binding |
| `resolveParseState` | `((value: string, providers: string[]) => EmbedParseState) \| undefined` | `undefined` | no | optional custom parse resolver; defaults to the built-in `resolveEmbedParseState` helper |

### Exported Helper Surface

Poodle also exports generic embed helpers from the same composite package:

- `parseEmbed(input, options)`
- `detectParsedEmbed(input)`
- `resolveEmbedParseState(input, providers)`
- `renderEmbed(parsed)`
- `lookupMeta(parsed)`
- `getThumbnailUrl(parsed, quality?)`
- `getProviderAccent(provider)`

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

type EmbedParseState = {
  parsed: ParsedEmbed | null;
  error: string | null;
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
| valid `http://` or `https://` URL (no whitespace) | `provider="generic"`, `id=input`, `originalUrl=input` |
| `<iframe ... src="...">` embed code | `provider="generic"`, `id=src or raw input`, `originalUrl=src when present`, `originalEmbed=input`, `width`/`height` parsed when numeric |
| empty or whitespace only | `parsed=null` |
| non-matching text | `parsed=null` |

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
| empty | value is empty or whitespace | status area is empty (no error or success shown) |
| parsing | value changed, debounce timer running | no dedicated loading surface; parsing remains silent until a result is available |
| success | valid embed detected | Pill showing provider name + "Embed detected" text |
| error | provider not allowed or external error set | red error message text in status area |
| disabled | `disabled=true` | TextInput is disabled; parsing still operates on current value |

### Component States

- `parseTimer` (internal): setTimeout handle for debounced parsing
- `parsed` (bindable): result of last successful parse
- `error` (bindable): current error message

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `parse` | after debounced parsing completes | `{ parsed: ParsedEmbed \| null, error: string \| null }` | fires on every parse attempt, including failures |
| `change` | input value changes | `{ value: string }` | fires immediately on every input change (before debounce) |

## 6. Accessibility

### Semantics

- TextInput: delegates to TextInput primitive accessibility
- Status messages: visible text only (no `aria-live` region); error is
  visually distinguished by color
- ProviderPill: decorative indicator; delegates to Pill contract

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | focuses the TextInput (standard form navigation) |
| (typing) | triggers debounced parsing after `parseDebounce` ms |

### Focus And Announcement

- Focus is managed by the TextInput primitive
- No custom focus management in this composite

## 7. Layout

### Sizing

- Root: flex column, gap `0.25rem` (4px)
- Status: flex row, gap `0.375rem` (6px), min-height `1.25rem` (20px),
  font-size `0.75rem` (12px)
- TextInput: 3 rows

### Composition

- Composes: `TextInput` and `Pill` from `@poodle/svelte`
- Parent expectations: form fields (often wrapped in Field), embed editing UIs
- Child expectations: none (self-contained inputs)
- Resizing rules: fills parent width; TextInput height determined by rows prop

## 8. Token Usage — Exact Values

### Root `.embed-input`

| Property | Value |
|----------|-------|
| display | `flex` |
| flex-direction | `column` |
| gap | `0.25rem` |

### Status `.embed-input__status`

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| gap | `0.375rem` |
| min-height | `1.25rem` |
| font-size | `0.75rem` |

### Error `.embed-input__error`

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-text-danger, #ef4444)` |

### SuccessText `.embed-input__success`

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-text-success, #22c55e)` |

### Composed Primitives

| Part | Delegates To |
|------|-------------|
| TextInput | TextInput contract (foundation), `rows=3` |
| ProviderPill | Pill contract (foundation), `tone="success"`, `sizeRole="chrome"` |

### Light Theme Overrides

None.

## 9. Svelte Notes

- Uses `createEventDispatcher` for `parse` and `change` events
- Composes `TextInput` and `Pill` from `@poodle/svelte`
- Debounce uses `setTimeout`/`clearTimeout` with configurable delay
- Parsing is routed through the exported `resolveEmbedParseState` helper in
  `embed-input.ts` rather than being inline; the helper calls
  `detectParsedEmbed` for pattern matching, then applies provider restriction
- callers can override parsing with `resolveParseState` when they need a richer
  provider/parser contract while keeping the same Poodle UI shell
- TextInput receives the `id` prop directly
- TextInput `on:valueChange` event drives input handling — value is extracted
  from `event.detail.value`
- Pill uses `sizeRole="chrome"` (not `size="xs"`)

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::embed_input`
- TextInput and Pill are composed from GPUI primitives
- GPUI consumes the same public `parsed` / `error` contract as Svelte
- The Rust composites contract should expose a parser helper for deriving
  `parsed` and provider-restriction `error` from `value` and `providers`
  without re-implementing the detection rules

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] event names and payloads match
- [ ] ParsedEmbed type is identical
- [ ] provider detection produces same results for same inputs
- [ ] provider restriction logic matches

### Tier 2: Visual Parity

- [ ] status area layout matches (gap, min-height, font-size)
- [ ] error and success colors match
- [ ] Pill appearance matches (tone="success", sizeRole="chrome")

### Tier 3: Implementation Freedom

- [ ] debounce mechanism may differ
- [ ] rendering internals stay internal

## 12. Specimen Definitions

### URL Or Embed Code Input

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| URL or embed code input | default props, `bind:value`, `bind:parsed`, `placeholder="Paste a YouTube URL, Vimeo link, or embed code..."` | TextInput with placeholder; status area shows parse result when URL is entered |

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
