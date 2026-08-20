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
    let app = TestAppContext::single();
    let mut cx = open_route_window(&app, "region");
    settle(&mut cx);
    assert_real_specimen(&mut cx, "region");
}

/// Seam proof 2: an axis page discovers its advertised `Sizes` and
/// `Densities` tabs and opens both panes through real pointer input.
#[test]
fn axis_route_opens_every_advertised_pane() {
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
