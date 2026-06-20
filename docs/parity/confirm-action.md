<!-- parity consv=ok gpui=7 jetstream=7 specimen=gap -->
# Parity: ConfirmAction

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/confirm-action.md`
- Svelte (authoritative): `packages/svelte/components/src/ConfirmAction.svelte`
- GPUI: `packages/gpui/components/src/composites/confirm_action.rs`
- Jetstream: `packages/jetstream/components/src/confirm_action.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/ConfirmActionSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/confirm_action_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/confirm_action.rs`

## Contract ↔ Svelte

Aligned. Props (title/description/tone/triggerLabel/confirmLabel/cancelLabel/onConfirm/onCancel/size/sizeRole/density), defaults, snippets (`trigger`, `children`), and the trigger-tone derivation (`danger→danger`, else `default`, line 48) all match. Default trigger is a `variant="secondary"` Button with derived tone (line 86); custom trigger is wrapped in `<span role="presentation">` with Enter/Space keydown + preventDefault (lines 70–84). AlertDialog prop mapping matches §8. No divergence.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Does **not compose AlertDialog** — the dialog is hand-rolled inline (`confirm_action.rs:165-259`) instead of delegating to the AlertDialog primitive (contract §1/§2/§10 require composing AlertDialog + Button). Every dialog visual is reimplemented and will drift from the AlertDialog contract.
- [ ] Hardcoded shadow color literals `hsla(0.0, 0.0, 0.0, 0.12)` / `hsla(0.0, 0.0, 0.0, 0.08)` at `:178, :184` — resolve from a shadow/overlay token, not raw HSLA.
- [ ] Hardcoded shadow geometry `px(8.0)/px(24.0)/px(2.0)/px(8.0)` at `:179-186` and `min_w(px(360.0))` at `:175` — dialog width + shadow offsets/blur must come from tokens, not literals.
- [ ] Default trigger (no custom trigger) is a hand-rolled `div` (`:130-145`), not a secondary Button primitive — contract §2 DefaultTrigger delegates to the Button contract with derived tone. It also ignores `tone` (no danger fill on the fallback trigger).
- [ ] Confirm button text forced `gpui::white()` at `:236`; should resolve from a token (e.g. text-inverse / on-accent), and the cancel/confirm buttons are hand-rolled `div`s rather than Button primitives.
- [ ] No backdrop dismiss / Escape handling — backdrop `div` (`:263-272`) has `occlude()` but no on-click cancel; contract §6 says backdrop + Escape close via AlertDialog.
- [ ] Gap heuristic `gap * 2.0` at `:174` — ad-hoc multiplier instead of a stack-gap token.
- accepted: no ARIA (gpui has no accessibility API) — `role="alertdialog"`, `aria-labelledby`, `aria-describedby`, focus trap not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] Does **not compose AlertDialog or Button** — both trigger and dialog are hand-rolled in `js_confirm_action` (`confirm_action.rs:30-104`); contract requires delegating to AlertDialog + Button primitives.
- [ ] Hardcoded card padding `rem_to_px(1.5)` / `rem_to_px(1.25)` at `:62-63` and `max_w(rem_to_px(28.0))` at `:64` — dialog padding + width must come from tokens, not literals.
- [ ] Title size ad-hoc `size_font_rem(...) + 0.1875` at `:15`; should resolve a heading-size token (GPUI uses `typography.heading.size`).
- [ ] Default trigger tone: closed-state trigger picks `color.status.danger` vs `text.primary` by `is_destructive()` (`:32-36`) — but renders as a bordered surface button, not a secondary Button with the contract's tone derivation; warning tone yields no visual distinction.
- [ ] Confirm/cancel buttons hand-rolled (`:84-99`) instead of Button primitives; no hover/active/focus treatment.
- [ ] No backdrop dismiss / Escape — open state renders a full backdrop (`:50-55`) with no cancel-on-backdrop wiring (interaction would live in the event loop, but nothing is plumbed).
- [ ] `children`/body-content slot unsupported — `js_confirm_action` takes no content arg; contract §3 `children` snippet (body between description and actions) is unimplemented.
- accepted: no ARIA channel; open/close interaction lives in preview event loop, not the component.

## Specimen parity

- Svelte covers: Default trigger (danger), Warning tone, **Custom trigger slot** (ghost Button), **With body content** (code block), Last-action readout.
- GPUI covers: Default trigger (danger), Warning tone, Custom trigger slot (ghost), With body content (code block), Last action — broad, interactive open/close. — note: all four use `with_trigger` custom Buttons; the no-trigger default-Button path is never exercised.
- Jetstream covers: Neutral (closed), Destructive (closed), Open state (static). — missing: **Warning tone** as distinct, **Custom trigger slot**, **With body content** (no body arg exists), interactive open/close, last-action readout.

## Notes

- The dominant cross-target gap is architectural: the contract defines ConfirmAction as a **composite over AlertDialog + Button**, but both Rust targets reimplement an inline dialog. This duplicates AlertDialog styling and is the source of every hardcoded-literal flag here — fixing it means routing through the AlertDialog/Button components, which already resolve their own tokens.
- Specimen-wise the GPUI suite mirrors Svelte closely; Jetstream is the laggard (no custom-trigger, no body content, no warning differentiation).
