<!-- parity consv=gap gpui=8 jetstream=9 specimen=gap -->
# Parity: TextInput

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/text-input.md`
- Svelte (authoritative): `packages/svelte/components/src/TextInput.svelte`
- GPUI: `packages/gpui/components/src/primitives/text_input.rs`
- Jetstream: `packages/jetstream/components/src/text_input.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/TextInputSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/text_input.rs` · jetstream `packages/jetstream/preview/src/specimens/text_input.rs`

## Contract ↔ Svelte

Svelte is authoritative. Where Svelte diverges from contract §8/§9 token text, the contract is stale — update it. Items below are Svelte-vs-contract mismatches; the side to fix is named.

- Validation indicator icons: contract §9 says built-in indicator renders `circle-check` (valid) / `circle-x` (invalid); Svelte renders `Icon icon="check"` and `Icon icon="x"` (`TextInput.svelte:189-193`, `603`). **Fix: contract — change §9 to `check`/`x`.**
- Pending spinner sizing: contract §8 "Spinner" says `variant="ring"`, `size="sm"`, `tone="current"`; Svelte uses `<Spinner variant="ring" sizeRole="chrome" tone="current" />` (`TextInput.svelte:601`) — `sizeRole="chrome"` not `size="sm"`. **Fix: contract — document `sizeRole="chrome"`.**
- Affix horizontal spacing: contract §8 Affix/Prefix/Suffix specify `padding-*: var(--poodle-space-inline-sm)` + `margin-*: space-inline-sm`; Svelte uses a flat `padding-inline: 0.625rem` with no margin (`TextInput.svelte:748`). **Fix: contract — replace prefix/suffix padding+margin model with single `padding-inline: 0.625rem`.**
- Affix separator color: contract §8 says `0.0625rem solid color-mix(border-subtle 52%, transparent)`; Svelte uses `border-right/left: 0.0625rem solid var(--poodle-color-border-default)` — solid `border-default`, no color-mix (`TextInput.svelte:760,764`). **Fix: contract — separator is `border-default` solid, not `border-subtle 52%`.** (This propagates: Tier-2 checklist "affix separator border uses the same color-mix formula" is wrong.)
- Affix opacity: Svelte adds `opacity: var(--poodle-state-opacity-muted)` to affixes and placeholder (`TextInput.svelte:754,794`); contract §8 affix/placeholder tables omit this. **Fix: contract — add affix + placeholder muted-opacity.**
- Char-count typography: contract §8 says `font-size: 0.6875rem`, `font-family: code-family`; Svelte uses the `font: var(--poodle-typography-code-xs)` shorthand and color `text-muted` (contract says `text-secondary`) (`TextInput.svelte:861-862`). **Fix: contract — char-count color is `text-muted`, font is `typography-code-xs` shorthand.**
- Density adjusts control padding: contract §8 size table owns padding; Svelte additionally shifts `padding-block`/inline by density via `--poodle-text-input-density-block-adjust` (`±0.0625rem` block, `TextInput.svelte:735-742,786`). Block padding from density borders on the size/density orthogonality rule (CLAUDE.md). **Fix: contract — document density inline/block adjust; confirm block-adjust is intended (it changes effective text vertical inset).**
- Adornment-driven control padding: Svelte computes `controlPaddingStart/End` from leading/trailing/clear/validation adornment counts (`TextInput.svelte:198-210`); contract §2/§8 describe affordances as absolutely-positioned overlays but never specify the control's reserved padding. **Fix: contract — document the adornment padding-reservation model.**
- `showValidationStatus` prop drives whether the built-in indicator shows (`TextInput.svelte:187`); contract §3 lists it but §4 state table doesn't note that indicators are gated by it. Minor. **Fix: contract — note gating in §4.**
- Multiline submit: Svelte fires `onSubmit` only on Cmd/Ctrl+Enter in multiline (`TextInput.svelte:535`); contract §5 says "Enter calls onSubmit" without the multiline exception (§1 mentions it). Consistent enough; no action.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded brand-raised shadow literal `hsla(0.0, 0.0, 1.0, 0.08)` at `text_input.rs:424` — resolve from a token, not raw HSLA.
- [ ] Hardcoded hover shadow literal `hsla(0.0, 0.0, 1.0, 0.10)` at `text_input.rs:443` — resolve from a token; also note Svelte has no hover shadow on the input (hover is border-only), so this hover shadow is an invented treatment.
- [ ] Hardcoded char-count font size `px(rem_to_px(0.6875))` at `text_input.rs:344` — add a char-count font-size token to the spec and resolve it (no `*_font_size_token` exists).
- [ ] Treatment alpha multipliers hardcoded: fill `*0.82`/`*0.88` (`text_input.rs:235,239`), border `*0.72`/`*0.92` (`243,247`), affix separator `*0.52` (`292,395`), focus-ring `*0.28` (`454`). These approximate Svelte treatment-token chains with raw literals — source from treatment tokens.
- [ ] Validation icons diverge from Svelte: GPUI uses `circle-check`/`circle-x` (`text_input.rs:355,362`); Svelte (authoritative) uses `check`/`x`. Align to `check`/`x`.
- [ ] No prefix/suffix as overlay-free affixes is fine, but affordances are rendered inline in a flex row (`text_input.rs:285-408`) rather than absolutely-positioned overlays per contract §2 anatomy + Svelte; leading/trailing/validation are not overlaid inside the field, so reserved control padding (Svelte `controlPaddingStart/End`) is absent.
- [ ] Missing props vs Svelte/contract: no `read_only` visual treatment branch, no `search` clear button, no `slug`/`source` handling, no `multiline` rendering (spec has `is_multiline()` + `rows` but builder ignores them — single-line only), no `show_clear_button`, no debounce. `read_only`/`disabled` only gate the key handler (`text_input.rs:471`).
- [ ] Editing model is char-append/backspace stub (`text_input.rs:477-505`) — no caret, selection, IME, or clipboard; contract §10 + Tier-1 require real text-system editing. Track in g10 roadmap (file header).
- accepted: no ARIA (gpui has no accessibility API) — `aria_label`/`aria-invalid`/`described_by` stored on spec but not emitted.
- accepted: per-size font ramp hardcoded `0.75..1.0` (`text_input.rs:219-225`) mirrors Svelte's `data-size` CSS; no per-size font token exists in spec — same pattern as other components, treat as shared-spec gap not a local violation.

## Jetstream gap (vs Svelte + contract)

- [ ] Char-count font size uses full `font_size` (`text_input.rs:193`) instead of the `0.6875rem`/`code-xs` contract size — wrong (counter renders at body size). Resolve a char-count font token.
- [ ] Validation icons diverge from Svelte: Jetstream uses `check-circle`/`alert-circle`/`loader` (`text_input.rs:118,124,131`); Svelte (authoritative) uses `check`/`x` icons + ring `Spinner` for pending. Align icons; pending should use the Spinner primitive, not a static `loader` glyph.
- [ ] Pending indicator color is `color.text.secondary` (`text_input.rs:132`); contract §8 + Svelte pending color is `accent-base`. Use the accent token.
- [ ] Affix separator divider is a hardcoded `w(1.0)` div (`text_input.rs:77,159`) — width should resolve from a border-width token; also Svelte separator is `border-default` solid, here it's `affix_sep_color` (border-subtle). Reconcile color to `border-default`.
- [ ] Root border width hardcoded `border(1.0)` (`text_input.rs:54`) — resolve from `border-width-default` token.
- [ ] Affordances/affixes are inline flex children (`text_input.rs:75-168`), not absolutely-positioned overlays per contract §2; control text does not reserve padding for adornments.
- [ ] No focus treatment: hover sets border only (`text_input.rs:60`); contract §8 focus-within (border-focus + fill-focus + focus-ring shadow) and validation-state border colors are absent — validation state only changes the trailing icon, never the root border color (contract §4 invalid/valid/pending border). Add focus + validation border.
- [ ] Missing modes/props: no `read_only` treatment, no `slug`/`source`, no `prefix`-into-validation, no debounce; `show_clear_button` clear button renders but has no clear handler (interaction is preview-owned).
- [ ] Char-count only renders when BOTH `show_char_count` AND `max_length` set (`text_input.rs:176-177`); Svelte shows bare `{count}` when only `showCharCount` is set (`TextInput.svelte:173`). Render count without max.
- accepted: no ARIA channel (documented pattern).
- accepted: text editing/caret/keyboard lives in preview `main.rs` event loop, not the component — component is render-only.
- accepted: per-size font ramp via `size_font_rem` shared helper (`text_input.rs:24`) — shared-spec font model, not a local literal.

## Specimen parity

- Svelte covers: Default, With validation (invalid→valid), Slug (with prefix + async validate + char limit), Search (clear), Prefix and suffix, Disabled, Multiline (rows + char count), plus Sizes + Densities snippets (`TextInputSpecimen.svelte`).
- GPUI covers: Default, With validation, Async/Pending, Slug (prefix), Search, Prefix+suffix, Suffix only, Multiline (explicit + auto + char count), Disabled, Sizes, Densities (`gpui/.../text_input.rs`). — broadest coverage; multiline specimens exist but component renders them single-line (see GPUI gap). Missing: nothing structurally vs Svelte.
- Jetstream covers: Default, With value, Disabled, Invalid, Valid, Pending, Leading icon, Trailing icon, Prefix, Suffix, Multiline (4 rows), Char count (`jetstream/.../text_input.rs`). — missing: **Search/clear** group, **Slug** group, **Sizes** sweep, **Densities** sweep, and **Field-wrapped label/description** (Svelte/GPUI wrap in `Field`; Jetstream renders bare inputs).

## Notes

- `consv=gap` driver: contract §8/§9 token text is stale vs the authoritative Svelte CSS in several places (validation icon names, spinner `sizeRole`, affix padding model, affix separator color, char-count color/font, density padding, adornment padding reservation). All belong updated in the contract per "Svelte is parity authority".
- Both Rust targets render affordances/affixes as inline flex children instead of contract §2's absolutely-positioned overlays. This is a shared architectural delta — acceptable visually for static previews but means neither reserves control padding the way Svelte does; flag if overlay fidelity becomes required.
- Validation-icon naming is inconsistent across all three (Svelte `check`/`x`, GPUI `circle-check`/`circle-x`, Jetstream `check-circle`/`alert-circle`/`loader`) — pick Svelte's names as the contract truth and align both Rust targets.
- Per-size font ramp is hardcoded as a numeric `match` in both Rust targets because the spec exposes only `body_size_token()` (single size), not a per-`ControlSize` font token. This is a spec-surface gap shared with other sized controls, not a text-input-specific violation.
- GPUI hover adds a shadow that Svelte does not have (Svelte root has no hover style — §4 "no explicit hover style on root"); the GPUI hover border/bg/shadow is invented treatment and should be reconciled to focus-only emphasis.
