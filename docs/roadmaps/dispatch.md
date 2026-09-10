# Queue Dispatch Projection

Status: active
Owner: Chatterbox
Updated: 2026-09-10 (g18.003 promoted; separate operator go required)
Planning authority: [`g18/README.md`](g18/README.md)

This file is the control-plane projection of the generation README's approved
frontier. It cannot make a task ready, create dependencies, or preserve a
second roadmap. The coordinator dispatches only the exact task named below and
verifies that this file and the generation README agree at the same pushed
commit.

## Ready queue task

[`g18.003`](g18/003-tiptap-prosemirror-rich-text-editor.md) —
TipTap/ProseMirror rich-text editor. Sole approved ready task. Dispatch still
requires a separate operator go; planning promotion alone is not execution
authority. No successor auto-starts.

## Held planning horizons

GPUI repair tranches, GPUI/shared-Rust CodeEditor and RichTextEditor work,
visual expansion, keyboard-origin focus, V2, M2, A2, the Nucleus switch packet,
web-pair extraction, the contributor-guidance pilot, Jetstream, and triage
holds are not yet queue tasks. Their gates remain in `g18/README.md` and
current triage.

## Historical queue evidence

Completed execution remains in queue history, PRs, logs, and generation
roll-ups. Historical queue records keep their original card/batch wording.
