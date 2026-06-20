<!-- parity consv=ok gpui=5 jetstream=6 specimen=gap -->
# Parity: PasswordRequirements

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/password-requirements.md`
- Svelte (authoritative): `packages/svelte/components/src/PasswordRequirements.svelte`
- GPUI: `packages/gpui/components/src/primitives/password_requirements.rs`
- Jetstream: `packages/jetstream/components/src/password_requirements.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/PasswordRequirementsSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/password_requirements.rs` · jetstream `packages/jetstream/preview/src/specimens/password_requirements.rs`

## Contract ↔ Svelte

Contract and Svelte agree on every public prop (name/type/default), the data contract, and the four checklist rules. No divergence in surface.

- Props match: `password`/`requirements`/`loading`/`error`/`title`/`hint`/`loadingLabel`/`size`/`sizeRole` all present with matching defaults. Default `hint` text matches contract's "default hint text" (`PasswordRequirements.svelte:23`).
- Anatomy match: title shown only when requirements present (Svelte hides title in loading/error states, `:42-44`); contract §4 says title is part of the checklist branch. Consistent.
- Accessibility note (not a divergence, but Svelte under-delivers vs its own contract §6): contract requires per-item `aria-label` with "met / not met" text and a non-color indicator (checkmark/cross). Svelte conveys pass/fail by **color only** (`__item--met` swaps text color, `:136-138`) with no icon and no `aria-label`. Svelte is the authority for visual reference but is missing contract-specified a11y. **Fix: add per-item `aria-label` + visible indicator to Svelte, or relax contract §6.** Flagging as a Svelte-side a11y gap, not a contract↔Svelte prop divergence — `consv` stays `ok`.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] No `size`/`sizeRole` support — `PasswordRequirementsSpec` has no size field (`packages/contracts/components/src/password_requirements.rs:54-62`) and GPUI hardcodes a single ladder rung: `body_font`/`title_font` = `rem_to_px(0.875)` (`password_requirements.rs:88-89`), `padding` = `rem_to_px(1.0)` (`:91`). Contract §7 size ladder (xs→xl) is unimplemented. Add `size`/`size_role` to spec + per-size token methods, resolve all five size-driven properties (padding, title size, body size, list indent, vertical rhythm).
- [ ] Hardcoded rem constants instead of token resolution — `rem_to_px(0.875)` title/body (`:88-89`), `rem_to_px(1.0)` padding (`:91`), `rem_to_px(0.5)` title margin-bottom (`:123`), `rem_to_px(0.125)` list gap (`:128`), `rem_to_px(0.375)` row gap (`:236`), `rem_to_px(0.875)`/`rem_to_px(0.75)` indicator box (`:242-243`). These are raw rem literals, not `resolve_px(theme, spec.<x>_token())`. Add size/typography tokens to the spec and resolve. (Description/hint margins at `:186`/`:198` correctly use `resolve_px(theme, "space.inline.*")` — keep.)
- [ ] Indicator divergence from Svelte: GPUI renders `✓`/`•` (checkmark/bullet, `build_rule_row:229`). Svelte renders no indicator glyph at all (color-only). Either align GPUI to Svelte (drop the glyph) or — preferred per contract §6 — drive Svelte to add the indicator and keep GPUI's check/cross. Flag for reconciliation.
- [ ] Title color: GPUI applies `title_color` (text-primary) to the title and `text_color` (text-secondary) to rows — matches Svelte. OK, no action; listed to confirm checked.
- [ ] Missing `description` vs `hint` ordering parity: GPUI renders description then hint (`:183-204`) matching Svelte `:65-70`. OK.
- accepted: no ARIA (gpui has no accessibility API) — per-item met/not-met `aria-label` and `aria-live="polite"` cannot be emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] No `size`/`sizeRole` support — same root cause as GPUI (spec lacks size field). Jetstream hardcodes `title_size`/`body_size` = `rem_to_px(0.8125)` (`password_requirements.rs:41-42`), `padding` = `rem_to_px(0.75)` (`:47`) — note this is the **xs/sm rung**, not the `md` default the contract specifies (md padding `1rem`, body `0.875rem`). Wrong default size + no ladder. Add size to spec, resolve per-size tokens.
- [ ] Hardcoded rem constants, not tokens — `title_size`/`body_size` `0.8125` (`:41-42`), `small_size` `0.75` (`:43`), `icon_size` `0.875` (`:44`), `gap` `0.5` (`:45`), `item_gap` `0.375` (`:46`), `padding` `0.75` (`:47`), `border_width` `0.0625` (`:48`), `radius` `rem_to_px(0.5)` (`:49`). File header claims "ALL dimensions resolve from tokens. ZERO hardcoded pixel values" (`:6`) — false; these are raw rem literals. Resolve from spec token methods (`radius` should use `resolve_radius(theme, "radius.surface")` like picker-shell does). Also `text_weight(600)` literal at `:66`.
- [ ] Label wording mismatch — Jetstream mixed-case row says "Upper and lowercase letters" (`:105`); Svelte/contract/GPUI say "Mix of uppercase and lowercase letters". Align to Svelte text.
- [ ] State branch ordering differs from Svelte — Jetstream renders the **title first**, then early-returns for loading (`:62-77`) and error (`:80-87`), so loading/error states show the title. Svelte shows title **only** in the requirements branch (`:42-44`); loading/error render a single paragraph with no title. Restructure to match Svelte: emit title only inside the requirements branch.
- [ ] Indicator: Jetstream uses icon registry `check`/`x` (`build_requirement_item:173`) — closer to contract §6 than Svelte's color-only, but diverges from Svelte visual reference and from GPUI's `✓`/`•`. Reconcile the indicator story across all three.
- [ ] Hint rendered even with no requirements (`:152-158` runs unconditionally) — Svelte only renders hint inside the requirements branch (`:68`), so empty-spec Svelte shows nothing but Jetstream shows title + hint (see test `no_requirements_shows_only_title_and_hint:245`). Gate hint on requirements presence like Svelte.
- accepted: no ARIA channel — per-item status labels / live region not emitted; interaction n/a (display-only component).

## Specimen parity

- Svelte covers: Default (live `TextInput` + checklist), Loading, Error, plus a **size ladder** (xs→xl via `SpecimenLayout showSizes`) (`PasswordRequirementsSpecimen.svelte:18-48`).
- GPUI covers: Default (static password `ClayRules123!`), Loading, Error — **missing: size ladder** (no size variants; spec can't express size). Uses min-length 12 policy like Svelte.
- Jetstream covers: Partially met, All met, Loading — **missing: Error state, size ladder, description rendering.** Uses min-length **8** policy (Svelte/GPUI use 12) and no `description`, so the description path is never exercised.

## Notes

- Root cause of most gaps: `PasswordRequirementsSpec` (`packages/contracts/components/src/password_requirements.rs`) has **no `size`/`size_role` field and no size/typography token methods** — only color tokens (`fill`/`border`/`title`/`text`/`met`/`error`). Adding size to the spec unblocks the ladder in both Rust targets and removes the rem-literal hardcodes.
- Three different indicator treatments across targets (Svelte color-only, GPUI `✓`/`•`, Jetstream `check`/`x` icons). Contract §6 mandates a non-color indicator, so Svelte is the one that should change — but per repo policy Svelte is the visual authority. Needs an explicit decision; until then all three "differ" but none is unambiguously wrong.
- Jetstream's default size resolves to the contract's xs rung, not md — even before a ladder exists, the single hardcoded size is the wrong one.
