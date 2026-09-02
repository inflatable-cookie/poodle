# 070 Compiled Web Distribution Contract

Status: active
Updated: 2026-09-02
Depends on: `../architecture/014-compiled-web-package-distribution.md`,
`022-packaging-versioning-and-release-channel-rules.md`,
`../architecture/002-token-system-and-package-layout.md`,
`../contracts/001-working-rules.md`

## Purpose

Freeze the exact compiled web package inventories, export maps, naming laws,
receipt schemas, forbidden archive content, markdown migration, and successor
oracles. Architecture 014 is the decision. This spec is the reviewable
denominator later build cards must implement without choosing package
semantics.

Current manifests still ship `src/` and older floors. That is evidence of
present drift, not permission to keep it. Do not repair manifests, builds, or
release metadata here.

## Package ownership

| Package | Role | Publication |
| --- | --- | --- |
| `@inflatable-cookie/poodle-core` | public runtime | preview-channel public release set |
| `@inflatable-cookie/poodle-svelte` | public runtime | preview-channel public release set |
| `@inflatable-cookie/poodle-react` | private runtime | compiled and certified; unpublished until a named consumer is admitted |

`packages/release-manifest.json` still lists React as preview `publicIntent`.
That classification mutation is release-candidate work (`g16.054`), not this
contract. React `publicIntent: true` is not publication authority.

## Files, side effects, declarations

```json
{
  "coreFiles": ["dist", "README.md", "LICENSE", "THIRD_PARTY_NOTICES.md"],
  "shellFiles": ["dist", "README.md", "LICENSE"],
  "sideEffects": ["**/*.css"]
}
```

All three packages use `sideEffects: ["**/*.css"]`. Permitted declaration
suffixes are `.d.ts`, `.d.mts`, and `.d.cts`. `declarationMap` is false.
Declarations must resolve under TypeScript `Bundler` and `NodeNext`.

`bin.poodle-icons` targets the compiled core CLI, not source:

```text
"poodle-icons" -> ./dist/icons/build.mjs
```

`./icons/build` keeps runtime plus declarations (`build.mjs` + `build.d.mts`).

## Dependency ownership

| Package | Externalized | Peers | Parser |
| --- | --- | --- | --- |
| core | none of svelte/react/marked | none | no `marked` edge |
| Svelte | `svelte`, core, `marked` | `svelte: >=5.56.8 <6`; optional `marked: ^18.0.9` | only `./markdown` |
| React | `react`, `react-dom`, core, `marked` | `react`/`react-dom` as today; optional `marked: ^18.0.9` | only `./markdown` |

Optional `marked` is required when a consumer imports `./markdown`. Ordinary
root or direct Button/Select graphs must not resolve `marked`. Lowering the
Svelte floor needs a separately proven older compiler/runtime build.

## Stable names

Public filenames are unhashed. Vite `[hash]` in a public file or CSS/token
basename fails. Shared code goes under `chunks/` with `[name]` only.

| Package | Public JS | Shared chunks | CSS/assets |
| --- | --- | --- | --- |
| core | `dist/<entry>.js` matching the export | `dist/chunks/<name>.js` | exact current basenames under `dist/styles/` and `dist/tokens/generated/css/` |
| Svelte | `<Name>.client.js` / `<Name>.server.js` at `dist/` root for public entries | `dist/chunks/<name>.client.js` and `dist/chunks/<name>.server.js` | none owned; import core style subpaths |
| React | `dist/<Name>.js` for public entries | `dist/chunks/<name>.js` | none owned; import core style subpaths |

Svelte internals that are not public entries, including `DragDropProvider.svelte`
and `MenuSurface.svelte`, compile only as chunks. They must not appear as
`dist/DragDropProvider.client.js` or any other public basename. Today's source
wildcard can resolve those two files; the compiled contract closes that leak.

## Canonical component denominator

The public component denominator is **176** names. One derived list must agree
across this spec, `docs/roadmaps/g15/release-baseline-roster.md`, and
`test/package-install/roster.ts`. A 175/176 disagreement is a blocking defect.

Markdown members stay in the 176. They leave shell **root barrels**. They
remain reachable through `./markdown` and through the direct subpaths below.

Sorted names:

```text
Accordion
ActionDiscoveryPanel
AgentChatInput
AgentMessage
AgentPlan
AgentPlanRecord
AgentQuestion
AgentQuestionRecord
AgentSubagent
AgentTranscript
AlertDialog
AppHeader
AudioMeter
AudioPlayer
AudioSwitch
Avatar
BlockEditor
Box
Breadcrumbs
BulkActionBar
Button
Calendar
Callout
Card
CardRadioGroup
CardToggleGroup
ChangedFiles
Checkbox
Code
CodeInput
CollapseToggle
Collapsible
ColorPicker
CommandPalette
ConfirmAction
ContextMenu
DataTable
DatePicker
DateRangePicker
DateTimePicker
DateTimeRangePicker
DateTimeZonePicker
DebugDialog
DetailItem
DetailSection
DetailSectionGroup
DetailShell
Dialog
DockRegion
DragNumberField
Drawer
DurationInput
EditableLabel
EditableList
EmbedInput
EmbedPreview
EmptyState
EnvelopeEditor
ErrorBoundary
Eyebrow
Fader
Field
FieldSet
FileUpload
FilterBuilder
FilterToolbar
FormActions
FormDialog
FormLayout
GainReductionMeter
Grid
HistoryCenter
HoverCard
Icon
IconButton
IconProvider
InlineListSection
Keyboard
Knob
LicenceActivation
LicenceSeats
LicenceStatus
ListCard
ListCardCounter
ListContainer
ListGrid
LogList
MarkdownEditor
MediaBrowsePanel
MediaPicker
MediaPreview
MediaThumbnail
Menu
Menubar
MessageCenter
MetaBar
MetaItem
Meter
MeterSurface
MetricTile
ModMatrixGrid
ModelCatalogueEditor
ModelConnectionCard
ModelConnectionPicker
ModelConnectionSetup
ModelPicker
MotionPolicyProvider
NavCard
NavigationMenu
NumberInput
OrderBy
PageHeader
PageLoading
Pagination
PaginationSummary
PasswordRequirements
PickerShell
Pill
Popover
Progress
Radio
RadioGroup
RangeSlider
Rating
RefSelect
Region
RelationPicker
RemediationBanner
ResizeHandle
ScrollShell
SegmentedControl
Select
SelectionSummary
Separator
SettingsShell
SidebarNav
Skeleton
Slider
Spacer
Spinner
SplitButton
SplitView
Stack
StateTile
StatusBar
StatusIndicator
Stepper
Surface
Switch
Table
Tabs
Text
TextInput
TextLink
ThemeSelect
TimeAgo
TimeInput
TimeZoneSelect
ToastHost
ToastStack
ToggleGroup
TokenInput
ToolCall
ToolCallGroup
Toolbar
Tooltip
Tree
TriStateSwitch
UiPresentationProvider
UpdateCenter
UpdateStatus
ValidationSummary
ValueReadout
VideoPlayer
WaveformDisplay
XYPad
```

## Core export map

Core is one JavaScript lane. Every JS export uses `types`, then `import`, then
`default`. `import` and `default` target the same compiled JavaScript. That
`import` is a module-format fallback, not an environment selector. There is no
`svelte` or `browser` condition. No target contains `src`, a source alias, or a
`main` fallback.

Svelte still forbids `import`. Core must not copy that prohibition.

Conditionless string targets are CSS-only.

```json
{
  ".": {
    "types": "./dist/index.d.ts",
    "import": "./dist/index.js",
    "default": "./dist/index.js"
  },
  "./icons": {
    "types": "./dist/icons/index.d.ts",
    "import": "./dist/icons/index.js",
    "default": "./dist/icons/index.js"
  },
  "./icons/build": {
    "types": "./dist/icons/build.d.mts",
    "import": "./dist/icons/build.mjs",
    "default": "./dist/icons/build.mjs"
  },
  "./icons/*": {
    "types": "./dist/icons/icons/*.d.ts",
    "import": "./dist/icons/icons/*.js",
    "default": "./dist/icons/icons/*.js"
  },
  "./tokens": {
    "types": "./dist/tokens/index.d.ts",
    "import": "./dist/tokens/index.js",
    "default": "./dist/tokens/index.js"
  },
  "./tokens/runtime": {
    "types": "./dist/tokens/runtime.d.ts",
    "import": "./dist/tokens/runtime.js",
    "default": "./dist/tokens/runtime.js"
  },
  "./tokens/css": {
    "types": "./dist/tokens/css.d.ts",
    "import": "./dist/tokens/css.js",
    "default": "./dist/tokens/css.js"
  },
  "./tokens/themes": {
    "types": "./dist/tokens/themes.d.ts",
    "import": "./dist/tokens/themes.js",
    "default": "./dist/tokens/themes.js"
  },
  "./tokens/metadata": {
    "types": "./dist/tokens/metadata.d.ts",
    "import": "./dist/tokens/metadata.js",
    "default": "./dist/tokens/metadata.js"
  },
  "./tokens/units": {
    "types": "./dist/tokens/units.d.ts",
    "import": "./dist/tokens/units.js",
    "default": "./dist/tokens/units.js"
  }
}
```

The same three-condition object applies to every core JS family in the table
below. CSS rows stay strings.

| Export | Target |
| --- | --- |
| `.` | `./dist/index.js` + `./dist/index.d.ts` |
| `./icons` | `./dist/icons/index.js` + `./dist/icons/index.d.ts` |
| `./icons/build` | `./dist/icons/build.mjs` + `./dist/icons/build.d.mts` |
| `./icons/*` | `./dist/icons/icons/*.js` + matching `.d.ts`; only the 108 icon modules below |
| `./tokens` | `./dist/tokens/index.js` + `./dist/tokens/index.d.ts` |
| `./tokens/runtime` | `./dist/tokens/runtime.js` + `.d.ts` |
| `./tokens/css` | `./dist/tokens/css.js` + `.d.ts` |
| `./tokens/themes` | `./dist/tokens/themes.js` + `.d.ts` |
| `./tokens/metadata` | `./dist/tokens/metadata.js` + `.d.ts` |
| `./tokens/units` | `./dist/tokens/units.js` + `.d.ts` |
| `./styles/*` | `./dist/styles/*` — CSS only; exactly the 167 files below |
| `./tokens/styles.css` | `./dist/tokens/generated/css/poodle-tokens.css` |
| `./tokens/css/poodle-tokens.css` | `./dist/tokens/generated/css/poodle-tokens.css` |
| `./tokens/themes.css` | `./dist/tokens/generated/css/poodle-themes.css` |
| `./tokens/theme-<name>.css` | `./dist/tokens/generated/css/poodle-theme-<name>.css` |
| `./tokens/density-<name>.css` | `./dist/tokens/generated/css/poodle-density-<name>.css` |
| `./tokens/control-size-<name>.css` | `./dist/tokens/generated/css/poodle-control-size-<name>.css` |

Theme names, sorted: `clay`, `cobalt`, `eclipse`, `forest`, `graphite`,
`hornet`, `iceberg`, `meadow`, `midnight`, `nord`, `rose`, `solarized`.
Densities: `comfortable`, `compact`, `default`. Control sizes: `lg`, `md`,
`sm`, `xl`, `xs`.

`./styles/*` CSS inventory, sorted:

```text
accordion.css
action-discovery-panel.css
agent-chat-input.css
agent-message.css
agent-plan-record.css
agent-plan.css
agent-question-record.css
agent-question.css
agent-subagent.css
agent-transcript.css
alert-dialog.css
anchored-surface.css
app-header.css
audio-meter.css
audio-player.css
audio-switch.css
avatar.css
block-editor.css
box.css
breadcrumbs.css
bulk-action-bar.css
button.css
calendar.css
callout.css
card-radio-group.css
card-toggle-group.css
card.css
changed-files.css
checkbox.css
code-input.css
code.css
collapse-toggle.css
collapsible.css
color-picker.css
command-palette.css
data-table.css
date-picker.css
date-range-picker.css
date-time-picker.css
date-time-range-picker.css
date-time-zone-picker.css
detail-item.css
detail-section-group.css
detail-section.css
detail-shell.css
dialog.css
dock-region.css
drag-drop.css
drag-number-field.css
drawer.css
duration-input.css
editable-label.css
editable-list.css
embed-input.css
embed-preview.css
empty-state.css
envelope-editor.css
eyebrow.css
fader.css
field-set.css
field.css
file-upload.css
filter-builder.css
filter-toolbar.css
form-actions.css
form-dialog.css
form-layout.css
gain-reduction-meter.css
grid.css
history-center.css
hover-card.css
icon-button.css
icon.css
inline-list-section.css
keyboard.css
knob.css
licence.css
list-card-counter.css
list-card.css
list-container.css
list-grid.css
log-list.css
markdown-editor.css
media-browse-panel.css
media-picker.css
media-preview.css
media-thumbnail.css
menu-surface.css
menu.css
menubar.css
message-center.css
meta-bar.css
meta-item.css
meter-surface.css
meter.css
metric-tile.css
mod-matrix-grid.css
model-connection.css
model-picker.css
motion-policy-provider.css
nav-card.css
navigation-menu.css
number-input.css
order-by.css
page-header.css
page-loading.css
pagination-summary.css
pagination.css
password-requirements.css
picker-shell.css
pill.css
popover.css
progress.css
radio-group.css
radio.css
range-slider.css
rating.css
ref-select.css
region.css
relation-picker.css
remediation-banner.css
resize-handle.css
scroll-shell.css
segmented-control.css
select.css
selection-summary.css
separator.css
settings-shell.css
sidebar-nav.css
skeleton.css
slider.css
spacer.css
spinner.css
split-button.css
split-view.css
stack.css
state-tile.css
status-bar.css
status-indicator.css
stepper.css
surface.css
switch.css
table.css
tabs.css
text-input.css
text-link.css
text.css
theme-select.css
time-ago.css
time-input.css
toast-host.css
toast-stack.css
toggle-group.css
token-input.css
tool-call-group.css
tool-call.css
toolbar.css
tooltip.css
tree.css
tri-state-switch.css
ui-presentation-provider.css
update-center.css
validation-summary.css
value-readout.css
video-player.css
waveform-display.css
xy-pad.css
```

`./icons/*` modules, sorted:

```text
alert-circle
alert-triangle
arrow-down
arrow-left
arrow-right
arrow-up
arrow-up-down
audio-waveform
bell
bold
calendar
check
check-check
check-circle
check-square
chevron-down
chevron-left
chevron-right
chevron-up
circle
circle-alert
circle-check
circle-help
circle-pause
circle-question-mark
circle-x
clock
cloud-off
code
columns-2
columns-3
copy
diff
dot
download
edit
ellipsis
ellipsis-vertical
external-link
eye
file
file-pen
file-question
file-question-mark
file-text
filter
folder
git-branch
git-commit-horizontal
grip-vertical
heading
heart
help-circle
home
house
image
inbox
info
italic
link
list
list-filter
loader
loader-circle
lock
lock-open
mail
maximize-2
menu
minimize-2
minus
monitor
monitor-play
more-horizontal
more-vertical
music
package
pause
pause-circle
pencil
piano
play
plus
quote
redo
refresh-cw
save
search
settings
spinner
square
square-check
star
tag
terminal
trash-2
trending-down
trending-up
triangle-alert
undo
unlock
upload
user
users
volume-2
volume-x
x
x-circle
```

No other core public export is admitted. `tokens/theme-options.ts` and
generated TS under `tokens/generated/ts/` stay implementation details.

## Svelte export map

Condition order is exact: `types`, then `browser`, then `default`. `import` is
absent. A `svelte` condition or top-level `svelte` field is forbidden until
browser and SSR resolution are separately proven.

```json
{
  ".": {
    "types": "./dist/index.d.ts",
    "browser": "./dist/index.client.js",
    "default": "./dist/index.server.js"
  },
  "./*.svelte": {
    "types": "./dist/*.svelte.d.ts",
    "browser": "./dist/*.client.js",
    "default": "./dist/*.server.js"
  },
  "./markdown": {
    "types": "./dist/markdown.d.ts",
    "browser": "./dist/markdown.client.js",
    "default": "./dist/markdown.server.js"
  },
  "./types": {
    "types": "./dist/types.d.ts",
    "browser": "./dist/types.js",
    "default": "./dist/types.js"
  }
}
```

Laws:

- `browser` → client. Node, worker-like, and unknown SSR consumers use
  `default` → server.
- `./*.svelte` may resolve only the 176 roster names. Extra public matches
  fail.
- Root `.` exports the 174 non-markdown roster components plus existing helper
  values/types. It does not export `AgentMessage` or `MarkdownEditor`.
- `./markdown` exports `AgentMessage` and `MarkdownEditor` and their public
  types. No root alias points at it.
- `./types` is compiled JS plus declarations. Declarations-only fails.
- Direct `./AgentMessage.svelte` and `./MarkdownEditor.svelte` remain valid.
  They are not a compatibility shim for the retired root import.

## React export map

React is one JavaScript lane. No `browser`/`import` environment selector.

```json
{
  ".": {
    "types": "./dist/index.d.ts",
    "default": "./dist/index.js"
  },
  "./markdown": {
    "types": "./dist/markdown.d.ts",
    "default": "./dist/markdown.js"
  },
  "./types": {
    "types": "./dist/types.d.ts",
    "default": "./dist/types.js"
  }
}
```

Direct barrels are derived, not a `./*` wildcard: for each frozen name `N`,
export `./N` → `./dist/N.js` + `./dist/N.d.ts`. A wildcard that can resolve
`file-upload`, chunks, or other internals fails.

Root `.` keeps today's non-component helpers and the 174 non-markdown
components. `AgentMessage` and `MarkdownEditor` move to `./markdown`.
`./types` keeps runtime plus declaration reachability even when the source
module is type-only.

React stays `private: true` and unpublished.

## CSS and parser graphs

CSS stays in core. Shells keep exact core style imports and emit no
all-components stylesheet.

| Graph | Must include | Must not include |
| --- | --- | --- |
| ordinary Button/Select (root or direct) | only the style subpaths that graph uses | `marked`; `markdown-editor.css`; `agent-message.css` |
| `./markdown` | `marked`; markdown/agent-message styles those components use | unrelated component stylesheets |

Missing `marked` for `./markdown` is a clear installed failure, not a silent
fallback.

## Forbidden archive content

Packed members are only the contracted `files` plus npm's required
`package.json`. Fail closed on any of:

| Class | Examples |
| --- | --- |
| source | `src/`, raw `.svelte`, non-declaration `.ts`/`.tsx` |
| maps | `*.map` |
| workspace | workspace protocol, sibling paths, TS path aliases inside the archive |
| selectors | `svelte` condition, top-level `svelte` field, `import` on any Svelte export; core JS missing `import`, or core `import`/`default` pointing at different files |
| fallbacks | `main` source fallback, compatibility alias, root markdown re-export |
| receipts | timestamps, absolute paths, unsorted inventories |

`package/dist/.poodle-build.json` must exist as an actual archive member.
Packer dotfile behavior is never assumed.

## Build receipt schema

Every package writes `dist/.poodle-build.json`. Keys are sorted. No timestamp
or absolute path. Two clean builds of the same commit and lock must hash
identically.

```json
{
  "schemaVersion": 1,
  "package": "@inflatable-cookie/poodle-core",
  "version": "0.2.3",
  "sourceCommit": "40-char lowercase hex",
  "tools": {
    "svelte": "5.56.8",
    "typescript": "locked version",
    "vite": "locked version"
  },
  "lanes": ["single"],
  "cssPolicy": "core-owned",
  "markdownPolicy": "none",
  "sourceMaps": false,
  "inputs": ["sorted relative paths from package root"],
  "outputs": [
    { "path": "dist/index.js", "sha256": "lowercase hex" }
  ]
}
```

`lanes` is `["single"]` for core and React, `["client", "server"]` for Svelte.
Shell `markdownPolicy` is `optional-peer-on-./markdown`. `inputs`/`outputs`
are sorted by `path`. Output paths are package-relative and start with `dist/`
or a contracted document name never used here. Tool versions come from the
lockfile, not `process.cwd()`.

A field that cannot be expressed without a timestamp or absolute path is
omitted, not nulled.

## Installed certification receipt schema

`test:web-pack-install` owns this receipt. It is not a package file and must
not mutate the certified tree.

```json
{
  "schemaVersion": 1,
  "kind": "poodle-installed-web-distribution",
  "sourceCommit": "40-char lowercase hex",
  "svelteFloor": "5.56.8",
  "belowFloorNegative": "5.38.6",
  "rosterDenominator": 176,
  "rosterNamesSha256": "f497bfa0a47e1627a1ee7076016ac5566d83584d458b3f3693b688885a02a84a",
  "artifactSetId": "64-char lowercase hex from the artifactSetId formula",
  "packages": {
    "@inflatable-cookie/poodle-core": {
      "archiveSha256": "lowercase hex",
      "buildReceiptSha256": "lowercase hex",
      "version": "recorded, not edited by distribution cards"
    },
    "@inflatable-cookie/poodle-react": {
      "archiveSha256": "lowercase hex",
      "buildReceiptSha256": "lowercase hex",
      "version": "recorded, not edited by distribution cards"
    },
    "@inflatable-cookie/poodle-svelte": {
      "archiveSha256": "lowercase hex",
      "buildReceiptSha256": "lowercase hex",
      "version": "recorded, not edited by distribution cards"
    }
  }
}
```

Package keys in `packages` are the three names below, sorted lexicographically
by the package name string.

### `rosterNamesSha256`

SHA-256 (FIPS 180-4) over the UTF-8 bytes of the 176 sorted names in
**Canonical component denominator**, in that order, with one LF (`0x0A`) after
every name including the last. No BOM, no CR, no spaces around names, no blank
lines. Digest is 64-char lowercase hex.

That payload is a POSIX text. `Array.prototype.join("\n")` without a final LF
is non-conforming. For the names frozen in this spec the digest is:

```text
f497bfa0a47e1627a1ee7076016ac5566d83584d458b3f3693b688885a02a84a
```

The same names joined with LF *between* names only hash to
`740db11b4f8a28e28b5aeae52fc28665197b7161e4fce65a6bd1b0e148293a79` and fail.

### `artifactSetId`

Let `Hcore`, `Hreact`, and `Hsvelte` be the 64-char lowercase hex
`archiveSha256` values for:

1. `@inflatable-cookie/poodle-core`
2. `@inflatable-cookie/poodle-react`
3. `@inflatable-cookie/poodle-svelte`

That order is lexicographic by package name, not publication role. Canonical
bytes are UTF-8 JSON with no whitespace and no trailing newline:

```text
["<Hcore>","<Hreact>","<Hsvelte>"]
```

`artifactSetId` is SHA-256 of those bytes, 64-char lowercase hex.

Worked vector, not a production archive: if the three hashes are 64 `a`, 64
`b`, and 64 `c` letters, the canonical bytes are 202 bytes starting
`["aaaa` and not ending in LF, and `artifactSetId` is
`a3e9a05750d52fe7168bb71489884987f74a7e0ff0a70b437132829956a308b0`.
Pretty-printed JSON, a trailing LF, or role order core/Svelte/React each
produce a different digest and fail.

The harness compares two builds and two packs, then records this object.
`g16.054` may copy identity into candidate history; it does not change this
schema.

## Root markdown migration

Breaking at `0.3.0`. No shim, alias, or silent fallback.

Retired:

```ts
import { AgentMessage, MarkdownEditor } from "@inflatable-cookie/poodle-svelte";
import { AgentMessage, MarkdownEditor } from "@inflatable-cookie/poodle-react";
```

Required:

```ts
import { AgentMessage, MarkdownEditor } from "@inflatable-cookie/poodle-svelte/markdown";
import { AgentMessage, MarkdownEditor } from "@inflatable-cookie/poodle-react/markdown";
```

Direct Svelte `./AgentMessage.svelte` / `./MarkdownEditor.svelte` and React
`./AgentMessage` / `./MarkdownEditor` stay. `g16.054` writes the `0.3.0`
release notes and candidate history. This spec does not edit those files.

Current-state drift this card does not repair: Svelte peer `>=5.38.6 <6`;
`marked` as a hard shell dependency; core `marked` devDependency; source
`files`/`exports`; React missing `./types` and `./markdown`; Svelte `svelte`
condition and field.

## Successor writable scopes and oracles

Successors inherit these rows. They do not reopen inventories.

### g16.057 core substrate

Writable: core build/staging driver, core manifest/export targets, core
declaration config, core asset-copy and receipt logic, focused build tests,
that card, one log, papercuts.

| Invariant | Smallest counterexample | Proof |
| --- | --- | --- |
| Stable public names | Vite hashes `index.js` or a token CSS basename | installed target check fails |
| CSS inventory complete | `./styles/button.css` or one theme CSS missing from `dist` | export/content test fails |
| Icons complete | `./icons/x` missing JS or declarations | export check fails |
| Source-free | staged non-declaration `.ts` | source audit fails |
| Receipt reproducible | timestamp or absolute path in JSON | second-build hash differs |
| Core import is format fallback | core `.` omits `import`, or `import` and `default` differ | export-map check fails |
| Parser isolation | `marked` in core output or receipt | dependency audit fails |
| Card stays core-only | Svelte component compilation | diff-scope check fails |

### g16.058 shell distributions

Writable: Svelte/React build and staging, their manifests/exports, root and
markdown barrels, peer/CSS declarations, focused build and disposable smoke
fixtures, that card, one log, papercuts. Not `test:web-pack-install`.

| Invariant | Smallest counterexample | Proof |
| --- | --- | --- |
| Lanes are distinct | client artifact renders through `svelte/server` | negative fixture fails |
| Conditions describe environment | `import` before `default`, or `import` selects client | worker/Node proof fails |
| Source-free Svelte | `./*.svelte` resolves raw `.svelte` | archive/export audit fails |
| Public wildcard is exact | `dist/MenuSurface.client.js` importable as `./MenuSurface.svelte` | unreviewed-file audit fails |
| Markdown isolated | Button root or `./Button.svelte` resolves `marked` | graph assertion fails |
| Types reachable | `./types` declarations-only | inventory check fails |
| Floor | browser or SSR fails at `5.56.8` | floor check fails |
| React private | `private` cleared or publishConfig public | publication-state check fails |
| Harness serial | card edits `test:web-pack-install` | scope check fails |

### g16.059 installed certification

Writable: `test:web-pack-install` and permanent fixtures, installed probes,
deterministic receipt evidence, package-install docs, that card, one log,
papercuts. Minimal package/build repairs only after explicit review.

| Invariant | Smallest counterexample | Proof |
| --- | --- | --- |
| Archives, not source | consumer resolves workspace core | realpath/export check fails |
| Browser and SSR | only browser fixture passes | server oracle blocks receipt |
| Floor truthful | `5.56.8` server render fails | floor leg blocks acceptance |
| Negatives bite | `branchCount` compiles | unsuppressed fixture fails |
| Receipt matches tree | evidence edit changes package tree | certified tree hash changes |
| Roster canonical | fixture remains 175, or names disagree with this spec | denominator gate fails |
| Receipt bytes are exact | `rosterNamesSha256` joins names with LF between them and omits the final LF, or `artifactSetId` pretty-prints JSON, appends LF, or orders core/Svelte/React | installed receipt comparison fails |
| Dotfile membership | receipt missing from tarball | archive member check fails |
| Not a release | tag, workflow, or npm change | scope gate fails |

## Current-state evidence

Inspected at planning base descendant `9e8e646f25a1dfde818083c798ffba53adea3e95`:

- core, Svelte, and React `files` still include `src`
- Svelte exports use `svelte` conditions and a top-level `svelte` field
- Svelte peer is `>=5.38.6 <6`; `marked` is a hard dependency
- React public exports are `.` only; `private: true` already
- 178 Svelte `*.svelte` files exist; 176 are roster components
- core has `marked` as a devDependency

Successors rewrite to this target. They do not preserve the drifted shape.
