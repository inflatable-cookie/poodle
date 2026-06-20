<!-- parity consv=gap gpui=9 jetstream=11 specimen=gap -->
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

Several §9 token values in the contract no longer match Svelte. Svelte is authoritative — fix the contract:

- **Close button background.** Contract §9 `.command-palette__close` `background: color-mix(... surface 62%, transparent)` plus a `:hover` rule (`84%` + text-primary). Svelte uses `background: transparent` (line 382) and has **no** `:hover` rule. **Fix: contract → transparent background, drop the hover row (or Svelte should add hover — Svelte wins per parity authority, so update contract).**
- **Status font-size / line-height.** Contract §9 `.command-palette__status` = `font-size: 0.8125rem`, `line-height: 1.5`. Svelte = `0.75rem`, `1.4` (lines 399-400). **Fix: contract → 0.75rem / 1.4.**
- **Hint font-size.** Contract §9 `.command-palette__hint` `font-size: 0.75rem`. Svelte = `var(--poodle-typography-label-size)` (line 368). **Fix: contract → token reference.**
- **Responsive width / max-height.** Contract §9 breakpoint `width: min(100vw - 1.25rem, 45rem)`, `max-height: calc(100vh - 1.25rem)`, `padding: 1rem`. Svelte = `width: calc(100vw - 1rem)`, no max-height override, `padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x-sm, 0.75rem)` (lines 404-407). **Fix: reconcile contract responsive block to Svelte.**
- **Responsive meta justification.** Contract §9 `.command-palette__meta` `justify-content: flex-start`. Svelte = `space-between` (line 414). **Fix: contract → space-between.**
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

- Svelte covers: Command Palette (button-triggered, grouped 7 items + shortcuts, interactive open/query), Semantic presentation (compact + prominent), sizes (per-size palette), densities (`CommandPaletteSpecimen.svelte`).
- GPUI covers: Command Palette (button + grouped items + shortcuts + active highlight), Semantic presentation (compact / prominent), but rendered inline without overlay/close/hint/status — structurally incomplete. No sizes/densities matrix groups (relies on size_role specimen only). — missing: overlay framing demo, close/hint/status, sizes & densities grids.
- Jetstream covers: Empty query, Filtered results, No results, Active item, Custom placeholder, With title+description. — but the component renders none of these distinctions (no filter, no active highlight, no title/description header, no no-results state), so the specimens are **labelled for states the component cannot show**. Under-covers vs Svelte and the labels overstate behavior → `specimen=gap`.

## Notes

- The Jetstream specimen is the sharpest mismatch: groups are labelled "Filtered results", "No results", "Active item", "With title and description" but `js_command_palette` ignores query filtering, discovery state, active id, and title/description entirely — so all six groups render the same plain list. That is exactly the "specimen hides incomplete work" anti-pattern CLAUDE.md warns against.
- Both Rust targets render only the inner results panel, not the modal shell (overlay + centered dialog + header + status). That is the dominant structural gap for this component in both.
- `consv=gap` is driven by five stale §9 token rows (close bg/hover, status type, hint font, responsive width/max-height, meta justify) where the contract drifted from Svelte.
