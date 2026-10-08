# Changelog

Notable changes to Poodle are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/spec/v2.0.0.html). Poodle is pre-1.0,
so minor releases may contain documented breaking changes.

## [Unreleased]

## [0.4.14] - 2026-10-08

`0.4.14` adds the `Listbox` composite across Svelte, React, shared Rust and
GPUI. Additive only.

### Added

- **`Listbox`.** Listbox semantics over host-rendered rows: `role="listbox"`
  with `role="option"` children, `aria-selected`, one tab stop with roving
  focus, typeahead, `selectionMode` `"single"` or `"multiple"` (Shift-arrow
  ranges, Ctrl/Cmd+A), `"vertical"` or `"horizontal"` orientation, per-option
  and root `disabled`, `onValueChange` / `onValuesChange`, and `onActivate`
  on Enter or double-click. Controlled (`value` / `values`) or uncontrolled
  (`defaultValue` / `defaultValues`). Row content is the host's and must not
  be interactive. Svelte, React (`items` plus a row renderer), core machine,
  `ListboxSpec` with `with_*` builders, `poodle-render` and GPUI. 2D card-grid
  navigation, filtering and virtualization are out of scope. Use it instead
  of a non-interactive `ListCard` inside a host `role="option"` element.

### Fixed

- **Listbox focus ring stays visible on stacked rows.** The option ring is
  inset, and the focused option paints above its siblings on web; GPUI insets
  by the focus-ring width.

### Changed

- **Web package dependency ranges.** Svelte and React require core
  `>=0.4.14 <0.5`. Peer ranges stay unchanged.
- **Cargo crates move to `0.4.14`** with internal path pins at the same exact
  version.

## [0.4.13] - 2026-10-08

`0.4.13` is a coordinated npm/web and native release with additive API only.

### Added

- **Keyboard equal-row layout.** `keyLayout="equal-rows"` gives every
  semitone the same pitch-axis space, with `rowHeightPx` (default `16`) and
  `scrollOffsetPx` (default `0`), so a vertical Keyboard stays aligned with a
  scrolled piano-roll note grid. Drawing and pointer hit testing use the same
  rows, including partly visible ones. Horizontal equal-rows gives equal
  columns. `"piano"` stays the default. Svelte, React, `KeyboardSpec`
  (`with_key_layout`, `with_row_height_px`, `with_scroll_offset_px`),
  `poodle-render` and GPUI.
- **`@inflatable-cookie/poodle-core/icons/geometry-realize`.** Test-support
  `realize(state)` returns the SVG attributes for every icon-geometry state,
  including reverse-midpoint, interruption and teardown. Poodle's own test
  shells render through it; sibling test hosts can import it. It is not a
  component.
- **Native preview arguments.** The GPUI and Jetstream previews accept
  `--window-bounds <x>,<y>,<w>,<h>`. The Jetstream preview accepts
  `--component`, `--theme` and `--size`; an unknown component exits before a
  window opens.

### Fixed

- **Jetstream preview compiles again** against current Poodle and Jetstream.
  This is a preview repair only; Jetstream admission stays deferred.

### Changed

- **Web package dependency ranges.** Svelte and React require core
  `>=0.4.13 <0.5`. Peer ranges stay unchanged.
- **Cargo crates move to `0.4.13`** with internal path pins at the same exact
  version.

## [0.4.12] - 2026-10-07

`0.4.12` is the first coordinated release of both trains: the npm/web
packages and the native Rust crates move together under one `v0.4.12` tag.
Web packages carry no behaviour change.

### Added

- **Native release train.** The Rust and GPUI crates release by git tag,
  never crates.io. An application pins public crates (`poodle-node`,
  `poodle-render`, `poodle-gpui`, `poodle-gpui-node-backend` and the contract
  crates) to `tag = "v0.4.12"` and uses `gpui-unofficial =1.22.0` so both
  sides share GPUI's types. `effigy release:bump` moves npm and Cargo
  versions together.
- **`toolbar_scope` and `ToolbarSpec::instance_id` (native).** Two Toolbars
  with the same label, or none, no longer share generated focus ids. Name the
  instance with `with_instance_id`; the aria label stays the fallback.

### Fixed

- **GPUI keyboard activation is scoped per window.** Suppressing a repeated
  key activation in one window no longer swallows the same key in another.
- **GPUI composite focus shows only for keyboard focus.** Pointer-focused
  composites no longer draw the focus ring, matching Svelte.

### Changed

- **Web package dependency ranges.** Svelte and React require core
  `>=0.4.12 <0.5`. Peer ranges stay unchanged.
- **Cargo crates move to `0.4.12`** (they were `0.4.0` at the `v0.4.11`
  tag) with internal path pins at the same exact version.

## [0.4.11] - 2026-10-06

`0.4.11` is a patch candidate with one fix and no API change.

### Fixed

- **Drag and drop commits fast or off-handle releases.** When a pointer drag
  was released away from its handle, browsers could deliver
  `lostpointercapture` before `pointerup`; the shared drag controller treated
  that as a lost transport and cancelled, so the drop did nothing. A capture
  lost after the button is already released now counts as the release and
  commits the drop; a capture lost while the button is still down still
  cancels. This fixes every web drag-and-drop component (Tabs, EditableList,
  OrderBy, ModelCatalogueEditor, BlockEditor and others) in Svelte and React.

### Changed

- **Web package dependency ranges.** Svelte and React require core
  `>=0.4.11 <0.5`. Svelte, React, React DOM and Marked peer ranges stay
  unchanged.
- **Release status.** Core and Svelte are the `0.4.11` npm publication set.
  React follows the web version for paired validation and remains private.
  Native work since `0.4.10` (AccessKit, keyboard-origin focus on GPUI)
  belongs to the native train, which has no release procedure yet.

## [0.4.10] - 2026-10-06

`0.4.10` is a patch candidate with one fix and no API change.

### Fixed

- **Collapsible and Accordion open at full height.** Opening a Collapsible or
  an Accordion item showed an empty panel: the open animation measured the
  content while it was still hidden, animated from 0px to 0px, and the
  finished animation held the panel at 0px. The height is now measured after
  the content is shown, and a finished animation no longer pins it, so the
  panel follows its content. This also fixes components built on them, such as
  ModelCatalogueEditor's hidden models. Present since the shared motion
  runtime (2026-09-01).

### Changed

- **Web package dependency ranges.** Svelte and React require core
  `>=0.4.10 <0.5`. Svelte, React, React DOM and Marked peer ranges stay
  unchanged.
- **Release status.** Core and Svelte are the `0.4.10` npm publication set.
  React follows the web version for paired validation and remains private.
  Native work since `0.4.9`, including the move to `gpui-unofficial` 1.22.0,
  belongs to the native train, which has no release procedure yet.

## [0.4.9] - 2026-10-05

`0.4.9` is a patch candidate with one behaviour change and no breaking API
change.

### Changed

- **ContextMenu returns focus to its invoker.** Closing a ContextMenu through
  Escape, outside interaction or item activation now moves focus back to the
  element that opened it (the keyboard `ContextMenu`/`Shift+F10` target or the
  right-clicked element), or to the nearest surviving fallback if that element
  is gone. Previously focus stayed where it was. Menus without a trigger,
  including SidebarNav's per-item menus, keep their consumer-owned
  restoration. No props or events change.
- **Web package dependency ranges.** Svelte and React require core
  `>=0.4.9 <0.5`. Svelte, React, React DOM and Marked peer ranges stay
  unchanged.
- **Release status.** Core and Svelte are the `0.4.9` npm publication set.
  React follows the web version for paired validation and remains private.
  GPUI parity work merged since `0.4.8` belongs to the native train, which has
  no release procedure yet; Cargo packages do not move in this patch.

### Fixed

- **PasswordRequirements semantics.** Each requirement row carries an
  accessible label stating whether it is met, its icon is hidden from
  assistive technology, and the error message is exposed as an alert.
- **GPUI motion plan.** `gpuiMotionPlan` applies `translateX` and
  `translateY` instead of dropping them, so native toast entrances move as
  contracted.

## [0.4.8] - 2026-10-04

`0.4.8` is an additive patch candidate with no breaking API change.

### Added

- **Tabs fill as a flex or grid child.** `layout="fill"` now takes its size
  from a flex or grid parent that has no definite block size, as well as from
  a sized container.
- **Tabs scroll owner.** Svelte and React `Tabs` accept
  `panelScroll="panel" | "content"` (default `"panel"`): the panel scrolls, or
  the consumer's own content owns scrolling.
- **Tabs chrome tokens.** `--poodle-tabs-gap`, `--poodle-tabs-list-padding`,
  `--poodle-tabs-list-background` and `--poodle-tabs-list-border` (schema keys
  `tabs.gap`, `tabs.list.padding`, `tabs.list.background`, `tabs.list.border`)
  set the root gap and the tab list's chrome. Defaults match the previous
  rendering.
- **Tabs part hooks.** Every Tabs part carries a contracted `data-part`
  (`root`, `list`, `item`, `trigger`, `label`, `close`, `separator`,
  `actions`, `indicator`, `panel`, `tooltip`), and the hidden measurement
  list carries `data-measure="true"` with no `data-part`. Internal
  `poodle-tabs__*` class names are unchanged but are not public API.

### Changed

- **Web package dependency ranges.** Svelte and React require core
  `>=0.4.8 <0.5`. Svelte, React, React DOM and Marked peer ranges stay
  unchanged.
- **Release status.** Core and Svelte are the `0.4.8` npm publication set.
  React follows the web version for paired validation and remains private.
  GPUI parity work merged since `0.4.7` belongs to the native train, which has
  no release procedure yet; Cargo packages do not move in this patch.

## [0.4.7] - 2026-09-30

`0.4.7` is an additive patch candidate with no breaking API change.

### Added

- **History and pin icons.** The default icon set adds `pin` and
  `rotate-ccw-clock`, with the semantic alias `history` for
  `rotate-ccw-clock` (Lucide 1.48.0 has no `history` glyph). HistoryCenter
  uses them for its history and checkpoint-pin affordances.
- **Surface hover token.** `--poodle-color-surface-hover`
  (`color.surface.hover`) joins the token schema.

### Fixed

- **Svelte package JavaScript.** The Svelte package build no longer ships a
  component whose script declares an optional parameter as invalid
  JavaScript; every consumer import of such a component failed before.
- **FormLayout errors.** A FormLayout (and FormDialog) submission error is
  exposed as an alert with its full message, so assistive technology and
  semantic snapshots read it.
- **Searchable Select.** A searchable Select opens showing every option
  instead of only those matching its committed label; choosing another option
  needs no clearing.
- **CodeEditor active line.** The active line and gutter use the real
  `--poodle-color-surface-hover` token instead of an undefined variable.
- **MessageCenter width.** MessageCenter content no longer fights its
  popover surface's width and spills past its edge.
- **SplitView and TextInput.** SplitView pins its seam to a fixed secondary
  pane, and multiline TextInput forwards `required`, `inputMode` and
  `enterKeyHint`.
- **Tabs focus.** Controlled Tabs keep focus on the latched transfer timer
  when a later commit supersedes it.

### Changed

- **Web package dependency ranges.** Svelte and React require core
  `>=0.4.7 <0.5`. Svelte, React, React DOM and Marked peer ranges stay
  unchanged.
- **Release status.** Core and Svelte are the `0.4.7` npm publication set.
  React follows the web version for paired validation and remains private.
  Native features admitted to Rust and GPUI since `0.4.6` (Text/Code wrap,
  ListCard eyebrow, Pill dismiss, Keyboard `computerBaseNote`, SidebarNav end
  label and context menu, `NodeKey::Escape`) belong to the native train, which
  has no release procedure yet; Cargo packages do not move in this patch.

## [0.4.6] - 2026-09-27

`0.4.6` is an additive patch candidate with no breaking API change.

### Added

- **ListCard eyebrow slot.** Svelte and React `ListCard` accept an optional
  `eyebrow` (text) or `eyebrowContent` (snippet/node), rendered above the title
  in the card body. The eyebrow never changes the card's accessible name.
  Web-admitted; the portable spec follows a later native evidence repin.
- **Pill dismiss control.** Svelte and React `Pill` accept `dismissible`,
  rendering a trailing native `<button class="poodle-pill__dismiss">` whose
  accessible name comes from `dismissLabel` (default `"Dismiss"`) and whose
  click fires `onDismiss`. Web-admitted; the portable spec follows a later
  native evidence repin.
- **SidebarNav per-item context menu.** Svelte and React `SidebarNavItem`
  accept `contextMenuItems` and `contextMenuAriaLabel`, opening the shared
  `ContextMenu` from a right-click or `ContextMenu`/`Shift+F10`.
  `onContextAction(itemValue, actionValue)` reports the nav item and the chosen
  menu row. Web-admitted; the portable spec follows a later native evidence
  repin.

### Changed

- **Web package dependency ranges.** Svelte and React require core
  `>=0.4.6 <0.5`. Svelte, React, React DOM and Marked peer ranges stay
  unchanged.
- **Release tooling.** The internal publish-mode registry check revalidates
  its reads with `npm view --prefer-online` over a longer read-only budget. It
  changes no consumer behavior.
- **Release status.** Core and Svelte are the `0.4.6` npm publication set.
  React follows the web version for paired validation and remains private.
  Cargo packages and native evidence do not move in this patch.

## [0.4.5] - 2026-09-27

`0.4.5` is an additive patch candidate with no breaking API change.

### Added

- **SidebarNav item end labels.** Svelte and React `SidebarNavItem` accept
  `endLabel` for compact, muted, end-aligned metadata such as a count. The
  item's accessible name remains `label`; the end label is its description.

### Changed

- **Web package dependency ranges.** Svelte and React require core
  `>=0.4.5 <0.5`. CodeMirror, Lezer and TipTap runtime dependencies use caret
  ranges. Svelte, React, React DOM and Marked peer ranges stay unchanged.
- **Release tooling.** Candidate admission derives its base after merge, and
  the internal `test:web-scope` per-test cap is 120 seconds. Neither changes
  consumer behavior.
- **Release status.** Core and Svelte are the `0.4.5` npm publication set.
  React follows the web version for paired validation and remains private.
  Cargo packages and native evidence do not move in this patch.

## [0.4.4] - 2026-09-27

`0.4.4` is an additive patch candidate with no breaking API change. It
supersedes `0.4.3`, which was never published.

### Added

- **Text and Code wrap.** Svelte and React `Text` and `Code` accept
  `wrap="normal" | "anywhere"` (default `normal`). `anywhere` applies
  `overflow-wrap: anywhere` so long identifiers can break; unset and
  `normal` leave current wrapping unchanged. Web-admitted; the portable
  spec and GPUI mapping wait for a later native evidence pin.
- **IconButton data attributes.** Svelte and React IconButton forward host
  `data-*` attributes to the root button.

### Changed

- **JavaScript dependencies.** Workspace JavaScript dependencies refresh to
  their newest compatible releases. Notable runtime moves: `lucide-static`
  1.48.0 (shipped default icon set; `trash` is an additive export, `trash-2`
  remains an alias, no export removed), `@codemirror/*` and
  `@lezer/highlight` patches, and root `react` / `react-dom` 19.3.0. Peer
  ranges stay `svelte >=5.56.8 <6`, `react` / `react-dom >=18`, and
  `marked ^18.0.9`. The Svelte build toolchain stays pinned at the published
  5.56.8 peer floor.
- **Bun 1.4.2.** `packageManager` is `bun@1.4.2`. Contributors and CI
  install from that pin.
- **Release admission.** The internal candidate-base derivation fix allows
  admission after a candidate merge. It does not change consumer behavior.
- **Release status.** Core and Svelte are the `0.4.4` npm publication set.
  React follows the web version for paired validation and remains private.
  Cargo packages and native evidence do not move in this patch.

## [0.4.3] - 2026-09-27

### Added

- **Text and Code wrap.** Svelte and React `Text` and `Code` accept
  `wrap="normal" | "anywhere"` (default `normal`). `anywhere` applies
  `overflow-wrap: anywhere` so long identifiers can break; unset and
  `normal` leave current wrapping unchanged. Web-admitted; the portable
  spec and GPUI mapping wait for a later native evidence pin.
- **IconButton data attributes.** Svelte and React IconButton forward host
  `data-*` attributes to the root button.

### Changed

- **JavaScript dependencies.** Workspace JavaScript dependencies refresh to
  their newest compatible releases. Notable runtime moves: `lucide-static`
  1.48.0 (shipped default icon set; `trash` is an additive export, `trash-2`
  remains an alias, no export removed), `@codemirror/*` and
  `@lezer/highlight` patches, and root `react` / `react-dom` 19.3.0. Peer
  ranges stay `svelte >=5.56.8 <6`, `react` / `react-dom >=18`, and
  `marked ^18.0.9`. The Svelte build toolchain stays pinned at the published
  5.56.8 peer floor.
- **Bun 1.4.2.** `packageManager` is `bun@1.4.2`. Contributors and CI
  install from that pin.
- **Release status.** Core and Svelte are the `0.4.3` npm publication set.
  React follows the web version for paired validation and remains private.
  Cargo packages and native evidence do not move in this patch.

## [0.4.2] - 2026-09-14

### Fixed

- **Tabs cross-window transfer.** Svelte Tabs once again forwards its public
  `crossWindowSourceBridge` to each tab item, so host preparation occurs during
  pointer pre-drag instead of leaving transfer state idle. Source and
  source-free packed-package regressions cover the bridge path.
- **Slider drag containment.** Elements with `role="slider"` are included in
  the shared interactive boundary, preventing fader and slider gestures from
  starting an ancestor drag source.
- **Menu accessible names.** Svelte and React MenuSurface rows expose their
  required visible item labels as exact explicit accessible names across Menu
  and ContextMenu.
- **npm publication reliability.** Publish mode consumes the certified local
  archives with explicit relative paths and waits for bounded npm registry
  propagation after successful trusted publication.

### Changed

- **Release status.** Core and Svelte are the `0.4.2` npm publication set.
  React follows the web version for paired validation and remains private.
  Cargo packages and native evidence do not move in this patch.

## [0.4.1] - 2026-09-13

### Added

- **Editor presentation controls.** `CodeEditor` now accepts `size` and
  `sizeRole`; `RichTextEditor` accepts `size`, `sizeRole`, and `density`; and
  `RichTextRenderer` and `MarkdownRenderer` accept `size` and `sizeRole`.
  Explicit values override the shared `UiPresentation` axes in both Svelte and
  React.

### Fixed

- **Slider and RangeSlider geometry.** Block presentation keeps thumbs inset,
  resolves two-thumb collisions without endpoint drift, selects the requisite
  thumb on track clicks, keeps labels spatially stable, docks values only when
  measured space is safe, and restores usable vertical layout. Bipolar sliders
  fill from the centre.
- **Editor specimens and sizing.** CodeEditor typography and chrome now respond
  consistently to size and density. Rich-text and markdown typography follows
  the shared size presentation. The rich-text image example now uses a modal
  picker with presets and custom URL, alt, and title fields.
- **Tooltip input modality.** Tooltip, IconButton, and Tabs still reveal
  descriptions on hover and genuine keyboard focus, but no longer open from
  pointer-triggered programmatic focus when a Dialog or Popover autofocuses a
  child.
- **Tabs underline scrolling.** The measured underline now uses tablist
  scroll-content coordinates, so manual scrolling, programmatic tab reveal,
  and resize remeasurement keep it aligned with the selected tab.
- **npm candidate isolation and CI routing.** The generic web-candidate
  admission keeps Cargo manifests and locks outside the npm release-input set
  and is selected by installed-package CI for versioned web candidates.

### Changed

- **Release status.** Core and Svelte are the `0.4.1` npm publication set.
  React follows the web version for paired validation and remains private.
  Cargo packages and native evidence do not move in this patch.

## [0.4.0] - 2026-09-12

### Breaking

- **Slider and RangeSlider presentation API.** Both controls now expose one
  `variant` prop. `variant="block"` is the default capsule and
  `variant="embedded"` is the dense track-and-thumb alternative. The pre-0.4
  `appearance` prop, `variant="standard"`, the external fallback-text helpers
  (`sliderFallbackText`, `rangeSliderFallbackText`), the combined
  `formatVisibleRange` string, `defaultVisibleRangeText`, and
  `resolveRangeVisibleRange` are removed with no aliases. Vertical block
  presentation is supported in every runtime, so `assertHorizontalBlockAppearance`
  and the Rust `reject_vertical_block` guard are gone. `SliderAppearance` is
  replaced by `SliderVariant`, and `SliderPolarity` is now exported too. Block
  always paints the lower and upper endpoint values at their scale anchors; use
  `formatVisibleValue` per thumb for custom text. Update consumer CSS from
  `[data-appearance="block"]` to `[data-variant="block"]`.
- **Slider-family recipe hooks.** Eighteen documented recipe hooks are no longer
  read because Slider and RangeSlider share one family foundation. Removed
  Slider hooks: `--poodle-recipe-slider-block-fallback-text` (no successor),
  `-control-thumb-fill` (use `-control-fill`), `-control-shadow` (none),
  `-focus-control-shadow` (use `-focus-ring`). Removed RangeSlider hooks:
  `--poodle-recipe-range-slider-track-fill`, `-track-border`, `-center-fill`,
  `-control-fill`, `-block-remainder-fill`, `-block-remainder-text`,
  `-block-handle-fill`, `-block-handle-border`, `-block-focus-ring` (all use
  the matching `--poodle-recipe-slider-*` family name),
  `-control-thumb-fill` and `-control-thumb-shadow` (none),
  `-focus-control-thumb-shadow` (use `--poodle-recipe-slider-focus-ring`),
  `-control-track-fill` and `-block-fallback-text` (none). The retained
  RangeSlider-specific hooks are `-block-selected-fill`, `-block-selected-text`,
  `-fill-fill`, `-fill-negative`, and `-focus-ring`. The new hook
  `--poodle-recipe-tabs-card-item-fill` was added.
- **Markdown preview is safe by default.** `MarkdownEditor` sanitizes the
  complete HTML result of the built-in `marked` path and of a custom
  `renderHtml` closure unless `htmlPolicy="trusted"` is set. Supplying
  `renderHtml` no longer implies trust. `MarkdownRenderer` renders through the
  same safe path. `MarkdownEditor.renderHtml` is now typed as the named
  `MarkdownHtmlRenderer` (same signature, no migration).
- **Rust source/tag consumers.** `poodle-specs` removes `SliderAppearance`,
  `SliderSpec.appearance`/`with_appearance`, `RangeSliderSpec.appearance`,
  `.visible_range_text`, `with_visible_range_text` and `reject_vertical_block`;
  `RangeSliderSpec::with_appearance` becomes `with_variant`. `poodle-headless`
  removes `slider_fallback_text`, `range_slider_fallback_text`,
  `default_visible_range_text` and `resolved_range_text`, changes
  `default_visible_value_text` and `resolved_visible_text` to take `min`/`step`,
  and replaces the `fallback` layout fields with `label_inline`/`value_inline`.
  `poodle-node` gains `Banner`, `Heading` and `SearchBox` `NodeRole` variants and
  `NodeA11y.initial_focus`, so exhaustive matches and struct literals must be
  updated. `poodle-render::slider_block::block_grab` is removed in favour of
  `block_grab_with_axis`, `block_surface` takes `cross_px`, and
  `fraction_anchor` takes a trailing `layer_offset`. `TabDefinition.pinned` and
  `TabsItem.pinned` are additive fields that break exhaustive struct literals;
  use the builder or `pinned: None`.

### Added

- **Web editor entries.** `@inflatable-cookie/poodle-svelte` and
  `@inflatable-cookie/poodle-react` gain `./editor`,
  `./editor/codemirror` and `./rich-text`. `CodeEditor` is
  CodeMirror-backed with a consumer-selected language registry;
  `RichTextEditor` and `RichTextRenderer` are TipTap/ProseMirror-backed with a
  configurable H1-H6 heading select; `MarkdownRenderer` joins the existing
  `./markdown` entry as a web-only companion. The new CodeMirror, `@lezer/highlight`
  and TipTap dependencies are ordinary `dependencies`, so consumers install them
  even when they never import the new entries.
- **Core root exports (91 new names).** CodeEditor types/constants and helpers,
  rich-text constants, labels, toolbar projection/validation and node/byte
  guards, `sanitizeMarkdownHtml`, `decodeHtmlEntities`, `MarkdownHtmlPolicy`,
  `sliderFamily*` helpers and `SLIDER_FAMILY_*` constants, `SliderVariant`,
  `SliderPolarity`, `TabsPin`, `isValidTabsPinnedOrder`, `isTabsReorderAllowed`,
  `getInputModality`, `installInputModality`, `INPUT_MODALITY_ATTR`,
  `installCodeEditorFocusEntry`, and six new icon registry identifiers
  (`between-horizontal-start`, `between-vertical-start`, `list-ordered`,
  `square-code`, `strikethrough`, `table`).
- **Tokens and styles.** Ten `--poodle-color-syntax-*` custom properties in
  `poodle-tokens.css` and every theme artifact, the
  `--poodle-recipe-tabs-card-item-fill` recipe hook, new
  `--poodle-md-renderer-*`, `--poodle-rich-text-*` and `--poodle-slider-family-*`
  variables, and the `code-editor.css`, `rich-text.css` and `slider-family.css`
  stylesheet modules behind the existing `./styles/*` entry.

### Changed

- **Tabs card items are surface-filled.** Every card item carries
  `background-surface`; inactive cards are filled without a border, and
  `activeFill="none"` keeps the variant's idle surface. Tabs reordering now
  partitions `pinned` items: pinned items are never reorder sources or targets,
  an unpinned item cannot cross a pinned partition, and `items` must already be
  partition-ordered.
- **Slider/RangeSlider semantics.** Exact or out-of-range bound input resolves
  directly to `min`/`safeMax`, default visible values round and zero-fill to the
  precision implied by `min` and a finite positive `step`, and negative zero is
  normalized. One fixed whole-track block layout serves both selected and
  remainder layers; the optional label yields under narrow fit while the value
  never moves or disappears.
- **Select navigation.** A non-searchable, non-freeform Select no longer filters
  its option list by the committed label, so keyboard navigation reaches every
  option.
- **Focus chrome modality.** Select, TokenInput, NumberInput, DurationInput,
  FilterBuilder, OrderBy, MenuSurface and AgentChatInput install the
  input-modality observer, so `:focus-within` chrome is gated by
  `:root[data-poodle-input-modality="keyboard"]`.
- **Release status.** This `0.4.0` candidate is prepared in the repository and
  is not yet tagged or published; `0.4.0` adds all public-intent work accumulated
  after the published `v0.3.0` release, including the final Slider-family
  consolidation and dual CodeEditor syntax palettes.
- Core, Svelte, the private React validation package, the Rust source/tag
  distribution set, and the private root repository manifest are versioned at
  `0.4.0`. Core and Svelte are the only eventual npm publication set; React
  stays private and Rust stays source/tag distribution. Internal preview/tooling
  packages are not consumer dependencies.
- **Downstream checks.** Remove `appearance`, `variant="standard"`, the removed
  fallback-text helpers and `formatVisibleRange`; rename consumer selectors from
  `data-appearance` to `data-variant`; migrate removed RangeSlider recipe hooks
  to their family successors; opt into `htmlPolicy="trusted"` where raw
  markdown HTML is intentional; and update Rust matches/struct literals for
  `SliderVariant`, `TabPin`, `TabsItem.pinned`, `NodeRole` and
  `NodeA11y.initial_focus`. See the
  [full 0.4.0 release notes](docs/release-notes/0.4.0.md) for the complete
  breaking table and migration text.

## [0.3.0] - 2026-09-05

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

### Added

- The public-intent delta accumulated after immutable `v0.2.2` includes the
  triggerless `ContextMenu` composition prepared as `0.2.3`, cross-runtime
  drag-and-drop and opaque file/cross-window bridges, Tree reorder authority,
  TimeInput and NumberInput value/draft contracts, continuous audio controls,
  shared motion policy, EditableLabel, block Slider/RangeSlider appearance,
  and same-id Toast updates.

### Changed

- **Release status.** Published from the certified `v0.3.0` tag on
  2026-09-05.
- Core, Svelte, the private React validation package, and the Rust source/tag
  distribution set are versioned at `0.3.0`. Core and Svelte are the only
  eventual npm publication set; React remains private and Rust remains
  source/tag distribution. Internal preview/tooling packages are not consumer
  dependencies.
- The web package boundary is compiled `dist/` JavaScript plus declarations.
  The candidate consumes the accepted installed certification and records its
  own exact tarball identities separately.
- **Downstream checks.** Migrate root markdown imports to `/markdown`, and
  update HistoryCenter callers from the v2 `branches` / `paths` /
  `branchCount` vocabulary to v3 `pages`, `continuationCount`, and
  host-owned continuation operations.
- **Downstream checks.** Pin a separately published exact version after
  certification. `0.2.3` was prepared but unpublished, and `0.2.4` is
  skipped.

## [0.2.3] - 2026-08-30

### Changed

- **Release status.** `0.2.3` was prepared in the repository but was never
  tagged or published. Its intended delta is carried into the `0.3.0`
  candidate above; `0.2.4` is skipped.

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
- The web packages carry no code change in this patch.
  `@inflatable-cookie/poodle-core` and `@inflatable-cookie/poodle-svelte`
  move to `0.2.2` so the ecosystem version set stays aligned across one tag.
  See the [0.2.2 release notes](docs/release-notes/0.2.2.md).

## [0.2.1] - 2026-08-23

### Fixed

- Release automation installs the pinned npm trusted-publishing CLI into an
  isolated runner prefix. The `0.2.0` workflow attempted to replace its own
  global npm installation in place and failed before validation or
  publication when the running CLI could no longer load `promise-retry`.
  Version `0.2.1` carries the same product changes prepared for `0.2.0`.

## [0.2.0] - 2026-08-23

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

- **Release posture.** The tagged candidate prepared against a frozen,
  measured denominator: 175 public Svelte components, each with a contract,
  implementation, export, specimen, focused tests, and packed-tarball import
  proof. Its release workflow failed before validation or npm publication;
  `0.2.1` is the replacement release. See the
  [full 0.2.0 release notes](docs/release-notes/0.2.0.md) for the
  publication set, per-runtime evidence posture, the visual-comparison
  boundary, and the complete migration checklist.
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
[0.4.14]: docs/release-notes/0.4.14.md
[0.4.13]: docs/release-notes/0.4.13.md
[0.4.12]: docs/release-notes/0.4.12.md
[0.4.11]: docs/release-notes/0.4.11.md
[0.4.10]: docs/release-notes/0.4.10.md
[0.4.9]: docs/release-notes/0.4.9.md
[0.4.8]: docs/release-notes/0.4.8.md
[0.4.7]: docs/release-notes/0.4.7.md
[0.4.6]: docs/release-notes/0.4.6.md
[0.4.5]: docs/release-notes/0.4.5.md
[0.4.4]: docs/release-notes/0.4.4.md
[0.4.3]: docs/release-notes/0.4.3.md
[0.4.2]: docs/release-notes/0.4.2.md
[0.4.1]: docs/release-notes/0.4.1.md
[0.4.0]: docs/release-notes/0.4.0.md
[0.3.0]: docs/release-notes/0.3.0.md
[0.2.3]: docs/release-notes/0.2.3.md
[0.2.2]: docs/release-notes/0.2.2.md
[0.2.1]: docs/release-notes/0.2.1.md
[0.2.0]: docs/release-notes/0.2.0.md
[0.1.0]: docs/release-notes/0.1.0.md
