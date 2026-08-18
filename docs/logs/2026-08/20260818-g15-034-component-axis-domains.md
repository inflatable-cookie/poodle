# g15.034 — Component-Specific Specimen Axis Domains (August batch log)

Date: 2026-08-18
Card: `docs/roadmaps/g15/034-component-specific-specimen-axis-domains.md`
Handoff: `docs/handoffs/20260818-215524-g15-034-component-axis-domains.md`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-b2503b72`
Branch: `t3code/component-axis-domains`

## Summary

Clean pre-v1.0 breaking migration: EmptyState and Icon public surfaces are
truthful in every active runtime; specimen shells accept explicit ordered axis
domains; generated scenes pass authored `sizeAxis` / `densityAxis` through; and
paired-web plus narrow GPUI evidence prove every advertised value renders.

## Package change class

- **Change class:** `breaking` on the pre-1.0 preview channel
- **Packages:** `@inflatable-cookie/poodle-svelte`, `@inflatable-cookie/poodle-react`,
  `poodle-specs`, `poodle-render`, `poodle-gpui-preview` consumers
- **Public entry points:**
  - Removed `Icon` `density` prop (Svelte/React) and `IconSpec::density` / `with_density` (Rust)
  - Removed `EmptyStateSpec::compact` / `with_compact`; use `with_size(EmptyStateSize::Compact)`
  - Expanded `IconSize` to `Xs | Sm | Md | Lg | Xl` with 1:1 `ControlSize` mapping (no endpoint collapse)

## Migration notes

- Replace `.with_compact(true)` on `EmptyStateSpec` with `.with_size(EmptyStateSize::Compact)`
- Remove `density` from direct `Icon` callers (e.g. HistoryCenter chevron)
- Update exhaustive `IconSize` matches for the two new variants
- IconButton glyph ladder test expectations shift at `xs` (supporting visual + 1:1 icon tokens)

## Downstream re-check

Command:

```bash
rg -l 'with_compact\(true\)|IconSpec.*with_density|Icon.*density=' ~/Dev/projects --glob '!**/poodle/**'
```

Result: no hits outside Poodle.

## Validation

| Command | Outcome |
|---------|---------|
| `effigy ir:check` | pass |
| `effigy test:components` | 343 files, 2710 tests pass |
| `effigy check:svelte` | 0 errors |
| `effigy react:build` | pass |
| `effigy test:parity` | 257 tests pass (incl. exact-domain assertions) |
| `effigy check:gpui` | pass |
| `effigy regressions:native` | 46 tests pass |
| `effigy test:web-pack-install` | pass |
| `effigy docs:check` | pass |
| `git diff --check origin/main...HEAD` | clean |

## Evidence added

- Paired-web census: EmptyState `default|compact`, Text/Eyebrow `xs|sm|md`, Icon five sizes, no Icon Densities tab
- GPUI: generated empty-state scene `size_axis`, `IconSize` endpoint mapping test
- IR validation: scene size axes union component `size` permitted subsets with control sizes

## Operator preview routes (Svelte + React catalogue)

- `/empty-state`
- `/icon`
- `/text`
- `/eyebrow`

Accept the Sizes rows on each before merge.

## Unresolved / out of scope

- Jetstream selectors not run; mechanical compile consumers updated only where required
- Live GPUI page probe (`g15.026`) not started from this worker
