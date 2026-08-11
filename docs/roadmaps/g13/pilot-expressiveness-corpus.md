# g13.002 Pilot Contract Expressiveness Corpus

Status: complete (research precursor)
Milestone: `g13.002` research precursor
Updated: 2026-08-11
Owner: Poodle core
Branch: `thread/g13-pilot-expressiveness-corpus`
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md`
(`IR-01`–`IR-12`), `docs/contracts/components/button.md`,
`docs/contracts/components/range-slider.md`,
`docs/contracts/components/text-input.md`
Batch card: `docs/roadmaps/g13/batch-cards/005-pilot-contract-expressiveness-corpus.md`

## 0. How to read this corpus

This corpus extracts the complete semantic vocabulary the pilot IR must
express, derived from the three component contracts (semantic authority) and
their current evidence across Svelte, React, GPUI, and Jetstream. It is a
stress corpus, not an IR schema, a representation proposal, or an
implementation design. Nothing here recommends Rust types, JSON shapes,
macros, compiler APIs, or crate placement.

### Requirement IDs

- `CROSS-*` — cross-component vocabulary shared by all three pilots
- `BTN-*` — Button (`docs/contracts/components/button.md`)
- `RNG-*` — RangeSlider (`docs/contracts/components/range-slider.md`)
- `TXT-*` — TextInput (`docs/contracts/components/text-input.md`)
- `SHELL-*` — preview-shell scene vocabulary (all four shells)
- `NEG-*` — negative cases exercising IR-03–IR-06
- `UNKNOWN-*`, `OBS-*`, `GAP-*` — registers in §7

### Classification codes (step 3 — vocabulary classes, no representation)

| Code | Class | Meaning |
|------|-------|---------|
| `SDD` | shared declarative definition | typed, serializable declaration authored once and shared across runtimes |
| `GTA` | generated target artifact | deterministic emitted output consumed by a target (web props surface, registries, evidence) |
| `AC` | adapter capability | named, typed environment work a runtime owns (focus, measurement, IME, …) |
| `CV` | conformance vector | shared machine semantics executed by each runtime machine |
| `EXT` | candidate explicit runtime extension | escape hatch or documented delta with a parity consequence |

A requirement may carry two codes (e.g. `SDD` for the declaration plus `CV`
for the executed semantics). Codes describe what the vocabulary is, never how
it is represented.

### Citation convention

- `B §3` = `docs/contracts/components/button.md` section 3 (same for `R` =
  range-slider, `T` = text-input)
- `S063 §…` = `docs/specs/063-rust-authored-component-and-scene-ir.md`
- Every requirement cites the contract and at least one current evidence
  path. Missing evidence is named in the `GAP-*` register rather than
  inferred.

### Sources surveyed (evidence paths)

- Contracts: `docs/contracts/components/{button,range-slider,text-input}.md`
- Svelte: `packages/svelte/components/src/{Button,RangeSlider,TextInput}.svelte`
- React: `packages/react/components/src/{Button,RangeSlider,TextInput}.tsx`
- Core machine + styles: `packages/core/src/{slider,input,audio/laws}.ts`,
  `packages/core/src/styles/{button,range-slider,text-input}.css`
- Rust spec: `packages/contracts/components/src/{button,range_slider,text_input,types}.rs`
- Headless machines: `packages/contracts/headless/src/{slider,text_input}.rs`,
  vectors `packages/contracts/headless/vectors/machines.json`
- Render crate: `packages/render/src/{button,range_slider,text_input}.rs`,
  node vocabulary `packages/contracts/node/src/lib.rs` (`Interaction`),
  adapter/event contracts `packages/contracts/{adapter,events}/src/lib.rs`
- GPUI: `packages/gpui/preview/src/{main,app_state}.rs`,
  `packages/gpui/node-backend/src/{lib,input_text,ime}.rs`,
  `packages/gpui/adapter/src/*`
- Jetstream: `packages/jetstream/preview/src/{main,shell,app_state}.rs`,
  `packages/jetstream/adapter/src/*`
- Specimens: `packages/{svelte,react}/preview/src/specimens/*`,
  `packages/{gpui,jetstream}/preview/src/specimens/*`
- Shells: `packages/svelte/preview/src/{App.svelte,router.ts,parity.ts,accessibility.ts,components/DisplayControls.svelte}`,
  `packages/react/preview/src/gallery/{App.tsx,DisplayControls.tsx}`,
  `packages/gpui/preview/src/{main.rs,app_state.rs}`,
  `packages/jetstream/preview/src/{main.rs,shell.rs,app_state.rs}`
- Tests/harness: `test/parity/component-parity.test.tsx`,
  `test/a11y/component-a11y.test.ts`, `test/fixtures/component-props.ts`,
  `test/visual/{config,allowlist}.ts`, `test/native-visual/{config,run}.ts`,
  `packages/core/test/{wave1,input}.test.ts`,
  `packages/contracts/headless/tests/conformance.rs`
- Parity/evidence docs: `docs/parity/{button,range-slider,text-input}.md`,
  `packages/svelte/preview/artifacts/{parity-report,accessibility-report,recipe-inventory}.json`,
  `packages/gpui/{native-accessibility-proof,cross-runtime-parity-report}.json`,
  `packages/jetstream/cross-runtime-parity-report.json`

## 1. Cross-cutting requirements (`CROSS-*`)

| ID | Requirement (semantic vocabulary) | Class | Contract | Evidence |
|----|-----------------------------------|-------|----------|----------|
| CROSS-01 | Component identity: name, layer (`foundation`), contract reference, and a stable identifier the IR and every generated artifact can cite | SDD | B/R/T §1; S063 "Component IR" | `packages/contracts/components/src/{button,range_slider,text_input}.rs`; `packages/svelte/preview/src/component-registry.ts` |
| CROSS-02 | Portable spec surface: every public prop with default and type that survives into the Rust spec (`ButtonSpec`, `RangeSliderSpec`, `TextInputSpec`) | SDD | B §3 + portable-spec table; R §3; T §3; S063 "stable identifiers" | `packages/contracts/components/src/*.rs` (fields + `Default` + builders) |
| CROSS-03 | Web-only prop surface excluded from the portable spec: Button form-submission family; TextInput native attributes (autocomplete, pattern, spellcheck, …); RangeSlider has none | SDD (web-target declaration) | B §3 portable-spec table; T §3 | `Button.svelte` `type/form/…`; `TextInput.svelte` native attrs; `TextInputSpec` lacks them |
| CROSS-04 | Controlled-state model per component: Button `pressed`/`defaultPressed` toggle; RangeSlider bindable pair; TextInput `value`/`defaultValue` with a "do not mix modes" rule | SDD | B §3; R §3; T §3 | Svelte `$bindable`/`isControlled`; React `useState` + `value !== undefined`; `ButtonSpec.pressed/default_pressed` |
| CROSS-05 | Event/effect vocabulary: activation (`onClick`), value-change vs value-commit, focus/blur, pressed-change, submit/cancel/clear — each with payload and firing condition | SDD + CV | B §5; R §5; T §5; S063 "declarative transition, guard, or effect-intent" | `packages/core/src/slider.ts` `SliderEffect`; Svelte/React handler wiring; `packages/contracts/events/src/lib.rs` |
| CROSS-06 | Event timing: change fires during interaction, commit on release; debounce; Enter/Escape; blur flush; ordering (`onPressedChange` before `onClick`) | CV | B §5 (ordering note); R §5; T §3/§5 | `packages/core/src/slider.ts` (INPUT→emitValueChange, COMMIT→emitValueCommit); `wave1.test.ts`; Svelte `handleClick` |
| CROSS-07 | Size axis: explicit `size` override (xs–xl) and `sizeRole` (chrome/control/prominent) resolved from inherited presentation; per-component size ladders | SDD | B/R/T §3 + §7/§8 size tables | `presentation.ts` `resolveSemanticControlSize`; `render/src/presentation.rs` `resolve_semantic_size`; `data-size` styles |
| CROSS-08 | Density axis: compact/default/comfortable, explicit override or inherited; per-component adjustments, including the two documented exceptions (RangeSlider vertical hit-area padding, TextInput block padding) | SDD | R §8 Density adjustments (exception); T §8 Density adjustments (exception); B §9 `data-density` | `range-slider.css` density blocks; `text-input.css` density vars; `button.css` density blocks; `render/src/button.rs` density offsets |
| CROSS-09 | Theme axis: semantic token paths resolved per theme; recipe-hook override chain (`--poodle-recipe-*` → component var → token) | SDD | B/R/T §8 (recipe hooks); `docs/architecture/007-appearance-recipe-contract.md`; S063 "semantic token and appearance-recipe hook references" | `button.css`/`text-input.css` var chains; `GpuiThemeProvider`/`JetstreamThemeProvider`; `render/src/button.rs` `resolve_*` |
| CROSS-10 | Contrast axis: continuous neutral-contrast override (`--poodle-contrast`) as a scene axis | SDD (scene axis) | S063 Scene IR "contrast axes"; T §7 | `App.svelte` `style:--poodle-contrast`; `gpui/preview/src/app_state.rs` `CONTRAST_MIN/MAX`; `jetstream/preview/src/app_state.rs` `contrast`; `test/visual/config.ts` `Axis.contrast` |
| CROSS-11 | Orientation/direction axis: horizontal/vertical (RangeSlider) and root `data-orientation`; directionality of fill geometry | SDD | R §3/§7/§8 | `RangeSlider.svelte` `data-orientation`; `range-slider.css` vertical blocks; `render/src/range_slider.rs` (`GAP-03`: native vertical open) |
| CROSS-12 | Anatomy parts with parent/child constraints and conditional/repeated nodes (Button spinner/icons/label/chevron; RangeSlider track/fill/two controls; TextInput affixes/affordances/indicator/count) | SDD | B §2; R §2; T §2; S063 "renderer-neutral render nodes and conditional/repeated composition" | Svelte/React render trees; `render/src/{button,range_slider,text_input}.rs` node builders |
| CROSS-13 | State-derived attributes: the `data-*` emission rules (presence-only vs valued, omitted vs always emitted) per component | SDD | B §9; R §9; T §9 | `Button.svelte` `data-tone` omit / `data-loading` always; `RangeSlider.svelte`; `TextInput.svelte` |
| CROSS-14 | VisualState: drawing consumes a serializable state projection; exact fields per component (`RNG-16`, plus Button/TextInput equivalents) | SDD + CV | R §4 + Behavior Machine; S063 IR-06 | `packages/core/src/slider.ts` `rangeSliderVisualState`; `render/src/range_slider.rs` `range_slider_visual_state` |
| CROSS-15 | Accessibility intent: role, accessible-name rule, ARIA state mapping, native-attribute projection per component | SDD | B §6; R §6; T §6 | Svelte/React `aria-*` attrs; spec `aria_*` fields; `node/src/lib.rs` `NodeRole`/`a11y` |
| CROSS-16 | Keyboard command tables per component, declared as vocabulary; delivery is adapter-owned | SDD + AC | B §6; R §6; T §6 | `headless/src/text_input.rs` `edit_transition`; `Interaction.on_key`/`on_edit_key` (`node/src/lib.rs`) |
| CROSS-17 | Adapter capability inventory: focus, measurement/shaping, pointer capture, scrub fraction, text editing/IME, clipboard, portal placement, timers, announcements — named per component | AC | S063 IR-05 + "adapter capability" list; T §6 caret ownership; R §3 Pointer | `Interaction` channels (`on_scrub`, `on_edit_key`, `on_select_range`, `on_edit_insert`, `on_focus_change`); `gpui/node-backend/src/{input_text,ime}.rs` |
| CROSS-18 | Conformance vectors: shared machine semantics executed by both runtimes from one vector file | CV | S063 "shared conformance vectors implemented by each runtime machine" | `vectors/machines.json` + `core/test/conformance.test.ts` + `headless/tests/conformance.rs`; `GAP-01`: no range/text vectors |
| CROSS-19 | Degenerate-range guard (`max <= min` widens to `min + 1`) and step snapping anchored at `min` | CV | R §3 (step) + machine notes | `core/src/slider.ts` `safeSliderMax`/`snapToStep`; `headless/src/slider.rs`; slider vectors |
| CROSS-20 | Disabled/loading unification: `isUnavailable = disabled || loading` (Button), disabled inertness and opacity per component | SDD | B §4/§6; R §4; T §4 | `Button.svelte` `isUnavailable`; `ButtonSpec::activation_allowed`; disabled styles |
| CROSS-21 | Specimen/scene definitions: contract §13 specimen sets are shared scene definitions rendered by all four shells | SDD (scene) + GTA | B/R/T §13; S063 Scene IR | `svelte/preview/src/specimens/*`, `react/preview/src/gallery/specimens/*`, `gpui/jetstream/preview/src/specimens/*` |

## 2. Button (`BTN-*`)

| ID | Requirement | Class | Contract | Evidence |
|----|-------------|-------|----------|----------|
| BTN-01 | `variant`: primary/secondary/ghost, default secondary | SDD | B §3 | `Button.svelte`; `Button.tsx`; `ButtonVariant` (`contracts/components/src/types.rs`) |
| BTN-02 | `tone`: default/danger/warning; composes with variant (3×3 treatment matrix, each state defined in-family) | SDD | B §3/§8 | `button.css` tone blocks; `render/src/button.rs` `(variant, tone)` match |
| BTN-03 | `size`: xs–xl explicit override, default `null` → resolve from inherited presentation + `sizeRole` | SDD | B §3/§7/§8 | `Button.svelte` `resolvedSize`; `presentation.ts` |
| BTN-04 | `sizeRole`: chrome/control/prominent, default control | SDD | B §3 | `Button.svelte`; `SemanticControlSizeRole` |
| BTN-05 | `density`: explicit `ControlDensity` override, default `null` → inherited; density padding/gap adjustments | SDD | B §3/§9; `GAP-06` (values stylesheet-only) | `button.css` density blocks; `render/src/button.rs` density offsets + gap laddering |
| BTN-06 | `type` + form-override family (`form`, `formaction`, `formenctype`, `formmethod`, `formnovalidate`, `formtarget`) — web-only, excluded from `ButtonSpec` | SDD (web) | B §3 portable-spec table | `Button.svelte`; `OBS-03` (React omits `formenctype`/`formmethod`) |
| BTN-07 | `disabled`: suppresses activation, native disabled, disabled visual | SDD | B §3/§4/§6 | `Button.svelte` `isUnavailable`; `ButtonSpec.is_disabled` |
| BTN-08 | `loading`: spinner (shared `Spinner` ring/sm/current), activation suppressed, `aria-busy`, `data-loading` always emitted, treated as disabled | SDD | B §2/§4/§6/§8/§9 | `Button.svelte`; `ButtonSpec.is_loading`; `render/src/button.rs` |
| BTN-09 | `leadingIcon`/`trailingIcon`: icon-registry identifiers; icon-only mode when no children (square, no min-width) | SDD | B §2/§3/§7/§8 | `Button.svelte` `iconOnly`; icon-only CSS; `ButtonSpec.leading_icon/trailing_icon` |
| BTN-10 | `chevron`: trailing disclosure indicator (registry `chevron-down` at `sm`, opacity 0.5, negative margin) | SDD | B §2/§3/§8/§9 | `Button.svelte`; `button.css` `.poodle-button__chevron` |
| BTN-11 | `truncate`: `data-truncate`, `overflow: hidden` + label ellipsis | SDD | B §3/§8/§9 | `button.css` `[data-truncate]` rules |
| BTN-12 | `fit`: `"default"`/`"content"`; content = `min-width: 0` + `padding-inline: 0.375rem`, `data-fit="content"` | SDD | B §3/§7/§8/§9 | `button.css` `[data-fit="content"]`; `GAP-07` (absent from `ButtonSpec`) |
| BTN-13 | `maxWidth`: composed into inline `style` as `max-width` | SDD | B §3/§9 | `Button.svelte` `resolvedStyle`; `ButtonSpec.max_width` |
| BTN-14 | Toggle mode: `pressed`/`defaultPressed`; activates when either is non-null; controlled vs uncontrolled; `onPressedChange` fires before `onClick`; `aria-pressed` + `data-pressed` (presence-only) | SDD | B §3/§4/§5/§8/§9 | `Button.svelte` `isToggle`/`handleClick`; `ButtonSpec.is_toggle_mode/current_pressed` |
| BTN-15 | `ariaLabel`, `ariaExpanded` (disclosure hint, `Option<bool>` in spec), `describedBy`; rest-attribute passthrough for unmodeled native attrs | SDD | B §3/§6 | `Button.svelte`; `ButtonSpec.aria_expanded` |
| BTN-16 | Slot/content rules: children = label (absence → icon-only); `leading()`/`trailing()` snippets override icon props | SDD | B §2/§3 | `Button.svelte` snippets; `Button.tsx` children |
| BTN-17 | Anatomy parts + conditional composition: spinner when loading; leading icon when snippet/icon; label when children; trailing icon; chevron; `has-leading` includes loading, `has-trailing` includes chevron | SDD | B §2/§9 | `Button.svelte` render tree; `button.css` |
| BTN-18 | State-derived attributes: `data-variant`, `data-tone` (omit when default), `data-size`, `data-density`, `data-icon-only`, `data-has-leading`, `data-has-trailing`, `data-truncate`, `data-fit`, `data-loading` (always), `data-pressed` (toggle only) | SDD | B §9 | `Button.svelte` attribute block |
| BTN-19 | Visual states: default, hover (not disabled), active (not disabled, `translateY(0.03125rem)`), focus (`outline` focusRing + 0.125rem offset), disabled, loading, icon-only, pressed (accent treatment for non-primary) | SDD | B §4/§8 | `button.css` state rules; `render/src/button.rs` (note GPUI translateY delta, B §12) |
| BTN-20 | Keyboard: Enter/Space activate (native button); Tab/Shift+Tab move focus; disabled buttons excluded | SDD + AC | B §6 | Native `<button>`; parity checklist Tier 1 |
| BTN-21 | Accessibility facts: native button role; icon-only requires accessible name; `aria-expanded`, `aria-describedby`, `aria-pressed` (toggle), `aria-busy` (loading); icon/spinner spans `aria-hidden`; disabled attribute | SDD | B §6 | `Button.svelte` ARIA block; `test/a11y/component-a11y.test.ts` (axe sweep) |
| BTN-22 | Token references: `--poodle-button-*` custom-property chain with recipe hooks (fill/fill-hover/fill-active/border/border-hover/text/shadow per variant × tone), typography-label, `radius-control`, focus-ring width/color, `state-opacity-disabled`, icon size, motion duration/easing | SDD | B §8 | `button.css` var chains; `ButtonSpec` token accessors (`resolved_fill_token`, …) |
| BTN-23 | Size ladder metrics: per-size height/min-width/padding/font-size (fixed rem heights); icon-only square widths; per-size icon-side padding adjustments (xs −0.1875 … xl +0.0625); fit-content | SDD | B §7/§8 | `button.css` size blocks; `render/src/button.rs` |
| BTN-24 | Hover/active/focus/disabled token treatments: hover fill+border via hover vars, active fill + transform, focus outline, disabled opacity + cursor | SDD | B §8 | `button.css` `:hover:not(:disabled)` etc. |
| BTN-25 | Rust `ButtonSpec` surface: fields, builders, `activation_allowed`, `requires_aria_label`, `effective_tone`, `is_toggle_mode`, `current_pressed`, token resolvers | SDD | B §10 GPUI Notes ("`ButtonSpec` in primitives crate") | `contracts/components/src/button.rs` |
| BTN-26 | GPUI deltas: active `translateY` may be omitted; box-shadow omitted; letter-spacing omitted; transition timing platform-owned | EXT | B §12 | `docs/parity/button.md` |
| BTN-27 | Jetstream deltas: no `onFocus`/`onBlur`; no `onPressedChange` (host derives from `on_click`); `on_click` carries no payload; loading treated as disabled | EXT | B §10a/§12 | `docs/parity/button.md`; `jetstream/preview/src/specimens/button.rs` |
| BTN-28 | Specimen set: Variants, Danger tone, With icons, With chevron, Sizes, States, Click counter | SDD (scene) | B §13 | `ButtonSpecimen.svelte`; `react/gallery/specimens/ButtonSpecimen.tsx`; `gpui/jetstream/preview/src/specimens/button.rs` |
| BTN-29 | Rust spec enum superset beyond contract union: `ButtonVariant::Danger`, `ButtonTone::Success` | EXT + UNKNOWN-02 | B §3 (union) | `contracts/components/src/types.rs`; `render/src/button.rs` status match |

## 3. RangeSlider (`RNG-*`)

| ID | Requirement | Class | Contract | Evidence |
|----|-------------|-------|----------|----------|
| RNG-01 | Controlled `value` pair `[lower, upper]`, default `[0, 100]`; uncontrolled via bindable/defaultValue | SDD | R §3 | `RangeSlider.svelte` `$bindable`; `RangeSlider.tsx` `defaultValue`; `RangeSliderSpec.low/high` |
| RNG-02 | `min`/`max`/`step` defaults 0/100/1; step snapping anchored at `min`; degenerate-range guard | SDD + CV | R §3 + machine notes | `slider.ts` `snapToStep`/`safeSliderMax`; `headless/src/slider.rs` |
| RNG-03 | `variant`: standard/embedded | SDD | R §3 | `RangeSlider.svelte` `variant`; `SliderVariant` |
| RNG-04 | `polarity`: unipolar/bipolar | SDD + CV | R §3/§4 | `rangeSliderVisualState` polarity arms; `RangeSliderSpec.polarity` |
| RNG-05 | `centerValue`: bipolar reference; defaults to zero when zero is inside range, otherwise midpoint | SDD + CV | R §3 | `slider.ts` `sliderCenterValue`; `headless/src/slider.rs` |
| RNG-06 | `law`: `AudioValueLaw` (linear/logarithmic/exponential/bipolar-center/stepped) for embedded mapping; standard stays native linear | SDD + CV | R §3 | `core/src/audio/laws.ts`; `RangeSliderSpec.law`; `headless/src/audio.rs` |
| RNG-07 | `orientation`: horizontal/vertical; web conveys via root `data-orientation` only; Rust targets must implement native vertical | SDD + GAP-03 | R §3/§6/§7/§12 | `RangeSlider.svelte`; `range-slider.css`; `docs/parity/range-slider.md` (vertical open on both Rust targets) |
| RNG-08 | `disabled`: inert + `state-opacity-disabled` | SDD | R §3/§4/§8 | `range-slider.css` `[data-disabled="true"]` |
| RNG-09 | `size`/`sizeRole`/`density`: explicit overrides, inherited resolution; size ladder (min-height, track thickness, thumb diameter); density = vertical hit-area padding exception | SDD | R §3/§7/§8 | `range-slider.css` size/density blocks; `RangeSlider.svelte` `resolvedSize` |
| RNG-10 | `lowerValueText`/`upperValueText`: per-thumb `aria-valuetext` | SDD | R §3/§6 | `RangeSlider.svelte`; `RangeSliderSpec.lower_value_text/upper_value_text` |
| RNG-11 | `onValueChange`/`onValueCommit`: pair payload; change during interaction (`input` event), commit on release (`change` event) | CV | R §5 | `slider.ts` effects; `RangeSlider.svelte` `send()`; `wave1.test.ts` |
| RNG-12 | lower ≤ upper invariant always preserved; a thumb cannot cross its sibling (lower clamps to `[min, upper]`, upper to `[lower, max]`); incoming pair ordered via normalize | CV | R §3/§4 | `slider.ts` `normalizeRangeValue`/`rangeSliderTransition`; `headless/src/slider.rs` |
| RNG-13 | Pointer model: press anywhere on track moves the **nearer** thumb; a drag keeps the thumb the press chose (gesture never transfers); Rust uses a single grab overlay reporting pointer fraction (`Interaction::on_scrub`) and the component/machine picks the thumb | AC + CV | R §3 Pointer; R §10a | `render/src/range_slider.rs` grab overlay + `on_scrub`; `node/src/lib.rs` `Interaction.on_scrub`; `docs/logs/2026-08/07-slider-scrub-and-drag-capture.md` |
| RNG-14 | Standard anatomy: Root (`role="group"`) / Track / Fill / two overlapping native range inputs; pointer-events none on control, auto on thumbs | SDD | R §2/§9 | `RangeSlider.svelte` markup; `range-slider.css` `__control` |
| RNG-15 | Embedded anatomy: two `role="slider"` focus stops (tabindex, `aria-valuemin/max/now/text`, `aria-orientation` on the embedded stops), pointer capture on shared root, embedded chrome (border, radius 0.25rem, min-width 2.5rem, center marker, segments) | SDD + UNKNOWN-01 | R §2/§6/§8 | `RangeSlider.svelte` embedded branch; `range-slider.css` embedded blocks |
| RNG-16 | VisualState fields: `value`, `lowerNorm`, `upperNorm`, `centerNorm`, `fillStartNorm`, `fillSpanNorm`, `negativeFillStartNorm`, `negativeFillSpanNorm`, `positiveFillStartNorm`, `positiveFillSpanNorm`, `fillSplitAtCenter`, `polarity`, `pointerActive`, `activeThumb`, `enabled` | SDD + CV | R §4 + machine notes; S063 IR-06 | `slider.ts` `rangeSliderVisualState`; `headless/src/slider.rs` `RangeSliderVisualState` |
| RNG-17 | Fill geometry custom properties: `--poodle-range-start/end/center/negative-start/negative-span/positive-start/positive-span` computed from normalized state, set inline on root | SDD | R §3 (CSS custom properties) + §8 | `RangeSlider.svelte` `rangeStyle`; `RangeSlider.tsx` |
| RNG-18 | Keyboard per thumb: ArrowLeft/Down decrement by step, ArrowRight/Up increment, Home → min (lower) / lower value (upper), End → upper value (lower) / max (upper), Tab cycles thumbs; embedded keyboard runs INPUT then COMMIT through the machine | AC + CV | R §6 | `RangeSlider.svelte` `embeddedKey`; `Interaction.on_key` |
| RNG-19 | Accessibility facts: per-thumb labels `"{ariaLabel} minimum/maximum"` with defaults "Minimum value"/"Maximum value"; `aria-valuemin/max/now` per thumb; NO `aria-orientation` on the standard range inputs; `data-orientation` on root only; disabled | SDD | R §6 | `RangeSlider.svelte` ARIA block; `docs/parity/range-slider.md` (consv=fixed) |
| RNG-20 | Focus: one thumb focusable at a time, Tab moves between thumbs, per-thumb focus ring (compound box-shadow, focusRing at 32%); GPUI needs two separately focusable named handles | AC + GAP-04 | R §6/§8/§10 | `range-slider.css` `:focus-visible` thumb rules; `docs/parity/range-slider.md` (GPUI per-thumb focus accepted delta) |
| RNG-21 | Token references: the 11 `--poodle-recipe-range-slider-*` hooks (track-fill, fill-fill, fill-negative, track-border, center-fill, control-fill, control-track-fill, control-thumb-fill, control-thumb-shadow, focus-ring, focus-control-thumb-shadow); accent, status-danger, background-elevated, border-default, disabled opacity, radius | SDD | R §8 | `range-slider.css` recipe hooks; `RangeSliderSpec` token methods |
| RNG-22 | Track/fill/thumb visuals: track `color-mix(surface 88%, transparent)`, radius 999px; fill accent-base (negative segment status-danger); thumb elevated background + 0.0625rem border-default + `0 0.125rem 0.5rem black@18%` shadow; size-scaled thumb margin `(diameter − thickness)/−2` | SDD | R §8 | `range-slider.css`; `docs/parity/range-slider.md` (pass 41–43 alignment) |
| RNG-23 | Embedded VisualState publishing: separate negative/positive selected-fill segments, center reference, `fillSplitAtCenter` corner squaring | SDD + CV | R §4 | `slider.ts` negative/positive span arms; `range-slider.css` `[data-fill-split="true"]` |
| RNG-24 | Bipolar semantics: window split at the center reference, not two unrelated scalars; unipolar publishes an empty negative segment and one positive segment | CV | R §4 | `slider.ts` `negativeFillSpanNorm`/`positiveFillSpanNorm`; `wave1.test.ts` bipolar case |
| RNG-25 | Specimen set: Default, With step, Disabled, Embedded unipolar, Embedded bipolar; Sizes tab renders standard + embedded unipolar + embedded bipolar at xs–xl; density specimens use embedded bipolar | SDD (scene) | R §13 | `RangeSliderSpecimen.svelte`; `react/gallery/specimens/RangeSliderSpecimen.tsx`; `gpui/jetstream/preview/src/specimens/range_slider.rs` |
| RNG-26 | GPUI: native vertical (not CSS rotation), two named value handles, invariant during keyboard and pointer interaction; pointer overlap/grab priority platform-owned | AC + EXT | R §10/§12 | `docs/parity/range-slider.md`; `render/src/range_slider.rs` (`GAP-03` vertical) |
| RNG-27 | Jetstream: `on_change`/`on_value_commit` report `(low, high)` together; a thumb stops against its sibling (never swaps — machine-driven); only thumbs draggable; dragging the filled span is a different gesture needing both values | CV + EXT | R §10a | `render/src/range_slider.rs`; `docs/parity/range-slider.md` |
| RNG-28 | Rust `RangeSliderSpec` surface: low/high/min/max/step/variant/polarity/center_value/law/orientation/is_disabled/aria_label/size/size_role/density/lower+upper_value_text | SDD | R §10 GPUI Notes (module surface) | `contracts/components/src/range_slider.rs` |
| RNG-29 | Conformance vectors for the range machine | CV + GAP-01 | S063 "conformance vectors" | `GAP-01`: absent from `vectors/machines.json`; behavior covered by `core/test/wave1.test.ts` + `headless/src/slider.rs` tests |

## 4. TextInput (`TXT-*`)

| ID | Requirement | Class | Contract | Evidence |
|----|-------------|-------|----------|----------|
| TXT-01 | `id`: required; element id for label association | SDD | T §3/§6 | `TextInput.svelte` `id`; `TextInputSpec.id` |
| TXT-02 | Controlled/uncontrolled value: `value` (null is a valid controlled empty state) + `onValueChange`; `defaultValue` seeds uncontrolled; do not mix modes | SDD | T §3 | `TextInput.svelte` `isControlled`; `TextInput.tsx` |
| TXT-03 | `placeholder`: hint when empty; placeholder never counts as the accessible name | SDD | T §3/§6 | `TextInput.svelte`; placeholder CSS |
| TXT-04 | Native attribute passthroughs: `name`, `autocomplete`, `required`, `pattern`, `spellcheck`, `autocapitalize`, `autocorrect`, `enterKeyHint`, `inputMode`, `list` (datalist) | SDD | T §3/§6 | `TextInput.svelte` native attrs; `OBS-03` (React omits `autocorrect`) |
| TXT-05 | `disabled`/`readOnly`: native attributes; readOnly allows selection without editing | SDD | T §3/§4/§6 | `TextInput.svelte`; `TextInputSpec.is_disabled/is_read_only` |
| TXT-06 | `type` modes: text/multiline/search/slug; `rows > 1` with default type auto-switches to multiline | SDD | T §3 | `TextInput.svelte` `isMultiline` |
| TXT-07 | Multiline: `<textarea>`; rows default 4; min-height `calc(1lh * 4)`; resize handle (`vertical` default); char-count overlaid bottom-right (0.375/0.5rem) with extra bottom padding so text never runs under it; Cmd/Ctrl+Enter submits; incompatible with other modes | SDD | T §3/§7/§8 | `TextInput.svelte` multiline branch; `text-input.css` `--multiline` |
| TXT-08 | Search mode: automatic leading search icon (when no leading slot), `showClearButton` (default true), `canClear` gating (has value, not disabled/readonly), `clear` event with immediate commit | SDD | T §3/§5/§9 | `TextInput.svelte` `canClear`/`handleClear`; `TextInputSpec.show_clear_button` |
| TXT-09 | Slug mode: semantic mode rendered as `input[type=text]` with `autocapitalize=off`, `spellcheck=false`, `inputmode=text`; code-family typography + `adjustmentRatio` for value and affixes; normalization (accents stripped, lowercase, spaces/underscores → hyphens, collapse, trim); built-in validation (format, length 2..maxLength default 100, reserved slugs); prefix joins candidate; `source` auto-generation until user edit, resumes after clear | SDD + CV | T §3/§4 | `slugify`/`isValidSlugFormat` (`core/src/input.ts`); `TextInput.svelte` slug effects; `input.test.ts` |
| TXT-10 | `prefix`/`suffix` affixes: static, non-editable, `user-select: none`, separator borders (border-default solid 0.0625rem), excluded from editable value, slug sizing applies | SDD | T §2/§6/§8 | `TextInput.svelte`; `text-input.css` `__affix` |
| TXT-11 | `debounce`: delays `onValueChange`; flush on blur; immediate for clear and slug source regeneration; timer cleanup on destroy | SDD + CV | T §3/§5 | `TextInput.svelte` `emitValueChange`/`flushDebouncedValue`; `TextInputSpec.debounce_ms` |
| TXT-12 | Validation orchestration: `validate`, `validationContext` (opaque), `validationKey` (merged), `validationDebounce` (300), `validateOnBlur` (true), `showValidationStatus` (true), `validationState` (none); built-in timing owned by component; mapping idle→caller/validating→pending/valid→valid/invalid→invalid; empty-value idle skip; validation-message id wired into `aria-describedby`; indicator `aria-hidden` | SDD + CV | T §3/§4/§6 | `TextInput.svelte` validation effects; `validationStatusToState` (`core/src/input.ts`); `TextInputSpec.validation_state` |
| TXT-13 | Callbacks and payloads: `onValueChange(string)`, `onValidationChange({status, valid, message})` (only when `validate`), `onSubmit(value)` on Enter / Cmd+Ctrl+Enter multiline, `onCancel()` on Escape, `onClear()` on built-in clear, `onKeyDown`/`onFocus`/`onBlur` native passthrough | SDD + CV | T §5 | `TextInput.svelte` handlers; `TextInput.tsx` |
| TXT-14 | `maxLength`/`showCharCount`: renders `{n}/{max}` when both set, `{n}` when only count; over-limit → `status-danger`; multiline overlay geometry | SDD | T §3/§8 | `TextInput.svelte` `charCountText`; `text-input.css` `__char-count` |
| TXT-15 | `size`/`sizeRole`/`density`: per-size table (min-height/padding/font-size); density inline/block adjustment vars (`compact` −0.125/−0.0625, `comfortable` +0.125/+0.0625), documented orthogonality exception | SDD | T §3/§8 | `text-input.css` size/density vars; `TextInputSpec.size/size_role/density` |
| TXT-16 | Adornment padding reservation: `--poodle-text-input-control-padding-start/end` computed from adornment counts (start: padding + icon + 1.5× gap when leading; end: padding + n×icon + n×gap) | SDD | T §8 | `TextInput.svelte` `controlPaddingStart/End`; `text-input.css` |
| TXT-17 | Anatomy parts + conditional rules: Root / Prefix / Field / Leading Affordance / Input Control (input|textarea) / Trailing Affordance / Clear Button / Validation Indicator / Suffix / Character Count | SDD | T §2 | `TextInput.svelte` markup; `TextInput.tsx` |
| TXT-18 | State-derived attributes: `data-validation-state`, `data-size`, `data-density`, `data-type` | SDD | T §9 | `TextInput.svelte` attribute block |
| TXT-19 | Visual states: default; hover = none (delegated to focus); focus-within (border-focus/fill-focus/shadow-focus); disabled (opacity); readOnly (no visual change); invalid/valid/pending border colors; trailing indicators (pending ring spinner accent, valid `check` success, invalid `x` danger) gated by `showValidationStatus`; char-over | SDD | T §4/§8 | `text-input.css` state rules; `TextInput.svelte` `showValidationIndicator` |
| TXT-20 | Keyboard: character input inserts; arrows move caret; Home/End; Shift+Arrow extends selection; platform copy/cut/paste/select-all; Enter submits; Escape cancels; Tab moves focus out; text-focused shortcut suppression while focused | SDD + AC | T §6 | `headless/src/text_input.rs` `edit_transition`; `TextInput.svelte` keydown |
| TXT-21 | Caret/selection ownership (Rust): host owns caret (`selectionStart`/`selectionEnd` controlled props, `onSelectionChange` like `TreeSpec::focused_value`); backend owns focus and caret drawing (glyph measurement: `shape_line`/`x_for_index`, `closest_index_for_x`); shared edit model once per target; `isFocused` prop does not drive the caret; blink (~1s) and scroll-into-view are backend-owned | AC + CV | T §6 Caret Ownership | `render/src/text_input.rs` (`TextInputHandlers`, caret channel, `selection_range`); `node/src/lib.rs` `on_edit_key`/`on_select_range`/`on_focus_change`; `headless/src/text_input.rs` |
| TXT-22 | Pointer selection: click = caret at nearest character boundary; drag = selection; double-click = word; triple-click = whole value; shift-click extends; "word" = run of alphanumerics/`_`; backend counts clicks and measures, component knows words | AC + CV | T §6 Pointer Selection | `headless/src/text_input.rs` `word_range_at`; `node/src/lib.rs` `on_select_range` + `SelectGranularity` |
| TXT-23 | Clipboard (Rust): copy/cut/paste on platform clipboard; backend owns (text from outside the tree); shared edit model owns paste landing; multi-line paste collapses to one line; empty selection leaves clipboard alone | AC | T §6 Clipboard | `headless/src/text_input.rs` `insert_transition`/`selected_text`; `node/src/lib.rs` `on_edit_insert` |
| TXT-24 | IME (Rust): platform text input handler (`Window::handle_input` with `InputHandler`); UTF-16 boundary (chars in vocabulary, bytes in text system, UTF-16 at the edge); composition (marked range) backend-owned per field | AC | T §6 Clipboard section | `gpui/node-backend/src/ime.rs`; contract narrative |
| TXT-25 | Undo (Rust): `accel+Z` back, `accel+shift+Z` forward; a typing run is one step (`coalesces`); run ends at deletion/paste/caret move; edit-after-undo discards redo tail; ephemeral backend-owned history | CV + AC | T §6 Undo | `headless/src/text_input.rs` `coalesces`/`EditSnapshot` |
| TXT-26 | Accessibility facts: native input role; `id` for label association; `aria-label` required without external label; `aria-describedby` (+ validation message id); `aria-invalid` when invalid; `aria-busy` when pending; native readonly (not `aria-readonly`); disabled/required/pattern/autocomplete/autocorrect/maxlength/inputmode/list passthrough; placeholder never the name; indicator `aria-hidden`; GPUI native mapping (role, name, value, readonly/disabled/invalid, selection/caret, IME-safe entry) | SDD | T §6 | `TextInput.svelte` ARIA block; `test/a11y/component-a11y.test.ts` |
| TXT-27 | Token references: recipe hooks (`text-input-fill/border/shadow` + focus variants); `radius-control`; focus ring `0 0 0 border-width-focus` color-mix focusRing 28%; status colors (danger/success/accent); typography body + `code-xs` char count; affix separator `border-default`; `state-opacity-muted`/`state-opacity-disabled`; motion tokens; icon sizes | SDD | T §8 | `text-input.css` var chains; `TextInputSpec` token methods |
| TXT-28 | Event timing: change per input (respecting debounce); blur flush; validation debounce with blur-immediate; slug source regeneration immediate; stale async validation guarded by value/context snapshot | CV | T §3/§5 | `TextInput.svelte` `triggerValidation`/`runValidation` guards; `TextInput.tsx` refs |
| TXT-29 | Rust `TextInputSpec` surface: id/value/default/placeholder/name/input_type/input_mode/disabled/read_only/selection_start/selection_end/is_focused/validation_state/aria_label/description_id/error_message_id/prefix/suffix/max_length/show_char_count/leading+trailing icons/rows/resize/source/show_clear_button/submit_enabled/cancel_enabled/is_required/pattern/autocomplete/debounce_ms/size/size_role/density/shows_validation_status | SDD | T §10 GPUI Notes (module surface) | `contracts/components/src/text_input.rs` |
| TXT-30 | Rust editing render surface: `text_input_with_handlers` + `selection_range` + caret channel + placeholder-vs-value caret flag; interaction via `on_edit_key`/`on_select_range`/`on_edit_insert`/`on_focus_change` | AC + CV | T §6; S063 IR-05 | `render/src/text_input.rs`; `node/src/lib.rs` |
| TXT-31 | Jetstream delta: `on_clear` is the only wired event (clear button is the only pointer-reachable part); no typing/key events (`onValueChange`/`onKeyDown`/`onSubmit`/`onCancel` have no route); host owns the editor and feeds value back through the spec; disabled/read-only fields do not clear | EXT | T §10a/§12 | `docs/parity/text-input.md`; `jetstream/adapter/src/render_input.rs` |
| TXT-32 | Specimen set: Default, With validation, Pending validation, Disabled; plus size/density ladders | SDD (scene) | T §13 | `TextInputSpecimen.svelte`; `react/gallery/specimens/TextInputSpecimen.tsx`; `gpui/jetstream/preview/src/specimens/text_input.rs` |

## 5. Preview shells (`SHELL-*`)

| ID | Requirement | Class | Contract | Evidence |
|----|-------------|-------|----------|----------|
| SHELL-01 | Theme selection control in all four shells (Svelte `ThemeSelect`, React `ToggleGroup`, GPUI theme-preset swatches, Jetstream theme presets) | SDD (scene) | S063 Scene IR ("theme … axes"); B/R/T §13 (specimens render identically) | `svelte/…/DisplayControls.svelte`; `react/…/DisplayControls.tsx`; `gpui/preview/src/app_state.rs` `ThemePreset`; `jetstream/preview/src/app_state.rs` |
| SHELL-02 | Control-size axis control (xs–xl) in all four shells | SDD (scene) | S063 Scene IR ("size" axis) | `DisplayControls.svelte`/`.tsx`; `gpui/preview/src/app_state.rs` `ControlSize`; `jetstream/…/app_state.rs` |
| SHELL-03 | Density axis control (compact/default/comfortable) in all four shells | SDD (scene) | S063 Scene IR ("density" axis) | `DisplayControls.svelte`/`.tsx`; GPUI/Jetstream `Density` enums |
| SHELL-04 | Contrast control: continuous neutral-contrast slider in all four shells (web 0.4–1.6, GPUI 0..1, Jetstream knob) | SDD (scene) | S063 Scene IR ("contrast axes") | `DisplayControls.svelte` contrast `Slider`; `gpui/app_state.rs` `CONTRAST_MIN/MAX`; `jetstream/app_state.rs` `contrast`; `test/visual/config.ts` |
| SHELL-05 | Navigation: top-level sections (Components/Tokens web; + Demo native), component sidebar groups, route state (hash + query params web) | SDD (scene) | S063 Scene IR ("layout nodes … groups") | `App.svelte`/`router.ts`; `react/gallery/App.tsx`; `gpui/preview/src/main.rs` `CatalogueSidebar`; `jetstream/preview/src/shell.rs` |
| SHELL-06 | Component search: case-insensitive filter over display name/description in all four shells | SDD (scene) | S063 Scene IR | `ComponentsSection.svelte` `filteredComponents`; `gpui/app_state.rs` `component_search`; `jetstream/app_state.rs` `search`/`matches_search` |
| SHELL-07 | Specimen tabs: Examples / Sizes / Densities in all four shells | SDD (scene) | S063 Scene IR ("size/density matrices") | `SpecimenLayout.svelte`; `react/gallery/SpecimenLayout.tsx`; `gpui/app_state.rs` `SpecimenView`; `jetstream/app_state.rs` `SpecimenView` |
| SHELL-08 | Preview state serialization: theme/density/controlSize/contrast persisted in URL query + hash (web shells) | SDD (scene) | S063 Scene IR | `App.svelte` `syncCurrentLocation`/`replaceState`; `svelte/preview/src/parity.ts` `normalizePreviewState` |
| SHELL-09 | Parity harness vocabulary: preview state defaults, review route presets, parity targets, package-surface coverage, axe sweep, visual-gate tiers (smoke/axis/sweep) with explicit axes, native visual baseline gate | CV | S063 IR-10 ("executed semantic, interaction, accessibility, recipe, axis, and visual evidence"); B/R/T §11 | `svelte/preview/src/{parity,accessibility}.ts`; `test/visual/config.ts` (`AXIS_TIER_SLUGS` includes button/text-input/slider); `test/native-visual/config.ts`; `test/a11y/component-a11y.test.ts` |
| SHELL-10 | Specimen registry + specimen-map wiring per shell so every component (including the three pilots) renders through the same shell chrome | SDD (scene) + GTA | S063 Scene IR ("component references and typed prop bindings") | `svelte/preview/src/specimens/registry.ts` (`button`, `range-slider`, `text-input`); `react/…/specimen-map.ts`; `gpui/…/component_registry.rs`; `jetstream/…/component_registry.rs` |

## 6. Negative cases (`NEG-*`) — behavior that must not move into the IR

Each case names the IR rule it directly exercises and the current evidence
that the boundary is respected.

| ID | Negative case | IR rule | Evidence |
|----|---------------|---------|----------|
| NEG-01 | Cross-runtime behavior is never arbitrary Rust execution translated to TypeScript: no closures, trait objects, runtime borrowing, or backend calls cross the boundary; behavior is declarative intent, a conformance vector, an adapter capability, or a documented extension | IR-03 | S063 "Hard Boundary: Data, Not Rust Transpilation"; `Interaction` channels are typed intent, not code (`node/src/lib.rs`) |
| NEG-02 | Web authoring never lowers from resolved `poodle-node` output: DOM semantics, CSS cascade and recipes, framework lifecycle, slots, portals, and native form behavior are retained above the node level | IR-04 | S063 "Component IR … above `poodle-node`"; TextInput native attrs/autofill/IME stay native on web (`TextInput.svelte`); Button form-override family stays HTML (`Button.svelte`) |
| NEG-03 | Environment work never moves into drawing: focus, IME, portals, measurement, pointer capture, text systems, and accessibility projection remain runtime capabilities | IR-05 | S063 IR-05; caret positioning measured by the backend (`render/src/text_input.rs` caret channel; contract T §6); scrub fraction measured by the backend (`Interaction.on_scrub`); a11y projection is a backend edge (`NodeRole`, `native-accessibility-proof.json`) |
| NEG-04 | Drawing never reads machine state or owns hit-testing/input: `VisualState` is a pure projection of serializable state; input/hit-testing/focus stay in machines and adapters | IR-06 | S063 IR-06; `rangeSliderVisualState` is a pure function of context (`slider.ts`); the nearer-thumb decision is machine state (`activeThumb`), not paint |
| NEG-05 | No untyped side channel between runtime and component for rendering or interaction | IR-06 (S063 Stop Conditions) | `Interaction`/`Node` fields are typed; `on_scrub(f32, ScrubPhase)`, `on_edit_key(&str, NodeModifiers)` |
| NEG-06 | Generated framework lifecycle is not the implementation: the compiler emits thin registries/shells; Svelte/React own idiomatic lifecycle, refs, context, snippets/children, DOM events, focus, portals, measurement, text editing | IR-04 (S063 "generated code") | S063 "The compiler may generate thin registries and static shells"; shared styles `core/src/styles/*.css` + framework components |
| NEG-07 | Native text-system capabilities are not reimplemented by the IR on either side: web keeps browser input internals; Rust keeps platform IME/clipboard/measurement adapter-owned | IR-05 | T §6 (native behavior retained; Rust splits ownership); `gpui/node-backend/src/{input_text,ime}.rs` |
| NEG-08 | Declared states are not drawing-owned behavior: loading `aria-busy`, disabled opacity, validation chrome derive from serializable state, never from paint-time side effects | IR-06 | B §4/§6; T §4; Svelte `data-*` derived values |

## 7. Registers

### 7.1 Contradiction register (semantic)

**Empty.** No requirement in this corpus required stopping: no contract↔live-
implementation disagreement on public semantics, event timing, accessibility,
or runtime ownership was found. The closest candidates are recorded as
observations (`OBS-01`, `OBS-02`) with the reasoning that the observable
contract holds; none changed a contract and none required inventing a schema
or changing code.

### 7.2 Unknown register (open semantic questions)

| ID | Question | Paths | Smallest unresolved question |
|----|----------|-------|------------------------------|
| UNKNOWN-01 | Does R §6 "`aria-orientation`: NOT set on the range inputs; orientation is conveyed via `data-orientation` on the root only" extend to the **embedded variant**'s `role="slider"` focus stops? Svelte and React currently emit `aria-orientation` on those stops; the contract is silent on the embedded stops | `docs/contracts/components/range-slider.md` §6; `packages/svelte/components/src/RangeSlider.svelte` (embedded branch); `packages/react/components/src/RangeSlider.tsx` | Is embedded-variant `aria-orientation` permitted, required, or forbidden by the contract? |
| UNKNOWN-02 | Is the Rust spec's enum superset (`ButtonVariant::Danger`, `ButtonTone::Success`) in-scope vocabulary for the pilot IR, or does it require a contract change per IR-09 before it can be authored? | `docs/contracts/components/button.md` §3; `packages/contracts/components/src/types.rs` (lines 233–304); `packages/render/src/button.rs` status match | Does the pilot contract the IR must express include the undocumented `Danger` variant / `Success` tone? |

### 7.3 Divergence / observation register (non-semantic)

| ID | Observation | Why not a semantic stop |
|----|-------------|------------------------|
| OBS-01 | R §9 Svelte Note ("Lower thumb input has its max clamped to the upper value; upper thumb input has its min clamped to the lower value") describes a DOM mechanism the current Svelte/React implementations no longer use: both inputs carry `min={min}` `max={safeMax}` and the machine (`normalizeRangeValue` + per-thumb clamp in `rangeSliderTransition`) preserves the invariant | The observable contract holds: the lower ≤ upper invariant, per-thumb semantics, and `aria-valuemin/max` (both `min`/`max`, matching R §6 since `safeMax == max` when `max > min`) are unchanged; only the described clamping mechanism differs |
| OBS-02 | Svelte/React `data-state="active"` reflects `pointerActive` only; R §4 lists "a thumb is being dragged **or keyboard-adjusted**" as the active trigger | Rendering attribute nuance; keyboard adjustment still emits change/commit correctly; no public-semantics divergence |
| OBS-03 | React ports omit contract-listed web-native props: `Button.tsx` has no `formenctype`/`formmethod`; `TextInput.tsx` has no `autocorrect`. Svelte implements all three | React is not the semantic authority (contract is, and Svelte is the parity authority per `docs/parity/*.md`); recorded as evidence gap + papercut, not a contract contradiction |
| OBS-04 | `docs/parity/{button,range-slider,text-input}.md` source paths reference pre-migration crate locations (`packages/gpui/components/…`, `packages/jetstream/components/…`) that no longer exist; implementations now live in `packages/render/src/` with node backends | Parity docs lag the render-tier migration; current render/specimen files are the live evidence |
| OBS-05 | GPUI/Jetstream render affixes/affordances as inline flex children rather than the absolutely-positioned overlays of T §2 anatomy; neither reserves control padding the way Svelte does | `docs/parity/text-input.md` records this as a shared architectural delta, visually equivalent for static previews |
| OBS-06 | Button density adjustment values (compact −0.125rem padding + gap 0.25rem; comfortable +0.125rem + `space-inline-md`) exist only in `button.css` and `render/src/button.rs`; B §8 has no density table | Stylesheet/evidence is concrete; contract silence recorded as `GAP-06` |

### 7.4 Evidence gaps (named, not inferred)

| ID | Gap | Affected requirements | Current evidence (insufficient alone) |
|----|-----|-----------------------|----------------------------------------|
| GAP-01 | No `rangeSlider` or text-input conformance vectors in `packages/contracts/headless/vectors/machines.json` (only `slider`); range behavior is tested per-implementation instead of from one shared vector file | RNG-29, CROSS-18 | `core/test/wave1.test.ts`; `headless/src/slider.rs` tests; `headless/tests/conformance.rs` (slider vectors only) |
| GAP-02 | No executed assistive-technology traces for the three pilot components on native targets; `native-accessibility-proof.json` records manual proof as still pending | BTN-21, RNG-19/20, TXT-26 | `packages/gpui/native-accessibility-proof.json` (manualReviewExpectations); `docs/parity/*.md` "accepted: no ARIA" notes |
| GAP-03 | Native vertical orientation for RangeSlider unimplemented on both Rust targets (render ignores `orientation` for layout) | RNG-07, RNG-26 | `docs/parity/range-slider.md` (open items); `render/src/range_slider.rs` |
| GAP-04 | GPUI per-thumb focus / Tab cycling for RangeSlider unimplemented (single wrapper focus ring) | RNG-20 | `docs/parity/range-slider.md` (accepted delta) |
| GAP-05 | Jetstream TextInput typing/key events absent (`onValueChange`, `onKeyDown`, `onSubmit`, `onCancel` have no route); host owns the editor | TXT-31 | T §12 delta (tracked g12.017); `jetstream/adapter/src/render_input.rs` |
| GAP-06 | Button density adjustment values are contract-silent (stylesheet-only); B §8 has no density table | BTN-05 | `button.css` density blocks; `render/src/button.rs` |
| GAP-07 | `truncate`/`fit`/`maxWidth` absent from `ButtonSpec` (representable, deferred feature pass) | BTN-11/12/13 | `docs/parity/button.md` |

## 8. Counts and validation exit states

### Requirement counts

| Set | Count |
|-----|-------|
| CROSS-* | 21 |
| BTN-* | 29 |
| RNG-* | 29 |
| TXT-* | 32 |
| SHELL-* | 10 |
| NEG-* | 8 |
| **Total requirements** | **129** |
| UNKNOWN-* | 2 |
| OBS-* (divergence) | 6 |
| GAP-* (evidence gaps) | 7 |

### Counts by component and classification

Corrected 2026-08-11 by the orchestrator — see the amendment note below.

| Component | SDD | GTA | AC | CV | EXT | Total |
|-----------|-----|-----|----|----|-----|-------|
| Cross-cutting (CROSS) | 16 | 1 | 2 | 5 | 0 | 21 |
| Button (BTN) | 26 | 0 | 1 | 0 | 3 | 29 |
| RangeSlider (RNG) | 20 | 0 | 4 | 13 | 2 | 29 |
| TextInput (TXT) | 23 | 0 | 7 | 9 | 1 | 32 |
| Shells (SHELL) | 9 | 1 | 0 | 1 | 0 | 10 |
| Negative (NEG) | 0 | 0 | 0 | 0 | 0 | 8 (IR-boundary cases) |
| **Total** | 94 | 2 | 14 | 28 | 6 | 129 |

(A requirement carrying two codes counts in both columns; NEG rows are
IR-boundary cases, not vocabulary classes.)

**Amendment note (2026-08-11).** As merged at `2f8dc5db`, this table read
`CROSS 16/1/4/9/0`, `BTN 21/0/2/2/5`, `RNG 16/0/5/15/3`, `TXT 18/0/11/12/2`,
total `80/2/22/39/10`. Those figures do not match the classification column of
the corpus's own requirement rows in §1–§5. Batch `g13-b002` found the
discrepancy on the `EXT` column (§4.5 of
`docs/roadmaps/g13/pilot-baseline-manifest.md`); the orchestrator then audited
all five columns and found four of them wrong. The table above is recomputed
directly from the row marks, which are the individually cited and evidenced
data; the previous figures were planning-time arithmetic. The requirement
counts themselves (129 total; `CROSS` 21, `BTN` 29, `RNG` 29, `TXT` 32,
`SHELL` 10, `NEG` 8) were verified correct at merge and are unchanged. Any
consumer that recorded `EXT = 10` — including a runtime-extension baseline —
should use `EXT = 6` (`BTN-26`, `BTN-27`, `BTN-29`, `RNG-26`, `RNG-27`,
`TXT-31`).

### Acceptance criteria

- [x] Corpus covers all three pilot components and all four preview shells.
- [x] Every requirement has a stable ID, classification, contract citation, and evidence path or explicit gap.
- [x] Controlled state, environment capabilities, accessibility, recipes, axes, VisualState, and event timing are distinct rows (`CROSS-04/06`, `CROSS-09/11`, `CROSS-14/16/17`, `BTN-14/21/22`, `RNG-11/16/19/21`, `TXT-12/21/26/27/28`).
- [x] Negative cases directly exercise IR-03–IR-06 (`NEG-01`…`NEG-08`).
- [x] No representation/schema recommendation appears anywhere in this corpus.
- [x] Batch log records command exit states and requirement counts: `docs/logs/2026-08/11-g13-pilot-expressiveness-corpus.md`.

## Promotion note

Per the batch card: this corpus is the acceptance input for the g13.002
schema card. It is research evidence; it does not become architecture by
itself.
