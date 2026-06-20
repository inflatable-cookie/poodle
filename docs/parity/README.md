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

<!-- BEGIN MATRIX (compiled from per-component files; do not hand-edit) -->
_Not yet compiled. Run the status compiler after the analysis wave._
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
