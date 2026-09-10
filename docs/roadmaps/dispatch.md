# Queue Dispatch Projection

Status: active
Owner: Chatterbox
Updated: 2026-09-10 (g18.003 executing; g18.004 promoted behind dependency)
Planning authority: [`g18/README.md`](g18/README.md)

This file is the control-plane projection of the generation README's approved
frontier. It cannot make a task ready, create dependencies, or preserve a
second roadmap. The coordinator dispatches only the exact task named below and
verifies that this file and the generation README agree at the same pushed
commit.

## Ready queue task

[`g18.004`](g18/004-tabs-card-inactive-surfaces.md) — Tabs card inactive
surfaces. Sole approved next task. Dispatch waits for g18.003 closeout and a
separate operator go; planning promotion alone is not execution authority. No
successor auto-starts.

## Active queue task

[`g18.003`](g18/003-tiptap-prosemirror-rich-text-editor.md) —
TipTap/ProseMirror rich-text editor, Queue task
`edb2d303-38c9-44a8-aa2c-a6ff5821643e`. Queue owns its worker, review, merge,
and closeout.

## Held planning horizons

[`g18.005`](g18/005-v040-web-editor-release-and-desktop-unblock.md) is planned
behind g18.003 and g18.004. It cannot dispatch until final-source recheck and
explicit operator release authority.

GPUI repair tranches, GPUI/shared-Rust CodeEditor and RichTextEditor work,
visual expansion, keyboard-origin focus, V2, M2, A2, the Nucleus switch packet,
web-pair extraction, the contributor-guidance pilot, Jetstream, and triage
holds are not yet queue tasks. Their gates remain in `g18/README.md` and
current triage.

## Historical queue evidence

Completed execution remains in queue history, PRs, logs, and generation
roll-ups. Historical queue records keep their original card/batch wording.
