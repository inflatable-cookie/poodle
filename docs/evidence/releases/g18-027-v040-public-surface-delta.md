# g18.027 — `v0.4.0` public-surface freeze audit

Status: frozen evidence — accepted delta from immutable `v0.3.0` to
post-g18.026 main
Date: 2026-09-12
Owner: Poodle release operations
Card: [`docs/roadmaps/g18/027-v040-public-surface-freeze-audit.md`](../../roadmaps/g18/027-v040-public-surface-freeze-audit.md)
Governing refs: [`docs/specs/022-packaging-versioning-and-release-channel-rules.md`](../../specs/022-packaging-versioning-and-release-channel-rules.md),
[`docs/specs/044-deprecation-change-control-and-release-channel-operations.md`](../../specs/044-deprecation-change-control-and-release-channel-operations.md),
[`docs/specs/070-compiled-web-distribution-contract.md`](../../specs/070-compiled-web-distribution-contract.md),
[`packages/release-manifest.json`](../../../packages/release-manifest.json)

## Purpose

This report is the `0.4.0` freeze gate. It binds every public delta between the
immutable `v0.3.0` release and the accepted post-g18.026 main head, classifies
each row, and states the consumer migration. g18.006 release notes are written
from this table. No later public break may enter the `0.4.0` candidate; any
further desired break targets `0.5.0`.

The audit does not repair product code, bump versions, or touch the candidate.

## Identities

| Identity | Value |
| --- | --- |
| Baseline tag | `v0.3.0` |
| Baseline commit | `85609d941a208ff2f854e9f7c0e457089cc77d0e` |
| Baseline tree | `d7a4a6533114ca8cd4864a5d62dc5c56a414917f` |
| Baseline commit date | 2026-09-05T00:15:48+01:00 |
| Final comparison ref | `main` |
| Final commit (`HEAD` at audit) | `31d529a6f0dd4dc632b11010cf1e16a29e6eb80b` |
| Final tree | `270547d0f033d882c7b736be8d24c68cbc5a9bce` |
| Final commit date | 2026-09-12T16:42:25+01:00 |
| g18.026 merge ancestor | `bfc0783b2020f6be7158e7f1a3cd74c4ed314be4` (PR #258) |

Package tree hashes (`git rev-parse <ref>:<path>`):

| Path (tree) | `v0.3.0` | final (`31d529a6f`) |
| --- | --- | --- |
| `packages/core` | `cd867193591218ca70769e966de92de2e55bb6e1` | `9408eb24624f0bfbe77d1f6c3dc8d8d6b9b2e16a` |
| `packages/svelte/components` | `a2265d7ecd6cde6a00404a45ef8f3e319f4367d5` | `abde297899a17663e2419f73afec15c710b4c1d9` |
| `packages/react/components` | `9df34f5fc794a3b461b791a96a739fa8295d2a88` | `dbe84a3ab40db616b09a9549140c6d3ac5739597` |
| `packages/tokens` | `c260c398ab6c95155ebff549f402196212763250` | `742ac9c846fbab1e7e3c96bff973bf9ec5b1aa49` |
| `packages/contracts` | `e826f00af0f15926e5a5e59c73bf70d7caa70102` | `be19fa74031e58b8c660dc1feac97a39c1ff4a91` |
| `packages/render` | `6c7da87f848e008807477c3e747f04b360cd5e6a` | `8f58b34f5a89dbd925ac884eb5e506dfd48b13af` |
| `docs/contracts/components` | `14e8e8d9e00840736a4455cb64974e52682122cd` | `68a7e0774bb983b905c4d7e055d000bb0388762c` |
| `packages/release-manifest.json` | `ec0148b2b350aa4f8e85d55267ab08c3b365e596` | `ec0148b2b350aa4f8e85d55267ab08c3b365e596` (unchanged) |
| `packages/release-operations.json` | `60bc2df381fbc47cd0d99acf1caf73266e5ea6eb` | `60bc2df381fbc47cd0d99acf1caf73266e5ea6eb` (unchanged) |

The release manifest and operations files are byte-identical across the
comparison, so the audited scope, channels, and change-control rules did not
move. Both public web packages still declare `version: 0.3.0` at the audited
head: the `0.4.0` bump belongs to g18.006.

## Reproduction

The delta is fully reproducible from a clean clone without building either
tree:

```sh
effigy audit:public-surface
bun scripts/audit-public-surface-delta.ts --base v0.3.0 --head 31d529a6f0dd4dc632b11010cf1e16a29e6eb80b
bun scripts/audit-public-surface-delta.ts --base v0.3.0 --head 31d529a6f0dd4dc632b11010cf1e16a29e6eb80b --json > /tmp/g18-027.json
bun test scripts/audit-public-surface-delta.test.ts
```

The script reads both refs through `git show`/`git ls-tree` and compares:
package entries, dependency and peer constraints, root and subpath exports
(TypeScript + Svelte + packed React), Svelte component props/defaults, packed
React prop types, the icon registry, token CSS custom properties, public recipe
hooks, and Rust `pub` declarations in the manifest's public-intent crates. The
test plants removed export, removed package entry, changed prop default,
removed recipe hook, moved internal variable, removed/shifted Rust item, and
added-file cases, plus the type/alias false-positive.

Summary of the frozen run: **433 additive, 65 breaking, 12 internal-only, 0
docs-only rows** (510 total). Every breaking row is enumerated below with a
migration; every additive family is listed explicitly. `--json` emits the raw
row set for g18.006 to diff its own re-run against.

## Publication scope used for classification

| Package | Channel (`packages/release-manifest.json`) | `0.4.0` distribution |
| --- | --- | --- |
| `@inflatable-cookie/poodle-core` | public, preview | published |
| `@inflatable-cookie/poodle-svelte` | public, preview | published |
| `@inflatable-cookie/poodle-react` | public intent, preview | **not published**; private packed parity proof |
| `@inflatable-cookie/poodle-tokens` | internal, source-of-truth | shipped through core token artifacts |
| `poodle-*` Rust crates | public intent, preview | source/tag distribution |
| preview / install-smoke / codegen packages | internal tooling | not distributed |

React rows are recorded because the manifest gives React public intent and
g18.006 keeps it as packed parity evidence, but they are not npm-visible in
`0.4.0`. Rust rows matter for source/tag consumers.

## Breaking rows (complete)

### 1. Slider and RangeSlider presentation API replacement (g18.022 + g18.026)

The single largest break. `appearance` is gone; `variant` changed value set and
default; the type and helper vocabulary was replaced; the shared family renderer
consolidated the recipe hooks.

| Surface | Removed / changed | Migration |
| --- | --- | --- |
| Svelte prop `Slider.appearance` | `"track" \| "block"`, default `"track"` | Remove the prop. `variant="embedded"` is the dense track control; `variant="block"` (now the default) is the capsule. |
| Svelte prop `Slider.variant` | `"standard" \| "embedded"`, default `"standard"` → `"block" \| "embedded"`, default `"block"` | `"standard"` has no alias: pass `variant="embedded"` for the old dense control, or omit for the new block default. |
| Svelte prop `RangeSlider.appearance` | `"track" \| "block"`, default `"track"` | Same as Slider. |
| Svelte prop `RangeSlider.variant` | `"standard" \| "embedded"`, default `"standard"` → `"block" \| "embedded"`, default `"block"` | Same as Slider. |
| Svelte prop `RangeSlider.formatVisibleRange` | removed | Removed with the combined range string. Block always paints lower/upper endpoint values at their scale anchors; use `formatVisibleValue` per thumb for custom text. |
| Packed React prop `Slider.appearance`, `RangeSlider.appearance` | removed | Mirror the Svelte migration. |
| Packed React prop `RangeSlider.formatVisibleRange` | removed | Mirror the Svelte migration. |
| Core/Svelte type `SliderAppearance` (`./types` too) | `"track" \| "block"` | Replaced by `SliderVariant`; `SliderPolarity` is now exported too. |
| Core helper `assertHorizontalBlockAppearance` | removed | Vertical block is now valid in every runtime; drop the guard. |
| Core helpers `sliderFallbackText`, `rangeSliderFallbackText`, `defaultVisibleRangeText`, `resolveRangeVisibleRange` | removed | The external fallback line and combined range text are gone. Block text stays inside the capsule and the optional label yields under narrow fit. |
| Core type `TabsState` | removed | Replaced by the narrower `TabsPin`. |
| Core type `TabsTooltipEffect` | removed | The tooltip sub-machine stays adapter-side; no public effect type is exported. |
| Slider `data-appearance="block"` root attribute | renamed to `data-variant="block"` | Update consumer CSS selectors; `data-variant` now carries `block`/`embedded`. |

Migration statement for release notes: *Slider and RangeSlider present one
control. `variant="block"` is the default capsule; `variant="embedded"` is the
dense track-and-thumb alternative. The pre-0.4 `appearance` prop,
`variant="standard"`, the fallback-text helpers, and the combined
`formatVisibleRange` string are removed with no aliases. Vertical block
presentation is supported in every runtime.*

### 2. Markdown safe-HTML default (g18.019)

| Surface | Change | Migration |
| --- | --- | --- |
| Svelte `MarkdownEditor.htmlPolicy` (new) | Default `"safe"` | Editor preview and `MarkdownRenderer` now sanitize the complete HTML result of the built-in `marked` path **and** of a custom `renderHtml` closure. |
| Svelte/React `MarkdownEditor.renderHtml` behavior | A custom renderer no longer bypasses sanitization | Pass `htmlPolicy="trusted"` for fully trusted content; there is no compatibility fallback. |
| `MarkdownEditor.renderHtml` prop type | Inline `((markdown: string) => string) \| null` → named `MarkdownHtmlRenderer` with the identical signature | **No migration.** The classifier flags the signature text change; the alias resolves to the same type. |

Migration statement for release notes: *Markdown preview is safe by default.
Consumers that relied on raw Markdown HTML must opt into
`htmlPolicy="trusted"`; supplying `renderHtml` no longer implies trust.*

### 3. Removed public recipe hooks (Slider family consolidation)

Eighteen documented recipe hooks are no longer read. The Slider and RangeSlider
implementation now shares one family foundation that reads the
`--poodle-recipe-slider-*` names for both components. Overrides on the removed
RangeSlider names silently stop applying.

Removed Slider hooks (4):

| Removed hook | Successor |
| --- | --- |
| `--poodle-recipe-slider-block-fallback-text` | none; no external fallback line exists |
| `--poodle-recipe-slider-control-thumb-fill` | `--poodle-recipe-slider-control-fill` |
| `--poodle-recipe-slider-control-shadow` | none |
| `--poodle-recipe-slider-focus-control-shadow` | `--poodle-recipe-slider-focus-ring` |

Removed RangeSlider hooks (14):

| Removed hook | Successor |
| --- | --- |
| `--poodle-recipe-range-slider-track-fill` | `--poodle-recipe-slider-track-fill` |
| `--poodle-recipe-range-slider-track-border` | `--poodle-recipe-slider-track-border` |
| `--poodle-recipe-range-slider-center-fill` | `--poodle-recipe-slider-center-fill` |
| `--poodle-recipe-range-slider-control-fill` | `--poodle-recipe-slider-control-fill` |
| `--poodle-recipe-range-slider-control-thumb-fill` | `--poodle-recipe-slider-control-fill` |
| `--poodle-recipe-range-slider-control-track-fill` | none |
| `--poodle-recipe-range-slider-control-thumb-shadow` | none |
| `--poodle-recipe-range-slider-focus-control-thumb-shadow` | `--poodle-recipe-slider-focus-ring` |
| `--poodle-recipe-range-slider-block-remainder-fill` | `--poodle-recipe-slider-block-remainder-fill` |
| `--poodle-recipe-range-slider-block-remainder-text` | `--poodle-recipe-slider-block-remainder-text` |
| `--poodle-recipe-range-slider-block-handle-fill` | `--poodle-recipe-slider-block-handle-fill` |
| `--poodle-recipe-range-slider-block-handle-border` | `--poodle-recipe-slider-block-handle-border` |
| `--poodle-recipe-range-slider-block-focus-ring` | `--poodle-recipe-slider-block-focus-ring` |
| `--poodle-recipe-range-slider-block-fallback-text` | none; no external fallback line exists |

Retained hooks (19): all 14 remaining `--poodle-recipe-slider-*` hooks, plus
`--poodle-recipe-range-slider-block-selected-fill`,
`-block-selected-text`, `-fill-fill`, `-fill-negative` and `-focus-ring`.

New hook: `--poodle-recipe-tabs-card-item-fill`.

> **Decision capsule — operator input required.** `docs/contracts/components/slider.md`
> and `range-slider.md` still list the removed hook names under "Recipe hooks",
> and the RangeSlider contract lists hooks the shared renderer no longer reads.
> The consolidation was not recorded in g18.026. This is a documentation defect;
> the audit does not repair contracts (reserved surface). Recommend correcting
> both recipe-hook lists to the family names in the post-release docs sweep, and
> recording the removals in the `0.4.0` release notes from this table. No
> product source fix is required to release unless the operator wants the old
> RangeSlider hook names to keep working, which would be a pre-v1 compatibility
> shim and is disallowed by `AGENTS.md`.

### 4. Rust public API (source/tag consumers)

`poodle-specs` (`packages/contracts/components`):

| Removed / changed | Migration |
| --- | --- |
| `pub enum SliderAppearance` and its `pub use` re-export | Removed; use `SliderVariant`. |
| `SliderVariant::Standard` → new default `Block` | `SliderVariant::Standard` has no alias; use `Block` for the capsule or `Embedded` for the dense control. |
| `SliderSpec.appearance` field, `SliderSpec::with_appearance` | Removed; set `variant`. |
| `pub fn reject_vertical_block` | Removed; vertical block is valid. |
| `RangeSliderSpec.appearance`, `RangeSliderSpec.visible_range_text` fields | Removed; use `variant` and per-thumb visible text. |
| `RangeSliderSpec::with_appearance` | Renamed to `with_variant`. |
| `RangeSliderSpec::with_visible_range_text` | Removed. |
| `TabDefinition.pinned` field + `with_pinned`, new `TabPin` enum | Additive in semantics, source-breaking for exhaustive struct literals; use the builder or add `pinned: None`. |

`poodle-headless` (`packages/contracts/headless`):

| Removed / changed | Migration |
| --- | --- |
| `slider_fallback_text`, `range_slider_fallback_text`, `default_visible_range_text`, `resolved_range_text` | Removed; the external fallback and combined range text are gone. |
| `default_visible_value_text(value)` → `(value, min, step)` | Pass the range's `min` and `step`; output is now a step-aware fixed-width decimal. |
| `resolved_visible_text(value, explicit)` → `(value, min, step, explicit)` | Same. |
| `SliderBlockLayout { inline, fallback }` → `{ label_inline, value_inline }` | Drop `fallback`; keep the exact value and suppress the optional label. |
| `RangeSliderBlockLayout { inline, fallback, selected_text }` → `{ label_inline, lower_inline, upper_inline }` | Same. |
| `layout_slider_block` / `layout_range_slider_block` signatures | The `selected_norm` / `lower_norm` / `upper_norm` / `range_text` parameters are removed; fit is whole-track. |
| `TabsItem` gained `pinned: Option<TabPin>` | Source-breaking for struct literals; use `pinned: None` or the builder. |

`poodle-node` (`packages/contracts/node`):

| Removed / changed | Migration |
| --- | --- |
| `NodeRole` gained `Banner`, `Heading`, `SearchBox` | Exhaustive matches must add arms. |
| `NodeA11y.initial_focus: bool` | Source-breaking for struct literals; use `..Default::default()` or set `initial_focus: false`. |

`poodle-render`:

| Removed / changed | Migration |
| --- | --- |
| `slider_block::block_grab` | Removed; use `block_grab_with_axis`. |
| `slider_block::block_surface(hit_px)` | Parameter renamed to `cross_px` (same arity); re-verify call sites. |
| `slider_block::fraction_anchor(...)` | Gained a trailing `layer_offset` argument. |
| `pub use detail_item::…` | Grew by `detail_item_with_slots_state` (additive). |

### 5. Non-breaking removals recorded for completeness

Twelve undocumented implementation variables disappeared from the public
`./styles/*` stylesheet while remaining metric owners in the shared family:

| Removed variable | Successor |
| --- | --- |
| `--poodle-slider-track-thickness` | `--poodle-slider-family-track-thickness` |
| `--poodle-slider-thumb-size` | `--poodle-slider-family-thumb-size` |
| `--poodle-slider-control-min-height` | `--poodle-slider-family-control-min-height` |
| `--poodle-slider-block-hit` | `--poodle-slider-family-block-hit` |
| `--poodle-slider-block-min-height` | `--poodle-slider-family-block-height` (metric changed from min-height to the control-height ladder) |
| `--poodle-slider-block-thumb` | `--poodle-slider-family-marker-thickness` (the block handle is a bounded inset marker line) |
| `--poodle-range-slider-track-thickness`, `-thumb-size`, `-control-min-height`, `-block-hit`, `-block-min-height`, `-block-thumb` | Same family successors |

These were never documented recipe hooks and no contract advertised them as
public; they are classified `internal-only`.

## Compatible behavioral rows

Same public shape, different semantics. Contract-backed and non-breaking.

| Area | Change | Basis |
| --- | --- | --- |
| Tabs card inactive fill | Every card item now carries `background-surface`; inactive cards are surface-filled without a border. `activeFill="none"` keeps the variant's idle surface (card stays filled; pill/block stay unfilled). | `docs/contracts/components/tabs.md` |
| Tabs reorder | New `pinned` partition law: pinned items are never reorder sources/targets; an unpinned item cannot cross a pinned partition. `items` must already be partition-ordered. | Same contract |
| Slider/RangeSlider normalization | Exact or out-of-range bound input resolves directly to `min`/`safeMax`, so both endpoints stay reachable when `step` does not divide the range evenly. | `docs/contracts/components/slider.md`, `range-slider.md` |
| Slider/RangeSlider display text | Default visible value rounds and zero-fills to the precision implied by `min` and a finite positive `step`, normalizes negative zero, and never exposes binary tails. | Same contracts |
| Slider/RangeSlider block text | One fixed whole-track layout for both selected and remainder layers; the optional label yields under narrow fit, the value never moves or disappears. | Same contracts |
| Select navigation | A non-searchable, non-freeform Select no longer filters its option list by the committed label, so keyboard navigation reaches every option. | `packages/core/src/select.ts` |
| Focus chrome modality | `Select`, `TokenInput`, `NumberInput`, `DurationInput`, `FilterBuilder`, `OrderBy`, `MenuSurface` and `AgentChatInput` install the input-modality observer; `:focus-within` chrome is gated by `:root[data-poodle-input-modality="keyboard"]`. Adds the public-ish root attribute `data-poodle-input-modality`. | Component contracts + sources |

## Additive rows (known-safe)

### Package entries and dependencies

| Package | Added entries | Added runtime dependencies |
| --- | --- | --- |
| `@inflatable-cookie/poodle-svelte` | `./editor`, `./editor/codemirror`, `./rich-text` | 25 (`@codemirror/*`, `@lezer/highlight`, `@tiptap/*`) |
| `@inflatable-cookie/poodle-react` | `./editor`, `./editor/codemirror`, `./rich-text` | 25 (identical set) |

Existing entries (`.`, `./*.svelte`, `./markdown`, `./types`) and every
`peerDependencies` constraint are unchanged. The new CodeMirror/TipTap
dependencies are ordinary `dependencies`, so consumers install them even when
they never import `./editor` or `./rich-text`.

### New public web components (additive, non-native)

- `CodeEditor` — `./editor` entry; CodeMirror-backed, consumer-selected language
  registry. Not added to the 176-name native roster.
- `RichTextEditor`, `RichTextRenderer` — `./rich-text` entry; TipTap/ProseMirror,
  configurable heading select H1–H6.
- `MarkdownRenderer` — additive export inside the existing `./markdown` entry;
  web-only companion, no native counterpart and no parity credit.
- `MarkdownEditor.htmlPolicy`, plus 37 new Svelte props and 37 new packed React
  props across these components (full list in `--json`).

### Core root exports (91 new names)

- CodeEditor family: types + `CODE_EDITOR_*` constants,
  `createCodeEditorLanguageRegistry`, `isCodeEditorValueAdmissible`,
  `toCodeEditorChange`, `applyCodeEditorEdits`, `validateCodeEditorDiagnostics`,
  `codeEditorByteLength`.
- Rich-text family: `RICH_TEXT_*` constants/labels, `projectRichTextToolbar`,
  `resolveRichTextToolbar`, `validateRichTextFeatures`,
  `validateRichTextToolbar`, heading helpers, node/byte guards.
- Markdown: `sanitizeMarkdownHtml`, `decodeHtmlEntities`,
  `type MarkdownHtmlPolicy`.
- Slider family: `sliderFamily*` helpers and `SLIDER_FAMILY_*` constants,
  `type SliderFamilyOrientation`, `type SliderFamilyRect`; `type SliderPolarity`
  and `type SliderVariant` re-exported from the Svelte root.
- Tabs: `isValidTabsPinnedOrder`, `isTabsReorderAllowed`, `type TabsPin`.
- DOM: `getInputModality`, `installInputModality`, `INPUT_MODALITY_ATTR`,
  `installCodeEditorFocusEntry`, `CODE_EDITOR_FOCUS_ENTRY_ATTR`,
  `type InputModality`.
- Icons: 6 new registry identifiers — `between-horizontal-start`,
  `between-vertical-start`, `list-ordered`, `square-code`, `strikethrough`,
  `table`.

### Subpath exports

New `./editor`, `./editor/codemirror`, `./rich-text`, `./markdown` and
`./types` exports for both web packages; `SliderVariant`/`SliderPolarity` and
`TabItem.pinned` in `./types`; 35 React and 32 Svelte rich-text names. The
exception is `type SliderAppearance`, which is a removal (above).

### Tokens and CSS

- 10 new `--poodle-color-syntax-*` custom properties in `poodle-tokens.css` and
  every theme artifact (30 declarations in the base tokens file, 10 per theme).
- New public recipe hook `--poodle-recipe-tabs-card-item-fill`.
- 18 new `--poodle-md-renderer-*`, `--poodle-rich-text-*`,
  `--poodle-slider-family-*` and marker-position variables in
  `poodle-core/styles`.
- New public stylesheet modules under the existing `./styles/*` entry:
  `code-editor.css`, `rich-text.css`, `slider-family.css`.

### Rust additions

Rust additive rows (16): `use types` re-export growth in `poodle-specs`,
`SliderSpec::with_variant` and `RangeSliderSpec::with_variant` (replacing the
removed `with_appearance`), `TabPin` and `TabDefinition::with_pinned` in
`poodle-specs`; `slider_display_precision`, `TabPin`,
`is_valid_tabs_pinned_order` and `is_tabs_reorder_allowed` in `poodle-headless`;
and `use detail_item` growth (`detail_item_with_slots_state`),
`agent_question_prompt_id`, `block_grab_with_axis`, `block_hit_inset`,
`block_surface_vertical` and `fraction_anchor_vertical` in `poodle-render`.

## Freeze enforcement for g18.006

The accepted `0.4.0` candidate must:

1. Descend from `31d529a6f0dd4dc632b11010cf1e16a29e6eb80b` (this frozen head)
   or later main with no public-surface change beyond this report.
2. Re-run `effigy audit:public-surface` with `--head <candidate>` (or the
direct script command) and find **no breaking row absent from this report**.
Any new breaking row is a stop condition: either revert it or move it to
`0.5.0`.
3. Keep `packages/release-manifest.json` and `packages/release-operations.json`
   byte-identical to the hashes above unless the operator explicitly changes
   scope.
4. Consume the breaking tables above verbatim for release-note migration text,
   including the React-only rows as "packed React parity" notes and the Rust
   rows as source/tag notes.

## Non-mutation statement

This audit changed no version, lock (`bun.lock`, `Cargo.lock`), changelog,
release note, workflow, Desktop file, g18.006/g18.009 surface, or product/Rust
source. It adds one evidence report, one script, one script test, one execution
log, and the release-evidence index entry.

## Known audit limits

- Deliberately excluded: builds, packed tarball contents, runtime behavior
  measurements and visual comparison. Those are g18.006/g18.011 surfaces.
- Svelte component prop extraction reads the `interface Props` block and the
  `$props()` destructuring defaults; React extraction reads the declared
  `*Props` type. A prop whose type is computed indirectly may need manual review,
  which is why the raw `--json` output is the canonical row set.
- Behavioral rows are contract-derived narrative, not machine-diffed; the
  script cannot see semantics behind an unchanged signature.
