# Changelog

Notable changes to Poodle are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/spec/v2.0.0.html). Poodle is pre-1.0,
so minor releases may contain documented breaking changes.

## [Unreleased]

Nothing yet.

## [0.2.1] - 2026-08-23

### Fixed

- Release automation installs the pinned npm trusted-publishing CLI into an
  isolated runner prefix. The `0.2.0` workflow attempted to replace its own
  global npm installation in place and failed before validation or
  publication when the running CLI could no longer load `promise-retry`.
  Version `0.2.1` carries the same product changes prepared for `0.2.0`.

## [0.2.0] - 2026-08-23

The tagged candidate prepared against a frozen, measured denominator: 175 public
Svelte components, each with a contract, implementation, export, specimen,
focused tests, and packed-tarball import proof. Its release workflow failed
before validation or npm publication; `0.2.1` is the replacement release. See the
[full 0.2.0 release notes](docs/release-notes/0.2.0.md) for the publication
set, per-runtime evidence posture, the visual-comparison boundary, and the
complete migration checklist.

### Added

- **42 new components**, taking Svelte and React from 133 to 175: AgentChatInput,
  AgentMessage, AgentPlan, AgentPlanRecord, AgentQuestion, AgentQuestionRecord,
  AgentSubagent, AgentTranscript, AudioMeter, AudioSwitch, ChangedFiles,
  DragNumberField, EnvelopeEditor, Fader, GainReductionMeter, HistoryCenter,
  Keyboard, Knob, LicenceActivation, LicenceSeats, LicenceStatus,
  MessageCenter, MeterSurface, ModMatrixGrid, ModelCatalogueEditor,
  ModelConnectionCard, ModelConnectionPicker, ModelConnectionSetup, ModelPicker,
  RefSelect, RemediationBanner, SettingsShell, StateTile, Stepper, ToolCall,
  ToolCallGroup, UpdateCenter, UpdateStatus, ValidationSummary, ValueReadout,
  WaveformDisplay, XYPad.
- **Breadcrumb item icons.** `BreadcrumbItem` is a discriminated union so
  `iconOnly` cannot be authored without an icon; every crumb supports
  text-only, icon-plus-label, or accessible icon-only presentation across
  Svelte, React, shared Rust, and the GPUI specimen.
- **Button `controls` prop** in Svelte and React, rendered as `aria-controls`
  when non-null, with `ButtonSpec::controls` / `with_controls` in
  `poodle-specs` projected to `NodeA11y.controls` on the shared render path
  (the existing IconButton seam). Both web roots also re-export the
  core-authored `PopoverTriggerState` type.
- **Shared `ToneFill = "tint" | "solid"`** with a `fill` prop on Callout and
  RemediationBanner, plus the matching Rust spec fields and builders.
- **A reusable focus-ring channel** in `poodle-node`, projected out-of-flow by
  GPUI. Button and Stepper declare it instead of recolouring their borders, and
  Stepper accepts keyboard entry with no pointer prelude.
- **Additive node accessibility**: `NodeA11y.value_min` / `NodeA11y.value_max`,
  `SplitViewSpec::divider_instance_id()`, and
  `poodle_render::resize_handle_focus_id`.

### Fixed

- **Tabs drag dead in WebKit hosts.** `draggable` and `dragstart` sat on the
  `__item` chip div wrapping the `__tab` button; WebKit does not initiate a
  native drag when the press target is a form control inside a draggable
  ancestor, so WKWebView (Tauri) hosts could not drag any tab. The drag source
  moved onto `.poodle-tabs__tab` itself (Svelte and React); drop-target
  handling stays on the item. Consumers styling `[draggable]` on
  `.poodle-tabs__item` should target `.poodle-tabs__tab` instead.

### Removed

- **Breaking — Tabs `variant`.** The six-member `TabVariant` union is now three
  members: `"card" | "pill" | "block"`. `"text"`, `"underline"`, and `"strip"`
  are gone. `"underline"` was never a distinct look — it aliased `"text"` and
  had no stylesheet rules of its own.

  Migrate as follows. Note that `"card"` is a **reused name, not a preserved
  one**: the old `"card"` filled the tab chip, and the new `"card"` is the old
  `"text"`. Applying this table before upgrading will change how your tabs look.

  | Before | After |
  |---|---|
  | `variant="text"` | `variant="card" bordered` |
  | `variant="underline"` | `variant="card" bordered` |
  | `variant="strip"` | `variant="block" activeEdge="underline" activeFill="none"` |
  | `variant="card"` | `variant="card"` — appearance changed; see `bordered`, `activeEdge`, `activeFill` |
  | `variant="pill"`, `variant="block"` | unchanged |

  `TabStrip` is a separate component and is unaffected.
- **Breaking — Icon `density`.** Icon size is decided by `size` or by the
  inherited presentation scale through `sizeRole`; a density input on a glyph
  had no contract behind it. `IconSpec::density` / `with_density` are gone in
  Rust too, and `IconSize` expands to `Xs | Sm | Md | Lg | Xl` with a 1:1
  `ControlSize` mapping. Remove `density` from direct `Icon` callers and update
  exhaustive `IconSize` matches.
- **Breaking (Rust) — `PillSpec::fill`, `with_fill`, and `is_solid_fill`;
  `EmptyStateSpec::compact` and `with_compact`.** Use
  `PillSpec::is_solid_appearance` and
  `EmptyStateSpec::with_size(EmptyStateSize::Compact)`.
- **Breaking (Rust) — the twelve `poodle_render::audio_specimens` page
  functions.** `AudioSpecimen::{examples, size, density}` replaces them, so
  hosts compose axis panes through their own layout instead of receiving a
  pre-appended matrix.

### Changed

- **Breaking — Popover interactive trigger composition.** Composing a real
  Button or IconButton as a Popover trigger no longer forces a choice between
  nested interactive semantics and a missing disclosure relationship. In
  interactive mode (`triggerIsInteractive`) the trigger is now a state-aware
  render that receives the core-authored `PopoverTriggerState` (`expanded`,
  `controls`, `disabled`): Svelte `trigger: Snippet<[PopoverTriggerState]>`,
  React `trigger: (state: PopoverTriggerState) => ReactNode`. The actual
  control owns `aria-expanded`, `aria-controls`, and the disabled state — in
  server output and hydrated DOM alike — while the wrapper stays a roleless,
  untabbable layout host. The old interactive shape (a static node or
  zero-argument snippet beside `triggerIsInteractive`) is gone: React rejects
  it at compile time; Svelte's discriminated snippet typing rejects a
  wrongly-typed payload and wrong-branch usage but cannot reject a
  zero-argument snippet (TypeScript function assignability), so Svelte
  migration is enforced by search and review.

  Migrate: give every interactive trigger the state parameter and apply all
  three fields to the real control — `Button` takes `ariaExpanded` /
  `controls` / `disabled`, `IconButton` takes `expanded` / `controls` /
  `disabled`, a native button takes `aria-expanded` / `aria-controls` /
  `disabled`. Direct Rust `ButtonSpec` struct literals must initialize the new
  `controls` field; builder callers are source-compatible.
- **Breaking — Tabs `bordered` now defaults to `false`.** This is a silent
  visual change: tabs rendered above a panel lose their separating line with no
  type or build error. Add `bordered` explicitly to any usage that draws tabs
  over content. The old default assumed "tabs above content", a layout Tabs
  cannot see, and every other usage paid for it in dead space.
- **Tabs selection decoration is now two orthogonal axes.** `activeEdge`
  (`"none" | "outline" | "underline"`, default `"none"`) and `activeFill`
  (`"none" | "tint" | "solid"`, default `"tint"`) compose freely and replace the
  former per-variant treatments. `NavigationMenu` takes the same two props. Both
  types are defined once in
  [`docs/contracts/004-shared-control-types.md`](docs/contracts/004-shared-control-types.md).
- **Breaking — Pill `appearance` defaults to `"tint"`.** `PillAppearance` gains
  `"tint"` and the implicit appearance moves from `"solid"` to `"tint"`. Default
  rendered output is unchanged; the emitted `data-appearance` value is not.
  Explicit `appearance="solid"` now resolves the opaque shared
  `solid_tone_surface` recipe rather than the former Pill-local treatment.
  Re-check CSS and tests asserting `[data-appearance="solid"]` on default pills.
- **Breaking (Rust) — presentation is a construction-time cascade.** Every
  public component renderer takes `ctx: &RenderContext<'_>` instead of
  `theme: &dyn ThemeProvider`; `poodle-render` re-exports `RenderContext`,
  `ui_presentation_provider`, and `SlotBuilder` at the crate root. Semantic spec
  inputs move to `Option<ControlSize>` / `Option<ControlDensity>` (default
  `None`), with `with_size` / `with_density` unchanged in name and parameter
  type. Explicit size stays final; `sizeRole` maps inherited presentation scale
  only. Six scoped host slots take `SlotBuilder` instead of built `Node`s.
- **Breaking (Rust) — native instance identity is caller-supplied.**
  `SegmentedControlSpec::new(instance_id, options)`,
  `ResizeHandleSpec::new(instance_id)`, and
  `SplitViewSpec::new(instance_id, orientation)` require a stable native
  instance scope, and `ResizeHandleSpec` no longer implements `Default`.
- **Breaking (Rust) — SegmentedControl has its own option type.**
  `SegmentedControlOption` replaces `ChoiceOption` for this family and carries
  icon and icon-only presentation. `ChoiceOption` is unchanged for Select,
  RadioGroup, CardRadioGroup, and every other family; the web
  `SegmentedControlOption` interface is unchanged.
- **`CallOutSpec::default()`** now matches the web and contract default
  (`Neutral` + `Tint`) instead of the incorrect implicit `Info` tone. Rust
  callers that relied on the old implicit tone must set `StatusTone::Info`
  explicitly; web callers are unaffected.
- GPUI AgentTranscript owns a real tracked viewport with a detach latch and
  jump-to-bottom; GPUI Stepper binds selection, re-run, and collapse to separate
  live controls. Native arrow / `Home` / `End` stepper movement remains a
  recorded web-only delta.
- GPUI resolves through an exact Poodle-owned Zed fork that removes GPL tracing
  crates from the normal dependency graph. Git sources are admitted fail-closed
  by reviewed URL and immutable revision.
- Prepared the repository, package documentation, licensing, security policy,
  and validation surfaces for public access.
- Completed the shared Rust render-tree migration. GPUI and Jetstream now
  interpret the same `poodle-node` output instead of maintaining duplicate
  component implementations.
- Completed the native accessibility naming audit across the Jetstream
  specimen catalogue.

## [0.1.0] - 2026-07-24

### Added

- Established the first documented preview baseline: framework-free core,
  Svelte and experimental React component packages, shared tokens and themes,
  Rust contracts, the shared render tree, and GPUI and Jetstream adapters.
  It was initially documented as a source/version baseline, then tagged as
  `v0.1.0`; core and Svelte were published to npm on 2026-08-10 while React
  remained source-only.

### Changed

- Renamed theme IDs and removed the obsolete `poodle-workstation` crate. See
  the [full 0.1.0 release notes](docs/release-notes/0.1.0.md) for package lists,
  migration guidance, and downstream checks.

[Unreleased]: https://github.com/inflatable-cookie/poodle/commits/main
[0.2.1]: docs/release-notes/0.2.1.md
[0.2.0]: docs/release-notes/0.2.0.md
[0.1.0]: docs/release-notes/0.1.0.md
