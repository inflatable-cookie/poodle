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
| accordion | fixed | 6 | 8 | gap |
| action-discovery-panel | fixed | 6 | 9 | gap |
| alert-dialog | fixed | 9 | 10 | gap |
| app-header | fixed | 8 | 8 | gap |
| audio-player | ok | 8 | 7 | gap |
| avatar | ok | 4 | 8 | gap |
| block-editor | fixed | 9 | 8 | gap |
| box | ok | 2 | 3 | gap |
| breadcrumbs | fixed | 5 | 7 | gap |
| bulk-action-bar | fixed | 8 | 9 | gap |
| button | fixed | 4 | 7 | gap |
| calendar | fixed | 9 | 10 | gap |
| callout | fixed | 7 | 9 | gap |
| card | fixed | 7 | 7 | gap |
| card-radio-group | fixed | 8 | 9 | gap |
| card-toggle-group | fixed | 9 | 11 | gap |
| checkbox | ok | 4 | 5 | gap |
| code | fixed | 8 | 9 | gap |
| code-input | fixed | 5 | 7 | gap |
| collapse-toggle | fixed | 4 | 6 | gap |
| collapsible | fixed | 6 | 8 | gap |
| color-picker | fixed | 9 | 8 | gap |
| command-palette | fixed | 9 | 11 | gap |
| confirm-action | ok | 7 | 7 | gap |
| context-menu | ok | 2 | 5 | gap |
| data-table | fixed | 6 | 6 | gap |
| date-picker | fixed | 4 | 6 | gap |
| date-range-picker | fixed | 3 | 3 | gap |
| date-time-picker | fixed | 6 | 4 | gap |
| date-time-range-picker | fixed | 7 | 4 | gap |
| date-time-zone-picker | fixed | 9 | 6 | gap |
| debug-dialog | ok | 3 | 2 | gap |
| detail-item | fixed | 3 | 5 | gap |
| detail-section | fixed | 3 | 3 | gap |
| detail-section-group | fixed | 4 | 2 | gap |
| detail-shell | ok | 4 | 6 | gap |
| dialog | ok | 2 | 3 | gap |
| dock-region | ok | 7 | 10 | gap |
| drawer | ok | 6 | 6 | gap |
| duration-input | fixed | 9 | 6 | gap |
| editable-label | ok | 8 | 11 | gap |
| editable-list | fixed | 11 | 11 | gap |
| embed-input | fixed | 6 | 7 | gap |
| embed-preview | fixed | 7 | 5 | gap |
| empty-state | ok | 9 | 5 | ok |
| error-boundary | ok | 2 | 1 | gap |
| eyebrow | ok | 5 | 6 | gap |
| field | ok | 6 | 6 | gap |
| field-set | fixed | 6 | 6 | gap |
| file-upload | ok | 8 | 8 | gap |
| filter-toolbar | fixed | 7 | 6 | gap |
| form-actions | ok | 6 | 7 | gap |
| form-dialog | ok | 9 | 9 | gap |
| form-layout | fixed | 8 | 8 | gap |
| form-shell | gap | 10 | 12 | gap |
| grid | fixed | 4 | 4 | gap |
| hover-card | ok | 4 | 6 | ok |
| icon | ok | 2 | 1 | gap |
| icon-button | fixed | 6 | 9 | gap |
| icon-provider | fixed | 1 | 1 | gap |
| inline-list-section | fixed | 6 | 1 | gap |
| inline-remediation | ok | 1 | 6 | gap |
| list-card | fixed | 7 | 10 | gap |
| list-card-counter | fixed | 2 | 2 | ok |
| list-container | fixed | 6 | 6 | gap |
| list-grid | fixed | 1 | 1 | gap |
| log-list | ok | 9 | 8 | gap |
| markdown-editor | fixed | 11 | 8 | gap |
| media-browse-panel | ok | 9 | 4 | gap |
| media-picker | fixed | 8 | 8 | gap |
| media-preview | fixed | 9 | 8 | gap |
| media-thumbnail | ok | 8 | 9 | gap |
| menu | fixed | 2 | 5 | ok |
| menubar | fixed | 3 | 6 | gap |
| meta-bar | fixed | 3 | 3 | gap |
| meta-item | fixed | 4 | 2 | gap |
| meter | fixed | 3 | 6 | gap |
| metric-tile | ok | 8 | 5 | gap |
| nav-card | ok | 9 | 10 | gap |
| navigation-menu | fixed | 5 | 8 | gap |
| number-input | gap | 5 | 6 | gap |
| order-by | fixed | 10 | 11 | gap |
| page-header | fixed | 8 | 8 | gap |
| page-loading | ok | 5 | 7 | gap |
| pagination | fixed | 8 | 10 | gap |
| pagination-summary | fixed | 2 | 2 | gap |
| password-requirements | ok | 5 | 6 | gap |
| picker-shell | gap | 4 | 4 | gap |
| pill | fixed | 3 | 5 | gap |
| popover | fixed | 5 | 7 | gap |
| progress | fixed | 3 | 4 | gap |
| radio-group | ok | 4 | 6 | gap |
| range-slider | gap | 6 | 8 | gap |
| rating | ok | 8 | 7 | gap |
| region | ok | 1 | 5 | gap |
| relation-picker | fixed | 9 | 10 | gap |
| remediation-banner | gap | 1 | 8 | gap |
| resize-handle | ok | 4 | 3 | gap |
| scroll-shell | ok | 3 | 6 | gap |
| segmented-control | fixed | 6 | 8 | gap |
| select | ok | 4 | 6 | gap |
| selection-summary | fixed | 5 | 6 | gap |
| separator | ok | 3 | 2 | gap |
| sidebar-nav | fixed | 6 | 11 | gap |
| skeleton | fixed | 6 | 8 | gap |
| slider | fixed | 6 | 9 | gap |
| spacer | ok | 2 | 2 | gap |
| spinner | fixed | 5 | 6 | gap |
| split-button | fixed | 4 | 7 | gap |
| split-view | fixed | 6 | 6 | gap |
| stack | fixed | 3 | 6 | gap |
| state-tile | gap | 1 | 8 | gap |
| status-bar | fixed | 6 | 8 | gap |
| status-indicator | fixed | 4 | 5 | gap |
| surface | fixed | 6 | 5 | gap |
| switch | fixed | 3 | 7 | gap |
| tab-strip | gap | 4 | 8 | gap |
| table | fixed | 4 | 5 | gap |
| tabs | fixed | 11 | 12 | gap |
| text | ok | 4 | 6 | gap |
| text-input | fixed | 8 | 9 | gap |
| text-link | ok | 4 | 8 | gap |
| time-ago | fixed | 4 | 4 | gap |
| time-input | fixed | 4 | 5 | gap |
| time-zone-select | fixed | 6 | 6 | gap |
| toast-host | fixed | 6 | 7 | gap |
| toast-stack | ok | 9 | 11 | gap |
| toggle-group | fixed | 4 | 8 | gap |
| token-input | fixed | 9 | 11 | gap |
| toolbar | fixed | 4 | 5 | gap |
| tooltip | fixed | 3 | 6 | gap |
| tree | ok | 2 | 4 | ok |
| tri-state-switch | fixed | 5 | 6 | gap |
| ui-presentation-provider | fixed | 3 | 2 | gap |
| validation-summary | gap | 8 | 9 | gap |
| video-player | fixed | 7 | 4 | gap |
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
