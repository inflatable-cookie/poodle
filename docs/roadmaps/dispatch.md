# Queue Dispatch Projection

Status: active
Updated: 2026-09-11 (g18.006 paused; g18.010–g18.013 editor acceptance lane approved; g18.009 held)
Planning authority: [`g18/README.md`](g18/README.md)

This file is the control-plane projection of the generation README's approved
frontier. It cannot make a task ready, create dependencies, or preserve a
second roadmap. The coordinator dispatches only the exact task named below and
verifies that this file and the generation README agree at the same pushed
commit.

## Completed queue tasks

[`g18.005`](g18/005-v040-release-preflight.md) — v0.4.0 release preflight —
merged as `93e165fd072aea27f44ede5a794042948654b265` (PR #238) on 2026-09-11
after exact-head re-review and green rust/web checks on the post-g18.007 head.

[`g18.007`](g18/007-ordinary-changelog-maintenance-scope.md) — ordinary
changelog maintenance scope — merged as `ef2e46bb949a766e844e48f071119c9576c6f723`
(PR #240) on 2026-09-10; it unblocked retained g18.005 without a CI exception.

[`g18.008`](g18/008-web-editor-preview-specimens.md) — web editor preview
specimens — merged as `998b6ddc69f94e405b515f6bddd682a2e8916ea5`
(PR #241) on 2026-09-10. It was serial before g18.006 release.

## Active and paused queue tasks

[`g18.006`](g18/006-v040-web-editor-release-and-desktop-unblock.md) candidate
preparation is operator-paused with its worker/workspace and any progress
preserved. Resume the same Queue task `17ac3fee-de90-4b32-9672-1134770bb086`
only after g18.010 and g18.013, the held g18.011 sweep, g18.012, and all other
blocking repairs close and the operator accepts the sweep.

[`g18.009`](g18/009-v040-release-certification-and-desktop-unblock.md) release
certification is already dependency-queued behind g18.006 and is now explicitly
held. Do not release it until the repaired candidate is accepted.

## Ready queue task

[`g18.010`](g18/010-code-editor-editing-focus-treatment.md) corrects the outer
focus ring during active editing in both web wrappers. It is approved for
immediate dispatch.

[`g18.011`](g18/011-web-editor-ux-acceptance-sweep.md) is dependency-queued
behind g18.010 and explicitly Queue-held until parallel g18.013 also merges. It
sweeps all three editor surfaces in both web previews and is serial before any
g18.006 continuation.

[`g18.012`](g18/012-code-editor-extensible-language-registry.md) is
dependency-queued behind g18.011. It replaces the closed grammar catalogue
with consumer-selected lazy CodeMirror language providers before g18.006.

[`g18.013`](g18/013-rich-text-editor-toolbar-controls.md) is approved for
immediate dispatch in parallel with g18.010. It replaces the link-like command
row with proper grouped Poodle controls before g18.011 is released.

## Held planning horizons

GPUI repair tranches, GPUI/shared-Rust CodeEditor and RichTextEditor work,
visual expansion, keyboard-origin focus, V2, M2, A2, the Nucleus switch packet,
web-pair extraction, the contributor-guidance pilot, Jetstream, and triage
holds are not yet queue tasks. Their gates remain in `g18/README.md` and
current triage.

## Historical queue evidence

Completed execution remains in queue history, PRs, logs, and generation
roll-ups. Historical queue records keep their original card/batch wording.
