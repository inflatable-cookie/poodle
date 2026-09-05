<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok | specimen pass: GPUI + Jetstream specimens backfilled to full contract state coverage with real VideoPlayer/js_video_player + VideoPlayerSpec only — paused-at-start (big play), playing mid-progress, muted (volume 0 → volume-x), fullscreen, all 4 aspect ratios, sizes, densities, semantic role; GPUI also shows captions via VideoPlayer::with_captions. Skipped (no spec/component field): captions on Jetstream, buffering, title overlay. No hand-rolled bars; seek fill from spec progress. Both previews build clean. | prior pass: GPUI rebuilt — all geometry resolves from the VideoPlayerSpec size/density rem ladders via rem_to_px, fixed white-on-black per contract §8, real transport Icons, m:ss monospace time, proportional seek fill via relative(frac), volume track+thumb, big-play transparent ring only when paused-at-0. Jetstream — pill radius now uses pill_radius_rem(), volume thumb added, transport uses real Icon glyphs. Both targets + 6 (jet) render-probe tests. Playback/seek/volume/fullscreen interaction stays preview-loop owned (accepted §10). -->
# Parity: VideoPlayer

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/video-player.md`
- Svelte (authoritative): `packages/svelte/components/src/VideoPlayer.svelte`
- GPUI: `packages/gpui/components/src/composites/video_player.rs`
- Jetstream: `packages/jetstream/components/src/video_player.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/VideoPlayerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/video_player_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/video_player.rs`

## Contract ↔ Svelte

Svelte and contract are largely aligned (props, anatomy, ARIA, token tables all match). Remaining divergences:

- [x] FIXED **Layer mismatch.** Contract §1 declared layer `primitives`; the GPUI impl lives in `composites/video_player.rs` (alongside `relation_picker` siblings). Svelte has no layer concept, so it can't directly arbitrate — picked the layer the code actually uses. Updated contract §1 to `composites` and added the `poodle_gpui::composites::video_player` module-surface line to §10.
- **TimeDisplay separator.** Contract §2 / §8 `.video-player__time` says `m:ss / m:ss`; Svelte renders `{formattedCurrent} / {formattedDuration}` = `m:ss / m:ss`. OK, matches.
- **Big play visibility.** Contract §4 "paused at currentTime=0"; Svelte gate is `!isPlaying && currentTime === 0` (`VideoPlayer.svelte:189`). OK.
- Otherwise contract↔Svelte is faithful; the `consv=gap` is driven solely by the layer/path mismatch above. Everything else (9 props, anatomy parts, all ARIA labels, size/density tables) matches Svelte exactly.

## GPUI gap (vs Svelte + contract)

GPUI renders chrome only (no playback) — accepted per contract §10. Remaining visual/token gaps:

- [x] FIXED All geometry resolves from the `VideoPlayerSpec` rem ladders via `rem_to_px` (`button_size_rem`, `big_play_size_rem`, `volume_width_rem`, `time_font_rem`, `track_height_rem`, `volume_thumb_rem`, `bar_gap_rem`). No raw `px(64.0)`-style literals; the only remaining numeric px is the 220px (13.75rem) chrome `min_h`, expressed as `rem_to_px(13.75)`.
- [x] FIXED `size` scaling honored — `effective_size = resolve_semantic_size(size, size_role)` feeds every size-driven dimension (button, big-play, volume, time-font, icon glyph). Contract §8 size variants flow through.
- [x] FIXED Progress track bg is fixed `rgba(255,255,255,0.2)` (`gpui::white().opacity(0.2)`); fill is `color.accent.base` per contract `.progress-fill`. No more theme text-inverse at the wrong alpha.
- [x] FIXED Volume slider rendered — `volume_width_rem`-wide track (`white 50%`) with a `0.625rem` white thumb positioned by the volume fraction.
- [x] FIXED Seek fill is proportional via `relative(frac)` (contract requires a transparent `input[type=range]` overlay for interaction, which is preview-loop owned in GPUI; the visual fill is now correct).
- [x] FIXED Time format is `m:ss / m:ss` via `format_time`, rendered in `.font_family("monospace")` at the per-size `time_font_rem`.
- [x] FIXED Big play is a transparent ring (`border_2` white-90, no fill) with a centered `play` Icon, shown only when `!is_playing && current_time <= 0` (contract §4 paused-at-0).
- accepted: no real `<video>` playback / fullscreen / auto-hide controls (contract §10 — platform media integration absent); interaction is preview-loop owned.
- accepted: no ARIA (gpui has no accessibility API).

## Jetstream gap (vs Svelte + contract)

Jetstream is the stronger Rust impl: size/density driven from contract tables, fixed-white colors correct. Remaining gaps:

- [x] Progress fill width bug — FIXED: `.w((progress * 100.0))` rendered a fixed ≤100px sliver (progress is a 0..1 fraction). Now uses the runtime `ui_element::progress(frac)` ProgressBar widget for a proportional fill.
- [x] FIXED Volume slider now renders a `0.625rem` white thumb (`volume_thumb_rem()`) positioned by the volume fraction within a `[filled | thumb | rest]` track layout.
- [x] FIXED Big-play + progress radii use the named `VideoPlayerSpec::pill_radius_rem()` constant instead of bare `999.0` literals.
- [x] FIXED Transport controls render real Icon glyphs (`play`/`pause`, `volume-2`/`volume-x`, `maximize-2`/`minimize-2`) that swap with state, instead of plain button text labels. The seek bar remains the runtime `progress(frac)` ProgressBar widget.
- [ ] No real playback / fullscreen / auto-hide — interaction (play/seek/mute/fullscreen, Space/Enter) must live in preview `main.rs` event loop; buttons are `.focusable()` with no handlers. (accepted runtime limit — the `js_*` builder is stateless.)
- accepted: white-on-black hardcoded colors are correct per contract §8 (intentional, not theme tokens).
- accepted: no ARIA channel (Jetstream has no a11y tree); aria-labels carried as fallback button text alongside the icon glyph.

## Specimen parity

- Svelte covers: Video player (default 16:9), Custom aspect ratio (4:3). (Contract §13 defines exactly these two.)
- GPUI covers: Basic, Playback states (playing + fullscreen+poster), Presentation variants (Sm/compact, prominent). — exceeds contract spec set; missing nothing from contract but does not demonstrate the **4:3 custom aspect** case from §13 as a labeled group (uses `AspectRatio::Landscape` enum, not 4:3).
- Jetstream covers: Paused, Playing, With poster. — missing: **Custom aspect ratio (4:3)** group from contract §13.

## Notes

- The `consv=gap` is driven only by the layer/path mismatch (contract says `primitives`, GPUI impl is in `composites/`). Prop/anatomy/ARIA parity between contract and Svelte is otherwise clean.
- GPUI is materially behind Jetstream here: Jetstream scales every dimension from the contract size/density tables, GPUI hardcodes px and ignores size. The GPUI progress-track color and time format are the most visible deviations.
- Both Rust targets correctly treat the player as black-background chrome; neither attempts real video decode (accepted, contract §10).
