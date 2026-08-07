# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

<!-- Keep entries short. Append newest entries at the top. Do not include secrets. -->

- 2026-08-07 — `effigy test:jetstream-a11y` reports 151 unnamed `TextInput`
  nodes across 13 specimens (text-input x22, field x17, form-layout x17, …).
  `poodle_render::text_input` names its root only when `spec.aria_label` is set
  and these specimens do not set one. Pre-existing and previously invisible:
  `ci:native` died at `drift:clicks` — whose subject was deleted in `ee704699` —
  long before reaching the audit, so the count silently regressed from zero.
  Now unmasked, and the last thing between `ci:native` and green.

- 2026-08-07 — The GPUI preview's icon set (`packages/gpui/preview/assets/icons/`,
  56 files) has no media transport glyphs, so `play`, `pause`, `volume-2`,
  `volume-x`, `maximize-2` and `minimize-2` resolve to nothing in AudioPlayer
  and VideoPlayer. Both the old tier and the node backend render the empty
  button box, so this is not a migration regression — it predates it. Add the
  six SVGs.

- 2026-08-06 — `effigy docs:check` assumes
  `packages/jetstream/components/Cargo.toml` exists and aborts before linting
  docs when the sibling-backed Jetstream package is absent. Make that release
  manifest check conditional on the package being available.

- 2026-08-06 — `effigy doctor` reports the repo's `isolation` manifest key as
  unsupported, so routine health checks cannot go green on the checked-in
  manifest. Align the manifest schema or update Effigy's accepted config keys.

- 2026-08-06 — The Jetstream visual runner has no focused `--slug` filter, so
  a one-component recipe check renders and compares all 138 specimens. Add the
  same slug selection surface as the native GPUI gate.

- 2026-08-06 — `effigy graph explore` can panic at `snippets.rs:210` while
  truncating a Unicode-bearing result. Truncate on character boundaries so
  normal component-flow queries return a JSON envelope instead of aborting.

- 2026-08-06 — Focused native-visual runs can stay silent for more than 90
  seconds after the batch header. Emit per-slug start/capture progress so a
  slow capture is distinguishable from a hung driver.
