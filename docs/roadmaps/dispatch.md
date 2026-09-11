# Queue Dispatch Projection

Status: active
Updated: 2026-09-11 (g18.010, g18.015 and g18.016 merged; g18.006 paused; g18.011 held behind g18.013, g18.014, g18.017 and g18.018; g18.009 held)
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

## Active and paused queue tasks

[`g18.006`](g18/006-v040-web-editor-release-and-desktop-unblock.md) candidate
preparation is operator-paused with its worker/workspace and any progress
preserved. Resume the same Queue task `17ac3fee-de90-4b32-9672-1134770bb086`
only after g18.010 and g18.013–g18.018, the held g18.011 sweep,
g18.012, and all other blocking repairs close and the operator accepts the
sweep.

[`g18.009`](g18/009-v040-release-certification-and-desktop-unblock.md) release
certification is already dependency-queued behind g18.006 and is now explicitly
held. Do not release it until the repaired candidate is accepted.

## Ready queue task

[`g18.011`](g18/011-web-editor-ux-acceptance-sweep.md) is explicitly Queue-held
until g18.013→g18.014/g18.018 and parallel g18.017 merge. It sweeps all
three editor surfaces in both web previews and is serial before any g18.006
continuation.

[`g18.012`](g18/012-code-editor-extensible-language-registry.md) is
dependency-queued behind g18.011. It replaces the closed grammar catalogue
with consumer-selected lazy CodeMirror language providers before g18.006.

[`g18.013`](g18/013-rich-text-editor-toolbar-controls.md) is active. It replaces
the link-like command row with proper grouped Poodle controls before g18.011 is
released.

[`g18.014`](g18/014-rich-text-image-policy-specimen-proof.md) is
dependency-queued behind g18.013. It replaces the dead external image fixture
with visible deterministic seeded/insertion proof before g18.011 is released.

[`g18.017`](g18/017-block-slider-fixed-inline-presentation.md) is approved for
immediate dispatch in parallel. It gives the block Slider family rounded-square
corners and keeps single-Slider text fixed inside the track with split-colour
crossover across active runtimes. It must merge before g18.011 is released.

[`g18.018`](g18/018-rich-text-controlled-echo-selection.md) is approved and
dependency-queued behind g18.013 because it touches the same RichTextEditor
shells. After that dependency closes it may run beside g18.014 and g18.017. It
makes accepted controlled echoes preserve caret, selection, history and focus
in both web wrappers and must merge before g18.011 is released.

## Held planning horizons

GPUI repair tranches, GPUI/shared-Rust CodeEditor and RichTextEditor work,
visual expansion, keyboard-origin focus, V2, M2, A2, the Nucleus switch packet,
web-pair extraction, the contributor-guidance pilot, Jetstream, and triage
holds are not yet queue tasks. Their gates remain in `g18/README.md` and
current triage.

## Historical queue evidence

Completed execution remains in queue history, PRs, logs, and generation
roll-ups. Historical queue records keep their original card/batch wording.
