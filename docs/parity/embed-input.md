<!-- parity consv=gap gpui=6 jetstream=7 specimen=gap -->
# Parity: EmbedInput

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/embed-input.md`
- Svelte (authoritative): `packages/svelte/components/src/EmbedInput.svelte` + `packages/svelte/components/src/embed-input.ts`
- GPUI: `packages/gpui/components/src/composites/embed_input.rs`
- Jetstream: `packages/jetstream/components/src/embed_input.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/EmbedInputSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/embed_input_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/embed_input.rs`

## Contract ↔ Svelte

Svelte and its `.ts` engine carry surface the contract does not document. Svelte authoritative.

- `embed-input.ts` supports an `audioboom` provider (`embed-input.ts:52-60`) and an `embedType` field (`"video"|"audio"|"generic"`) on `ParsedEmbed` — neither in contract `ParsedEmbed` type §3 nor the detection-rules table. **Fix: add `audioboom` + `embedType` to contract.**
- Svelte component adds `size`/`sizeRole`/`density` props (`EmbedInput.svelte:18-20`) — not in contract prop table §3. **Fix: add to contract.**
- Class prefix drift: implementation uses `.poodle-embed-input*` (`EmbedInput.svelte:72,86`); contract anatomy table uses `.embed-input*`. **Fix: reconcile contract to `.poodle-` prefix (matches other components).**
- All 11 contract props (id/value/parsed/placeholder/parseDebounce/providers/disabled/error/onParse/onValueChange/resolveParseState), anatomy (TextInput rows=3 + status + error + ProviderPill + success text), states (empty/parsing-silent/success/error/disabled), callbacks, and the no-aria-live a11y rule all match.

## GPUI gap (vs Svelte + contract)

- [ ] TextInput is a hand-rolled static `div` "textarea" (`embed_input.rs:68-87`), not the real TextInput primitive — no input semantics, no debounced parse, no `onValueChange`.
- [ ] Error and success share one status text node — `unwrap_or("Embed detected")` at `embed_input.rs:124` conflates the error span and success span; both use a single `status_color` (`:109`). Contract requires `text-danger` for error, `text-success` for success.
- [ ] ProviderPill uses `PillSize::Sm` (`embed_input.rs:111-119`); contract specifies Pill `sizeRole="chrome"`, not a fixed size.
- [ ] No `onParse`/`onValueChange` callbacks, no `parseDebounce` (spec pre-resolves parse state; no interaction).
- [ ] Hardcoded px dimensions: `.min_h(px(72.0))` `:75`, `.px(px(12.0))` `:77`, `.py(px(8.0))` `:78`, root `.gap(px(6.0))` `:98` (contract root gap `0.25rem`=4px), status `.px(px(4.0))` `:105`, status `.gap(px(6.0))` `:108`, `.min_h(px(20.0))` `:104` — resolve from space tokens.
- accepted: no ARIA (gpui has no accessibility API). Note contract itself requires no aria-live, so only TextInput-delegated semantics are lost.

## Jetstream gap (vs Svelte + contract)

- [ ] TextInput is a hand-rolled static `div` with one `label` child (`embed_input.rs:37-44`), not the real TextInput primitive — no input semantics, no debounce, no callbacks.
- [ ] ProviderPill is FAKED as a styled `label` (`embed_input.rs:60-66`, bg `color.background.subtle` + padding + rounded) — contract requires the real Pill primitive (tone="success", sizeRole="chrome"). Forbidden hand-styled fake per CLAUDE.md.
- [ ] Error and success both use single `status_color` (`embed_input.rs:67-75`) — no `text-danger`/`text-success` split.
- [ ] Hardcoded disabled opacity `.opacity(0.5)` at `embed_input.rs:47` — contract/CLAUDE.md require `disabled_opacity_token()` (GPUI does this correctly).
- [ ] Status gap `rem_to_px(0.25)` at `embed_input.rs:55` — contract status gap is `0.375rem`; root gap (`:25`) is correct.
- [ ] No `onParse`/`onValueChange` callbacks, no `parseDebounce`.
- [ ] Literal rem dimensions via `rem_to_px(0.8125|0.75|0.375|0.125|0.25|4.5)` at `embed_input.rs:17-25` — resolve from named space/typography tokens rather than magic rem constants.
- accepted: no ARIA / no real input model (Jetstream has no text-input primitive here).

## Specimen parity

- Svelte covers: Supported-providers table, Detection matrix (4 samples), live input, Field wrapper, Restricted providers, Parsed-result dump, plus size + density snippets.
- GPUI covers: Supported-providers table, Detection matrix (4 `.with_detected_parse`), live input placeholder, Field wrapper, Restricted providers, Preset states (incl. custom parsed). Broad. — missing: nothing major vs Svelte (specimen is rich, though it leaks many hardcoded px + a `Menlo` font literal in the harness).
- Jetstream covers: With URL (detected), Empty (2 groups). — missing: **detection matrix, Field wrapper, restricted providers, provider table** (weakest coverage).

## Notes

- Both Rust targets collapse error/success into a single status color — the one functional parity break shared by both. Jetstream additionally fakes the Pill (CLAUDE.md "no fakes" violation) and hardcodes disabled opacity.
- The `.ts` parse engine is the real authority for detection rules (`youtu.be`, `youtube.com/watch|embed`, `vimeo`, `audioboom`, `<iframe>`, generic URL); the contract's detection table omits `audioboom` and the `embedType` field.
- consv=gap driven by undocumented Svelte surface (audioboom/embedType/size/sizeRole/density) + class-prefix drift.
