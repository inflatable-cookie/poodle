# g15.031 — Screen-clear review: foundation content and status

Date: 2026-08-21
Card: `docs/roadmaps/g15/031-review-foundation-content-status.md`
Handoff: `docs/handoffs/20260821-100723-g15-031-review-foundation-content-status.md`
Parent: `docs/roadmaps/g15/027-screen-clear-human-review.md`
PR: pending

## Outcome

Fourth serial screen-clear review child. All nine owned foundation content/status
pages received the human teaching review against the carried rubric — live Svelte
and React routes at 768px and ordinary width, GPUI specimen source, and the
`g15.026` headless construction/axis evidence. **Eight pages keep unchanged, one
page needed bounded Sv/Rc specimen repair.** No contract, public API, component,
shared-CSS, generated catalogue, or infrastructure file moved outside specimen
presentation.

The nine human-teaching verdicts are recorded in the existing audit rows in
`docs/roadmaps/g15/specimen-catalogue-audit.md`; screening `keep` / "no named
defect" text was replaced, not extended with a second table. Mechanical totals
unchanged (all nine remain `keep` at A/A/A).

## Verdict inventory

### Unchanged (8)

| Page | Verdict |
| --- | --- |
| `Code` | keep — block, inline, and no-copy examples teach syntax-highlighted code; copy is clipboard-only; Sv/Rc paired; Gp mirrors web plus renderer-owned inline variants |
| `EmbedPreview` | keep — provider, trusted raw, loading, error, and empty states are distinct; Sv/Rc paired; Gp mirrors the same state set |
| `IconProvider` | keep — provider boundary, custom set, and default fallback without icon-catalogue drift; Sv/Rc paired; Gp explains the compatibility boundary |
| `Pill` | keep — generated Examples cover tone, mono, muted, badge, inherit, accent, and appearance vocabulary; axis panes own size/density |
| `PageLoading` | keep — inline, indeterminate, determinate, and cancellable overlays are live and distinct from Progress/Spinner; Sv/Rc paired; Gp static mirror |
| `Progress` | keep — determinate ladder, indeterminate, and custom max teach bar semantics; Sv/Rc paired; Gp adds label/value-text evidence |
| `Spinner` | keep — ring, CLI grid, and context-tone sections teach standalone indicators; Sv/Rc/Gp aligned with axis panes |
| `StateTile` | keep — one grid shows plain, trend, and sparkline metric tiles; Sv/Rc paired; Gp mirrors |

### Repaired (1, Sv/Rc only)

- **`ErrorBoundary`** — the caught-error child threw on every render, so Reset
  boundary never surfaced recovered content and Throw again looked inert while
  already in the error state. Sv/Rc now use a preview-only crash-once child
  (matching the component test harness pattern) so Reset shows recovered content
  and Throw again re-arms the failure. Focused regression test added. Gp keeps
  static normal/error evidence per the headless native lane.

## Changed routes for operator review

Changed Svelte routes: `error-boundary`
Changed React routes: `error-boundary`
Changed GPUI routes: none

Operator live sign-off on the changed Svelte and React `error-boundary` routes
is **pending**.

## Changed files

- `packages/svelte/preview/src/specimens/ErrorBoundaryCrashOnce.svelte`
- `packages/svelte/preview/src/specimens/ErrorBoundarySpecimen.svelte`
- `packages/react/preview/src/gallery/specimens/ErrorBoundaryCrashOnce.tsx`
- `packages/react/preview/src/gallery/specimens/ErrorBoundarySpecimen.tsx`
- `packages/svelte/preview/test/g15-031-foundation-content-status.test.ts`
- `docs/roadmaps/g15/specimen-catalogue-audit.md` — nine human verdict rows

## Validation

- `bunx vitest run packages/svelte/preview/test/g15-031-foundation-content-status.test.ts` — 1 passed
- `effigy catalogue:check` — passed
- `effigy check:svelte` — passed
- `effigy react:build` — passed
- `effigy docs:check` — passed
- `git diff --check origin/main...HEAD` — clean

Live review used Svelte preview on `http://127.0.0.1:4175` and React preview on
`http://127.0.0.1:4181` with `--strictPort`. No `*-windowed`,
`test:native-visual`, browser screenshot gate, Jetstream, or release selector ran.

## Operator checkpoint

Pending live review of changed `error-boundary` routes in Svelte and React
previews before card closeout.
