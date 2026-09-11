# Queue Dispatch Projection

Status: active
Updated: 2026-09-11 (g18.018 merged; g18.020 converges on g18.011; g18.006 and g18.009 are the final release lanes)
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

[`g18.010`](g18/010-code-editor-editing-focus-treatment.md) — CodeEditor
editing focus treatment — merged as `a71b48573c7253dfd45f35e482b9bbc7432ea0ca`
(PR #242) on 2026-09-11 after exact-head independent re-review and green
rust/web checks at the reviewed head.

[`g18.015`](g18/015-preview-distribution-build-preflight.md) — preview
distribution build preflight — merged as `0cf6073eb2067c4fc4127ec5c318a0f682c5859f`
(PR #243) on 2026-09-11 after exact-head independent review and green
rust/web checks at the reviewed head.

[`g18.016`](g18/016-code-editor-live-line-number-reconfiguration.md) — CodeEditor
live line-number reconfiguration — merged as `5932bd0027cab2cf86c07a7878c3b48193252306`
(PR #244) on 2026-09-11 after exact-head independent review and green
rust/web checks at the reviewed head.

[`g18.013`](g18/013-rich-text-editor-toolbar-controls.md) — RichTextEditor
toolbar controls — merged as `1e11f59d01dc79f47196ee5f5b18e9e10cce5a70`
(PR #245) on 2026-09-11 after exact-head independent review and green
rust/web checks at the reviewed head.

[`g18.019`](g18/019-markdown-renderer.md) — MarkdownRenderer shared
safe/trusted rendering — merged as `cc26dd3c09e12addab7c0b6c5a243f83f33cff94`
(PR #247) on 2026-09-11 after exact-head independent review and green
rust/web checks at the reviewed head.

[`g18.017`](g18/017-block-slider-fixed-inline-presentation.md) — block Slider
fixed inline presentation — merged as `f91be412b60739e96c29a45e9c17969c05b85f49`
(PR #246) on 2026-09-11 after exact-head independent re-review and green
rust/web checks at the reviewed head.
[`g18.014`](g18/014-rich-text-image-policy-specimen-proof.md) — rich-text
image-policy specimen proof — merged as `72c7a9e5e6ce288780c7a2e3e44715949a035221`
(PR #249) on 2026-09-11 after exact-head independent review and green
rust/web checks at the reviewed head.

[`g18.012`](g18/012-code-editor-extensible-language-registry.md) — CodeEditor
extensible language registry — merged as `f011df5e0a97856bfe022569d77faf69a9f75553`
(PR #250) on 2026-09-11 after exact-head independent review and green
rust/web checks at the reviewed head.

[`g18.018`](g18/018-rich-text-controlled-echo-selection.md) — RichTextEditor
controlled-echo selection preservation — merged as
`08e377517af58a2033a145af2fe5c5875fb38215` (PR #248) on 2026-09-11 after
exact-head independent review and green rust/web checks at the reviewed head.

## Active and paused queue tasks

[`g18.006`](g18/006-v040-web-editor-release-and-desktop-unblock.md) candidate
preparation is operator-paused with its worker/workspace and any progress
preserved. Resume the same Queue task `17ac3fee-de90-4b32-9672-1134770bb086`
only after g18.010 and g18.013–g18.020, the dependency-queued g18.011 sweep,
g18.012, and all other blocking repairs close and the operator accepts the
sweep.

[`g18.009`](g18/009-v040-release-certification-and-desktop-unblock.md) release
certification is dependency-queued behind g18.006 with no manual hold. It
dispatches only after the repaired candidate closes.

## Active and dependency-queued product tasks

[`g18.011`](g18/011-web-editor-ux-acceptance-sweep.md) is dependency-queued as
task `aad6b776-1c3e-438c-bc9c-4e8ba8750462` behind merged g18.018 and ready
g18.020 (g18.012 merged as `f011df5e0a97856bfe022569d77faf69a9f75553`, PR #250).
The version-57 dependency mutation made it wait exactly on g18.012 task
`697c0380-bcc4-4433-9bce-a6c77fa0452a` and g18.020 task
`276cd890-5904-4c79-a08f-253c704e4185`. It sweeps all four editor surfaces in
both web previews and is serial before any g18.006 continuation.

[`g18.012`](g18/012-code-editor-extensible-language-registry.md) is merged (PR #250).
It replaced the closed grammar catalogue with consumer-selected lazy CodeMirror
language providers before the sweep; Queue task
`697c0380-bcc4-4433-9bce-a6c77fa0452a` is closed with its former g18.011 edge
satisfied at version 10.

[`g18.013`](g18/013-rich-text-editor-toolbar-controls.md) is merged (PR #245).
It replaced the link-like command row with proper grouped Poodle controls;
g18.014 and g18.018 are unblocked.

[`g18.014`](g18/014-rich-text-image-policy-specimen-proof.md) is merged (PR #249).
It replaced the dead external image fixture with visible deterministic
seeded/insertion proof. The image-policy leg of g18.011 is unblocked.

[`g18.017`](g18/017-block-slider-fixed-inline-presentation.md) is merged (PR #246).
It gave the block Slider family rounded-square corners and kept
single-Slider text fixed inside the track with split-colour crossover across
active runtimes. The Slider leg of g18.011 is unblocked.

[`g18.018`](g18/018-rich-text-controlled-echo-selection.md) is merged (PR #248).
It makes accepted controlled echoes preserve caret, selection, history and
focus in both web wrappers. The controlled-echo leg of g18.011 is unblocked.

[`g18.019`](g18/019-markdown-renderer.md) is merged (PR #247).
It added the paired standalone MarkdownRenderer surfaces, shared one
safe/trusted content path with MarkdownEditor preview, and added both specimen
pages. The MarkdownRenderer leg of g18.011 is unblocked.

[`g18.020`](g18/020-rich-text-heading-mode-select.md) is ready behind merged
g18.018 (PR #248). It replaces fixed H1–H3 buttons with one
consumer-configurable Normal/H1–H6 Select and extends real document support
through H6. Queue task `276cd890-5904-4c79-a08f-253c704e4185` is already a
dependency of existing g18.011 without replacement.

## Held planning horizons

GPUI repair tranches, GPUI/shared-Rust CodeEditor and RichTextEditor work,
visual expansion, keyboard-origin focus, V2, M2, A2, the Nucleus switch packet,
web-pair extraction, the contributor-guidance pilot, Jetstream, and triage
holds are not yet queue tasks. Their gates remain in `g18/README.md` and
current triage.

## Historical queue evidence

Completed execution remains in queue history, PRs, logs, and generation
roll-ups. Historical queue records keep their original card/batch wording.
