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

- [ ] **DockRegion** — last component. Blocked on the in-progress
  Svelte edit (tabVariant prop, uncommitted). Port once landed.
- [ ] **SegmentedControl option icons** — in-progress Svelte edit adds
  `icon`/`iconOnly` to `SegmentedControlOption`. Mirror in the React
  type + component when it lands (interface-invariance sync).
- [ ] **ListCard anchor `data-size`** — suspected Svelte bug (chrome
  role on the anchor variant), replicated in React for pixel parity.
  Fix both together if confirmed (noted in 006).
- [ ] Docs closeout: README/consumer notes for `@poodle/react`,
  update g12 README runway.
