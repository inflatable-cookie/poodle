# g02.007 Loading, Empty, Error, Notification, And Remediation Depth

Status: completed
Date: 2026-03-11
Owner: Pug Core

## Summary

- completed `g02.007`
- added reusable hardening surfaces at `packages/svelte/primitives/src/Banner.svelte`, `packages/svelte/primitives/src/Skeleton.svelte`, `packages/svelte/composites/src/EmptyState.svelte`, and `packages/svelte/composites/src/ToastStack.svelte`
- extended the preview to use those components across browse, detail, picker, and embed states instead of relying on ad hoc `stateTitle` and `stateMessage` copy alone
- added the transient notification contract at `docs/contracts/composites/toast-stack.md`
- added the normative hardening baseline at `docs/specs/015-loading-empty-error-notification-and-remediation-rules.md`

## Validation

- `bun run preview:build`
- `bun run tokens:build`
- `git diff --check`

## Notes

- this tranche freezes loading/remediation posture without pretending Pug owns retry policy, data fetching, or system notification integrations
- accessibility focus remains on explicit state meaning, adjacent recovery actions, and transient announcement without focus theft

## Next Task

Open `docs/roadmaps/g02/008-command-palette-and-action-discovery-depth.md` and build the next workstation-facing batch above the now-stable hardening layer.
