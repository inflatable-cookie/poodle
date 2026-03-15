# g08.003 — Structural Primitives

Status: Completed
Updated: 2026-03-14

## Objective

Implement `RenderComponent` for 8 structural primitive specs in the Jetstream
adapter.

## Deliverables

### RenderComponent implementations (render_structural.rs)

| Spec | Node ID | Widget | Notes |
|------|---------|--------|-------|
| BoxSpec | `box` | Panel | Basic container |
| StackSpec | `stack` | Panel | Directional layout |
| GridSpec | `grid` | Panel | Emulated with nested panels |
| SurfaceSpec | `surface` | Panel | Themed container |
| SeparatorSpec | `separator` | Panel | Visual divider |
| ScrollShellSpec | `scroll-shell` | List | Scrollable container |
| BannerSpec | `banner` | Panel | Alert container |
| CallOutSpec | `callout` | Panel | Informational container |

### Test coverage

8 tests verifying spec_type and widget_kind propagation.

## Verification

```
cargo test — 8 structural tests passing
```
