<!-- parity consv=fixed gpui=7 jetstream=3 specimen=gap -->
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

- [ ] Hardcoded pixel literals: big-play `.size(px(64.0))` (`video_player.rs:91`), viewport `.min_h(px(160.0))` (`:101`), progress `.h(px(4.0))` (`:124`), ctrl button `.w(px(28.0)).h(px(28.0))` (`:188-189`), gaps `.gap(px(6.0))` (`:204,:219`), outer `.min_h(px(220.0))` (`:244`). Contract gives exact rem (big-play `4rem`, button `1.75rem`, progress `0.25rem`) — resolve via `rem_to_px`, not raw px.
- [ ] No `size` scaling — `_effective_size` computed then discarded (`video_player.rs:63`); button/big-play/volume/time do not scale per size. Contract §8 size variants (xs–xl) ignored.
- [ ] Progress track bg `text_color.opacity(0.3)` (`video_player.rs:128`); contract is `rgba(255,255,255,0.2)`. Wrong alpha and wrong source (uses theme text-inverse, not fixed white).
- [ ] No volume slider — control bar omits the `3.5rem` volume range entirely (contract §2/§8 require it; Svelte renders it).
- [ ] No seek slider overlay — progress bar is display-only, no transparent `input[type=range]` overlay (contract `.video-player__seek`).
- [ ] Time format wrong — `format!("{:.0}s / {:.0}s", ...)` → "13s / 42s" (`video_player.rs:138`); contract/Svelte is `m:ss / m:ss` monospace. Also no `typography.code.family` font applied.
- [ ] Big play is a filled circle `bg(overlay)` with pause/play icon (`video_player.rs:88`); contract big-play is transparent bg, ring SVG, only shown when paused-at-0. GPUI shows it always and fills it.
- accepted: no real `<video>` playback / fullscreen / auto-hide controls (contract §10 — platform media integration absent).
- accepted: no ARIA (gpui has no accessibility API).

## Jetstream gap (vs Svelte + contract)

Jetstream is the stronger Rust impl: size/density driven from contract tables, fixed-white colors correct. Remaining gaps:

- [x] Progress fill width bug — FIXED: `.w((progress * 100.0))` rendered a fixed ≤100px sliver (progress is a 0..1 fraction). Now uses the runtime `ui_element::progress(frac)` ProgressBar widget for a proportional fill.
- [ ] Volume slider is a static bar (`video_player.rs:134-139`) with no thumb and no `0.625rem` thumb styling from contract §8; acceptable as chrome but note the thumb is absent.
- [ ] Big-play button radius `.rounded(999.0)` (`video_player.rs:91`) and progress `.rounded(999.0)` (`:104,:108`) use raw `999.0` literal — contract uses `999rem` pill; fine numerically but should be a named pill constant, not a magic float.
- [ ] No real playback / fullscreen / auto-hide — interaction (play/seek/mute/fullscreen, Space/Enter) must live in preview `main.rs` event loop; currently buttons are `.focusable()` with no handlers. (accepted runtime limit, but note no event wiring exists.)
- accepted: white-on-black hardcoded colors are correct per contract §8 (intentional, not theme tokens).
- accepted: no ARIA channel (Jetstream has no a11y tree); labels passed as button text instead.

## Specimen parity

- Svelte covers: Video player (default 16:9), Custom aspect ratio (4:3). (Contract §13 defines exactly these two.)
- GPUI covers: Basic, Playback states (playing + fullscreen+poster), Presentation variants (Sm/compact, prominent). — exceeds contract spec set; missing nothing from contract but does not demonstrate the **4:3 custom aspect** case from §13 as a labeled group (uses `AspectRatio::Landscape` enum, not 4:3).
- Jetstream covers: Paused, Playing, With poster. — missing: **Custom aspect ratio (4:3)** group from contract §13.

## Notes

- The `consv=gap` is driven only by the layer/path mismatch (contract says `primitives`, GPUI impl is in `composites/`). Prop/anatomy/ARIA parity between contract and Svelte is otherwise clean.
- GPUI is materially behind Jetstream here: Jetstream scales every dimension from the contract size/density tables, GPUI hardcodes px and ignores size. The GPUI progress-track color and time format are the most visible deviations.
- Both Rust targets correctly treat the player as black-background chrome; neither attempts real video decode (accepted, contract §10).
