# g12 — React Full Parity And Web Verification Depth

**Status: active, strict-paused.** The React parity program (`001`–`008`) and
the bounded work through `016` are complete. No implementation card is ready.

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
- `016-public-dock-drag-extension-and-preview-artifact.md` — add the public
  async pre-drag/source lifecycle and external-drop target seam needed by
  desktop hosts, then prove the exact packed Svelte preview artifact outside
  sibling source resolution. **Complete.**
- `017-native-interaction-parity.md` — the "render-only natives" claim in eleven
  contracts was false: GPUI wires clicks in 71 of ~97 components, and 35 of them
  accepted a handler they never read. The agent chat set is now interactive on
  GPUI and `drift:handlers` gates the dead-handler class. Jetstream needs no
  engine work — its runtime dispatches clicks and the preview already feeds
  pointer state — only a decision about handler shape.
  **GPUI done and gated; Jetstream scoped.**

- Ongoing: `check:svelte` (svelte-check over `@poodle/svelte`, driven
  through the isolated `install-smoke` consumer) now runs in `ci:web` and
  `ci-web.yml`. Added after five type errors in `Rating.svelte` were found
  from a consumer repo rather than from Poodle's own gates.

## Next Task

Choose and promote the next bounded owner before implementation resumes. The
accessibility follow-up candidates below remain evidence, not ready work:

- **Accessible names: zero unnamed controls, and gated.** `test:jetstream-a11y`
  projects every specimen's tree headlessly and now runs in `ci:native`. It
  found 629 and all 629 are closed. It lives in `ci:native` rather than
  `ci:rust` because it renders specimens, so it needs the sibling jetstream
  repo — the same constraint that already makes that group local-only.

  Almost none of the 629 were missing information. Components held the words
  and discarded them: `bulk-action-bar` dropped the `label` its own action
  struct carried, `markdown-editor` bound tool captions as `_label`, `dialog`
  never read the `close_label` that had sat on its spec defaulting to "Close
  dialog". One engine bug accounted for 186 alone — a caption living in a
  child left its button unnamed, now ARIA name-from-content.

  **The method was checking Svelte, not reasoning about the residue.** Twice
  the remainder looked like caller error and twice it was a default the
  reference already had: `switch` composing a name from
  `leftLabel`/`rightLabel`, `app-header` falling back to `title`,
  `markdown-editor` defaulting to "Markdown editor", `file-upload` naming each
  remove button after its file, `text-input` naming its clear button "Clear
  search query". `SelectSpec` turned out to have an `aria_label` field with no
  builder to set it.

- **Contract ARIA roles: complete, gated in `ci:native`.** All 124 requirements
  a contract names are projected by their component, bar 10 exempt with a
  recorded reason. `effigy drift:roles` extracts every `role="…"`, maps it onto
  `accesskit::Role`, and checks the tree that component actually renders.

  **The overlay category is now empty, and that is the finding.** It started at
  48 requirements excused as "observable only with an overlay open" — a list I
  wrote by reasoning rather than checking. Every single one was wrong. The
  specimens already rendered those overlays: `select`, `menubar` and
  `context-menu` outright, and `popover`, `drawer`, `hover-card`,
  `command-palette` and the date pickers regardless of their open flag, which
  the visual gate confirmed by showing **no pixel change** when they were
  opened. In each case the component simply never claimed the role, and the
  exemption hid it.

  The category is kept because `specimenRendersOpen` now polices it: an entry
  whose specimen opens the overlay is reported as stale rather than honoured.

  The 10 remaining exemptions each carry a justification, not just a key —
  triggers the consumer composes (`menu`, `hover-card`, `tooltip`), surfaces
  this API does not model (`data-table`'s column menu, `field`'s info popover),
  and states only reachable at runtime (`editable-list`'s alert and status).

  The two gates check each other: the naming gate caught three regressions
  introduced by role work, because giving an element a role that requires a
  name makes it answerable where it was previously ignorable.

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
