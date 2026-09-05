<!-- parity consv=fixed gpui=2 jetstream=2 specimen=ok | pass: Jetstream specimen brought to full contract coverage — grouped 7-item palette + shortcuts, active-item highlight, compact (sm+compact), all five §6 open-states (ready/loading/error/empty/no-results), Sizes + Densities sweeps; all real js_command_palette, each open palette in a relative/min_h container; both previews build clean. -->
<!-- pass 41: GPUI specimen completed to full contract coverage — Open-states group (ready/
     loading/error/empty/no-results), Sizes sweep (xs–xl via with_size), Densities sweep
     (compact/default/comfortable). All real CommandPalette::from_spec, no fakes; gpui/preview
     builds clean. GPUI specimen done; Jetstream pending engine recovery — specimen=gap held
     (Jetstream half unverifiable while engine build-blocked). -->
<!-- pass 35: Jetstream command-palette built to match GPUI — overlay backdrop scrim, centered
     modal surface (45rem cap, dialog border mix, radius.surface), header (title/description +
     hint pill + close), composed js_text_input (type=search + leading icon), grouped results
     (group headers, badges, shortcuts, active-id accent, disabled dim), status region (matches
     Svelte paletteStatus). 4 probe tests pass; suite 122. Remaining jetstream: scrim is
     color.background.overlay not the contract literal (no token) + no box-shadow (JsEl gap,
     shadow_lg approx); query/nav = preview-loop. -->
<!-- pass 33: GPUI full anatomy built — overlay backdrop scrim (color.background.overlay),
     centered modal surface (elevation_dialog_shadow, 45rem cap, dialog border mix), header
     (title/description + invocation-hint pill + close button), composed search TextInput
     (type=search, leading search icon), grouped results panel (kept), status region (matches
     Svelte paletteStatus). Specimen reworked to mount open palettes on the page region so the
     backdrop fills. on_open_change builder (no spec change). Both crates clean. Remaining GPUI:
     scrim is color.background.overlay not the contract's literal color-mix(black 44%,
     transparent) (no matching token) + surface-mix nuance; typing/arrow-nav = preview-loop;
     ARIA accepted. -->
# Parity: CommandPalette

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/command-palette.md`
- Svelte (authoritative): `packages/svelte/components/src/CommandPalette.svelte` (composes `ActionDiscoveryPanel`, `TextInput type="search"`, `Icon`)
- GPUI: `packages/gpui/components/src/composites/command_palette.rs`
- Jetstream: `packages/jetstream/components/src/command_palette.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/CommandPaletteSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/command_palette.rs` · jetstream `packages/jetstream/preview/src/specimens/command_palette.rs`

## Contract ↔ Svelte

All five stale §9 token rows now match the authoritative Svelte CSS. FIXED.

- [x] **Close button background** → `transparent`; dropped the `:hover` table (Svelte has no hover rule). FIXED.
- [x] **Status font-size / line-height** → `0.75rem` / `1.4`. FIXED.
- [x] **Hint font-size** → `var(--poodle-typography-label-size)` token. FIXED.
- [x] **Responsive width / max-height** → `width: calc(100vw - 1rem)`, max-height override removed, `padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x-sm, 0.75rem)`. FIXED.
- [x] **Responsive meta justification** → `space-between`. FIXED.
- Anatomy, props, callbacks, ARIA (role=dialog, aria-modal, aria-describedby, status role/live/atomic), keyboard (Esc/Arrows/Home/End/Enter/Tab-trap), focus management, scroll lock — all present in Svelte and match contract. Those are solid.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] **No overlay backdrop / modal positioning** — contract §8 requires a fixed full-viewport overlay (44% black + blur, z-index 40) and a fixed centered dialog (`translate(-50%,-50%)`, z-index 41). GPUI renders the panel inline as a plain flex column (`command_palette.rs:160-183`); no backdrop, not centered, not fixed.
- [ ] **No real search input** — contract §3 Query = `TextInput type="search"`; GPUI renders a static `div` showing the query or placeholder text (`command_palette.rs:224-240`). Not editable, no clear/search affordance.
- [ ] **No close button, no invocation-hint pill, no status region** — contract §3 Header (close button), Invocation Hint, and Status (`role=status` live region) are all absent from the GPUI tree. `invocation_hint` is repurposed as the search placeholder (`command_palette.rs:220-223`) instead of a header pill.
- [ ] Hardcoded width literal `px(720.0)` at `command_palette.rs:163` — contract width is `min(45rem, calc(100vw - 2rem))`; resolve, don't pin 720.
- [ ] Hardcoded max-height literal `px(400.0)` at `command_palette.rs:164` — contract `max-height: min(78vh, 52.5rem)`.
- [ ] Hardcoded shadow color literals `hsla(0.0,0.0,0.0,0.12)` / `hsla(0.0,0.0,0.0,0.08)` at `command_palette.rs:171,177` — contract shadow is `var(--poodle-elevation-dialog)`; resolve from token.
- [ ] Hardcoded shadow float literals `px(8.0)`, `px(24.0)`, `px(2.0)`, `px(0.0)` at `command_palette.rs:172-180` — build from the elevation token.
- [ ] Hardcoded spacing literals `px(2.0)` (header gap, line 192), `px(4.0)` (results pad / group pad / row margin / badge pad, lines 279,294,314,345) — resolve from space tokens.
- [ ] No ungrouped-result fallback verification + no `aria-modal`/focus-trap equivalent — keyboard (Arrow/Home/End/Enter/Esc) handling absent in the component (no `on_key_down`); selection is render-only.
- accepted: no ARIA (gpui has no accessibility API) — role=dialog/status, aria-live not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] **No overlay backdrop / modal positioning** — `.overlay()` is called (`command_palette.rs:33`) but there is no dim/blur backdrop layer and no centered fixed dialog framing per contract §8.
- [ ] **No header at all** — title, description, invocation-hint pill, and close button (contract §3 Header) are never rendered; `js_command_palette` jumps straight from root to the search row.
- [ ] **No status region** — contract §3/§7 Status (`role=status`, live count/active announcement) absent.
- [ ] **No grouping, shortcuts, badges, or active highlighting** — the action loop renders only `action.title` (`command_palette.rs:46-53`); ignores `action.group`, `action.shortcut`, `action.badge`, and `active_action_id`. Svelte/contract show grouped rows with shortcut hints and an active row.
- [ ] **No discovery-state handling** — `spec.state` (loading/error/empty/no-results) is ignored; the component always renders the full action list. Contract §6 requires distinct postures per state.
- [ ] **No real search input** — static `label("Type a command...")` (`command_palette.rs:42`); not a TextInput, not editable, ignores `spec.query`.
- [ ] Hardcoded width/height literals `rem_to_px(25.0)` / `rem_to_px(12.5)` at `command_palette.rs:31` — contract width `min(45rem,…)`; no token.
- [ ] Hardcoded gap literal `rem_to_px(0.5)` at `command_palette.rs:18` — resolve from a space token.
- [ ] Magic spacing offsets `panel_space_y_rem(...) - 0.25` and `- 0.375` at `command_palette.rs:16-17` — arbitrary subtractions; derive from tokens.
- [ ] `icon_size` aliased to `size_font_rem` (`command_palette.rs:14`) — should use an icon-size token, not the body font size.
- [ ] No keyboard / selection interaction in component (must live in preview `main.rs`; **verify** — specimen wires none).
- accepted: interaction (typing, arrow nav, Enter, Esc, backdrop click) lives in preview event loop, not the component.
- accepted: no ARIA channel.

## Specimen parity

> GPUI specimen done; Jetstream pending engine recovery. `specimen=gap` held because the Jetstream half is unverifiable while the engine is build-blocked.

- Svelte covers: Command Palette (button-triggered, grouped 7 items + shortcuts, interactive open/query), Semantic presentation (compact + prominent), sizes (per-size palette), densities (`CommandPaletteSpecimen.svelte`).
- GPUI covers (**complete**): Command Palette (button-triggered, grouped 7 items + shortcuts + active highlight), Semantic presentation (compact), **Open states** group exercising all five contract §6 postures (ready / loading / error / empty / no-results) as always-open palettes — each wrapped in its own `relative` container so the backdrop is region-scoped, **Sizes** sweep (xs–xl open palettes via `with_size`), **Densities** sweep (compact / default / comfortable via `with_density`). The component renders the full modal shell (overlay backdrop, header with title/hint pill/close, real search `TextInput`, grouped results, status region, per-state areas), so every group shows its distinguishing UI. No fakes — every palette is a real `CommandPalette::from_spec`. Full contract specimen coverage on GPUI.
- Jetstream covers: Empty query, Filtered results, No results, Active item, Custom placeholder, With title+description. — but the component renders none of these distinctions (no filter, no active highlight, no title/description header, no no-results state), so the specimens are **labelled for states the component cannot show**. Under-covers vs Svelte and the labels overstate behavior → `specimen=gap`.

## Notes

- The Jetstream specimen is the sharpest mismatch: groups are labelled "Filtered results", "No results", "Active item", "With title and description" but `js_command_palette` ignores query filtering, discovery state, active id, and title/description entirely — so all six groups render the same plain list. That is exactly the "specimen hides incomplete work" anti-pattern CLAUDE.md warns against.
- Both Rust targets render only the inner results panel, not the modal shell (overlay + centered dialog + header + status). That is the dominant structural gap for this component in both.
- `consv=gap` is driven by five stale §9 token rows (close bg/hover, status type, hint font, responsive width/max-height, meta justify) where the contract drifted from Svelte.
