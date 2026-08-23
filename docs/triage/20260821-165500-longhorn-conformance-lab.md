# Longhorn-Backed Poodle Conformance Lab

Status: open — the `g15.044`–`g15.047` Button comparator lane is complete;
full app ownership and process lifecycle remain a post-release decision
Captured: 2026-08-21
Operator idea: build a small Longhorn-backed Tauri app that agents can control
over MCP to compare Poodle runtimes without stealing focus or pointer control.

## What The Evidence Supports

Longhorn contract 022 already supplies the useful web control plane:

- stateless MCP control of an unfocused Tauri app;
- semantic snapshot/click/type/press/scroll/drag per opted-in webview;
- fresh screenshot composition across UI and child webviews while occluded,
  unfocused, or minimized;
- explicit window sizing and targeting.

That makes one Tauri window with separately labelled Svelte and React child
webviews credible. Both web implementations should ship as precompiled static
assets. Bun is a build tool here, not a runtime to bundle.

Tauri can package an external binary, so a GPUI companion process is also a
credible distribution shape. It is not the solved part: Longhorn explicitly
does not capture genuinely native surfaces, and GPUI cannot render inside a
webview. A sidecar still needs its own headless pixel readback and control
provider.

## Recommended Shape

```text
Conformance Lab controller (Tauri + Longhorn dev agent control)
  ├─ Svelte webview — precompiled assets, semantic target
  ├─ React webview  — precompiled assets, semantic target
  └─ GPUI sidecar   — bundled process, local IPC, returns pixels/receipts
```

The controller selects a named fixture, theme, viewport, size/density, and
visual state, then asks each runtime adapter to render. It may display captures
side-by-side and expose them through MCP. It does not define component props,
behavior, or completion.

## Boundaries That Prevent A Third Failed Architecture

- Poodle contracts remain semantic authority; focused runtime tests remain
  behavior evidence.
- Named fixtures are bounded diagnostic identities with hand-written runtime
  adapters, not a universal component/scene schema.
- The lab is internal tooling outside Poodle's published package graph.
- Poodle packages do not depend on Longhorn. Preferred ownership is a small
  dedicated internal lab repository; a Longhorn example is the second choice.
- The GPUI sidecar is accepted only if it returns deterministic pixels without
  a visible/focused window.
- The app consumes the comparator after the first primitive batch works; it is
  not required to invent or validate the comparator.

## Open Decisions

- Dedicated `poodle-conformance-lab` repository or Longhorn-owned example?
- Does the GPUI offscreen seam proved by `g15.044` support a long-running
  sidecar, or should the controller invoke one capture process per fixture?
- Is MCP control needed inside GPUI, or is typed local IPC plus returned
  screenshot/geometry evidence enough?
- Should the first UI show three captures or Svelte↔React and web↔GPUI
  comparisons separately to reflect their different tolerances?

## Promotion Route

1. Complete `g15.044` and, if successful, `g15.045`.
2. Land the first named fixture/comparison batch through `g15.046`–`g15.047`.
3. Decide ownership and process lifecycle from measured use, then write a
   separate lab architecture/card. Do not block v0.2.0 on the full app.
