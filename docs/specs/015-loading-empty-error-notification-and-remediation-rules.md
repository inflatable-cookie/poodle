# 015 Loading, Empty, Error, Notification, And Remediation Rules

Status: active
Updated: 2026-03-11
Depends on: `003-accessibility-and-assistive-technology-baseline.md`, `011-browse-shell-filter-search-and-loading-rules.md`, `014-media-preview-embed-and-asset-surface-rules.md`

## Purpose

Freeze the shared hardening rules for loading, empty, error, notification, and remediation behavior so shells, pickers, and media surfaces stop inventing one-off fallback posture.

## Loading Rule

Loading needs two distinct layers:

- structural loading posture that tells users what is happening
- optional decorative skeleton scaffolds that reserve expected layout

Skeletons may support loading.
They may not replace real loading context when the user needs announced state.

## Empty And No-Results Rule

`empty` and `no-results` are not interchangeable.

`empty` means there is no content available for the current destination.
`no-results` means content exists conceptually, but the current query or filter scope excluded it.

These two cases may share visual language.
They must not share the same explanation and remediation copy by default.

## Error And Remediation Rule

Recoverable errors must keep remediation actions adjacent to the affected surface.

At minimum an error state should preserve:

- stable shell or region framing
- textual explanation
- retry, fallback, or alternate path when available

Persistent errors should prefer inline banner-style messaging over transient notifications alone.

## Notification Rule

Notifications split into two classes:

- persistent inline messaging for conditions that remain relevant until resolved or dismissed
- transient toasts for confirmations, warnings, and recoverable failures that should not permanently occupy layout

Transient notifications must not become the only place a user can discover an unresolved persistent problem.

## Accessibility Rule

Both runtimes must preserve:

- explicit loading, empty, no-results, and error meaning
- recovery actions with accessible names
- transient notification announcement without focus theft
- persistent notification urgency that matches actual severity

Svelte should use native regions, text, and button semantics first.
GPUI must recreate equivalent state and announcement meaning in the native accessibility tree and notification surfaces.

## Seed Evidence

- `docs/contracts/composites/empty-state.md`
- `docs/contracts/composites/toast-stack.md`
- `docs/contracts/foundation/banner.md`
- `docs/contracts/foundation/skeleton.md`
- `packages/svelte/composites/src/EmptyState.svelte`
- `packages/svelte/composites/src/ToastStack.svelte`
- `packages/svelte/primitives/src/Banner.svelte`
- `packages/svelte/primitives/src/Skeleton.svelte`
- `packages/svelte/preview/src/App.svelte`

## Next Task

Carry this hardening baseline into `g02.008` and later milestones so command discovery, workspace shells, and accessibility review reuse one explicit remediation and notification posture.
