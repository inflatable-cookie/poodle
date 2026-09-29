# Product Guardrails

Status: active
Updated: 2026-09-04
Owner: Poodle core

- Keep Poodle focused on reusable tokens, primitives, composites, workstation
  shells, and cross-runtime contracts.
- Keep product-specific DAW, admin, and Jetstream-local behavior in its owning
  application or runtime.
- Treat contracts as behavioral authority. Preview specimens and screenshots
  are evidence, not proof by themselves.
- Change observable behavior through the shared contract before changing one
  renderer.
- Put shared web behavior and styling in `poodle-core`; put shared native
  component recipes in `poodle-render`.
- Consumers, including Underlay and its applications, import Poodle's
  published packages directly and own any translation; Poodle carries no
  consumer-named adapter or directory.
- Keep Bits Svelte and runtime engines as implementation details rather than
  public Poodle contracts.
- Freeze a bounded owner before widening a migration or parity tranche.
- Record intentional runtime differences and validate them against the contract.
- Poodle's internal class names are not public API. A consumer's `:global`
  override of Poodle internals marks an unmet need; Poodle closes it with a
  contracted prop or token rather than keeping the selector stable (operator
  ruling 2026-09-29).
- The default icon set carries generic application affordances, including
  undo, redo, history and pin (operator ruling 2026-09-29).
