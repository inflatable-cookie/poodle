# g13.004 Shared Preview Shell Scene Pilot

Status: complete — closed by card 036 (`db4e4510` + `thread/g13-036-shell-scene-native-shells`): all four shells read the one scene; the native shells consume the self-contained Rust artifact
Owner: Poodle core
Depends on: `g13.003`

## Objective

Define the preview shell once and render equivalent navigation and display
controls across Svelte, React, GPUI, and Jetstream.

## Deliverables

- Scene definitions for navigation, search, theme selection, size, density,
  contrast, specimen tabs, and content framing.
- Thin runtime hosts for local state, routing, focus, and native controls.
- The same theme selector and axis vocabulary in all four previews.
- Interaction and visual fixtures for shell state changes.

## Acceptance

- All four previews expose the same shell capabilities and labels.
- Theme, size, density, and tab changes are interactive in every runtime.
- Runtime hosts contain capability glue, not copied shell composition.

## Next

`g13.005` adds the first complete component definition and specimen.
