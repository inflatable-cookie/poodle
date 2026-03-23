# g07.013 — GPUI Downstream Reference-App Adoption Proof

Status: Completed
Updated: 2026-03-14

## Objective

Demonstrate downstream consumption of the `poodle-gpui` crate by creating a
reference application that validates the multi-app adoption pattern.

## Deliverables

### Reference app example (examples/reference_app.rs)

Executable example that demonstrates:

1. **Adapter creation** — `GpuiAdapter::new(GpuiThemeProvider::default())`
2. **Token resolution** — `theme.resolve_color()`, `theme.resolve_space()`
3. **Spec construction** — building specs from `poodle-primitives`, `poodle-composites`, `poodle-workstation`
4. **Rendering** — calling `adapter.render(&spec, &style, theme)` to get element handles
5. **Screen composition** — assembling multiple rendered components into screens
6. **Demo app integration** — calling `demo_app::render_all_screens()` for full coverage

### Integration patterns documented

| Pattern | Evidence |
|---------|----------|
| Single-adapter setup | `GpuiAdapter::new()` with default theme |
| Token resolution | `resolve_color()`, `resolve_space()` return typed values |
| Cross-layer rendering | Primitives, composites, and workstation specs all render |
| Multi-screen composition | 6 screens, 69 components total |
| Manifest introspection | `name()`, `supported_components()`, `unsupported_components()` |

## Verification

```
cargo run --example reference_app — runs successfully, outputs 69 rendered components
cargo test — 145 tests passing (no regression)
```
