# Queue Dispatch Projection

Status: active
Owner: Chatterbox
Updated: 2026-09-10 (g18.007 and g18.008 approved in parallel)
Planning authority: [`g18/README.md`](g18/README.md)

This file is the control-plane projection of the generation README's approved
frontier. It cannot make a task ready, create dependencies, or preserve a
second roadmap. The coordinator dispatches only the exact task named below and
verifies that this file and the generation README agree at the same pushed
commit.

## Ready queue tasks

[`g18.005`](g18/005-v040-release-preflight.md) — v0.4.0 release preflight — is
implemented and reviewed in retained PR #238 but fails ordinary-scope CI.

[`g18.007`](g18/007-ordinary-changelog-maintenance-scope.md) — ordinary
changelog maintenance scope — is reviewing in PR #240. After its merge, Queue
retries validation on the existing g18.005 task and PR.

[`g18.008`](g18/008-web-editor-preview-specimens.md) — web editor preview
specimens — is approved for immediate independent dispatch alongside g18.007.
It is serial before g18.006 release.

## Held planning horizons

[`g18.006`](g18/006-v040-web-editor-release-and-desktop-unblock.md) is planned
behind g18.005 and g18.008. It cannot dispatch until final-source recheck and
explicit operator release authority.

GPUI repair tranches, GPUI/shared-Rust CodeEditor and RichTextEditor work,
visual expansion, keyboard-origin focus, V2, M2, A2, the Nucleus switch packet,
web-pair extraction, the contributor-guidance pilot, Jetstream, and triage
holds are not yet queue tasks. Their gates remain in `g18/README.md` and
current triage.

## Historical queue evidence

Completed execution remains in queue history, PRs, logs, and generation
roll-ups. Historical queue records keep their original card/batch wording.
