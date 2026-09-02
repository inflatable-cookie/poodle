# Changelog

Notable changes to Poodle are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/spec/v2.0.0.html). Poodle is pre-1.0,
so minor releases may contain documented breaking changes.

## [Unreleased]

Nothing yet.

## [0.3.0] - 2026-09-02 (candidate — not published)

This is an immutable release candidate, not a release. No `v0.3.0` tag or
registry publication exists; certification, tagging, and publication remain
separate operator gates.

### Breaking

- **HistoryCenter v3 `HistoryEntry`.** `branchCount` is replaced by
  `continuationCount`, and the v2 `branches` / `paths` input is replaced by
  paged history data. Consumers must migrate to the v3 `pages` shape and the
  host-owned continuation callbacks. The packed Svelte root and `/types`
  paths retain the v3 type and reject the retired field with an unsuppressed
  diagnostic.
- **HistoryCenter rejection meanings.** The renderer-neutral rejection
  surface is now the five-code union `AlreadyAtTarget`, `UnknownEntry`,
  `StaleHistory`, `ProtectedEntry`, and `DeletionUnavailable`; hosts map
  authority outcomes onto those meanings instead of treating every refusal as
  an unknown entry.
- **Markdown entry points.** `AgentMessage`, `AgentPlan`, `AgentPlanRecord`,
  `AgentTranscript`, and `MarkdownEditor` move out of the Svelte and React
  package roots into explicit `/markdown` entries. Their direct component
  entries remain available. There is no compatibility alias or root fallback.

### Added and changed

- The public-intent delta accumulated after immutable `v0.2.2` includes the
  triggerless `ContextMenu` composition prepared as `0.2.3`, cross-runtime
  drag-and-drop and opaque file/cross-window bridges, Tree reorder authority,
  TimeInput and NumberInput value/draft contracts, continuous audio controls,
  shared motion policy, EditableLabel, block Slider/RangeSlider appearance,
  and same-id Toast updates.
- Core, Svelte, the private React validation package, and the Rust source/tag
  distribution set are versioned at `0.3.0`. Core and Svelte are the only
  eventual npm publication set; React remains private and Rust remains
  source/tag distribution. Internal preview/tooling packages are not consumer
  dependencies.
- The web package boundary is compiled `dist/` JavaScript plus declarations.
  The candidate consumes the accepted installed certification and records its
  own exact tarball identities separately.

### Downstream checks

- Migrate root markdown imports to `/markdown`, and update HistoryCenter
  callers from the v2 `branches` / `paths` / `branchCount` vocabulary to v3
  `pages`, `continuationCount`, and host-owned continuation operations.
- Pin a separately published exact version after certification. `0.2.3` was
  prepared but unpublished, and `0.2.4` is skipped.

## [0.2.3] - 2026-08-30 (prepared — unpublished)

`0.2.3` was prepared in the repository but was never tagged or published. Its
intended delta is carried into the `0.3.0` candidate above; `0.2.4` is skipped.

### Added

- **Triggerless `ContextMenu` overlay.** `trigger={false}` omits the
  invocation button so a tree row, list card, or canvas that already owns
  contextmenu / Shift+F10 can drive a controlled `open` + `anchorPoint`.
  `menuTransition` still owns the overlay lifecycle. Default slotted-trigger
  composition is unchanged. Svelte is the reference; React matches.
  [0.2.3 release notes](docs/release-notes/0.2.3.md).

## [0.2.2] - 2026-08-24

### Fixed

- **Breaking for Rust consumers — the public GPUI dependency identity.**
  `0.2.1` resolved `gpui` from an `inflatable-cookie/zed` Git fork instead of
  crates.io. A forked `gpui` is a different crate identity, so a consumer
  declaring `gpui = "0.2.2"` for itself could not pass GPUI values across
  Poodle's node-backend boundary at all. `poodle-gpui-node-backend` and
  `poodle-gpui-preview` now resolve `gpui = "0.2.2"` from crates.io; the fork
  and `gpui_platform` are gone from every active manifest and lockfile, and
  both Rust graphs resolve zero Git sources. `effigy
  drift:gpui-consumer-identity` proves a clean dual-dependency consumer
  compiles, with a negative control that must fail.
- **Inset box shadows are painted, not dropped.** crates.io GPUI 0.2.2 has no
  `BoxShadow::inset` field. The node backend paints the declared band itself,
  so Accordion, ActionDiscoveryPanel, ListCard, Popover, and Tabs keep the
  inset layers they declare. Renderer-internal; no contract or API change.

- **Licence and notice surfaces match the final GPUI graph.** `bzip2` and
  `libbz2-rs-sys` left both graphs with the fork while the third-party
  notices, the `deny.toml` allow entry, and spec 022 still claimed them. The
  stale claims are removed, `packages/gpui/node-backend/THIRD_PARTY_NOTICES.md`
  is deleted with the last claim it carried, and `audit:licenses` now checks
  notice truth in both directions against the lockfiles.

### Changed

- **GPUI pixel capture is a non-activating window diagnostic, not offscreen
  rendering.** Stock GPUI 0.2.2 publishes no offscreen readback API, so
  `poodle-offscreen-capture` is removed with no alias.
  `poodle-window-capture` opens one real GPUI window with `focus: false`,
  never activates the application, and refuses to publish if the frontmost
  application changed. Default native evidence stays on GPUI's in-memory test
  platform; the diagnostic sits outside `qa`, CI, and every release gate
  behind `-windowed` selectors that need operator approval. Receipt schemas
  were renamed rather than reused, and now assert `gpuiSource: "crates.io"`.
- Source policy is fail-closed: the approved-Git-revision allowlist is empty,
  `gpui` and `gpui_platform` are rejected from any Git source, and
  `deny.toml` carries `allow-git = []`.

The web packages carry no code change in this patch. `@inflatable-cookie/poodle-core`
and `@inflatable-cookie/poodle-svelte` move to `0.2.2` so the ecosystem
version set stays aligned across one tag. See the
[0.2.2 release notes](docs/release-notes/0.2.2.md).

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
[0.3.0]: docs/release-notes/0.3.0.md
[0.2.3]: docs/release-notes/0.2.3.md
[0.2.2]: docs/release-notes/0.2.2.md
[0.2.1]: docs/release-notes/0.2.1.md
[0.2.0]: docs/release-notes/0.2.0.md
[0.1.0]: docs/release-notes/0.1.0.md
