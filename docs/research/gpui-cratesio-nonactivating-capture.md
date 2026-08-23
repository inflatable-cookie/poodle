# GPUI crates.io non-activating capture

Status: decision prototype complete — **go for non-activating window capture;
no-go for true offscreen capture on stock GPUI 0.2.2**
Created: 2026-08-23
Promotes to: `../roadmaps/g16/005-gpui-cratesio-recovery.md`
Corrects: `gpui-offscreen-capture-feasibility.md` as a public-dependency
decision; that earlier proof remains technically valid for the forked graph

## Question

Can Poodle restore crates.io `gpui = "0.2.2"` and retain useful native pixel
evidence without the capture process taking focus?

This is narrower than true offscreen rendering. Stock GPUI 0.2.2 exposes no
scene readback or headless renderer. Its test window discards the scene. A
downstream crate cannot obtain genuine GPUI pixels without a real platform
window or a modified GPUI source.

## Prototype

A disposable Rust harness depended only on crates.io `gpui = "=0.2.2"`. It:

- opened one 420 × 120 GPUI window with `focus: false` and `show: true`;
- never called `App::activate`;
- sampled the frontmost macOS application every 50 ms for the full run;
- found its window through `CGWindowListCopyWindowInfo`;
- captured that exact window with `screencapture -x -l`;
- exited after the capture.

The harness and PNG were deliberately discarded after the decision run. They
were throwaway transport evidence, not a visual baseline.

## Result

```text
baseline=T3 Code (Nightly)
observed={"T3 Code (Nightly)"}
window_id=35113
capture_success=true
capture_path=/tmp/poodle-gpui-022-nonactivating.png
```

The captured PNG was valid 976 × 442 RGBA. The foreground application did not
change in any sample. Compilation plus the run completed in 39 seconds from a
fresh disposable crate; most of that was the first crates.io GPUI build.

## Verdict

The practical focus problem has a crates.io-only solution. Poodle can render a
real GPUI window without activating it and capture the window by id. This is
not true offscreen rendering and must not be named or gated as such.

The public dependency decision is therefore:

- restore every public Poodle GPUI package to crates.io `gpui = "0.2.2"`;
- remove the Zed fork and `gpui_platform` from the public graph;
- retain in-memory GPUI construction and interaction checks as the default
  headless evidence;
- make real GPUI pixel capture an explicit, non-activating, window-server
  tool outside default QA and CI;
- never let optional capture capability choose the dependency source exposed
  to consumers again.

## Limits

- The probe used one simple rendered window, not the 18-case Button fixture
  runner.
- Window capture still needs a macOS window server and Screen Recording
  permission.
- A window exists and may be visible even though it does not take focus.
- The implementation must prove that repeated fixture capture does not call
  activation APIs or raise the window over the operator's work.
- If literal no-window GPUI pixels become mandatory, stock 0.2.2 cannot supply
  them. That would require either upstream support or a separately approved
  modified GPUI source.
