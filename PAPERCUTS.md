# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

<!-- Keep entries short. Append newest entries at the top. Do not include secrets. -->

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

- 2026-08-07 — The in-process click driver (`--click`) posts mouse-down and
  mouse-up inside a single frame, so it cannot catch a bug that only appears
  when a click spans frames — which every real click does. It passed the
  node-backend id-stability bug (see g12.019) in both the broken and fixed
  states. Give it an optional inter-event delay, or a separate down/up pair, so
  a press that outlives a rebuild is exercised.
