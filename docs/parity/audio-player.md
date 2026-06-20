<!-- parity consv=ok gpui=8 jetstream=7 specimen=gap -->
# Parity: AudioPlayer

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/audio-player.md`
- Svelte (authoritative): `packages/svelte/components/src/AudioPlayer.svelte`
- GPUI: `packages/gpui/components/src/composites/audio_player.rs`
- Jetstream: `packages/jetstream/components/src/audio_player.rs`
- Spec: `packages/contracts/components/src/audio_player.rs` (`AudioPlayerSpec`)
- Specimens: svelte `packages/svelte/preview/src/specimens/AudioPlayerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/audio_player_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/audio_player.rs`

## Contract ↔ Svelte

Contract §2/§3/§8 match the Svelte implementation prop-for-prop and token-for-token (gap/pad/button-size/icon-size/time-width/volume-width/speed-x/-y, all size + density tables verified against `AudioPlayer.svelte:213-396`). No divergences. `consv=ok`.

- Props (`src`, `ariaLabel`, `showSpeedControl`, `size`, `sizeRole`, `density`) and defaults match §3.
- Anatomy parts (audio, play, current-time, seek, total-time, mute, volume, speed-select) all present (lines 124-210).
- ARIA labels for play/mute/seek/volume/speed and `aria-hidden` icons match §6.
- Speed options `[0.5, 0.75, 1, 1.25, 1.5, 2]` match §2 (line 37).

## GPUI gap (vs Svelte + contract)

GPUI renders UI chrome only (no audio playback — accepted, file header). Visual/token gaps remain.

- [ ] Hardcoded button size `.w(px(32.0)).h(px(32.0))` at `audio_player.rs:136-137` and `:158-159` — must resolve from the size-driven `button-size` token (xs 1.5 … xl 2.5rem, contract §8); currently fixed 2rem regardless of size.
- [ ] Hardcoded seek track `.h(px(4.0))` + `.rounded(px(2.0))` at `:98,:100,:105` — contract track height `0.25rem`, radius `0.125rem`; resolve via rem_to_px, no float px literals.
- [ ] Density pad-y uses inline `match` with `rem_to_px(0.375/0.5/0.625)` literals (`:61-65`) instead of a token method on the spec. Move to a spec token (`pad_y_token`) or documented presentation helper.
- [ ] Icon size fixed to `IconSize::Sm` (`:84,:117`) — does not scale with the size variant (contract icon-size 0.875…1.25rem); thread effective size into icon sizing (`_font_size` is computed then discarded at `:57`).
- [ ] No CurrentTime / TotalTime split: GPUI renders a single combined `"{cur}s / {dur}s"` label (`:90-93`) using a raw `s`-suffixed seconds format, not two monospace `m:ss` spans (contract §2 CurrentTime + TotalTime, `code-family`). Jetstream formats `m:ss` correctly — GPUI should too.
- [ ] No volume slider rendered — only play, time, seek track, mute (`:122-167`). Contract requires a `VolumeSlider` part. Add a volume track (Jetstream has one).
- [ ] No speed select rendered — `spec.show_speed_control` is never read; contract §2 SpeedSelect part missing.
- [ ] Time label color uses `color.text.secondary` (`:71`) but not `code-family` monospace typography (contract §8 `.audio-player__time` font-family). Apply monospace.
- accepted: no ARIA (gpui has no accessibility API) — play/mute/seek/volume/speed labels not emitted.
- accepted: no real audio element; playback state is static from spec.

## Jetstream gap (vs Svelte + contract)

`js_audio_player` is the most complete of the three Rust impls (renders all parts) but carries token-literal violations and one icon gap.

- [ ] Hardcoded `track_height = rem_to_px(0.25)` and `min_w(rem_to_px(4.0))` (`audio_player.rs:68,:102`) — acceptable rem-derived values but should map to documented seek-track/min-width tokens, not inline floats.
- [ ] Hardcoded `.border(1.0)` (`:73`) and `.rounded(999.0)` button/track radii (`:84,:99,:105,:124,:131,:136`) — border-width and pill radius should resolve from tokens (`border.width`, a pill-radius token), not raw floats.
- [ ] Play/mute buttons render text labels `"Play"`/`"Pause"`/`"Mute"`/`"Unmute"` (`:79-86,:119-126`) instead of play/pause/volume icons (contract §2 PlayIcon/MuteIcon `<svg>`). GPUI uses Icon glyphs; Jetstream should use the icon registry (`play`/`pause`/`volume-2`/`volume-x`).
- [ ] Volume track fill is wrong: inner fill width is `vol_frac * volume_width` in px but applied via `.w(f32)` as a flex/px value while the outer track is already `volume_width` wide — the fraction math double-applies width (`:129-139`). Verify fill renders as a proportional sub-bar.
- [ ] Volume + seek tracks both use `accent` for the *filled* portion but the seek base track uses `text_primary` (`:101`) and volume base uses `accent` (`:133`) — contract §8 seek track base = `text-primary`, volume track base = `accent-base`; the inner seek fill should be `text-primary` too (matches), but confirm base vs fill colors per contract.
- [ ] Speed control renders a static `"1x"` label (`:144-147`) — contract §2 SpeedSelect is a `<select>` with all 6 options; Jetstream shows a non-interactive single value. Acceptable as static chrome but should at least show `spec`-derived rate, not hardcoded `"1x"`.
- accepted: no ARIA channel; no real audio playback; interaction (transport clicks) would live in preview event loop — not wired in `main.rs` (grep: no match).

## Specimen parity

- Svelte covers: Basic, With speed control, Sizes (xs–xl, all with speed control), Densities (compact/default/comfortable) — `AudioPlayerSpecimen.svelte`.
- GPUI covers: Basic, Playback states (playing+speed, muted), Semantic presentation (sm+compact, prominent+compact). — missing: explicit per-size sweep; volume + speed parts not rendered by the component so not demonstrable.
- Jetstream covers: Paused, Playing (mid-track), With speed control. — missing: per-size sweep, density sweep, muted state. (`jetstream/.../audio_player.rs`)

## Notes

- Spec carries playback fields (`is_playing`, `current_time`, `duration`, `volume`, `is_muted`, `playback_rate` absent — speed is bool-gated only via `show_speed_control`). Svelte tracks `playbackRate` internally; the Rust spec has no `playback_rate` field, so Jetstream's `"1x"` and GPUI's missing speed select both stem from the spec lacking a rate value. Consider adding `playback_rate: f64` to `AudioPlayerSpec` so static specimens can show non-default speeds.
- Biggest gap: GPUI omits the volume slider and speed select entirely (2 of 8 contract parts missing) and uses a non-`m:ss` combined time label; both Rust targets carry pill-radius/border token literals.
