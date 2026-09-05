# g16.110 — `gpui-unofficial` Feasibility Spike

Status: complete — report only; spike branch unmerged
Date: 2026-09-05
Card: `docs/roadmaps/g16/110-gpui-unofficial-feasibility-spike.md`
Base: `da8c9c37a2a5fd43d7767434fccc5dfceceb81e6` (`origin/main` at dispatch)
Workspace: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-110-gpui-unofficial-feasibility-spike`
Spike branch: `spike/gpui-unofficial` (pushed, not merged)
Target: `gpui-unofficial = "1.19.0-pre"` (crates.io; no stable `1.19.x` during the spike; latest stable remains `1.18.1`)
Time box: two worker days. This run finished in one.

## Recommendation

**Adopt later.** Do not pin production to `1.19.0-pre`.

AccessKit is real and maps from the existing `poodle-node` record without a
vocabulary change. The compile delta on the node backend is small. Three
blockers keep this off `main`:

1. `cargo deny check licenses` is red on `bzip2-1.0.6` (`libbz2-rs-sys` via
   `http-client-gpui-unofficial`). Not GPL. No `deny.toml` exception was added.
2. Live `Application::new()` is gone. The replacement is
   `gpui-platform-gpui-unofficial`, whose macOS backend
   (`gpui-apple-gpui-unofficial`) cannot **build** from crates.io: `build.rs`
   looks for sibling `../gpui-unofficial`, and cargo extracts
   `gpui-unofficial-1.19.0-pre`.
3. The in-memory test platform never activates AccessKit
   (`TestWindow::a11y_init` is the trait no-op). `debug_a11y_tree_json()` is
   `None`. Headless tree-read proof cannot execute.

A real migration card waits on a stable `1.19.x`, a crates.io-buildable
`gpui-apple` (or an upstream `Application` constructor that does not need it),
and an operator decision on `bzip2-1.0.6`. Estimated size of that card: one
substantial native lane — mechanical API port is 1–2 days; overlay/focus
behaviour plus live-app restore is extra.

Do not reopen the fork-free macOS adapter on this evidence. The AccessKit API
is the right target once the packaging and licence gates are green.

## 1. Compile delta

No vendoring, `[patch]`, git sources, or `deny.toml` edits.

**Files (non-lock):** `packages/gpui/node-backend/{Cargo.toml, src/a11y.rs (new, 151 lines), src/lib.rs, src/style.rs, src/interaction.rs, src/layers.rs, src/tooltip.rs, src/tracked_scroll.rs, src/input_text.rs, src/drag.rs}`, `packages/gpui/preview/{Cargo.toml, src/main.rs, src/headless_driver.rs, src/specimens/{mod,region,embed_input_specimen}.rs, src/bin/window_capture/{transport,focus_evidence}.rs, tests/headless_regressions.rs}`, `test/consumer-dual-dependency/{run.ts, consumer/Cargo.toml.template, consumer/src/main.rs}`. Locks rewritten for the 1.19.0-pre graph.

`packages/gpui/adapter` has no `gpui` dependency. `packages/jetstream/*` does not compile against gpui. `poodle-node` / `poodle-render` were not changed.

### API categories

**Crate split.** Package is `gpui-unofficial`; `[lib] name = "gpui"`, so `use gpui::*` stays. Headless `TestAppContext` does not need platform siblings. Live `Application` does: `gpui_platform::application()` from `gpui-platform-gpui-unofficial`, which pulls `gpui-macos` / `gpui-apple` / `gpui-wgpu`. Those crates **resolve** from crates.io; `gpui-apple` does not **compile** from the registry layout. The preview `main` on this branch is a documented `exit(2)` stub for that reason. Not a `poodle-node` vocabulary change.

**Renames / signature growth (mechanical).**

| 0.2.2 | 1.19.0-pre |
| --- | --- |
| `flex_grow()` | `flex_grow(1.0)` |
| `FocusHandle::focus(window)` | `focus(window, cx)` |
| `Window::focus_next/prev/blur()` | take `&mut App` |
| `on_window_closed(\|app\|)` | `(\|app, WindowId\|)` |
| `AsyncApp::update` → `Result` | returns `R`; panics if the app is gone |
| `ScrollHandle::max_offset().height` | `max_offset()` is `Point`; use `.y` |
| `ShapedLine::paint(origin, height, window, cx)` | also `TextAlign`, `Option<Pixels>` align width |
| `KeyDownEvent { keystroke, is_held }` | plus `prefer_character_input` |

**Removed.** `Application::new()`. Design decision, not a rename: the live preview cannot start without the platform facade.

**Added.** `BoxShadow.inset`. Drop shadows set `inset: false`. The existing inset-shadow painter is unchanged (no behaviour migration on this spike). AccessKit fluent API on `StatefulInteractiveElement`: `.role`, `.aria_label`, `.aria_toggled`, `.aria_numeric_value` / min / max, `.aria_value`, `.aria_selected`, `.aria_expanded`, `.aria_orientation`, `.aria_level`, `.on_a11y_action`. An element needs `.id()` and a non-`GenericContainer` role to appear in the tree.

**Behaviour (headless).** Three `regressions:native` failures, all focus/overlay/tooltip:

- `a_nested_popover_paints_without_nesting_deferred_draws` — outer dismiss does not restore the matching trigger
- `select_two_instances_search_pointer_and_dismiss_through_mounted_rebuilds` — innermost close does not restore that instance trigger
- `tabs_show_tooltips_delay_and_hide_through_mounted_gpui` — removing the hovered tab does not cancel the pending show

Window capture was not run (windowed). Text-input compile path only: `line.paint` signature.

**Design vs rename.** Live `Application` construction. Whether to keep the custom inset-shadow painter now that `BoxShadow.inset` exists (not done here). Relationship fields (`controls`, `labelled_by`, `described_by`) and `invalid`/`busy` have no fluent GPUI counterpart — reported, not patched into `poodle-node`.

## 2. Test results

Selectors from the card, spike vs `main` at dispatch (`da8c9c37a`). No windowed selectors.

| Selector | Spike | Notes |
| --- | --- | --- |
| `effigy check:gpui` | pass | preview `cargo check` + `poodle-render` tests + node-backend 53 tests |
| `effigy gpui:test` | pass | 134 adapter tests |
| `effigy regressions:native` | **201 pass, 3 fail** | 203 on `main`; plus one new AccessKit test that passes. Failures listed above |
| `effigy probe:gpui-specimens` | **8 pass, 1 fail** | shard 3 constructed 44/44 routes then exceeded the two-minute body budget at 122.5s. Known wall-clock papercut, not a construction miss |
| `effigy drift:gpui-consumer-identity` | pass | re-pointed to `gpui-unofficial = "1.19.0-pre"` |
| `cargo deny check licenses` | **fail** | `bzip2-1.0.6` only |
| `cargo deny check advisories sources` | pass | no git sources |

## 3. Licence

`cargo deny --manifest-path packages/gpui/node-backend/Cargo.toml check licenses` against the unmodified root `deny.toml`:

```text
error[rejected]: failed to satisfy license requirements
   ┌─ .../libbz2-rs-sys-0.2.5/Cargo.toml:27:12
   │
27 │ license = "bzip2-1.0.6"
   │            ━━━━━━━━━━━
   │            rejected: license is not explicitly allowed
   ├ libbz2-rs-sys v0.2.5
     └── bzip2 v0.6.1
         └── compression-codecs v0.4.39
             └── async-compression v0.4.44
                 └── http-client-gpui-unofficial v1.19.0-pre
                     └── gpui-unofficial v1.19.0-pre
                         └── poodle-gpui-node-backend v0.3.0
licenses FAILED
```

No GPL crate resolves at 1.19.0-pre. `ztracing-gpui-unofficial`, `zlog-gpui-unofficial`, and `ztracing-macro-gpui-unofficial` are Apache-2.0 on crates.io. All unofficial crates in the node-backend lock are `1.19.0-pre` from `registry+https://github.com/rust-lang/crates.io-index`. `allow-git = []` holds. No exceptions added.

`http-client-gpui-unofficial` is a required dep of `gpui-unofficial`. There is no feature to drop it without forking.

## 4. AccessKit proof

Projection lives in `packages/gpui/node-backend/src/a11y.rs`. `NodeRole` maps 1:1 onto `accesskit::Role` for Checkbox / Slider / Tab / TabList / TabPanel. `NodeToggled` maps to `Toggled`. Click is registered with `.on_a11y_action(AccessibleAction::Click, …)` when `on_activate` is present.

**Tests**

- `a11y::tests::checkbox_slider_and_tabs_roles_map` — pass
- `a11y::tests::a11y_role_forces_element_state` — pass
- `accesskit_projects_checkbox_slider_and_tabs_and_test_platform_builds_no_tree` — pass. Conversion records `accessibility.projection.applied`. Painted snapshots carry the three roles and labels. `Window::is_a11y_active() == false` and `debug_a11y_tree_json() == None`.

Tree-read invariant is **not** satisfied: the test platform never builds the AccessKit tree. Action round-trip through `handle_a11y_action` is `pub(crate)` and has no public test dispatch. Pointer `on_activate` still works on other regressions; that is not an a11y action.

## 5. Portfolio identity

Longhorn GPUI prototypes and a future Nucleus shell must declare the **package** `gpui-unofficial`, not crates.io `gpui = "0.2.2"`. Lib name stays `gpui`, so `use gpui::*` is unchanged.

```toml
gpui-unofficial = "1.19.x"   # once a stable 1.19 exists
# or, equivalent identity:
gpui = { package = "gpui-unofficial", version = "1.19.x" }
```

Declaring both `gpui = "0.2.2"` and `gpui-unofficial` in one crate is two identities. The v0.2.1 class of defect.

`drift:gpui-consumer-identity` re-points without weakening: the consumer still declares the package itself, forbids `[patch]`/`[replace]`, compiles across the boundary, asserts exactly one `gpui-unofficial` from the registry, asserts crates.io `gpui` is absent, keeps the tinyvec `std` shape, and keeps the negative type-mismatch control.

## 6. Continuity

Read, not run: `https://github.com/iamnbutler/gpui-unofficial` `xtask`.

Commands: `transform --zed-tag <tag>`, `publish [--dry-run]`, `bump-version`, `patch-only`, `list-crates`, `verify --tag`. Transform clones the Zed tag, copies the publish-order crates, renames packages (`gpui` → `gpui-unofficial`, others `{kebab}-gpui-unofficial`), sets `[lib] name = "gpui"`, rewrites workspace/git deps to crates.io versions, vendors out-of-crate `include_bytes!` assets, and writes `transform-metadata.json`. Publish walks the same order.

If the upstream pipeline stopped, Poodle would run that `xtask` against a Zed tag in a throwaway clone and `publish --dry-run` first. Do not vendor the result into this repo. `gpui-apple`'s `../gpui-unofficial` rewrite is the packaging bug that already blocks a crates.io consumer; a Poodle-run transform would hit the same registry-layout issue unless that path is fixed upstream (for example `CARGO_MANIFEST_DIR` of the `gpui-unofficial` crate via the `gpui` package, not a sibling directory).

## 7. Remaining work (time box)

Not done, not in scope to paper over:

- Live preview / window-capture `Application` restore, blocked on `gpui-apple` crates.io build
- Overlay dismiss focus-restore and tooltip-cancel regressions (3 tests)
- Headless AccessKit tree read / `on_a11y_action` dispatch
- Operator decision on `bzip2-1.0.6`
- Wait for stable `1.19.x`

## Review oracle

| Invariant | Result |
| --- | --- |
| Nothing vendored or git-sourced | hold — `cargo metadata` / lock `source = registry+` only |
| Licence claim is real | GPL-clean; `bzip2-1.0.6` fails deny with no exception |
| AccessKit proof is executed | projection applied; tree unread on the test platform |
| Main is untouched by spike code | this report PR only |
| Time box respected | report in one worker day; remaining work listed |
