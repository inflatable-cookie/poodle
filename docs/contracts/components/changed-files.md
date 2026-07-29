# ChangedFiles

Status: detailed contract
Updated: 2026-07-29

## 1. Purpose

- Component name: `ChangedFiles`
- Layer: `composites`
- Summary: the file changes a turn produced — a counted summary that expands
  into a directory tree with per-node line counts
- In scope: the summary header with totals, the collapsed chip row, the expanded
  directory tree with rolled-up counts, the diff action
- Out of scope: computing the changes, rendering a diff, staging or reverting,
  file icons by type beyond what `Tree` provides

The card answers "what did this turn touch?" without leaving the transcript,
and hands off to the host for anything deeper.

## 2. Anatomy

```text
[Root .changed-files] <div>  (carries data-expanded/data-size/data-density)
  ├── [Header .changed-files__header] <div>
  │   ├── [Toggle .changed-files__toggle] <button type="button" aria-expanded aria-controls>
  │   │   ├── [Toggle Icon] Icon (icon="chevron-right", rotated when expanded)
  │   │   ├── [Count .changed-files__count] <span>  ("9 changed files")
  │   │   ├── [Additions .changed-files__additions] <span>  ("+376")
  │   │   └── [Deletions .changed-files__deletions] <span>  ("-16")
  │   ├── [Files Toggle .changed-files__files-toggle] <button type="button">  ("Show files" / "Hide files")
  │   └── [Actions .changed-files__actions] <div>
  │       ├── [Collapse All] IconButton  (conditional: expanded)
  │       └── [Open Diff] Button  (icon="diff", variant="secondary")
  ├── [Summary .changed-files__summary] <div>  (conditional: collapsed)
  │   ├── [Scope .changed-files__scope] <span>  (repeated; "cp-api 7 files")
  │   ├── [Chip .changed-files__chip] <button type="button">  (repeated; icon + filename)
  │   └── [More .changed-files__more] <button type="button">  ("Show all 9 files")
  └── [Tree .changed-files__tree] Tree  (conditional: expanded)
      └── per node: [Node Additions] <span>  [Node Deletions] <span>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | bordered card on the elevated surface | `--poodle-radius-surface`, `--poodle-color-background-elevated`, `--poodle-color-border-subtle` |
| Header | yes | the always-visible summary row | `--poodle-space-inline-md` |
| Count | yes | "N changed files" | `--poodle-color-text-primary`, `--poodle-typography-label-weight` |
| Additions | yes | `+N` in the success colour | `--poodle-color-status-success` |
| Deletions | yes | `-N` in the danger colour | `--poodle-color-status-danger` |
| Files Toggle | no | the text affordance mirroring the chevron | `--poodle-color-text-secondary` |
| Open Diff | no | hands off to the host | (Button contract) |
| Summary | no | scope counts plus a few file chips, collapsed only | `--poodle-space-inline-sm` |
| Chip | no | one file, icon plus truncated name | `--poodle-radius-control`, `--poodle-color-background-surface` |
| Tree | no | the directory tree, expanded only | (Tree contract) |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string` | — | yes | identity for callbacks |
| `files` | `ChangedFile[]` | `[]` | yes | flat paths; the component builds the tree |
| `expanded` | `boolean` | `false` | no | controlled when bound |
| `expandedPaths` | `string[]` | `[]` | no | expanded directory nodes in the tree |
| `chipLimit` | `number` | `3` | no | file chips shown before "Show all N files" |
| `showOpenDiff` | `boolean` | `true` | no | renders the diff action |
| `openDiffLabel` | `string` | `"Open diff"` | no | action label |
| `showFilesLabel` | `string` | `"Show files"` | no | collapsed text affordance |
| `hideFilesLabel` | `string` | `"Hide files"` | no | expanded text affordance |
| `countLabel` | `(n: number) => string` | `` (n) => `${n} changed files` `` | no | header wording; singular is the host's problem to phrase |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role |
| `size` | `ControlSize \| null` | `null` | no | explicit size override |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |
| `onToggle` | `((id: string) => void) \| null` | `null` | no | the card opened or closed |
| `onOpenDiff` | `((id: string) => void) \| null` | `null` | no | the diff action was used |
| `onFileSelect` | `((path: string) => void) \| null` | `null` | no | a chip or tree row was chosen |
| `onExpandedPathsChange` | `((paths: string[]) => void) \| null` | `null` | no | tree expansion changed |

### Computed Values

| Name | Formula |
|------|---------|
| `totals` | `changedFilesTotals(files)` — file count, summed additions, summed deletions |
| `tree` | paths split on `/` and folded into nodes; single-child directory chains collapse into one row (`crates/latex`) |
| `nodeTotals` | each directory's counts are the sum of its descendants' |
| `scopes` | top-level directories with their file counts |
| `visibleChips` | `files.slice(0, chipLimit)` |

Collapsing single-child directory chains is what keeps the tree readable: a path
like `cp-api/crates/latex/src/parser.rs` would otherwise cost four rows to say
one thing. The chain collapses to `crates/latex` exactly as long as no node in
it has a sibling.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| collapsed | default | header, scope counts, chips up to `chipLimit`, "Show all N files" |
| expanded | `expanded` | header, directory tree with per-node counts; chips and scopes hidden |
| empty | no files | the card does not render at all |
| no diff action | `showOpenDiff={false}` | header without the trailing button |

An empty card renders nothing rather than an empty state. A turn that changed no
files should not have a box saying so — the absence is the message.

## 5. Events

| Event | Payload | When |
|-------|---------|------|
| `onToggle` | id | the card opened or closed |
| `onOpenDiff` | id | the diff action was used |
| `onFileSelect` | path | a chip or a file row was chosen |
| `onExpandedPathsChange` | paths | a directory was opened or closed |

## 6. Accessibility

### Semantics

- The header toggle is a `<button>` with `aria-expanded` and `aria-controls`
  pointing at the tree.
- Additions and deletions are not conveyed by colour alone: the accessible name
  of the header is `"{count} changed files, {additions} added, {deletions}
  removed"`.
- The tree uses the `Tree` contract's semantics (`role="tree"`, `treeitem`,
  `aria-level`, `aria-expanded`), and file choice arrives through its
  `onActivate`, not a bespoke selection event.
- The "Show files" text affordance and the chevron are the **same** control
  duplicated visually, so only one carries the accessible name; the other is
  `aria-hidden`. Two controls announcing the same action is a worse outcome than
  one.

### Keyboard

| Key | Action |
|-----|--------|
| `Enter` / `Space` on the header | expands or collapses the card |
| arrow keys inside the tree | per the `Tree` contract |
| `Tab` | header, files toggle, actions, then into the tree |

## 7. Layout

### Sizing

| Aspect | Rule |
|--------|------|
| card inset | `--poodle-space-inline-md` |
| header gap | `--poodle-space-inline-md` |
| chip row gap | `--poodle-space-inline-sm`, wrapping |
| count column | additions and deletions right-aligned per tree row |

### Composition

Rendered by `AgentTranscript` for `changed-files` blocks. Uses `Tree` for the
expanded view, with a trailing slot for the per-node counts.

## 8. Token Usage

| Property | Token |
|----------|-------|
| card background | `--poodle-color-background-elevated` |
| card border | `--poodle-color-border-subtle` |
| card radius | `--poodle-radius-surface` |
| count text | `--poodle-color-text-primary` |
| additions | `--poodle-color-status-success` |
| deletions | `--poodle-color-status-danger` |
| scope text | `--poodle-color-text-secondary` |
| chip background | `--poodle-color-background-surface` |
| chip radius | `--poodle-radius-control` |
| focus ring | `--poodle-border-width-focus`, `--poodle-color-accent-focusRing`, offset outline |
| numeric font | `--poodle-typography-code-family`, `font-variant-numeric: tabular-nums` |

Counts use tabular figures so the additions and deletions columns line up down
the tree rather than jittering per row.

### Size Variants

Size sets the type scale and the card radius. Density sets the card inset and
the header and chip gaps. Density never changes the header's height.

### Data Attributes

| Attribute | Values | On |
|-----------|--------|-----|
| `data-expanded` | `true`/`false` | root |
| `data-file-count` | number | root |
| `data-size` / `data-density` | the ladders | root |

## 9. Svelte Notes

- The tree is `$derived` from `files`; there is no stored tree to invalidate.
- Chip and tree rendering are mutually exclusive, so only one is in the DOM.

## 10. GPUI Notes

- Reuses the GPUI `Tree` implementation and its flat-mode metrics.
- Expansion is host-driven through the spec.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] totals sum additions and deletions across all files
- [ ] directory counts roll up from descendants
- [ ] single-child directory chains collapse into one row
- [ ] an empty file list renders nothing at all
- [ ] the header's accessible name carries both counts in words
- [ ] the chevron and the text affordance are one control, announced once
- [ ] no vendor vocabulary anywhere in the component

### Tier 2: Visual Parity

- [ ] additions are the success colour, deletions the danger colour
- [ ] counts use tabular figures and align down the tree
- [ ] chips truncate long filenames and wrap rather than overflow
- [ ] card inset, header gap and radius match per size and density
- [ ] density never changes the header height

### Tier 3: Implementation Freedom

- [ ] expand/collapse animation is platform-owned
- [ ] chip overflow measurement is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Natives render the expansion state the spec describes | shared render-only posture | accepted | host drives expansion |
| `countLabel` is an optional resolved string on the native spec, not a formatter | a Rust spec holds data, not closures; `None` uses the default phrasing | accepted | none |
| No diff rendering on any target | a diff viewer is a component in its own right, not a card detail | accepted (by design) | `onOpenDiff` hands off |

## 13. Approval And Adoption Notes

- contract status: `drafted`
- approvers: pending review
- downstream adopters: Figmatic, Loophole, future agent surfaces
- future follow-up: per-file diff preview on hover, staging controls, file
  status glyphs

## 14. Specimen Definitions

Required specimen coverage (Svelte preview authoritative): the worked example
(9 files, +376 −16, two scopes) collapsed; the same expanded; a single file; a
deep path exercising chain collapse; files across many scopes; additions only;
deletions only; `showOpenDiff={false}`; a long filename truncating in a chip;
more files than `chipLimit`; full size ladder; density variants.
