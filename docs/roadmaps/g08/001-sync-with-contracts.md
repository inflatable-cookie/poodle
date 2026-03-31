# g08.001 Sync With Contracts: Verify Names, Props, And Token Methods

Status: complete
Owner: Poodle Core
Depends on: —

## Why This Comes First

The Svelte reference implementation is being actively refactored — component
names, prop types, composite boundaries, and contracts are changing. Before
fixing any GPUI component quality issues, we must ensure our implementations
target the correct contracts. Fixing a component that's been renamed or whose
props have changed is wasted work.

## Work Completed

- [x] Map every contract to its GPUI component (or identify gaps)
- [x] Delete 14 orphaned GPUI components with no matching contract:
  `badge` (→ pill), `banner` (→ callout), `command_palette_shell`,
  `form_shell`, `inline` (→ stack), `inline_remediation`, `panel_header`,
  `panel_surface`, `panel_tabs`, `project_header`, `remediation_banner`,
  `surface_tabs`, `validation_summary` (deferred), `workspace_shell`
- [x] Rename `call_out` → `callout` (struct: `PoodleCallOut` → `PoodleCallout`)
- [x] Rename `shell_status_bar` → `status_bar` (struct: `PoodleShellStatusBar` → `PoodleStatusBar`)
- [x] Update component registry: remove deleted entries, rename
  `AutonomousList` → `EditableList`, `StateTile` → `MetricTile`
- [x] Update all specimen files referencing deleted/renamed components
- [x] Update demo view to use surviving equivalents
- [x] Verify clean compile (zero errors, warnings only)

## Spec Struct Alignment Audit (15-component sample)

Sampled: button, checkbox, switch, text_input, select, tabs, slider, accordion,
callout, pill, progress, separator, dialog, tooltip, surface

### Fully Aligned (5)
- **slider** — all props and tokens match contract
- **progress** — all props and tokens match contract
- **separator** — all props and tokens match contract
- **dialog** — all props and tokens match contract
- **tooltip** — aligned (extra `aria_label` is acceptable; default delay 400ms
  vs contract 300ms — minor)

### Missing Props (6) — fix in quality batches 005–007
- **button** — missing `chevron` prop (contract requires boolean)
- **checkbox** — missing `id` prop (needed for label association)
- **switch** — missing `id` and `name` props
- **select** — missing `id` and `name` props
- **tabs** — missing `variant` prop (contract has underline/card/pill/strip) and
  `isReorderable`
- **text_input** — missing `prefix`, `suffix`, `maxLength`, `showCharCount`;
  has `leading_icon`/`trailing_icon` not in contract (may need contract update)

### Major Misalignment (2)
- **pill** — spec implements selectable/removable chips (label, is_removable,
  is_selected, is_disabled) but contract defines a toned label display (tone,
  appearance, size, font, isMuted). Needs full spec rewrite.
- **callout** — spec uses `content` where contract says `message`; missing
  `ariaLabel`, `announceMode`, `isDismissible`, `dismissLabel`. Token mapping
  for neutral/pending tones questionable.

### Missing Spec
- **accordion** — no spec file found in `packages/contracts/primitives/src/`

### Not Verified
- **surface** — no contract file found (may be internal/structural only)

## Remaining Work

- [x] Verify spec struct alignment for a sample of 15 components
- [x] Document spec structs with generation drift that need updating (see above)
- [x] Confirm which contracts are still in flux vs stable on the Svelte side
  - Primitives: stable
  - Composites: mostly solid, some edits in progress

## 20 Contracts Needing New GPUI Components

Foundation (16): `alert-dialog`, `breadcrumbs`, `bulk-action-bar`, `card`,
`collapse-toggle`, `combobox`, `detail-item`, `list-card`, `nav-card`,
`nav-card-grid`, `order-by`, `pagination`, `region`, `resize-handle`,
`status-bar`, `table`

Composites (4): `detail-section`, `metric-tile`, `page-header`, `toast-stack`

These are queued for milestones 002 and 003.

## Contracts To Ignore

- `icon-provider` — context provider, not a visual component
- `surface-elevation` — visual concept, not a component
- `browse-search-shell` — being removed (no Svelte component)
- `embed-shell` — being removed (no Svelte component)
- `split-divider` — already removed

## Acceptance Criteria

- [x] Every GPUI component maps to a current contract
- [x] No orphaned components targeting deleted/renamed contracts
- [x] Clean compile after all deletions/renames
- [x] Spec struct divergences documented and queued for fix (see audit above)
- [ ] Clear picture of which contracts are stable vs still changing
