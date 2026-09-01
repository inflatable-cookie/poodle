# Value Track: Semantic Interaction Cues

Status: complete (evidence documented; recommendation requires operator decisions)
Created: 2026-09-01
Updated: 2026-09-01
Origin: ready card `docs/roadmaps/g16/042-semantic-interaction-cues-research.md`
Scope: optional semantic interaction cue roles across web and native hosts;
Jetstream remains deferred

This is a point-in-time research dossier. It proposes a bounded Poodle
direction; it does not change a contract, add an API, or implement a
component. `CuePolicy`, `full | muted | silent`, and a closed role list are
hypotheses under the card, not public API decisions.

Evidence labels used below:

- **[LF] Local fact** — observed in this worktree at the cited path.
- **[SF] Source fact** — stated by an external standard, platform, or library
  source at the cited URL; checked 2026-09-01 unless noted.
- **[WI] Worker inference** — Poodle-specific synthesis or recommendation from
  those facts.

## Executive Summary

Sound and haptic cues are viable interaction feedback in web products — two
independent zero-dependency Web Audio synthesis libraries prove it — but
Poodle has no defensible owner seat for them today, and the strongest evidence
points to **consumer-owned**, with a documented boundary, rather than
architecture, recipe, or a Poodle-owned role API.

Three facts drive the recommendation:

1. **No platform expresses a sound-reduction preference or a detectable mute
   state.** Media Queries Level 5 defines `prefers-reduced-motion`,
   `prefers-reduced-transparency`, `prefers-contrast`, and
   `prefers-reduced-data` but no audio/sound/mute query. Web pages cannot
   detect the OS mute state or the iOS silent switch (WebKit bug 237322).
   macOS exposes no system-mute API to apps; Windows exposes only per-session
   mute for the app's own session. A motion-style `full | reduced | frozen`
   policy has no canonical input to bind `reduced` to.
2. **The active cohort cannot carry the surface.** GPUI 0.2.2 exposes no
   audio, haptic, or accessibility API at all (003-native-accessibility,
   hub-gpui). Under the working rules, a capability present in Svelte/React
   and absent from GPUI is a failing completion condition, not an accepted
   delta. A Poodle-owned cue surface would create an unclosable native gap
   until a separate host-audio capability decision is made.
3. **Correctness is already audio-free by law.** Poodle's contracts and the
   motion policy give every semantic state a visual, focus, and
   announcement channel (architecture 012 mode law; WCAG 1.3.3). Cues would
   add no semantic state; they are pure feedback attached to transitions the
   contracts already expose.

**Recommendation: do not add a CuePolicy, cue roles, synthesis, samples,
haptics, or an accessibility promise in the active cohort.** Consumers own
cues: they map Poodle's existing explicit state transitions and events to
their own sound/haptic engine (Cuelume, @web-kits/audio, or a native engine),
and they own preference storage, volume, and product character — matching the
card's boundary that product-specific sound stays consumer-owned. Poodle's
contribution today is the event/state surface it already documents; no new
API is needed for that mapping. If the operator later wants a shared
vocabulary, the bounded path is a **semantic cue-role recipe** (a small closed
set of meanings attached to existing transitions, no waveforms, no policy)
and only after a native audio capability decision exists — never before.

## Method And Source Inventory

### Method

Access/check date for all external sources below: **2026-09-01**. The local
audit read the repository authority chain before source inspection:

- `AGENTS.md`, the ready card, the worker handoff, `docs/README.md`, and the
  g16 roadmap;
- architecture: `docs/architecture/001-poodle-system-shape.md`,
  `006-headless-core-and-machine-model.md`,
  `008-audio-control-family.md`,
  `010-native-presentation-construction-context.md`,
  `012-semantic-motion-policy.md`, and `product-guardrails.md`;
- contracts/specs: `docs/contracts/001-working-rules.md`,
  `docs/contracts/003-native-accessibility.md`,
  `docs/contracts/components/motion-policy-provider.md`,
  `docs/contracts/components/toast-stack.md`, and
  `docs/specs/069-dependable-drag-and-drop-substrate.md`;
- the two pinned library sources cloned to fixed commits and read in full
  (engine, context, recipes, bindings, CLI, tests, licenses);
- a full-tree grep of `packages/` for any audio/haptic implementation;
- platform guidance from Apple, Microsoft, Google, W3C, WHATWG, and WebKit
  primary sources.

External research prioritized normative platform/accessibility sources, then
the two library precedents. No third-party code or asset is copied by this
dossier.

### Pinned primary sources

| Source | Pin | Evidence used | Licence/access record |
| --- | --- | --- | --- |
| [Cuelume](https://github.com/Danilaa1/cuelume) | commit `b879b72c01f3b3fa74c45c9b20bbd064baffb282` (2026-08-04, tag `0.2.2`), npm `cuelume` 0.2.2 | 17-cue synthesized palette; shared lazy `AudioContext`; `navigator.userActivation` gate; `setEnabled`/`setVolume` with app-owned preference storage; `bind()` delegated `data-cuelume-*` wiring; hover throttle; limiter; cleanup; runtime test | MIT © 2026 Daniel Belyi. Zero runtime dependencies; no audio files. Site [cuelume.dev](https://cuelume.dev) checked 2026-09-01. |
| [@web-kits/audio](https://github.com/raphaelsalaja/audio) | commit `3a9fe941c589d26d3487db17f5183eb9cecf3258` (2026-04-26, release `#14`); package `0.2.0` at HEAD; npm published `0.1.0` | Declarative `defineSound` synthesis engine; `SoundPatch` JSON + CLI/registry; `OfflineAudioContext` `renderToBuffer`/`renderToWav` determinism path; `VoiceHandle.stop`; master bus; React `useSound` gated on `prefers-reduced-motion` + provider state; jitter option | MIT © 2026 Raphael Salaja. Site [audio.raphaelsalaja.com](https://audio.raphaelsalaja.com) checked 2026-09-01. Registry API base `https://audio.raphaelsalaja.com/api` ([LF] `packages/audio/src/commands/utils.ts:6` in the pinned clone). In-repo patch sets (`core`, `crisp`, `drums`, `mechanical`, `minimal`, `organic`, `playful`, `retro`, `soft`, `synths`) carry no per-file license field; the repository is MIT. |

### Normative and platform sources

| Source | Evidence used | Licence/access record |
| --- | --- | --- |
| [Apple HIG — Playing audio](https://developer.apple.com/design/human-interface-guidelines/playing-audio) | Silent mode plays only explicitly initiated audio; system volume governs final output; ambient/solo-ambient categories respond to the silence switch, `playback` does not; avoid communicating important information using only sound | Apple HIG; checked 2026-09-01. Standalone "Sound"/"Haptics" HIG URLs return 404; content moved here and to "Playing haptics". |
| [Apple HIG — Playing haptics](https://developer.apple.com/design/human-interface-guidelines/playing-haptics) | Haptics on supported iPhones, Apple Watch, Macs with Force Touch trackpad; "Make haptics optional. Let people turn off or mute haptics" | Apple HIG; checked 2026-09-01. |
| [AVAudioSession](https://developer.apple.com/documentation/avfaudio/avaudiosession) | Default session: silent mode and device lock silence app audio; `.playback` category continues with the switch in silent mode (iOS only) | Apple developer documentation; checked 2026-09-01. |
| [AVAudioSession.isOutputMuted](https://developer.apple.com/documentation/avfaudio/avaudiosession/isoutputmuted) | iOS 17+ exposes output mute state, `setOutputMuted(_:)`, `outputMuteStateChangeNotification`; historically no API exposed the silent switch | Apple developer documentation; checked 2026-09-01. |
| [UIImpactFeedbackGenerator](https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator) and [Core Haptics](https://developer.apple.com/documentation/corehaptics) | UIKit impact/selection/notification feedback generators; Core Haptics (iOS 13+) custom haptic+audio patterns | Apple developer documentation; checked 2026-09-01. |
| [NSHapticFeedbackManager](https://developer.apple.com/documentation/appkit/nshapticfeedbackmanager) | macOS haptics require a Force Touch trackpad; `defaultPerformer` depends on input device, accessibility settings, and user preferences; patterns `alignment`, `levelChange`, `generic` | Apple developer documentation; checked 2026-09-01. |
| [iPhone User Guide — sounds and vibrations](https://support.apple.com/guide/iphone/change-the-sounds-and-vibrations-iph07c867f28/ios) | Settings > Sounds & Haptics: System Haptics toggle; ringtone/alert haptics "Always Play / Play in Silent Mode / Don't Play in Silent Mode / Never Play" | Apple Support; checked 2026-09-01. |
| [Windows Core Audio — ISimpleAudioVolume](https://learn.microsoft.com/en-us/windows/win32/api/audioclient/nn-audioclient-isimpleaudiovolume) and [Volume Controls](https://learn.microsoft.com/en-us/windows/win32/coreaudio/volume-controls) | Per-session master volume/mute via WASAPI; an app can change only its own sessions; users control per-app volume in the mixer | Microsoft Learn; checked 2026-09-01. |
| [Windows accessibility — Make Windows easier to hear](https://support.microsoft.com/en-us/accessibility/windows/make-windows-easier-to-hear) | Mono audio; "Flash my screen during audio notifications"; live captions — visual alternatives instead of relying on sound alone | Microsoft Support; checked 2026-09-01. |
| [ShowSounds in applications](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/dnacc/supporting-showsounds-in-your-applications) | ShowSounds instructs apps to display visual equivalents for speech and sounds; Microsoft recommends all applications provide such support | Microsoft Learn; checked 2026-09-01. |
| [Android — Audio focus](https://developer.android.com/guide/topics/media-apps/audio-focus) | Android 12+ (API 31) system-managed focus; automatic ducking since API 26; request/abandon focus; transient vs permanent loss handling; API 35+ focus only for top apps or foreground services | Android developer documentation; checked 2026-09-01. |
| [Android — Settings.System.HAPTIC_FEEDBACK_ENABLED](https://developer.android.com/reference/android/provider/Settings.System) and [VibrationAttributes](https://developer.android.com/reference/android/os/VibrationAttributes) | Haptic-feedback setting deprecated in API 33 in favor of `VibrationAttributes.USAGE_TOUCH`; user settings are applied by the system service, not apps | Android developer documentation; checked 2026-09-01. |
| [Android — NotificationChannel.enableVibration](https://developer.android.com/reference/android/app/NotificationChannel) | Per-channel vibration, user-editable via notification settings | Android developer documentation; checked 2026-09-01. |
| [Chrome autoplay policy](https://developer.chrome.com/blog/autoplay/) | Muted autoplay always allowed; sound requires user interaction, MEI threshold, or installed PWA; `play()` without gesture rejects with `NotAllowedError`; `AudioContext` created before a user gesture starts `suspended` and needs `resume()` | Chrome Developers blog; checked 2026-09-01. |
| [WebKit — new video policies for iOS](https://webkit.org/blog/6784/new-video-policies-for-ios/) and [macOS autoplay policy](https://webkit.org/blog/7734/auto-play-policy-changes-for-macos/) | iOS requires a user gesture for media with sound; unmuting without a gesture pauses; macOS Safari blocks sound autoplay by default and assumes a gesture is required | WebKit blog; checked 2026-09-01. |
| [Apple — HTML5 audio and video, Web Audio](https://developer.apple.com/library/archive/documentation/AudioVideo/Conceptual/Using_HTML5_Audio_Video/PlayingandSynthesizingSounds/PlayingandSynthesizingSounds.html) | On iOS, Web Audio requires sounds to be triggered from an explicit user action such as a tap | Apple archive documentation; checked 2026-09-01. |
| [WebKit bug 237322](https://bugs.webkit.org/show_bug.cgi?id=237322) | Web Audio output follows the iOS ringer/silent switch; no web API exposes the silent-switch or system-mute state | WebKit Bugzilla; checked 2026-09-01. |
| [Media Queries Level 5](https://drafts.csswg.org/mediaqueries-5/) | User-preference features are `prefers-reduced-motion`, `prefers-reduced-transparency`, `prefers-contrast`, `forced-colors`, `prefers-color-scheme`, `prefers-reduced-data`; no audio/sound/mute query exists | W3C Working Draft 2026-06-29; checked 2026-09-01. |
| [WCAG 2.2](https://www.w3.org/TR/WCAG22/) | 1.4.2 Audio Control; 1.4.7 Low or No Background Audio; 1.3.3 Sensory Characteristics; 3.2.5 Change on Request; 1.2.x media alternatives | W3C Recommendation; checked 2026-09-01. |
| [WAI-ARIA 1.2](https://www.w3.org/TR/wai-aria-1.2/) | No role, state, or property for sound cues; the `alert` role is the accessible alternative for audio warnings | W3C Recommendation; checked 2026-09-01. |
| [Apple HIG — Accessibility](https://developer.apple.com/design/human-interface-guidelines/accessibility) | Use haptics in addition to audio cues; pair sound with matching haptics and visual cues; avoid autoplaying audio without controls | Apple HIG; checked 2026-09-01. |
| [NVDA User Guide — audio ducking](https://www.nvaccess.org/files/nvda/documentation/userGuide.html) | NVDA can lower other applications' audio while outputting speech and sounds (No Ducking / Duck when outputting speech and sounds / Always duck) | NV Access; checked 2026-09-01. |

### High-quality precedent

| Precedent | Evidence used | Licence/access record |
| --- | --- | --- |
| [Cuelume](https://cuelume.dev) (pinned above) | Curated 17-cue palette with no samples; declarative attribute binding; app-owned preferences; autoplay-gated engine | MIT; no code copied. |
| [@web-kits/audio](https://audio.raphaelsalaja.com) (pinned above) | Declarative synthesis as data (`SoundPatch`); registry/CLI distribution; offline rendering for determinism; React provider gating sound on motion preference | MIT; no code copied. |

The precedents are evidence of viable design choices, not Poodle authorities.
Poodle's architecture, component contracts, and active-runtime admission
rules remain authoritative.

## Current Poodle Audit

### Authority and architecture

**[LF]** `docs/architecture/001-poodle-system-shape.md:200-203` puts foundation
primitives, reusable composites, and workstation shells in Poodle; applications
own routing, data fetching, persistence, authorization, product language, and
service orchestration. `docs/architecture/001-poodle-system-shape.md:38-56`
ranks parity as semantic states/behavior first, then accessibility, then
tokens/layout, then rendering details.

**[LF]** `docs/architecture/006-headless-core-and-machine-model.md:11-31`
defines machines as pure functions with effects; contracts classify
machine-backed / shared-machinery / adapter-owned / styled-only surfaces.

**[LF]** `docs/architecture/008-audio-control-family.md:11-21` places the
audio control family (Fader, Knob, XYPad, meters, keyboard) in Poodle as
control-value and meter semantics; `008:97-101` says host playback does not
fabricate input gestures and `008:196-210` has hosts push aggregate meter
frames. The "audio" in Poodle today is **control signals and metering, not
sound output**.

**[LF]** `docs/architecture/010-native-presentation-construction-context.md:16-33`
and `docs/contracts/components/motion-policy-provider.md:15-18` define the
provider boundary: `RenderContext` carries theme, size/density defaults, and
effective `MotionPolicy`; providers are construction boundaries that add no
layout, paint, accessibility node, focus target, or interaction state. Host
integrations resolve system preference at the edge
(`docs/architecture/012-semantic-motion-policy.md:21-23`); components never
perform ambient discovery.

**[LF]** `docs/architecture/012-semantic-motion-policy.md` mode law: semantic
and accessibility state always updates immediately; motion never owns ARIA,
labels, status text, progress meaning, focus, correctness, or semantic timers.
Its rejected alternatives include ambient runtime discovery, which "cannot
give web orchestration, Rust composition, GPUI, and deterministic capture one
explicit input".

**[LF]** `docs/contracts/components/motion-policy-provider.md:127-135` —
neither the provider contract nor architecture 012 mentions sound or haptics
anywhere.

### Candidate semantic states (where a cue could attach)

Every candidate state below already has explicit visual, focus, and
announcement ownership in its contract. Cues would be additive feedback on
existing transitions, not new state:

| Surface | State vocabulary | Contract |
| --- | --- | --- |
| ToastStack | tones `info | success | warning | danger`; danger escalates to assertive live region; host-driven items, no auto-dismiss | `docs/contracts/components/toast-stack.md:81-84,40-44` |
| Spinner | `ring | dots | grid` variants, animation-only, optional `role="status"` | `docs/contracts/components/spinner.md:57-59,95-97` |
| Skeleton | `animated | static` shimmer, decorative, aria-hidden | `docs/contracts/components/skeleton.md:59-61,89-92` |
| Checkbox / Switch | `unchecked|checked|mixed` / `off|on` (+ focus/disabled/readOnly) | `docs/contracts/components/checkbox.md:69-71`, `switch.md:73-75` |
| Accordion / Collapsible | `collapsed|expanded` / `closed|open` per item | `docs/contracts/components/accordion.md:87-89`, `collapsible.md:73-75` |
| Tabs | `idle|selected|focus|disabled|drag-source|drop-target` | `docs/contracts/components/tabs.md:200-203,232-241` |
| Slider | `default|focus|active|disabled`, live change + one commit | `docs/contracts/components/slider.md:73-75,112-130` |
| Select | `placeholder|selected|loading|load error|open|empty results|ghost` | `docs/contracts/components/select.md:156-158,226-244` |
| Dialog | `closed|open` | `docs/contracts/components/dialog.md:125-127,148-162` |
| HistoryCenter | `default|busy|loading|failed|rejected|disclosed` | `docs/contracts/components/history-center.md:192-194,243-252` |
| MarkdownEditor | `edit|preview|split`, mode-active/inactive | `docs/contracts/components/markdown-editor.md:86-88,98-102` |
| Drag and drop | phases `idle→preparing→armed→dragging→dropping→ended/cancelled`; eligibility `accepted|rejected`; commit `committed|rejected|failed`; announcements for pickup, intent, clear, committed, rejected, failed, cancelled | `docs/specs/069-dependable-drag-and-drop-substrate.md:82-90,100-108,64-66` |

**[LF]** Spec 069 already owns an announcement channel for every drag outcome
(`docs/specs/069-dependable-drag-and-drop-substrate.md:481-484`): pickup,
position/target, rejection reason, successful drop, cancellation, throttled
so pointer motion does not flood assistive technology. Drag-drop cues would be
a second feedback channel on top of an existing accessibility channel.

### Existing audio and haptic code

**[LF]** Full-tree grep of `packages/` for
`AudioContext|new Audio(|AudioBuffer|oscillator|haptic|vibration|NSHaptic|
UIImpact|CoreHaptics|beep` returns zero implementation matches. The only
sound-capable surface is `AudioPlayer` (media playback through a hidden
`<audio>` element, a consumer transport, not UI cues; GPUI renders an inert
handle — `packages/react/components/src/AudioPlayer.tsx:35`,
`docs/contracts/components/audio-player.md:44-46`,
`packages/gpui/adapter/src/render_editing_composites.rs:24-35`).

**[LF]** The "continuous audio machine" (`g16.031`/`g16.032`) is a
continuous-gesture/value model for DAW controls — explicitly not payload
drag-and-drop, and with no `AudioContext`, unlock, or synthesis anywhere
(`docs/roadmaps/g16/031-continuous-audio-machine-and-web-lifecycle.md:15-21`,
`packages/core/src/audio/`). Its "web lifecycle" is pointer/entry lifecycle,
not audio lifecycle.

### Native capability limits

**[LF]** `docs/contracts/003-native-accessibility.md:11-19,70-77` — GPUI 0.2.2
exposes no accessibility tree (7 AX elements, window chrome only) and no
audio or haptic API; `packages/gpui/native-accessibility-proof.json:12-13`
records status `manual`, assistive-technology evidence `missing`. Hub-gpui and
hub-bits contain zero mentions of audio, haptic, sound, or vibration.

**[LF]** The working rules (`docs/contracts/001-working-rules.md`) treat a
capability present in Svelte and absent from another active runtime as a gap
to port, not an accepted delta; capability absence must be declared with a
reason and does not count as parity.

## Detailed Findings

### 1. The pinned engines prove web synthesis is real, small, and dependency-free

**[SF]** Cuelume 0.2.2 (pin `b879b72c…`) is a curated palette of 17 cues
(`chime sparkle droplet bloom whisper tick press release toggle success error
page loading ready pulse scan arrival`), every sound synthesized live through
one shared lazy `AudioContext`, no audio files, zero runtime dependencies,
MIT. The site claims all seventeen sounds together are smaller than one MP3
click; the published package is ~2 KB-class.

**[LF]** Pinned engine facts (`/tmp/cuelume-pin/src/audio/engine.ts`): `play`
is a no-op when `navigator.userActivation?.hasBeenActive === false` (autoplay
gate, proven by its runtime test), lazily creates the context, `resume()`s a
suspended context before rendering, and catches synchronous constructor
throws; `setEnabled`/`setVolume` are global with "preference storage stays
with the app"; output runs through a master gain and a `DynamicsCompressor`
limiter (−8 dB threshold, 12:1); nodes are disconnected via `setTimeout`
after the recipe tail. `bind()` wires delegated `data-cuelume-hover`
(mouse-only via `(hover: hover) and (pointer: fine)`, throttled to one tick
per 150 ms), `data-cuelume-press` (pointerdown), `data-cuelume-release`
(pointerup), and `data-cuelume-toggle` (click) — `src/interactions/bind.ts`.

**[SF]** @web-kits/audio (pin `3a9fe94…`, MIT) is a declarative synthesizer:
sounds are plain JSON-serializable objects (`SoundDefinition`/`SoundPatch`
with `$schema`), played through `defineSound`; sources include oscillators,
procedural white/pink/brown noise, wavetables, URL samples, and streams;
envelopes, filters, LFOs, 3D panners, and an effects chain; each voice returns
a `VoiceHandle` with `stop(releaseTime)`; one shared context with a master
bus, `dispose()`, and `ensureReady()`.

**[SF]** @web-kits/audio ships a deterministic evidence path absent from
Cuelume: `renderToBuffer` renders a definition through an
`OfflineAudioContext` into an in-memory `AudioBuffer`, and `renderToWav`
encodes 16-bit PCM WAV — no speakers involved (`packages/audio/src/offline.ts`
in the pin).

**[SF]** @web-kits/audio React integration (`useSound`) gates playback on
provider state (`enabled`, `volume`) **and** on `prefers-reduced-motion`
(`packages/audio/src/react.tsx` in the pin). This is a design choice, not a
standard: MQ5 defines no sound-reduction preference, so sound is being
conservatively tied to the motion signal.

**[WI]** Together they prove the two shapes a Poodle cue surface could borrow:
an engine-owned synthesis palette with app-owned preferences (Cuelume) and a
data-defined sound vocabulary distributed as patches with offline rendering
for evidence (@web-kits/audio). Neither requires Poodle to own anything; both
are exactly the consumer-owned pattern the card's boundary describes.

### 2. No policy input exists: availability, preference, and capability cannot share one axis

**[SF]** MQ5 (WD 2026-06-29) defines `prefers-reduced-motion`,
`prefers-reduced-transparency`, `prefers-contrast`, `forced-colors`,
`prefers-color-scheme`, and `prefers-reduced-data` — no audio/sound/mute
query. `prefers-reduced-motion` is explicitly motion-only.

**[SF]** Web pages cannot detect the OS mute state or the iOS silent switch:
WebKit bug 237322 records that Web Audio output follows the iOS ringer mute,
with no API to read it. iOS 17+ added `AVAudioSession.isOutputMuted` for
native apps; macOS exposes no system-mute API; Windows exposes
`ISimpleAudioVolume` per-session mute for the app's own session only.

**[SF]** Chrome's autoplay policy gates sound on user interaction, the Media
Engagement Index, or installed-PWA status; `AudioContext` created before a
user gesture starts `suspended`; iOS Safari requires sounds to be triggered
from an explicit user action.

**[WI]** A motion-style policy (`full | reduced | frozen`) models a
restriction ladder with one canonical platform input (motion preference) and
one evidence input (capture). For sound, "reduced" has no canonical signal to
resolve from: mute is a hard off, not a reduction; volume is continuous and
system-governed; capability is browser-gated by gesture; and the one library
precedent for a "reduced" signal (web-kits' reduced-motion gate) couples sound
to a different sensory preference. A Poodle `CuePolicy` would have to be a
triple (capability × preference × host policy) with no standard input for two
of the three axes — the opposite of motion's single-source design.
Restriction-only inheritance is implementable (a muted subtree stays muted)
but there is no evidence any consumer needs subtree muting; the natural
granularity is app-level preference and per-call volume, which Cuelume already
demonstrates without a policy.

### 3. Runtime ownership is almost entirely host-owned

**[SF]** First-gesture unlock is browser-owned on web (user activation,
suspended `AudioContext`, `NotAllowedError`) and platform-owned on native
(iOS silent switch via session category; Android audio focus request with
API 31+ system-managed focus; Windows per-session volume). Apple HIG: system
volume always governs final output; silent mode silences nonessential sound.

**[SF]** Assistive-technology interaction is screen-reader-owned: NVDA can
duck other applications' audio; Apple HIG says to pair audio cues with
haptics and visual cues and to avoid communicating important information using
only sound; Windows ShowSounds instructs apps to provide visual equivalents
for sounds; WCAG 1.3.3 forbids instructions that rely solely on sound.

**[SF]** Android 12+ fades or mutes playback on focus loss and on incoming
calls, and API 35+ restricts focus requests to top apps or foreground
services; Windows per-app volume is user-controlled in the mixer; iOS device
lock silences the default session. Background/teardown behavior is therefore
enforced by the platform whether or not an app cooperates.

**[WI]** The only genuinely Poodle-owned concerns are **concurrency and
repetition policy** (what happens when two cues fire at once, or the same cue
repeats) — and the pinned engines show these are engine-local: Cuelume
renders fresh voices into a limiter with a 150 ms hover throttle; web-kits
gives per-voice handles with `stop()` and optional jitter. Poodle would
duplicate engine territory to own them, for zero cross-runtime benefit,
because native hosts cannot receive the same policy anyway.

### 4. Meaning can be shared without sharing waveforms

**[SF]** Both pinned libraries synthesize from parameterized recipes
(frequency, envelope, noise color, filters) rather than recorded samples;
@web-kits/audio additionally makes the definition itself a portable JSON
document. Nothing about the semantic meaning (success vs error vs busy vs
arrival) is encoded in the waveform — the same role name can be rendered by
different engines on different hosts.

**[WI]** If Poodle ever wanted to own a cue vocabulary, the defensible shape
is a small closed set of **semantic cue roles** attached to transitions the
contracts already expose (complete/confirm, error, busy/ready, arrival,
toggle-flip, drag accept/refuse/commit), with waveform synthesis explicitly
consumer- and engine-owned. That is the card's "recipe" hypothesis: Poodle
names the meaning and the attachment point; consumers and engines render it.
It requires no CuePolicy, no samples, no synthesis code, and no haptic
support. The live question is whether a shared role list has enough value to
justify a contract surface; the dossier's evidence says no consumer or
in-repo precedent exists for such a list, and Cuelume's own naming proves
engines can converge on a vocabulary without coordination.

### 5. Haptics are a separate capability, not part of the first family

**[SF]** Apple: haptics exist only on supported devices (iPhone Taptic
Engine, Apple Watch, Macs with Force Touch trackpad), are user-mutable via
Settings (System Haptics toggle; per-alert "Always/Play in Silent Mode/Don't
Play in Silent Mode/Never"), and HIG says to make haptics optional.
`NSHapticFeedbackManager.defaultPerformer` is chosen from the current input
device, accessibility settings, and user preferences — the app cannot force
it. Android: haptic settings are system-applied via
`VibrationAttributes.USAGE_TOUCH` (API 33+ deprecates the old setting read);
web: `navigator.vibrate` exists but has no iOS support and no settings
integration.

**[WI]** Haptics have their own hardware matrix, their own settings gates,
and no cross-platform abstraction; they are also the one cue channel GPUI
could plausibly reach someday (a backend could forward a semantic request to
the platform). They should be evaluated as a separate later capability only
after a semantic role surface exists — never folded into the first sound
policy. Apple's "pair audio with haptics" guidance supports designing the
semantic role so that both channels can attach to it later, which the
existing state vocabulary already allows.

### 6. Edge scenarios the card requires

- **Muted environments:** web cannot detect mute or silent switch (WebKit bug
  237322); iOS Web Audio follows the ringer mute; macOS exposes no mute API.
  A cue system must be a graceful no-op with zero correctness dependency —
  which Poodle's audio-free semantics already guarantee by construction.
- **Missing devices / headless:** `AudioContext` construction can throw or be
  absent (SSR); Cuelume catches and no-ops. Deterministic capture must not
  depend on an audio device: the @web-kits/audio `OfflineAudioContext` path
  renders to a buffer in memory, and a capture policy can simply request
  silence.
- **Repeated cues and overlap:** the motion identity law (latest-state-wins,
  keyed by semantic owner + role) is the right *shape* of concurrency law for
  cues, but its cross-runtime enforcement (rendered clocks) has no native
  counterpart; at minimum a cue policy would need a per-owner, per-role
  dedupe and a repeat throttle (Cuelume's 150 ms hover gap is the precedent).
- **Failure:** context resume rejection, limiter behavior, and blocked
  creation must fail silent; no callback, no error surface.
- **Reduced sensory preference:** no standard signal exists (Finding 2);
  apps own their sound settings (Cuelume `setEnabled`); the web-kits
  reduced-motion gate is a conservative consumer choice, not a standard
  Poodle could normatively adopt.
- **Assistive technology:** live regions and sounds can double-report; NVDA
  can duck app audio; WCAG 1.3.3 and Apple HIG require non-sound channels.
  Any cue design must treat announcements as the correctness channel and
  sound as an optional additive layer — which is exactly the relationship the
  contracts already document.
- **Capture/testing determinism:** web capture can mute playback outright or
  render offline (web-kits `renderToBuffer`); jitter and scheduling must be
  pinnable; a future capture policy would be "silent", trivially — no frozen
  analog needed.

### 7. The active cohort cannot host the surface

**[LF]** GPUI 0.2.2 has no audio, haptic, or accessibility API
(`docs/contracts/003-native-accessibility.md`, hub-gpui, grep of `packages/`).
Under the working rules, Svelte-only capability is a gap to port, and a
declared absence is debt, not parity. A Poodle-owned cue role surface would
therefore be born with a failing native completion condition, and Jetstream
is deferred under its admission status — the cohort cannot close the gap on
any horizon inside this card.

## Recommendation

**Consumer-owned, now; no CuePolicy, no role API, no synthesis, no haptics,
no accessibility promise in the active cohort.**

1. Poodle keeps its documented state/event surfaces as the cue attachment
   points. Consumers map their own cue engines onto existing transitions
   (toast arrival, spinner/ready, checkbox/switch flip, drag accept/refuse/
   commit, select loading/error) with zero Poodle changes. The pinned
   libraries are exactly this pattern and prove it works.
2. Poodle does not add a motion-style cue policy: there is no canonical
   availability, preference, or capability signal to bind it to, and the
   native cohort cannot receive it (Findings 2 and 7).
3. Poodle does not absorb sound design: synthesized palettes and patches are
   product character; the card's boundary ("product-specific sound stays
   consumer-owned", "reject product-specific jingles and sample libraries")
   is consistent with every source examined. Cuelume's palette and web-kits'
   patch registry are consumer-side artifacts, MIT-licensed, and need no
   Poodle wrapper.
4. A future semantic **cue-role recipe** (closed meaning list on existing
   transitions, no waveforms) is the only promotion worth revisiting, and it
   is gated: (a) a native audio/haptic capability decision for GPUI or an
   explicit host-bridge pattern, (b) an operator decision that a shared
   vocabulary is valuable without default sounds, (c) an evidence plan using
   offline rendering or explicit silence, and (d) a naming decision that does
   not collide with motion policy. Nothing in this dossier blocks that
   decision; nothing in it justifies making it now.

This is not a rejection of cues as a product feature — it is a rejection of
Poodle ownership. The evidence shows the value is in consumer control, the
risk is in Poodle absorbing an engine role it cannot complete across the
cohort.

## Unresolved Operator Gates

- Whether a shared semantic cue-role vocabulary is worth a contract surface
  with no default synthesis (recommend: not yet).
- Whether GPUI should ever receive a host-owned audio/haptic bridge (outside
  this card; prerequisite for any Poodle-owned surface).
- Whether "silent capture" is a research/programme goal for web evidence
  (recommend: use explicit mute or offline rendering, no new policy).
- Whether haptics should be tracked as a separate future capability
  (recommend: track, do not design into the first surface).

## Licensing Record

- Cuelume: MIT, © 2026 Daniel Belyi, pin `b879b72c…`. Zero runtime
  dependencies; no audio assets; no code copied.
- @web-kits/audio: MIT, © 2026 Raphael Salaja, pin `3a9fe94…`. Core engine
  has no runtime dependencies (React integration is a separate entry point);
  patch JSONs carry no per-file license but live in an MIT repository; the
  registry service (`audio.raphaelsalaja.com/api`) and GitHub-repo patch
  installation are consumer distribution channels. No code copied.
- No sample library, recording, waveform, or product jingle was evaluated as
  a Poodle asset; the card's sample-library rejection is consistent with the
  evidence.

## Related

- Ready card: `../../roadmaps/g16/042-semantic-interaction-cues-research.md`
- Governing architecture: `../../architecture/012-semantic-motion-policy.md`,
  `../../architecture/008-audio-control-family.md`,
  `../../architecture/010-native-presentation-construction-context.md`
- Governing contracts: `../../contracts/001-working-rules.md`,
  `../../contracts/components/motion-policy-provider.md`,
  `../../contracts/003-native-accessibility.md`
- Parallel lanes: `g16.037`–`g16.044` research dossiers; `g16.036` serial

## Follow-up

Orchestrator owns promotion. If the operator accepts consumer-owned, close the
card with this dossier as the boundary record. If the operator wants the
cue-role recipe option evaluated, the next card must first carry a native
audio/haptic capability decision and an evidence plan (offline rendering or
explicit silence); no implementation is implied by this dossier.
