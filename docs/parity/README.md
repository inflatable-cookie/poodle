<!-- parity-index -->
# Poodle Parity Audit

Systematic contract ↔ Svelte ↔ GPUI ↔ Jetstream parity pass across every
component. Goal: Svelte and the contracts agree, and the two Rust targets
(GPUI, Jetstream) match Svelte's behavior + anatomy as closely as the runtime
allows.

## Authority order

1. **Svelte is the parity reference** for behavior + visuals. When Svelte and a
   contract disagree, Svelte usually wins and the contract is corrected — *unless*
   Svelte is missing functionality the contract specifies (then Svelte gets the
   fix, like `Field`'s `aria-describedby` description wiring).
2. **Contracts are the spec of record** once reconciled — Rust targets implement
   to the contract + Svelte.
3. **Known runtime deltas are accepted**, not bugs: GPUI has no accessibility API
   (no accesskit) → ARIA/role gaps are expected; Jetstream renders from spec with
   no ARIA layer and interaction lives in the preview event loop; virtualization
   is Svelte-only.

## Method

- **Breadth-first**: one `docs/parity/<component>.md` per component first (the full
  map + per-target todo lists), then the fix pass closes gaps.
- **Verification**:
  - Svelte — runtime-verified via the preview MCP (real DOM/ARIA/interaction).
  - Jetstream — `poodle_jetstream_components::render_probe` runs real `GameUi`
    layout headless (no GPU) and introspects the `UiTree` (sizes, colors, text,
    token keys). See `packages/jetstream/components/src/render_probe.rs`.
  - GPUI — build-verified + shared logic unit-tested in `poodle-specs`. No
    headless render (gpui needs a live window); a11y is an accepted gap.
- **Fix aggressiveness**: close every feasible gap (behavior + visual). Hard
  runtime limits (GPUI a11y) are logged, not forced.

## Per-component file

Each `docs/parity/<component>.md` starts with a machine-readable status comment:

```
<!-- parity consv=ok|fixed|gap gpui=<n-todos> jetstream=<n-todos> specimen=ok|gap -->
```

`scripts/parity-status.py` (or the snippet at the bottom) compiles these into the
matrix below. Agents/writers only touch their own component file — never this
README's table — to avoid write conflicts.

## Excluded from component parity

Contract docs that are **not components** (no impl expected):

- `format-display-date`, `format-file-size` — pure formatting utilities.
- `size-and-density`, `surface-elevation`, `treatment-tokens` — concept/token docs.

## Naming aliases (same component, different filename)

- `box` → GPUI/Jetstream `bx.rs`
- `breadcrumbs` → Jetstream `breadcrumbs_comp.rs`
- `pagination` → Jetstream `pagination_comp.rs`

## Impl without contract (contract gaps to reconcile)

Jetstream-only files with no contract / no Svelte peer — triage whether they map
to an existing contract under another name, are legacy to remove, or need a
contract: `badge.rs`, `banner.rs`, `reorderable_list.rs`, `shell_status_bar.rs`,
`floating_overlay.rs` (also GPUI; infra), `iconregistry`/`icon_registry` (GPUI).

## Status matrix

<!-- BEGIN MATRIX (compiled; do not hand-edit) -->
| component | consv | gpui | jet | spec |
|---|---|---|---|---|
| accordion | gap | 6 | 8 | gap |
| action-discovery-panel | gap | 6 | 9 | gap |
| alert-dialog | gap | 9 | 10 | gap |
| app-header | gap | 8 | 8 | gap |
| audio-player | ok | 8 | 7 | gap |
| avatar | ok | 4 | 8 | gap |
| block-editor | gap | 9 | 8 | gap |
| box | ok | 2 | 3 | gap |
| breadcrumbs | gap | 5 | 7 | gap |
| bulk-action-bar | gap | 8 | 9 | gap |
| button | gap | 4 | 7 | gap |
| calendar | gap | 9 | 10 | gap |
| callout | gap | 7 | 9 | gap |
| card | gap | 7 | 7 | gap |
| card-radio-group | gap | 8 | 9 | gap |
| card-toggle-group | gap | 9 | 11 | gap |
| checkbox | ok | 4 | 5 | gap |
| code | gap | 8 | 9 | gap |
| code-input | gap | 5 | 7 | gap |
| collapse-toggle | gap | 4 | 6 | gap |
| collapsible | gap | 6 | 8 | gap |
| color-picker | gap | 9 | 8 | gap |
| command-palette | gap | 9 | 11 | gap |
| confirm-action | ok | 7 | 7 | gap |
| context-menu | ok | 2 | 5 | gap |
| data-table | gap | 6 | 6 | gap |
| date-picker | gap | 4 | 6 | gap |
| date-range-picker | gap | 3 | 3 | gap |
| date-time-picker | gap | 6 | 4 | gap |
| date-time-range-picker | gap | 7 | 4 | gap |
| date-time-zone-picker | gap | 9 | 6 | gap |
| debug-dialog | ok | 3 | 2 | gap |
| detail-item | gap | 3 | 5 | gap |
| detail-section | gap | 3 | 3 | gap |
| detail-section-group | gap | 4 | 2 | gap |
| detail-shell | ok | 4 | 6 | gap |
| dialog | ok | 2 | 3 | gap |
| dock-region | ok | 7 | 10 | gap |
| drawer | ok | 6 | 6 | gap |
| duration-input | gap | 9 | 6 | gap |
| editable-label | ok | 8 | 11 | gap |
| editable-list | gap | 11 | 11 | gap |
| embed-input | gap | 6 | 7 | gap |
| embed-preview | gap | 7 | 5 | gap |
| empty-state | ok | 9 | 5 | ok |
| error-boundary | ok | 2 | 1 | gap |
| eyebrow | ok | 5 | 6 | gap |
| field | ok | 6 | 6 | gap |
| field-set | gap | 6 | 6 | gap |
| file-upload | ok | 8 | 8 | gap |
| filter-toolbar | gap | 7 | 6 | gap |
| form-actions | ok | 6 | 7 | gap |
| form-dialog | ok | 9 | 9 | gap |
| form-layout | gap | 8 | 8 | gap |
| form-shell | gap | 10 | 12 | gap |
| grid | gap | 4 | 4 | gap |
| hover-card | ok | 4 | 6 | ok |
| icon | ok | 2 | 1 | gap |
| icon-button | gap | 6 | 9 | gap |
| icon-provider | gap | 1 | 1 | gap |
| inline-list-section | gap | 6 | 1 | gap |
| inline-remediation | ok | 1 | 6 | gap |
| list-card | gap | 7 | 10 | gap |
| list-card-counter | gap | 2 | 2 | ok |
| list-container | gap | 6 | 6 | gap |
| list-grid | gap | 1 | 1 | gap |
| log-list | ok | 9 | 8 | gap |
| markdown-editor | gap | 11 | 8 | gap |
| media-browse-panel | ok | 9 | 4 | gap |
| media-picker | gap | 8 | 8 | gap |
| media-preview | gap | 9 | 8 | gap |
| media-thumbnail | ok | 8 | 9 | gap |
| menu | gap | 2 | 5 | ok |
| menubar | gap | 3 | 6 | gap |
| meta-bar | gap | 3 | 3 | gap |
| meta-item | gap | 4 | 2 | gap |
| meter | gap | 3 | 6 | gap |
| metric-tile | ok | 8 | 5 | gap |
| nav-card | ok | 9 | 10 | gap |
| navigation-menu | gap | 5 | 8 | gap |
| number-input | gap | 5 | 6 | gap |
| order-by | gap | 10 | 11 | gap |
| page-header | gap | 8 | 8 | gap |
| page-loading | ok | 5 | 7 | gap |
| pagination | gap | 8 | 10 | gap |
| pagination-summary | gap | 2 | 2 | gap |
| password-requirements | ok | 5 | 6 | gap |
| picker-shell | gap | 4 | 4 | gap |
| pill | gap | 3 | 5 | gap |
| popover | gap | 5 | 7 | gap |
| progress | gap | 3 | 5 | gap |
| radio-group | ok | 4 | 6 | gap |
| range-slider | gap | 6 | 8 | gap |
| rating | ok | 8 | 7 | gap |
| region | ok | 1 | 5 | gap |
| relation-picker | gap | 9 | 10 | gap |
| remediation-banner | gap | 1 | 8 | gap |
| resize-handle | ok | 4 | 5 | gap |
| scroll-shell | ok | 3 | 6 | gap |
| segmented-control | gap | 6 | 8 | gap |
| select | ok | 4 | 6 | gap |
| selection-summary | gap | 5 | 6 | gap |
| separator | ok | 3 | 2 | gap |
| sidebar-nav | gap | 6 | 11 | gap |
| skeleton | gap | 6 | 8 | gap |
| slider | gap | 6 | 9 | gap |
| spacer | ok | 2 | 2 | gap |
| spinner | gap | 5 | 6 | gap |
| split-button | gap | 4 | 7 | gap |
| split-view | gap | 6 | 6 | gap |
| stack | gap | 3 | 6 | gap |
| state-tile | gap | 1 | 8 | gap |
| status-bar | gap | 6 | 8 | gap |
| status-indicator | gap | 4 | 5 | gap |
| surface | gap | 6 | 5 | gap |
| switch | gap | 3 | 7 | gap |
| tab-strip | gap | 4 | 8 | gap |
| table | gap | 4 | 5 | gap |
| tabs | gap | 11 | 12 | gap |
| text | ok | 4 | 6 | gap |
| text-input | gap | 8 | 9 | gap |
| text-link | ok | 4 | 8 | gap |
| time-ago | gap | 4 | 4 | gap |
| time-input | gap | 4 | 5 | gap |
| time-zone-select | gap | 6 | 6 | gap |
| toast-host | gap | 6 | 7 | gap |
| toast-stack | ok | 9 | 11 | gap |
| toggle-group | gap | 4 | 8 | gap |
| token-input | gap | 9 | 11 | gap |
| toolbar | gap | 4 | 5 | gap |
| tooltip | gap | 3 | 6 | gap |
| tree | ok | 2 | 4 | ok |
| tri-state-switch | gap | 5 | 6 | gap |
| ui-presentation-provider | gap | 3 | 2 | gap |
| validation-summary | gap | 8 | 9 | gap |
| video-player | gap | 7 | 4 | gap |
<!-- END MATRIX -->

## Status compiler

```python
# scripts/parity-status.py
import glob, re
rows=[]
for f in sorted(glob.glob('docs/parity/*.md')):
    if f.endswith('README.md') or f.endswith('TEMPLATE.md'): continue
    head=open(f).read(400)
    m=re.search(r'parity consv=(\S+) gpui=(\S+) jetstream=(\S+) specimen=(\S+)',head)
    name=f.split('/')[-1][:-3]
    rows.append((name,)+(m.groups() if m else ('?','?','?','?')))
w=max(len(r[0]) for r in rows)
print(f'| {"component":<{w}} | consv | gpui | jet | spec |')
print(f'|{"-"*(w+2)}|-------|------|-----|------|')
for r in rows:
    print(f'| {r[0]:<{w}} | {r[1]:<5} | {r[2]:<4} | {r[3]:<3} | {r[4]:<4} |')
```
