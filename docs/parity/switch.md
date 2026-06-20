<!-- parity consv=fixed gpui=3 jetstream=6 specimen=gap -->
<!-- note: the "no tones" todo was a recall-biased over-flag — left_tone/right_tone
     ARE resolved (switch.rs); only custom on_color/off_color string overrides are
     unapplied. Several remaining items are preview-loop (toggle, readOnly cursor)
     or render-runtime (focus ring) deltas, not pure render gaps. -->
# Parity: Switch

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/switch.md`
- Svelte (authoritative): `packages/svelte/components/src/Switch.svelte`
- GPUI: `packages/gpui/components/src/primitives/switch.rs`
- Jetstream: `packages/jetstream/components/src/switch.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/SwitchSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/switch.rs` · jetstream `packages/jetstream/preview/src/specimens/switch.rs`

## Contract ↔ Svelte

Props match exactly (every contract §3 prop is present in `Switch.svelte:10-30` with identical
types/defaults: `id`, `checked`, `defaultChecked`, `disabled`, `readOnly`, `label`, `leftLabel`,
`rightLabel`, `ariaLabel`, `describedBy`, `name`, `offColor`, `onColor`, `leftTone`/`rightTone`,
`size`, `sizeRole`, `density`, `onCheckedChange`). Anatomy, ARIA (`role="switch"`, `aria-checked`
via native `checked`, `aria-readonly`, `aria-label` fallback composed from left/right labels),
and readOnly-revert all match contract. Divergences are in the **size table** — Svelte is
authoritative, contract §7/§8 is stale:

- [x] FIXED **md track size mismatch.** Contract §7/§8 md track was `2.125rem × 1.25rem`, thumb `0.875rem`. Updated to Svelte md `2.25rem × 1.375rem` / `1.125rem` thumb / `0.875rem` travel (`Switch.svelte:299-312`); §7 now clarifies the `--switch-unit` base vs the per-size override.
- [x] FIXED **Size formulas vs literals.** Rewrote §8 Size adjustments table with Svelte's flat rem literals (`Switch.svelte:269-342`): xs `1.75×1`/0.75/0.75; sm `2×1.125`/0.875/0.875; md `2.25×1.375`/1.125/0.875; lg `2.75×1.625`/1.375/1.125; xl `3×1.75`/1.5/1.25. Track padding `0.125rem` at every size.
- [x] FIXED **Label font-size mismatch.** §8 label-size table aligned to Svelte (`Switch.svelte:345-349`): xs `0.75`, sm `0.75`, md `0.8125`, lg `0.875`, xl `0.875rem`.
- [x] FIXED **Inactive dual-label color.** Added a "Label color rules" subsection to §8: single label → text-primary; dual left/right rest at text-muted; active side re-tints to off/on color (`Switch.svelte:241-257`). Also added a Density variants gap table (compact `0.25rem` / default `space-inline-sm` / comfortable `space-inline-md`, `Switch.svelte:259-266`).

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] **md geometry follows the stale contract, not Svelte.** `switch.rs:156` hardcodes md to
  `2.125rem × 1.25rem` track / `0.875rem` thumb — Svelte md is `2.25rem × 1.375rem` / `1.125rem`.
  Same drift for xs/sm/lg/xl: GPUI uses the contract `icon-*` formula scaling (`switch.rs:153-159`)
  while Svelte uses flat literals. Switch to Svelte's literal per-size values.
- [ ] **Label font-size uses `size_font_rem` shared helper** (`switch.rs:135`), not the
  switch-specific label table (xs `0.75`, sm `0.75`, md `0.8125`, lg/xl `0.875`). Verify the helper
  yields those rem values per size; if not, resolve the contract label-size table.
- [ ] **No readOnly revert semantics** — `is_read_only` only sets `cursor_default` (`switch.rs:271-272`)
  and gates the click handler off (`is_interactive`, `switch.rs:138,333`). That suppresses toggles,
  matching the contract effect, but there's no native change-revert path; acceptable as state-driven,
  flag for verification.
- accepted: no ARIA (gpui has no accessibility API) — `role="switch"`/`aria-checked` not emitted; contract §6 GPUI note asks for native a11y-tree exposure, unmet.
- accepted: hardcoded shadow alpha literals `hsla(0.0,0.0,1.0,0.08)` (track inset, `switch.rs:219`) and `hsla(0.0,0.0,0.0,0.18)` (thumb, `switch.rs:245`) — these are the contract's `white 8%` / `black 18%` color-mix constants with no semantic token; allowed per §12 color-mix delta.
- accepted: thumb `blur_radius: px(8.0)` / `offset px(2.0)` (`switch.rs:246-248`) are the contract `0.125rem 0.5rem` shadow geometry; runtime shadow params, allowed per §12.

## Jetstream gap (vs Svelte + contract)

- [ ] **No size scaling at all.** `effective_size` is resolved (`switch.rs:47`) but never used for
  geometry — track/thumb/travel are fixed md literals `rem_to_px(2.125/1.25/0.875)` for every size
  (`switch.rs:89-94`). xs/sm/lg/xl render identical to md. Implement the per-size table.
- [ ] **Gap uses wrong token.** `gap = rem_to_px(control_space_x_rem(density))` → 0.5/0.75/1.0rem
  (`switch.rs:60`, `presentation.rs:123-129`). Contract/Svelte gap is `space-inline-sm`
  (compact `0.25rem`, default `space-inline-sm`, comfortable `space-inline-md`). GPUI resolves the
  `space.inline.{xs,sm,md}` tokens correctly (`gpui switch.rs:124-128`); Jetstream should too, not the `control_space_x_rem` heuristic.
- [ ] **No focus ring.** Contract §8 track `:focus-visible` outline (`accent.focusRing`, offset `0.125rem`)
  is absent; GPUI applies one (`gpui switch.rs:262-265`). `root` is `.focusable()` (`switch.rs:126`) but draws no ring.
- [ ] **No tones / custom colors.** `left_tone`/`right_tone` are read (`switch.rs:64-71`) but
  `off_color`/`on_color` hex overrides (contract §3, used in Svelte `toneToColor`/`switchStyles`) are
  ignored — no `spec.on_color`/`spec.off_color` branch. GPUI handles both (`gpui switch.rs:178-196`).
- [x] **DONE: dual-label mode.** `leftLabel`/`rightLabel` now flank the track (active side tone-colored, inactive muted) via `is_dual_label`. Probe-tested. — was: only the single
  trailing `spec.label` renders (`switch.rs:129-136`). GPUI implements left|track|right with active-side
  tinting (`gpui switch.rs:280-316`).
- [ ] **No readOnly handling.** `is_read_only` is never read in `js_switch`; no cursor/non-interactive
  treatment beyond the (absent) click path. Contract §4 requires default cursor + revert-on-toggle.
- [ ] **No interactive toggle in component or preview.** `js_switch` has no click/key handler, and
  `main.rs` has no switch toggle action (`main.rs` `ShellAction` handles tree/probe toggles only,
  lines 768-781) — specimen switches are static. GPUI wires `on_change` + Space/Enter (`gpui switch.rs:332-347`).
- accepted: no ARIA channel (`role="switch"`/`aria-checked` not emitted) — consistent runtime limit.
- accepted: `.rounded(999.0)` pill literal (`switch.rs:114`) — contract `border-radius: 999px` is itself a literal sentinel, not a token.

## Specimen parity

- Svelte covers: Default (3 interactive), States (disabled off/on, read-only on), Custom colors
  (on/off hex), Dual labels + tones (danger/success, warning/success), Sizes snippet, Densities snippet (`SwitchSpecimen.svelte`).
- GPUI covers: Default (3 interactive w/ toggle), States, Custom colors, Dual labels + tones, plus
  Sizes + Densities via `specimen_layout` (`gpui/.../switch.rs:167-186`). — full parity with Svelte.
- Jetstream covers: States (Off/On — static, non-interactive), Disabled (off/on). — **missing: Custom colors, Dual labels + tones, Sizes, Densities, Read-only, and interactivity** (`jetstream/.../switch.rs:25-36`). Largest specimen shortfall of the three targets.

## Notes

- `consv=fixed`: the stale **size/label tables** in contract §7/§8 are now the Svelte per-size literals
  (md track/thumb/travel + label font-sizes), and §8 gained the dual-label color rules + density gap table.
  Props/anatomy/ARIA/readOnly were already clean.
- Jetstream is the weakest target: fixed-md geometry, no tones/custom-colors/dual-labels/focus-ring/readonly/toggle.
  It renders a correct *single md off/on switch* and nothing else.
- GPUI is near-complete behaviorally (size table, tones, custom colors, dual labels, toggle, keyboard)
  but inherits the stale contract md/size geometry rather than Svelte's; ARIA is the only accepted gap.
- Both Rust impls compute `color-mix` via `mix(other, f)` helpers matching the contract 18%/24%/58%
  formulas (`gpui switch.rs:178-209`, `jetstream switch.rs:77-86`) — allowed per §12.
