<!-- parity consv=gap gpui=7 jetstream=9 specimen=gap -->
# Parity: Callout

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/callout.md`
- Svelte (authoritative): `packages/svelte/components/src/Callout.svelte`
- GPUI: `packages/gpui/components/src/primitives/callout.rs`
- Jetstream: `packages/jetstream/components/src/callout.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/CalloutSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/callout.rs` · jetstream `packages/jetstream/preview/src/specimens/callout.rs`

## Contract ↔ Svelte

Mostly aligned. One geometry literal and the spec struct lagging the contract props.

- Root padding: contract §8 says `var(--poodle-space-panel-y) var(--poodle-space-panel-x)`; Svelte hardcodes block padding `0.625rem var(--poodle-space-panel-x)` (`Callout.svelte:129`). **Fix: contract should state `0.625rem` block (or add the literal to the panel-y note) to match Svelte.**
- Content gap: contract §8 Content table says `gap: 0.25rem`; Svelte `.callout__content` uses `var(--poodle-space-inline-sm)` (`Callout.svelte:186`). **Fix: reconcile contract content gap to the inline-sm token.**
- `CalloutTone` includes `pending` (contract §3) and Svelte handles it (`:85-87`, spinner badge) — present and correct in Svelte.
- All public props (`tone`, `title`, `message`, `ariaLabel`, `announceMode`, `dismissible`, `dismissLabel`, `size`, `sizeRole`, `density`) are present in Svelte (`:10-25`). Contract↔Svelte prop surface matches. **The divergence is the Rust SPEC, not Svelte** — see below.
- Icon mapping: contract §6 maps danger→`circle-x`, warning→`triangle-alert`, success→`check`. Svelte maps success→`check`, warning→`triangle-alert`, danger→`circle-x` (`Callout.svelte:29-35`). Matches. Both Rust targets use DIFFERENT icon names (see gaps).

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded block padding `px(rem_to_px(0.625))` at `callout.rs:113` — resolve from a panel-y token, not a raw `0.625`.
- [ ] Hardcoded dismiss radius offset `px(rem_to_px(0.0625))` at `callout.rs:235` — contract dismiss radius is `calc(radius-control - 0.0625rem)`; the `0.0625` should come from a border-width token.
- [ ] Hardcoded hover background literal `Hsla { a: 0.08, ..text_secondary }` at `callout.rs:241` — resolve the dismiss hover overlay from a state token, not a raw `0.08` alpha.
- [ ] Tone icons wrong: success→`check-circle`, danger→`alert-circle` at `callout.rs:130-133` — Svelte uses `check` and `circle-x` (`Callout.svelte:29-35`). Rename to match the contract icon map.
- [ ] No `pending` tone branch — `icon_name` match `_ => "info"` (`callout.rs:128-134`) and bg/border treat only neutral-vs-toned; pending should use accent `8%`/`26%` mix + shared ring spinner badge (contract §8 pending row, §6). Currently pending renders as info.
- [ ] No `announceMode` support — struct has no announce field; no role/alert mapping. (Distinct from ARIA — this is the prop being absent from the builder entirely.)
- [ ] No `actions` slot region — contract §2 anatomy `.callout__actions`; GPUI builder renders only icon/content/dismiss (`callout.rs:163-253`).
- accepted: no ARIA (gpui has no accessibility API) — `role`/`aria-live`/`aria-label`/`aria-hidden` on icon all unexpressed.

## Jetstream gap (vs Svelte + contract)

- [ ] Border literal `.border(1.0)` at `callout.rs:48` — contract border width is `0.0625rem`; resolve from `border-width` token, not raw `1.0`.
- [ ] Title weight literal `.text_weight(600)` at `callout.rs:71` — Svelte title uses `typography-label` family/weight; resolve weight from a token.
- [ ] Gap literals `rem_to_px(0.5)` and `rem_to_px(0.25)` at `callout.rs:34-35` — outer gap should be `space-inline-md` token, content gap `space-inline-sm` token; drop the literals.
- [ ] No icon badge — icon rendered bare (`callout.rs:57-61`); contract §8 requires a circular `1.375rem` badge with `surface 78%` background and `999px` radius. Missing entirely.
- [ ] Tone icons wrong: warning→`alert-triangle`, danger→`x-circle`, pending→`loader` at `callout.rs:18-26` — Svelte uses `triangle-alert`, `circle-x`, and a ring SPINNER (not a loader icon) for pending.
- [ ] No `pending` spinner — pending maps to a static `loader` icon (`callout.rs:24`); contract §6 requires the shared `Spinner` primitive (ring/sm/accent).
- [ ] No dismiss button — `js_callout` has no dismissible path at all; contract §2/§5 `.callout__dismiss` + `onDismiss` absent.
- [ ] No `actions` slot region (`callout.rs:46-83`).
- [ ] Neutral tone not handled — fill is always `tone_color.mix(panel, 0.10)` (`callout.rs:44`); neutral should be `color-mix(panel 94%, transparent)` + `border-subtle 88%`, not an accent-base 10% mix. Neutral renders accent-tinted.
- accepted: no ARIA channel (`role`/`aria-live`/`aria-label`) — documented platform limit.
- accepted: interaction (dismiss click) would live in the preview event loop; component is render-only.

## Specimen parity

- Svelte covers: Tones (neutral/info/success/warning/danger — 5), Message prop, Dismissible (with live dismiss + success follow-up), Without title, With actions, Sizes snippet, Densities snippet (`CalloutSpecimen.svelte`).
- GPUI covers: Tones (neutral/info/success/warning/danger), Message prop, Dismissible (info + warning, interactive), Without title, Sizes, Densities. — missing: **pending tone**, **With actions** group.
- Jetstream covers: Info, Success, Warning, Danger (4 tones only). — missing: **neutral tone**, **pending tone**, **Message prop**, **Dismissible**, **Without title**, **With actions**, **Sizes**, **Densities** groups. Specimen is the thinnest of the three.

## Notes

- `CallOutSpec` (`packages/contracts/components/src/call_out.rs`) lacks fields the contract §3 requires: `dismissible`, `announce_mode`, `dismiss_label`, `aria_label`. GPUI stuffs `is_dismissible`/`on_dismiss` onto the wrapper struct outside the spec (`callout.rs:21-22`); Jetstream has no dismiss path. **Add these to the spec so both Rust targets resolve from one source.**
- `fill_token()`/`border_token()` (`call_out.rs:53-73`) return a single color per tone (accent-base for info/neutral/pending). The contract distinguishes neutral (`panel 94%` / `border-subtle 88%`) and pending (`accent 8%` / `26%`) from info (`status-info 10%` / `34%`). Spec token methods cannot express neutral or pending correctly — both Rust impls then approximate or fall through. **Add explicit neutral + pending token paths.**
- Contract §4 lists `info` as using a dedicated `--poodle-color-status-info` blue token; `fill_token()` returns `COLOR_ACCENT_BASE` for info, so GPUI/Jetstream tint info with accent, not the status-info blue. Visual mismatch vs Svelte (`Callout.svelte:138`).
- `consv=gap` driver is two stale §8 literals (block padding, content gap); the heavier work is the Rust spec/token surface, captured in the gap sections.
