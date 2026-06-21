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

## Cross-cutting findings (block multiple components)

- **~~Missing `color.status.info` token.~~ RESOLVED.** Added `color.status.info`
  to the token schema (`semantic/color.json` → `primitives.color.blue.500` =
  `#2d86f3` for dark/light; `modes/themes/loophole-studio.json` → `#6ea9d4`),
  regenerated all artifacts (Rust + TS + CSS), and pointed `StatusTone`,
  `ToastTone`, and `PillTone` `Info` at `COLOR_STATUS_INFO` instead of the
  accent-base workaround. This closes the info-tone color todos in
  status-indicator, toast-stack, pill, and callout across both Rust targets.
  (In dark theme `accent.base` is gold `#f0b24d`, so info was rendering gold —
  now correctly blue.)
- **~~GPUI elevation shadows hardcoded as hsla.~~ RESOLVED (GPUI).** The typed
  `ELEVATION_{SURFACE,OVERLAY,DIALOG}` constants are structured `ShadowValue`s, so
  `theme_ext::elevation_{surface,overlay,dialog}_shadow()` convert them to
  `gpui::BoxShadow`. **20 hardcoded-hsla overlay/dialog shadow sites re-wired**
  (popover, menu, hover-card, navigation-menu, select, split-button, color-picker,
  time-zone-select, all date pickers; dialog, drawer, command-palette,
  media-picker, page-loading, picker-shell; surface). Left hardcoded (no matching
  token — would need new schema tokens): card-elevated, tooltip, filter-toolbar
  sticky. Jetstream has no box-shadow (JsEl gap) — N/A.
- **Specs lacking token methods force literals.** Recurring root cause of the
  Rust hardcoded-px/color todos: the `poodle-specs` struct exposes no token
  method for a value (spinner border/gap/opacity, skeleton shimmer stops,
  progress track-mix/gradient/height), so both Rust targets hand-assemble or
  hardcode. Fix at the source — add the token method to the spec, then both
  targets resolve from it. Higher leverage than per-file literal swaps.
- **Hand-rolled mockups instead of composing primitives** (CLAUDE.md "No Mockups"):
  GPUI date-time pickers fabricate calendar-grid overlays; media-browse-panel
  fakes MediaThumbnail/Callout/Button; confirm-action/alert-dialog hand-roll the
  dialog instead of composing Dialog/AlertDialog. These are rebuilds, not tweaks.
- **Missing implementations.** Jetstream: avatar, card-toggle-group, debug-dialog,
  detail-section-group, error-boundary, text, text-link, token-input,
  icon-provider, inline-list-section (+ scroll-shell is a stub). GPUI:
  validation-summary, form-shell. Svelte authority absent: form-shell,
  validation-summary, remediation-banner, inline-remediation, state-tile,
  tab-strip (need a Svelte reference written first, or the contract stands alone).
- **Analysis is recall-biased.** Some todos are borderline-acceptable, e.g.
  `rem_to_px(<exact contract rem>)` is NOT a hardcoded-px violation (it mirrors a
  contract exact-value table). Treat each todo as a candidate to confirm, not a
  guaranteed defect.

## Status matrix

<!-- BEGIN MATRIX (compiled; do not hand-edit) -->
| component | consv | gpui | jet | spec |
|---|---|---|---|---|
| accordion | fixed | 2 | 2 | ok |
| action-discovery-panel | fixed | 2 | 2 | gap |
| alert-dialog | fixed | 2 | 2 | ok |
| app-header | fixed | 2 | 2 | gap |
| audio-player | ok | 2 | 2 | ok |
| avatar | ok | 0 | 0 | ok |
| block-editor | fixed | 1 | 2 | gap |
| box | ok | 0 | 0 | fixed |
| breadcrumbs | fixed | 0 | 0 | ok |
| bulk-action-bar | fixed | 2 | 2 | gap |
| button | fixed | 2 | 2 | ok |
| calendar | fixed | 2 | 2 | ok |
| callout | fixed | 2 | 2 | ok |
| card | fixed | 2 | 2 | ok |
| card-radio-group | fixed | 2 | 2 | gap |
| card-toggle-group | fixed | 2 | 2 | gap |
| checkbox | ok | 1 | 1 | ok |
| code | fixed | 2 | 2 | gap |
| code-input | fixed | 0 | 0 | ok |
| collapse-toggle | fixed | 0 | 1 | gap |
| collapsible | fixed | 2 | 2 | ok |
| color-picker | fixed | 0 | 2 | gap |
| command-palette | fixed | 2 | 2 | gap |
| confirm-action | ok | 0 | 0 | ok |
| context-menu | ok | 0 | 0 | gap |
| data-table | fixed | 1 | 0 | ok |
| date-picker | fixed | 0 | 0 | ok |
| date-range-picker | fixed | 0 | 0 | gap |
| date-time-picker | fixed | 0 | 0 | gap |
| date-time-range-picker | fixed | 1 | 1 | gap |
| date-time-zone-picker | fixed | 2 | 1 | gap |
| debug-dialog | ok | 0 | 1 | gap |
| detail-item | fixed | 0 | 0 | gap |
| detail-section | fixed | 0 | 0 | gap |
| detail-section-group | fixed | 0 | 0 | gap |
| detail-shell | ok | 0 | 1 | ok |
| dialog | ok | 0 | 0 | ok |
| dock-region | ok | 2 | 2 | gap |
| drawer | ok | 0 | 0 | ok |
| duration-input | fixed | 2 | 2 | gap |
| editable-label | ok | 2 | 2 | gap |
| editable-list | fixed | 4 | 3 | gap |
| embed-input | fixed | 1 | 1 | gap |
| embed-preview | fixed | 0 | 0 | gap |
| empty-state | ok | 0 | 0 | ok |
| error-boundary | ok | 2 | 1 | gap |
| eyebrow | ok | 1 | 2 | gap |
| field | ok | 1 | 1 | gap |
| field-set | fixed | 1 | 1 | gap |
| file-upload | ok | 0 | 0 | ok |
| filter-toolbar | fixed | 0 | 0 | ok |
| form-actions | ok | 1 | 1 | gap |
| form-dialog | ok | 0 | 0 | ok |
| form-layout | fixed | 2 | 2 | ok |
| form-shell | gap | 0 | 0 | gap |
| grid | fixed | 0 | 0 | ok |
| hover-card | ok | 1 | 0 | ok |
| icon | ok | 2 | 1 | gap |
| icon-button | fixed | 0 | 0 | gap |
| icon-provider | fixed | 1 | 1 | gap |
| inline-list-section | fixed | 0 | 1 | gap |
| inline-remediation | ok | 1 | 0 | gap |
| list-card | fixed | 1 | 1 | gap |
| list-card-counter | fixed | 2 | 2 | ok |
| list-container | fixed | 0 | 0 | ok |
| list-grid | fixed | 1 | 1 | gap |
| log-list | ok | 1 | 2 | ok |
| markdown-editor | fixed | 2 | 1 | ok |
| media-browse-panel | ok | 0 | 0 | gap |
| media-picker | fixed | 0 | 0 | ok |
| media-preview | fixed | 1 | 1 | ok |
| media-thumbnail | ok | 1 | 1 | ok |
| menu | fixed | 0 | 0 | ok |
| menubar | fixed | 2 | 1 | gap |
| meta-bar | fixed | 0 | 0 | ok |
| meta-item | fixed | 0 | 0 | ok |
| meter | fixed | 0 | 0 | ok |
| metric-tile | ok | 1 | 1 | ok |
| nav-card | ok | 1 | 2 | ok |
| navigation-menu | fixed | 2 | 0 | gap |
| number-input | gap | 1 | 2 | ok |
| order-by | fixed | 2 | 2 | ok |
| page-header | fixed | 1 | 1 | ok |
| page-loading | ok | 0 | 0 | gap |
| pagination | fixed | 0 | 1 | ok |
| pagination-summary | fixed | 2 | 2 | ok |
| password-requirements | ok | 0 | 0 | gap |
| picker-shell | gap | 0 | 0 | gap |
| pill | fixed | 0 | 0 | ok |
| popover | fixed | 0 | 0 | ok |
| progress | fixed | 0 | 1 | ok |
| radio-group | ok | 1 | 1 | ok |
| range-slider | fixed | 1 | 1 | ok |
| rating | ok | 1 | 2 | ok |
| region | ok | 1 | 0 | ok |
| relation-picker | fixed | 2 | 2 | gap |
| remediation-banner | gap | 0 | 0 | ok |
| resize-handle | ok | 1 | 1 | gap |
| scroll-shell | ok | 1 | 0 | gap |
| segmented-control | fixed | 0 | 0 | ok |
| select | ok | 0 | 0 | ok |
| selection-summary | fixed | 0 | 0 | ok |
| separator | ok | 0 | 0 | ok |
| sidebar-nav | fixed | 0 | 1 | ok |
| skeleton | fixed | 0 | 0 | ok |
| slider | fixed | 1 | 1 | ok |
| spacer | ok | 2 | 2 | gap |
| spinner | fixed | 2 | 2 | ok |
| split-button | fixed | 0 | 0 | gap |
| split-view | fixed | 2 | 2 | ok |
| stack | fixed | 2 | 2 | ok |
| state-tile | gap | 0 | 0 | ok |
| status-bar | fixed | 0 | 0 | ok |
| status-indicator | fixed | 0 | 0 | ok |
| surface | fixed | 1 | 1 | ok |
| switch | fixed | 0 | 0 | ok |
| tab-strip | gap | 0 | 0 | gap |
| table | fixed | 0 | 0 | ok |
| tabs | fixed | 2 | 2 | ok |
| text | ok | 0 | 1 | gap |
| text-input | fixed | 2 | 2 | ok |
| text-link | ok | 0 | 1 | gap |
| time-ago | fixed | 2 | 2 | ok |
| time-input | fixed | 0 | 0 | ok |
| time-zone-select | fixed | 0 | 0 | gap |
| toast-host | fixed | 0 | 0 | ok |
| toast-stack | ok | 0 | 3 | ok |
| toggle-group | fixed | 0 | 0 | ok |
| token-input | fixed | 2 | 3 | gap |
| toolbar | fixed | 1 | 1 | gap |
| tooltip | fixed | 0 | 0 | ok |
| tree | ok | 0 | 0 | ok |
| tri-state-switch | fixed | 1 | 2 | ok |
| ui-presentation-provider | fixed | 0 | 0 | gap |
| validation-summary | gap | 0 | 0 | gap |
| video-player | fixed | 0 | 0 | ok |
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
