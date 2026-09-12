# Queue Dispatch Projection

Status: active
Updated: 2026-09-12 (g18.023 and g18.024 operator-approved in parallel; both precede release)
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

[`g18.020`](g18/020-rich-text-heading-mode-select.md) — RichTextEditor heading
mode select — merged as `d8ad00b848acf62cc51357fc0caa6dbd559ff7c9` (PR #251) on
2026-09-11 after exact-head independent review and green rust/web checks at the
reviewed head.

[`g18.021`](g18/021-code-editor-token-bound-syntax-presentation.md) — CodeEditor
token-bound syntax presentation — merged as `e69512038a4f032ecfad398562bbab28d40e9ffc` (PR #252) on
2026-09-11 after exact-head independent review and green rust/web checks at the
reviewed head.

[`g18.011`](g18/011-web-editor-ux-acceptance-sweep.md) — web editor UX
acceptance sweep — merged as `211ec0cb707eba62eadc5b33d1bb54d3605c5239` (PR #253) on
2026-09-12 after exact-head independent review (PR comment `5641275365`,
`ready_to_merge`) at `09f298dce257c1414a6d07ddb74c6442e9cbacb0` with green
rust/web checks at the reviewed head. Zero unresolved release-blocking
findings; one retained non-blocking follow-up (F12, preview-harness owner).
Operator acceptance is still required before g18.006 resumes.

[`g18.022`](g18/022-block-first-slider-family.md) — block-first Slider family — merged as `02ab7f7ec9122d85364beca77d05d681fa4d0124` (PR #254) on
2026-09-12 after exact-head independent review (PR comment `5644429883`,
`ready_to_merge`) at `7ed0daf093789177b8dbd67268f95c0a7151bed2` with green
rust/web checks at the reviewed head.

## Active and paused queue tasks

[`g18.006`](g18/006-v040-web-editor-release-and-desktop-unblock.md) candidate
preparation is operator-paused with its worker/workspace and any progress
preserved. Resume the same Queue task `17ac3fee-de90-4b32-9672-1134770bb086`
only after operator acceptance of merged g18.011 and both g18.023/g18.024
close.

[`g18.009`](g18/009-v040-release-certification-and-desktop-unblock.md) release
certification is dependency-queued behind g18.006 with no manual hold. It
dispatches only after the repaired candidate closes.

## Active and dependency-queued product tasks

[`g18.011`](g18/011-web-editor-ux-acceptance-sweep.md) is merged (PR #253,
merge `211ec0cb707eba62eadc5b33d1bb54d3605c5239`). Queue task
`aad6b776-1c3e-438c-bc9c-4e8ba8750462` is closed with the four-surface sweep
complete, zero unresolved release-blocking findings, and one retained
non-blocking follow-up (F12, preview-harness owner). Operator acceptance of
the sweep capsule is the gate for resuming retained g18.006.

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

[`g18.020`](g18/020-rich-text-heading-mode-select.md) is merged (PR #251).
It replaced fixed H1–H3 buttons with one consumer-configurable Normal/H1–H6
Select and extended real document support through H6. The heading-mode leg of
g18.011 is unblocked.

[`g18.021`](g18/021-code-editor-token-bound-syntax-presentation.md) is merged (PR #252).
It added internal token-bound CodeMirror syntax presentation without bundling
grammars or widening the public extension API. The syntax-presentation leg of
g18.011 is unblocked.

[`g18.022`](g18/022-block-first-slider-family.md) is merged (PR #254,
merge `02ab7f7ec9122d85364beca77d05d681fa4d0124`). Queue task
`28f7942c-6586-497a-8d18-045602f654df` is closed with block the default and
embedded the sole alternate for both Slider controls, the old presentation
vocabulary removed, RangeSlider fixed endpoint/center text, vertical block
parity, and proven block Slider bipolar geometry. The breaking pre-v1
migration is classified in g18.006 release evidence on resume.

[`g18.023`](g18/023-code-editor-dual-syntax-palettes.md) is operator-approved
for Queue dispatch. It replaces g18.021’s accent/status mapping with dedicated
dark/light syntax ramps and sparse theme overrides. It may run beside g18.024
but must merge before retained g18.006 resumes.

[`g18.024`](g18/024-slider-family-layout-and-vertical-repair.md) is
operator-approved for Queue dispatch. It repairs the merged block family’s
shared-size alignment, layout-neutral 44×44 targets, step-aware visible values,
and vertical block geometry. It may run beside g18.023 but must merge before
retained g18.006 resumes.

## Held planning horizons

GPUI repair tranches, GPUI/shared-Rust CodeEditor and RichTextEditor work,
visual expansion, keyboard-origin focus, V2, M2, A2, the Nucleus switch packet,
web-pair extraction, the contributor-guidance pilot, Jetstream, and triage
holds are not yet queue tasks. Their gates remain in `g18/README.md` and
current triage.

## Historical queue evidence

Completed execution remains in queue history, PRs, logs, and generation
roll-ups. Historical queue records keep their original card/batch wording.
