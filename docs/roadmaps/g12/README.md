# g12 — React Full Parity And Web Verification Depth

**Status: active.** The React parity program (`001`–`008`) is complete;
the generation continues with cross-framework verification depth.

**React parity: COMPLETE.** All 132 components ported to `@poodle/react` and
Playwright-verified against the Svelte preview. The React preview is a full
per-component gallery matching the Svelte preview (shell, controls, Tokens
inspector, usage docs, 131/131 specimen slugs) plus docs / parity /
accessibility report generators. `@poodle/react` has a consumer README
(`packages/react/components/README.md`). Docs and parity data are authored
canonically in the Svelte preview and re-exported live into React — no fork.

Goal: `@poodle/react` grows from the 3-component pilot (g11.007) to full
library parity (132 components), as a strategic second web target — no
consuming app yet, so the acceptance bar is the React preview plus
interaction verification, not a consumer rollout.

Decisions inherited from g11.007:

- Hand-written TSX shells (~90 LOC each) over `@poodle/headless` machines.
  Mitosis rejected twice (spec 062); no compiler layer.
- Tokens, recipes, and the contrast axis are plain CSS — shared unchanged.
- The Svelte implementation remains the visual proof reference.

Runway:

- `001-shared-styles-package.md` — extract every remaining Svelte
  `<style>` block and move all component CSS to `@poodle/styles`, imported
  by both frameworks. Single styling source; React never duplicates CSS.
- `002-react-infra-and-conversion-playbook.md` — React preview harness
  (hash-routed specimens like the Svelte preview), shared type strategy,
  documented conversion recipe.
- `003`–`008` — family batches: primitives/display, controls/forms,
  overlays/navigation, layout/form shells, data/date, media/workstation
  composites.
- `008-parity-verification.md` — preview coverage sweep, Playwright
  behavior parity samples against the Svelte preview, docs. **Complete.**
- `009-visual-regression-gate.md` — cross-framework pixel diff (Svelte vs
  React, same slug, same axes). The structural gates cannot see a
  component render at the wrong size; the ListCard `data-size` bug proved
  it. **Complete** — 256/256 sweep pairs and 180/180 axis pairs green,
  after 14 divergences found and fixed across specimens, shells and
  `@poodle/styles`.
- `010-agent-composer.md` — the agent composer family across all four
  targets: `Meter shape="ring"`, `ModelPicker` (model + host-declared
  capability axes in one popover) and `AgentChatInput`. **Complete.**
- `011-overlay-portalling.md` — every anchored overlay portals to the theme
  root and is positioned in viewport coordinates. Found from g12.010: a
  scrolling ancestor clips a surface whatever its z-index, and a transformed
  one traps even `position: fixed`. 23 components across both web frameworks,
  behind one shared primitive. **Complete.**
- `012-workstation-tier-removal.md` — delete the retired `poodle-workstation`
  spec crate and its GPUI remnants. Six of its thirteen specs duplicated
  `poodle-specs`; the other seven had no component, contract or Svelte
  counterpart on any target. Closes the "Remaining" section g09.006 left open.
  **Complete.**
- `013-native-spec-surface-parity.md` — nothing measured whether a documented
  prop reached `poodle-specs`, so nothing could tell whether GPUI and Jetstream
  had tracked the web. New `contract-spec-drift` gate found 93 real gaps; all
  93 closed, plus a renderer pass so both targets draw what they can now
  reach. **Complete.**
- `014-native-visual-gate.md` — pixel baselines for the GPUI preview, closing
  the hole `g12.009` left on the native side. Found that GPUI *was* runnable
  here all along; the "build-only" note was wrong. Then Jetstream's headless
  offscreen render landed and settled the question: 90s and zero flake, against
  20min and ~3% for window capture. **Complete.**
- `015-native-accessibility-options.md` — `003-native-accessibility.md`
  recorded that neither runtime exposes an accessibility API; this costed the
  way out per engine and then built the half that was tractable. The two were
  never in the same position: gpui 0.2.2 is the latest published version and
  has nothing, while Jetstream ran winit 0.30 against a version-compatible
  `accesskit_winit`, over a `UiTree` that already carried bounds, roles and
  parent/child links. **Jetstream now has a live AccessKit surface** — tree
  projection, adapter, and actions routed through the same handlers as pointer
  input — with 108 Poodle components attaching their `aria_label` to it. GPUI
  still waits on upstream, because the work it needs is the work upstream would
  obsolete. **Complete for Jetstream; GPUI held deliberately.**

- Ongoing: `check:svelte` (svelte-check over `@poodle/svelte`, driven
  through the isolated `install-smoke` consumer) now runs in `ci:web` and
  `ci-web.yml`. Added after five type errors in `Rating.svelte` were found
  from a consumer repo rather than from Poodle's own gates.

## Next Task

`g12.015` is done: Jetstream has accessibility, GPUI is held on purpose. Open
for this generation:

- **59 unnamed controls on Jetstream, measured not estimated.**
  `effigy test:jetstream-a11y` projects every specimen's accessibility tree
  headlessly. It found 629; 570 are closed.

  Most were not missing information — components held the words and discarded
  them. `bulk-action-bar` dropped the `label` its own action struct carried;
  `markdown-editor` bound its tool captions as `_label` and threw them away;
  `switch` and `split-button` never used their visible `label` as the
  accessible name. One engine bug accounted for 186 alone (a button's caption
  living in a child left the button unnamed — now ARIA name-from-content).

  **Three were contract clauses nobody had implemented**, found by reading the
  contracts rather than assuming the residue was caller error: `switch`
  specifies a name composed from `leftLabel`/`rightLabel`, `app-header`
  specifies `aria-label` falling back to `title`, and `number-input` spells its
  steppers "Increment"/"Decrement" where the code had invented
  "Increase"/"Decrease".

  What is left is genuinely caller-supplied: `number-input` 16 (its spec has no
  label prop at all, so only a caller can name it), plus unlabelled switches,
  icon-only dialog actions and raw text inputs. Deciding whether a labelless
  `NumberInput` is valid usage is a contract question, and answering it is what
  takes this to zero and makes the audit CI-able.

- **The AX audit sees one screen.** `effigy test:jetstream-ax` reads the real
  macOS tree (471 elements of ours, 467 named, against GPUI's 7/1) but only for
  whatever the preview is showing. Per-slug is the same shape as the visual
  gate's sweep.
- **Nobody has listened to it.** The tree is correct and named; whether
  VoiceOver's announcements read sensibly is a judgement a machine cannot make.
- **Both visual gates cover one axis only** (`eclipse-compact-sm`).
- **GPUI baselines are display-dependent.** A capture taken on a 1x display is
  1348x1478 where the 2x baseline is 2696x2396, so every baseline fails on size
  after the machine's display setup changes. The gate compares against the
  machine's own last capture, so this is a rebaseline rather than a defect —
  but it is one more thing the offscreen Jetstream gate does not have.
