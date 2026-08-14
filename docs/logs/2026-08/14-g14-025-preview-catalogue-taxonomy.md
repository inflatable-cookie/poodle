# 14 — g14.025 Preview Catalogue Taxonomy and Generated Navigation

Branch: `t3code/preview-catalogue-handoff`
Date: 2026-08-14
Base SHA: `d3ad2229`
Card: `docs/roadmaps/g14/025-preview-catalogue-taxonomy-and-generated-navigation.md`

One renderer-neutral catalogue manifest, checked TypeScript/Rust generation, and
matching section/family navigation in the active Svelte, React, and GPUI
previews. Jetstream gets the generated module only. No component APIs, specimens,
routes, contracts, or conformance meaning changed.

## Before / after

Canonical count stayed **174**. Previous Svelte anatomy `tag` split:

| Tag         | n   |
| ----------- | --- |
| display     | 31  |
| input       | 28  |
| workstation | 21  |
| control     | 18  |
| data        | 18  |
| overlay     | 12  |
| feedback    | 12  |
| layout      | 11  |
| navigation  | 9   |
| form        | 8   |
| media       | 6   |

GPUI copied 155 entries and defaulted unknown slugs to Workstation. Jetstream
copied 151. Neither native copy is an authority anymore.

After: 3 sections, 16 families, 11 kinds, 0 collections.

| Section     | Families                                                                                                    | n   |
| ----------- | ----------------------------------------------------------------------------------------------------------- | --- |
| Foundations | actions-selection 12, text-value-entry 15, date-time 10, layout 11, content-identity 14, status-progress 14 | 76  |
| Composition | navigation 9, overlays-disclosure 14, forms-validation 9, data-collections 16, media 6                      | 54  |
| Systems     | application-shell 12, agent-tools 11, model-connections 5, audio-music 11, account-lifecycle 5              | 44  |

Kinds: display 30, input 28, composite 19, control 18, data 18, feedback 14,
overlay 13, layout 13, navigation 8, form 7, media 6.

## Generated outputs

Manifest: `packages/codegen/fixtures/preview-catalogue.json` (schema v1).

Selectors: `effigy catalogue:build` / `effigy catalogue:check`. Not composed
into `ir:build` / `ir:check`.

| Runtime   | Artifact                                                          |
| --------- | ----------------------------------------------------------------- |
| Svelte    | `packages/svelte/preview/src/generated/catalogue/catalogue.ts`    |
| React     | `packages/react/preview/src/generated/catalogue/catalogue.ts`     |
| GPUI      | `packages/gpui/preview/src/generated/catalogue/catalogue.rs`      |
| Jetstream | `packages/jetstream/preview/src/generated/catalogue/catalogue.rs` |

Package names and `hasSpecimen` stay in runtime overlays. Unknown slugs return
`None` — no Workstation fallback.

## Ambiguity register

Fixed placements from the card, all asserted in tests:

- every `Agent*` / `ToolCall*` / `ChangedFiles` → agent-tools
- `ModelPicker`, `ModelCatalogueEditor`, every `ModelConnection*` → model-connections
- audio controls/displays listed on the card → audio-music
- every `Licence*` / `Update*` → account-lifecycle
- app chrome, detail/settings shells, `MessageCenter`, `HistoryCenter` → application-shell

Judgement calls (still one explicit family, no fallback bucket):

| Component                                                                  | Family              | Why                                                          |
| -------------------------------------------------------------------------- | ------------------- | ------------------------------------------------------------ |
| ConfirmAction                                                              | actions-selection   | Adopters look with Button/IconButton; kind is overlay        |
| ThemeSelect, RefSelect                                                     | text-value-entry    | Value choosers, not chrome                                   |
| Card                                                                       | content-identity    | Identity/surface, not a collection widget                    |
| BulkActionBar                                                              | data-collections    | Lives on selected rows                                       |
| Accordion, Collapsible, DebugDialog                                        | overlays-disclosure | Disclosure/overlay anatomy                                   |
| CommandPalette                                                             | overlays-disclosure | Overlay search; ActionDiscoveryPanel stays application-shell |
| FormDialog                                                                 | overlays-disclosure | Dialog chrome wrapping a form                                |
| TimeAgo                                                                    | date-time           | Temporal display, not status                                 |
| CollapseToggle                                                             | actions-selection   | Control, not the disclosure region                           |
| Icon, IconProvider, UiPresentationProvider                                 | content-identity    | Identity/presentation, not layout                            |
| EmptyState, ErrorBoundary, RemediationBanner, StateTile, Meter, MetricTile | status-progress     | Status/progress, not audio (Meter ≠ AudioMeter)              |
| Rating, Slider, RangeSlider, DragNumberField                               | text-value-entry    | Value entry                                                  |
| PickerShell, RelationPicker                                                | data-collections    | Collection pickers                                           |
| CardRadioGroup, CardToggleGroup                                            | data-collections    | Option collections                                           |
| FilterBuilder, FilterToolbar, OrderBy                                      | data-collections    | Collection query chrome                                      |
| BlockEditor, MarkdownEditor                                                | forms-validation    | Form editors                                                 |
| AudioPlayer, VideoPlayer, Media*                                           | media               | Playback/browse, not the audio-music instrument suite        |
| NavCard                                                                    | navigation          | Navigation affordance                                        |
| ValueReadout                                                               | audio-music         | Card-fixed audio display suite                               |

Not classified (stale GPUI-only slugs, never promoted): `form-shell`,
`tab-strip`, `inline-remediation`. They now miss instead of mapping to
Workstation.

React overlay still drops `AgentPlan` / `AgentPlanRecord` and marks embedded-only
specimens. `ErrorBoundary` now keeps one renderer-neutral canonical description. Jetstream
`has_specimen` stays local (false for `radio`, `time-input`, `status-bar`, plus
the 23 slugs that were never in the old 151-entry copy).

`packages/svelte/preview/src/catalog.ts` documentation/adoption taxonomy is
untouched.

## Active-runtime evidence

Svelte and React share `catalogue-nav.ts` and `catalogue.css`. Family headings
use Icon chevrons, not public `Collapsible` (nested-button + happy-dom `animate`
papercut). Search stays in the existing shell field.

Headless tests: codegen catalogue 10, Svelte audit+nav 10, React nav 3, GPUI
catalogue 7.

Screenshots (headless Chromium, this worktree's Vite on :4273 / :4280; did not
reuse occupied 4174/4180 from other checkouts):

- `docs/logs/2026-08/assets/g14-025/svelte-landing.png`
- `docs/logs/2026-08/assets/g14-025/svelte-agent.png`
- `docs/logs/2026-08/assets/g14-025/svelte-model.png`
- `docs/logs/2026-08/assets/g14-025/svelte-search.png`
- `docs/logs/2026-08/assets/g14-025/react-landing.png`
- `docs/logs/2026-08/assets/g14-025/react-agent.png`
- `docs/logs/2026-08/assets/g14-025/react-model.png`
- `docs/logs/2026-08/assets/g14-025/react-search.png`

Landing: three sections, families collapsed in the sidebar, all family cards
expanded in the content region, 174 count. Agent route auto-opens Agent & tools
(Svelte 11, React 9). Model route auto-opens Model connections (5). Search
`model connection` replaces the hierarchy with `Family · Kind` results.

GPUI: same grouping, counts, auto-disclose, search fields/order, and
`#components/<slug>` identity via generated Rust. Headless tests only; native
preview not launched.

## Deferred Jetstream

Generated catalogue module matches the manifest. Overlay preserves local
`has_specimen`. Shell still consumes `tag_label()` (now the family label) with
no collapse/search rebuild. No paired Jetstream QA. Runtime admission for
navigation UX remains open.

## Validation

| Selector                                  | Result                                            |
| ----------------------------------------- | ------------------------------------------------- |
| focused codegen catalogue tests           | pass (10)                                         |
| focused Svelte/React/GPUI catalogue tests | pass (20)                                         |
| `effigy catalogue:check`                  | pass                                              |
| `effigy check:svelte`                     | pass (0 errors; 4 existing warnings)              |
| `effigy react:build`                      | pass                                              |
| `effigy check:gpui`                       | pass (existing dead-code warnings in node_compat) |
| `effigy test:components`                  | pass (103 files, 1323 tests)                      |
| `effigy test:parity`                      | pass (174)                                        |
| `effigy docs:check`                       | pass                                              |
| `git diff --check`                        | clean                                             |

Not run, per card: `qa`, `qa:jetstream`, `ci:conformance-windowed`,
`test:native-visual`, `gpui:run`, `jetstream:run`.

## Residual risk

- Empty collection vocabulary emits `CatalogueCollectionId = never`; nav uses a
  string-keyed lookup until collections exist.
- React shares Svelte nav/CSS via relative paths (`src/gallery` is one extra
  `../` vs `src/main.tsx`).
- Jetstream metadata cannot drift, but its sidebar UX still will until a later
  shell card.
- Secondary collections are empty on purpose.

Roadmap/dispatch status not edited. Stop here.

## Orchestrator review corrections

- Explicit disclosure state now overrides direct-route auto-disclosure, so an
  active family can be collapsed and reopened in Svelte, React, and GPUI. A
  later route change clears the destination family's override and restores
  active context.
- Catalogue-size tests derive their expectation from generated data instead of
  pinning 174, preserving the one-manifest-entry addition path.
- Removed the React-only `ErrorBoundary` description override; canonical
  catalogue identity stays renderer-neutral.
