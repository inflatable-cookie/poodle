<!-- parity consv=fixed gpui=1 jetstream=1 specimen=gap --><!-- pass: both targets compose real TextInput(rows=3)+Pill, split danger/success status, token-resolved spacing; only preview-loop callbacks remain -->
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

`consv=fixed`. Undocumented Svelte/`.ts` surface and class-prefix drift reconciled.

- [x] FIXED `audioboom` provider (`embed-input.ts:52-60`) + `embedType` field (`"video"|"audio"|"generic"`) added to contract `ParsedEmbed` type §3 and the detection-rules table (with detection order + recursive-iframe + `"Could not parse embed source"`/`"Generic embeds are not allowed"` error strings).
- [x] FIXED `size`/`sizeRole`/`density` props (`EmbedInput.svelte:18-20`) added to contract §3 + §9 forwarding note.
- [x] FIXED Class prefix reconciled to `.poodle-embed-input*` throughout anatomy + §8 token tables + §9 note (`EmbedInput.svelte:72,86`).
- All 11 contract props (id/value/parsed/placeholder/parseDebounce/providers/disabled/error/onParse/onValueChange/resolveParseState), anatomy (TextInput rows=3 + status + error + ProviderPill + success text), states (empty/parsing-silent/success/error/disabled), callbacks, and the no-aria-live a11y rule all match.

## GPUI gap (vs Svelte + contract)

- [x] FIXED TextInput now composes the real `TextInput` primitive via `from_spec` (`input_type="multiline"`, `rows=3`, value/placeholder/disabled forwarded) — input semantics, sizing, and disabled-opacity delegate to the primitive. (Multi-row height is a TextInput-primitive limitation, not embed-input's.)
- [x] FIXED Error and success are separate nodes: error renders an Error span in `text-danger`; success renders the ProviderPill + a SuccessText span in `text-success`. No shared status color.
- [x] FIXED ProviderPill resolves chrome sizing — `PillSize::Sm` is the faithful resolution of `sizeRole="chrome"` at default presentation (chrome = one stop down from Md). `PillSpec` has no size-role field, so this is the correct mapped value (documented inline).
- [x] FIXED Spacing token-resolved: root gap → `space.inline.xs` (0.25rem), status min-height → `space.stack.lg` (1.25rem), status font → `typography.label.size`. Status gap 0.375rem has no named token → exact rem (noted).
- [ ] No `onParse`/`onValueChange` callbacks, no `parseDebounce` — preview-loop: the spec pre-resolves parse state; paste/fetch/debounce wiring lives in the preview event loop.
- accepted: no ARIA (gpui has no accessibility API). Contract itself requires no aria-live, so only TextInput-delegated semantics route through the primitive.

## Jetstream gap (vs Svelte + contract)

- [x] FIXED TextInput now composes the real `js_text_input` primitive (`input_type="multiline"`, `rows=3`) — input semantics, sizing, token resolution, and disabled-opacity delegate to the primitive.
- [x] FIXED ProviderPill is the real `js_pill` (tone=Success, size Sm = chrome resolved) — the hand-styled `label` fake is gone (CLAUDE.md "no fakes" satisfied).
- [x] FIXED Error and success split: error span resolves `color.status.danger`; success text resolves `color.status.success`. No single `status_color`.
- [x] FIXED Disabled opacity is delegated to `js_text_input` (which applies `disabled_opacity_token()`) — the hardcoded `.opacity(0.5)` is gone.
- [x] FIXED Status gap is now `0.375rem` (contract value); root gap → `space.inline.xs`, status min-height → `space.stack.lg`, status/error/success font → `typography.label.size`.
- [ ] No `onParse`/`onValueChange` callbacks, no `parseDebounce` — preview-loop (paste/fetch/debounce wiring lives in the preview event loop).
- accepted: no ARIA (Jetstream has no accessibility API).

## Probe tests (Jetstream)

`embed_input::tests` (render_probe, theme DARK): field+placeholder renders a filled panel; success state shows the provider pill + "Embed detected"; error state shows only the error message (no success pill/text); disabled value renders via the composed primitive; empty value emits no status row.

## Specimen parity

- Svelte covers: Supported-providers table, Detection matrix (4 samples), live input, Field wrapper, Restricted providers, Parsed-result dump, plus size + density snippets.
- GPUI covers: Supported-providers table, Detection matrix (4 `.with_detected_parse`), live input placeholder, Field wrapper, Restricted providers, Preset states (incl. custom parsed). Broad. — missing: nothing major vs Svelte (specimen is rich, though it leaks many hardcoded px + a `Menlo` font literal in the harness).
- Jetstream covers: With URL (detected), Empty (2 groups). — missing: **detection matrix, Field wrapper, restricted providers, provider table** (weakest coverage).

## Notes

- Both Rust targets collapse error/success into a single status color — the one functional parity break shared by both. Jetstream additionally fakes the Pill (CLAUDE.md "no fakes" violation) and hardcodes disabled opacity.
- The `.ts` parse engine is the real authority for detection rules (`youtu.be`, `youtube.com/watch|embed`, `vimeo`, `audioboom`, `<iframe>`, generic URL); the contract's detection table omits `audioboom` and the `embedType` field.
- consv=gap driven by undocumented Svelte surface (audioboom/embedType/size/sizeRole/density) + class-prefix drift.
