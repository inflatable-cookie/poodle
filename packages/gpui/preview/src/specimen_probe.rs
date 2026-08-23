//! Headless native specimen probe (g15.026) — test-only module.
//!
//! Mounts the production [`PreviewRoot`] on GPUI's in-memory test platform
//! (`TestAppContext` / `VisualTestContext`) and walks
//! `component_registry::CANONICAL_COMPONENTS` directly. Every portable
//! catalogue route must paint a real specimen card, never the
//! `missing_specimen` fallback, and every `Sizes` / `Densities` tab the
//! mounted page advertises must open its pane through real pointer input.
//!
//! This is construction evidence only. It claims nothing about visual parity,
//! arbitrary component interaction, teaching quality, or horizontal overflow;
//! those keep their existing owners (`regressions:native`, the screen-clear
//! review children, and the deferred visual lane).
//!
//! Observation uses GPUI's test-only `debug_selector` markers, which compile
//! to no-ops outside `test-support` builds. Nothing here is a published API.
//!
//! One test-platform limitation shapes the sweep: gpui 0.2.2's
//! `Frame::clear` does not clear `debug_bounds`, so selector entries
//! accumulate for the life of a window. Each route therefore gets a fresh
//! window (and a fresh root, which is also the route-state reset); only the
//! `TestAppContext` is reused.

// Explicit imports only: a glob would pull in gpui's `test` proc macro and
// shadow the built-in `#[test]` (gpui-macros 0.2.2 crashes on current rustc).
// Same discipline as `file_pick_tests` in `main.rs`.
use crate::component_registry::CANONICAL_COMPONENTS;
use crate::{specimens, PreviewRoot};
use gpui::{
    div, point, px, size, AnyElement, App, AppContext, Bounds, IntoElement, Modifiers, Pixels,
    Size, TestAppContext, VisualTestContext, WindowBounds, WindowOptions,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// The catalogue's portable-route denominator: 174 canonical entries. The
/// web-only `MeterSurface` (spec 068) is the single native `n/a` and must
/// never join this list.
const EXPECTED_ROUTES: usize = 174;

/// The card's stop condition: the post-compilation sweep body must stay under
/// two minutes so the probe can live inside the QA boards.
const MAX_SWEEP_BODY: Duration = Duration::from_secs(120);

/// Parallel sweep shards; each runs its own test context on its own thread.
const SWEEP_SHARDS: usize = 4;

/// Exclusion between test threads that render node trees.
///
/// `poodle-gpui-node-backend`'s generated element-id counter is a process
/// global, restarted once per rendered frame by `reset_element_ids`. Two test
/// threads rendering at the same time interleave their resets, so a control
/// that declares no id can take a different `ElementId` between a press and
/// its release — and gpui, which keys the pending mouse-down by that id,
/// silently drops the click. The sweep never clicks a node-backed control, so
/// its shards share this lock; a test that does takes it exclusively.
static NODE_TREE_RENDER: RwLock<()> = RwLock::new(());

/// Share the render lock. Poisoning is irrelevant here — the guard carries no
/// data, and a panicking shard has already failed its own test.
fn shared_render_guard() -> std::sync::RwLockReadGuard<'static, ()> {
    NODE_TREE_RENDER.read().unwrap_or_else(|e| e.into_inner())
}

// Test-only debug selectors, mirrored by the markers in `specimens/mod.rs`
// and `specimens/specimen_layout.rs`.
const CARD: &str = "specimen-card";
const MISSING: &str = "specimen-missing";
const EXAMPLES_TAB: &str = "specimen-tab-examples";
const SIZES_TAB: &str = "specimen-tab-sizes";
const DENSITIES_TAB: &str = "specimen-tab-densities";
const SIZES_PANE: &str = "specimen-pane-sizes";
const DENSITIES_PANE: &str = "specimen-pane-densities";

/// The audit's narrow width. The height keeps the page header, the tab strip,
/// and the top of each axis pane inside the virtualized page list's painted
/// range. The probe proves construction at 768px, not that content avoids
/// horizontal overflow.
fn probe_viewport() -> Size<Pixels> {
    size(px(768.0), px(1200.0))
}

/// Open a fresh window already at the probe viewport with the route selected
/// before the first draw, so the window never pays for the catalogue landing
/// page or a resize redraw.
///
/// A fresh window per route is required: gpui 0.2.2's `Frame::clear` does not
/// clear `debug_bounds`, so a reused window would keep reporting an earlier
/// route's tabs. A fresh root is also the route-state reset — no specimen's
/// retained tab or toggle can leak into another route.
fn open_route_window(app: &TestAppContext, slug: &str) -> VisualTestContext {
    let window = app.update(|app| {
        app.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.0), px(0.0)),
                    size: probe_viewport(),
                })),
                ..Default::default()
            },
            |_window, cx| {
                cx.new(|cx| {
                    let mut root = PreviewRoot::new(cx);
                    root.state.active_component_slug = Some(slug.to_string());
                    root
                })
            },
        )
        .expect("probe window opens")
    });
    VisualTestContext::from_window(window.into(), app)
}

/// Let queued work land, then flush once more so any state it dirtied is
/// drawn into the rendered frame before assertions read it.
fn settle(cx: &mut VisualTestContext) {
    cx.run_until_parked();
    cx.update(|_window, _app| {});
}

/// The route painted a real specimen card and did not reach the fallback.
fn assert_real_specimen(cx: &mut VisualTestContext, slug: &str) {
    assert!(
        cx.debug_bounds(CARD).is_some(),
        "{slug}: route did not paint a specimen card"
    );
    assert!(
        cx.debug_bounds(MISSING).is_none(),
        "{slug}: route reached the missing_specimen fallback"
    );
}

/// Open every advertised axis tab through GPUI's real pointer event path and
/// assert its pane paints, returning to `Examples` before advancing. Returns
/// the `(sizes, densities)` tab counts discovered on the mounted page.
fn exercise_axis_tabs(cx: &mut VisualTestContext, slug: &str) -> (usize, usize) {
    let mut counts = (0, 0);
    for (tab, pane) in [(SIZES_TAB, SIZES_PANE), (DENSITIES_TAB, DENSITIES_PANE)] {
        let Some(bounds) = cx.debug_bounds(tab) else {
            continue;
        };
        cx.simulate_click(bounds.center(), Modifiers::none());
        settle(cx);
        assert!(
            cx.debug_bounds(pane).is_some(),
            "{slug}: clicking {tab} did not paint {pane}"
        );
        if tab == SIZES_TAB {
            counts.0 += 1;
        } else {
            counts.1 += 1;
        }
        let examples = cx
            .debug_bounds(EXAMPLES_TAB)
            .unwrap_or_else(|| panic!("{slug}: Examples tab missing after {tab}"));
        cx.simulate_click(examples.center(), Modifiers::none());
        settle(cx);
    }
    counts
}

/// Seam proof 1: an ordinary route mounts through the production
/// `PreviewRoot` and paints a real specimen card, not the fallback.
#[test]
fn ordinary_route_constructs_a_real_specimen_card() {
    let _shared = shared_render_guard();
    let app = TestAppContext::single();
    let mut cx = open_route_window(&app, "region");
    settle(&mut cx);
    assert_real_specimen(&mut cx, "region");
}

/// Seam proof 2: an axis page discovers its advertised `Sizes` and
/// `Densities` tabs and opens both panes through real pointer input.
#[test]
fn axis_route_opens_every_advertised_pane() {
    let _shared = shared_render_guard();
    let app = TestAppContext::single();
    let mut cx = open_route_window(&app, "button");
    settle(&mut cx);
    assert_real_specimen(&mut cx, "button");
    assert_eq!(
        exercise_axis_tabs(&mut cx, "button"),
        (1, 1),
        "button advertises one Sizes tab and one Densities tab"
    );
}

/// Test-only host that paints one pre-built element as a window root, so the
/// production dispatcher's fallback arm can be observed through a real frame.
/// The window draws once when it opens; the `take` serves exactly that frame.
struct FallbackHost {
    content: Option<AnyElement>,
}

impl gpui::Render for FallbackHost {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.content.take().unwrap_or_else(|| div().into_any_element())
    }
}

/// Seam proof 3: dispatching a slug the catalogue does not know reaches the
/// `missing_specimen` arm and paints its marker, never a specimen card.
#[test]
fn unknown_dispatch_paints_the_fallback_marker() {
    let _shared = shared_render_guard();
    let mut app = TestAppContext::single();
    let fallback = {
        let (root, cx) = app.add_window_view(|_window, cx| PreviewRoot::new(cx));
        cx.update(|_window, app: &mut App| {
            root.update(app, |root, cx| {
                specimens::render_single_specimen("not-a-catalogue-route", &root.state, cx)
                    .into_any_element()
            })
        })
    };
    let (_host, cx) = app.add_window_view(move |_window, _cx| FallbackHost {
        content: Some(fallback),
    });
    assert!(
        cx.debug_bounds(MISSING).is_some(),
        "unknown dispatch did not paint the missing_specimen fallback"
    );
    assert!(
        cx.debug_bounds(CARD).is_none(),
        "unknown dispatch painted a specimen card"
    );
}

/// The durable sweep, sharded so wall time stays far under the two-minute
/// budget on slower CI machines: each shard walks its contiguous slice of the
/// canonical registry, and every shard re-asserts the 174-entry denominator
/// so a registry change cannot silently shrink coverage. Between them the
/// shards visit every route exactly once.
fn sweep_shard(shard: usize, routes: &'static [crate::component_registry::CanonicalComponent]) {
    assert_eq!(
        CANONICAL_COMPONENTS.len(),
        EXPECTED_ROUTES,
        "the native probe denominator is exactly the 174 portable catalogue \
         entries; a registry change must reconcile the audit, not this number"
    );
    assert!(
        CANONICAL_COMPONENTS
            .iter()
            .all(|component| component.slug != "meter-surface"),
        "MeterSurface is web-only (spec 068) and must stay out of the native denominator"
    );

    let _shared = shared_render_guard();
    let app = TestAppContext::single();
    let started = Instant::now();
    let mut sizes_tabs = 0usize;
    let mut densities_tabs = 0usize;

    for component in routes {
        let slug = component.slug;
        // Last-route marker in captured output, so a panic still names its slug.
        eprintln!("probe: mounting {slug}");
        let mut cx = open_route_window(&app, slug);
        settle(&mut cx);
        assert_real_specimen(&mut cx, slug);
        let (sizes, densities) = exercise_axis_tabs(&mut cx, slug);
        sizes_tabs += sizes;
        densities_tabs += densities;
    }

    let elapsed = started.elapsed();
    eprintln!(
        "probe shard {shard}: {}/{} routes constructed; \
         {sizes_tabs} Sizes tabs and {densities_tabs} Densities tabs opened; \
         test body {:.1}s",
        routes.len(),
        routes.len(),
        elapsed.as_secs_f64()
    );
    assert!(
        elapsed < MAX_SWEEP_BODY,
        "probe shard {shard} exceeded the two-minute test-body budget: {:.1}s",
        elapsed.as_secs_f64()
    );
}

/// Contiguous per-shard slices of the canonical registry.
fn sweep_shards() -> Vec<&'static [crate::component_registry::CanonicalComponent]> {
    let shard_len = EXPECTED_ROUTES.div_ceil(SWEEP_SHARDS);
    CANONICAL_COMPONENTS.chunks(shard_len).collect()
}

/// Sweep shard 1 of 4.
#[test]
fn canonical_catalogue_constructs_every_route_and_axis_pane_1() {
    sweep_shard(1, sweep_shards()[0]);
}

/// Sweep shard 2 of 4.
#[test]
fn canonical_catalogue_constructs_every_route_and_axis_pane_2() {
    sweep_shard(2, sweep_shards()[1]);
}

/// Sweep shard 3 of 4.
#[test]
fn canonical_catalogue_constructs_every_route_and_axis_pane_3() {
    sweep_shard(3, sweep_shards()[2]);
}

/// Sweep shard 4 of 4.
#[test]
fn canonical_catalogue_constructs_every_route_and_axis_pane_4() {
    sweep_shard(4, sweep_shards()[3]);
}

// ── g15.042 Stepper adapter and specimen seam ──────────────────────────────

/// Open a route in a window tall enough for the whole page to paint, and keep
/// the root entity so retained specimen state can be read back.
///
/// `debug_bounds` cannot answer that question here: gpui 0.2.2 never clears
/// `debug_bounds` between frames, so a selector keyed by the current value
/// would still report the value it held three clicks ago. The retained map is
/// the thing under test anyway.
fn open_stateful_route_window(
    app: &TestAppContext,
    slug: &str,
    height: f32,
) -> (gpui::Entity<PreviewRoot>, VisualTestContext) {
    let holder: Rc<RefCell<Option<gpui::Entity<PreviewRoot>>>> = Rc::default();
    let captured = Rc::clone(&holder);
    let window = app.update(|app| {
        app.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.0), px(0.0)),
                    size: size(px(768.0), px(height)),
                })),
                ..Default::default()
            },
            move |_window, cx| {
                let root = cx.new(|cx| {
                    let mut root = PreviewRoot::new(cx);
                    root.state.active_component_slug = Some(slug.to_string());
                    root
                });
                *captured.borrow_mut() = Some(root.clone());
                root
            },
        )
        .expect("stateful probe window opens")
    });
    let root = holder.borrow_mut().take().expect("root entity captured");
    (root, VisualTestContext::from_window(window.into(), app))
}

/// Seam proof 4 (g15.042): the Stepper page's live controls run through the
/// preview adapter's builders and land in retained specimen state.
///
/// The mounted regressions in `tests/headless_regressions.rs` drive
/// `poodle_render::stepper` directly, so they prove the shared renderer and
/// the backend and would stay green if `node_compat::Stepper::on_change` or
/// `on_rerun` were replaced with a no-op body. This one would not: every
/// click here travels the specimen's `Stepper::from_spec(..).on_change(..)`
/// builders, `IntoElement`, the node backend, and the specimen event queue
/// before it becomes retained text. It is the original defect, stated as a
/// test.
#[test]
fn stepper_route_selection_and_rerun_run_through_the_preview_adapter() {
    use crate::specimens::stepper::{RERUN_MARKER, WIZARD_MARKER};
    use poodle_render::presentation::rem_to_px;
    use poodle_specs::StepperSpec;

    // Exclusive: this is the one probe that clicks controls the shared render
    // gives no id, so no other thread may restart the id counter mid-click.
    let _exclusive = NODE_TREE_RENDER
        .write()
        .unwrap_or_else(|e| e.into_inner());
    let app = TestAppContext::single();
    let (root, mut cx) = open_stateful_route_window(&app, "stepper", 3200.0);
    settle(&mut cx);
    assert_real_specimen(&mut cx, "stepper");

    let retained = |cx: &mut VisualTestContext, key: &str| -> Option<String> {
        let key = key.to_string();
        cx.update(|_window, app| root.read(app).state.specimens.text.get(&key).cloned())
    };

    assert_eq!(
        retained(&mut cx, "stepper-current"),
        None,
        "the page opens on its declared default, with nothing retained yet"
    );

    let sizing = StepperSpec::new(Vec::new());
    let theme = poodle_gpui::GpuiThemeProvider::new();
    let ctx = poodle_render::context::RenderContext::new(&theme);
    let marker = rem_to_px(sizing.marker_size_rem(ctx.base_size(sizing.size)));
    // Trailing inset of a step's rerun control: the contract's inline padding
    // plus half the marker square it is sized by (`stepper.md` §8).
    let inset =
        rem_to_px(sizing.padding_inline_rem(ctx.resolve_density(sizing.density))) + marker / 2.0;

    // Every click resets the virtualized page's measurements, so bounds are
    // re-read each time rather than carried across a repaint.
    let bounds = |cx: &mut VisualTestContext, marker: &'static str| {
        cx.debug_bounds(marker)
            .unwrap_or_else(|| panic!("{marker} paints"))
    };

    // The wizard's four steps share the track evenly, so the first step is
    // the first quarter of the mounted control.
    let wizard = bounds(&mut cx, WIZARD_MARKER);
    let column = f32::from(wizard.size.width) / 4.0;
    cx.simulate_click(
        point(wizard.origin.x + px(column * 0.5), wizard.center().y),
        Modifiers::none(),
    );
    settle(&mut cx);
    assert_eq!(
        retained(&mut cx, "stepper-current").as_deref(),
        Some("state"),
        "clicking the first step selected it — `on_change` reached the host"
    );

    // The last step is disabled, so the fourth quarter must change nothing.
    let wizard = bounds(&mut cx, WIZARD_MARKER);
    cx.simulate_click(
        point(wizard.origin.x + px(column * 3.5), wizard.center().y),
        Modifiers::none(),
    );
    settle(&mut cx);
    assert_eq!(
        retained(&mut cx, "stepper-current").as_deref(),
        Some("state"),
        "the disabled step is not a control, even with the handler wired"
    );

    // Two equal columns, both complete, so both carry a rerun control. The
    // second step's trigger sits well inside its own column.
    let group = bounds(&mut cx, RERUN_MARKER);
    let half = f32::from(group.size.width) / 2.0;
    cx.simulate_click(
        point(group.origin.x + px(half + inset), group.center().y),
        Modifiers::none(),
    );
    settle(&mut cx);
    assert_eq!(
        retained(&mut cx, "stepper-rerun-current").as_deref(),
        Some("extract"),
        "selecting the second step moved that group's current step"
    );
    assert_eq!(
        retained(&mut cx, "stepper-rerun-last"),
        None,
        "and re-ran nothing"
    );

    // The first step's rerun sits at that column's trailing edge. It is a
    // fixed square, so it rides the top of its row rather than filling it —
    // the row's mid-line misses it entirely.
    let group = bounds(&mut cx, RERUN_MARKER);
    let half = f32::from(group.size.width) / 2.0;
    cx.simulate_click(
        point(
            group.origin.x + px(half - inset),
            group.origin.y + px(1.0 + marker / 2.0),
        ),
        Modifiers::none(),
    );
    settle(&mut cx);
    assert_eq!(
        retained(&mut cx, "stepper-rerun-last").as_deref(),
        Some("read"),
        "the rerun control recorded its own step — `on_rerun` reached the host"
    );
    assert_eq!(
        retained(&mut cx, "stepper-rerun-current").as_deref(),
        Some("extract"),
        "and re-running left the current step where selection put it"
    );
}
