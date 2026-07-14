# g12.008 React Parity Verification

Status: in progress (2026-07-14)
Owner: Poodle core
Depends on: `g12.007`

## Full-suite verification re-run (2026-07-14)

All 33 headless Playwright suites re-run as one pass against the React
preview (vite, port 4180) at 131/132 converted. **Zero failures, zero
page errors** across ~380 checks:

| Suite | Checks | Suite | Checks |
| --- | --- | --- | --- |
| harness | 3 | listcard | 15 |
| batch3 (primitives) | 17 | cmdchrome | 20 |
| wave2 / 3a / 3b / 4 | 8 / 3 / 8 / 7 | navchrome | 22 |
| controls / controls2 | 4 / 5 | cardstiles | 28 |
| forms | 5 | toolbars | 35 |
| orderby | 6 | pickers (shells) | 30 |
| overlays 1 / 2 / 3 | 7 / 6 / 4 | heavypickers | 28 |
| misc | 2 | media | 30 |
| data 1 / 2 / 3 | 8 / 3 / 2 | editors | 18 |
| dtp (date pickers) | 1 | workstation | 27 |
| dt (DataTable) | 6 | sweep | 28 |
| el / loglist / tree | 3 / 2 / 7 | | |

Scripts live in the session scratchpad (`scratchpad/verify/react_*.ts`);
they drive the same hash-routed specimens as the Svelte preview.

## Open items to 132/132 + closeout

- [x] **DockRegion** — ported (with the full Tabs upgrade it required).
  30/30 probes + Tabs-consumer regression. **132/132.**
- [x] **SegmentedControl option icons** — icon/iconOnly mirrored in the
  React type + component; verified in the dock-region suite.
- [ ] **ListCard anchor `data-size`** — suspected Svelte bug (chrome
  role on the anchor variant), replicated in React for pixel parity.
  Fix both together if confirmed (noted in 006).
- [x] **React preview gallery** — full per-component gallery matching
  the Svelte preview: app shell (theme/density/size/contrast controls +
  URL sync), tag-grouped SidebarNav, catalogue landing, per-component
  page (hero + specimen + import), SpecimenLayout Examples/Sizes/
  Densities tabs. All 129 specimen files ported (131/131 slugs) so both
  apps route identically at `#components/<slug>` for side-by-side visual
  diffing. Bulk port via an 18-agent orchestration pass; typecheck
  clean; 128/131 render with zero console errors (the 3 others emit only
  benign warnings — see the gallery commit). Deferred vs the Svelte
  preview: Tokens section, component-docs/UsageDocs, and the
  accessibility.ts/parity.ts report generators.
- [ ] **ListCard anchor `data-size`** — suspected Svelte bug (chrome
  role on the anchor variant), replicated in React for pixel parity.
  Fix both together if confirmed (noted in 006).
- [x] **Gallery closeout (deferred items done)**:
  - Removed the detached probe harness (`harness.tsx` + 34 batch
    specimens) now that the per-component gallery supersedes it.
  - Tokens section ported (top-nav Components/Tokens tabs, live
    semantic-token value cards, searchable inspector).
  - Per-component usage docs (props/slots/events tables) on every
    component page, from `component-docs.ts` copied verbatim.
  - Report generators: `docs:export` (React's own component-docs.json),
    `parity:report` + `accessibility:report` — React-pathed artifacts
    that reuse the canonical cross-runtime contract-parity data
    (imported from the Svelte preview, no fork) with a
    `frameworks: [@poodle/react, @poodle/svelte]` field. `bun run
    reports` runs all three.
- [ ] Docs closeout: README/consumer notes for `@poodle/react`,
  update g12 README runway.
