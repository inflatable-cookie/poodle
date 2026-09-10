# Queue Dispatch Projection

Status: active
Owner: Chatterbox
Updated: 2026-09-10 (g18.002 ready; dispatch held for operator go)
Planning authority: [`g18/README.md`](g18/README.md)

This file is the control-plane projection of the generation README's approved
frontier. It cannot make a task ready, create dependencies, or preserve a
second roadmap. The coordinator dispatches only the exact task named below and
verifies that this file and the generation README agree at the same pushed
commit.

## Ready queue task

[`g18.002`](g18/002-codemirror-web-code-editor.md) is the sole approved ready
task. Dispatch is held until the operator gives a separate queue go. No release,
Desktop adoption, GPUI CodeEditor work, or successor task auto-starts.

## Held planning horizons

GPUI repair tranches, GPUI/shared-Rust CodeEditor work, visual expansion,
keyboard-origin focus, V2, M2, A2, the Nucleus switch packet, web-pair
extraction, the contributor-guidance pilot, Jetstream, and triage holds are not
yet queue tasks. Their gates remain in `g18/README.md` and current triage.

## Historical queue evidence

Completed execution remains in queue history, PRs, logs, and generation
roll-ups. Historical queue records keep their original card/batch wording.
