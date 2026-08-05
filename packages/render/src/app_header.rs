//! AppHeader — the app-level three-region header shell.
//!
//! Contract: `docs/contracts/components/app-header.md`
//! Ported from: `packages/jetstream/components/src/app_header.rs`.
//!
//! Renders the contract §2 three-region shell:
//!   - Identity region (title group: title + optional subtitle, or a custom
//!     identity slot)
//!   - Actions region (optional global-actions slot)
//!   - Utility region (optional trailing utility slot, right-aligned)
//!
//! The contract `grid minmax(0,1fr) auto auto` is emulated with flex —
//! identity grows (`Grow` + min-width 0 for truncation), actions/utility hold
//! intrinsic width (shrink 0), utility justifies to the end.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node,
};
use poodle_specs::AppHeaderSpec;

use crate::color::with_alpha;
use crate::presentation::rem_to_px;

pub fn app_header(
    spec: &AppHeaderSpec,
    theme: &dyn ThemeProvider,
    identity: Option<Node>,
    actions: Option<Node>,
    utility: Option<Node>,
) -> Node {
    // ── Token / contract-rem resolution ──────────────────────────
    let panel = theme.resolve_color(spec.background_token());
    // Contract §9: color-mix(background-panel 94%, transparent) → panel @ 94% alpha.
    let bg = with_alpha(panel, panel.3 * 0.94);
    let border = theme.resolve_color(spec.border_token());
    let title_color = theme.resolve_color(spec.title_color_token());
    let subtitle_color = theme.resolve_color(spec.subtitle_color_token());

    // Size ladder (height + title/subtitle font) and density ladder
    // (region gaps + padding) all carried on the spec.
    let min_height = rem_to_px(spec.min_height_rem());
    let title_font = rem_to_px(spec.title_size_rem());
    let subtitle_font = rem_to_px(spec.subtitle_size_rem());
    let grid_gap = rem_to_px(spec.gap_rem());
    let region_gap = rem_to_px(spec.region_gap_rem());
    let pad_y = rem_to_px(spec.pad_y_rem());
    let pad_x = rem_to_px(spec.pad_x_rem());

    // ── Identity region (grid column 1: minmax(0, 1fr)) ──────────
    // Grows to fill; min-width 0 so the subtitle can truncate.
    let mut identity_region = Node::container();
    {
        let s = &mut identity_region.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = region_gap;
        s.descriptor.layout.width = poodle_node::LayoutSizing::Grow;
        s.min_width = Some(0.0);
    }

    if let Some(custom) = identity {
        identity_region = identity_region.child(custom);
    } else if spec.title.is_some() {
        // Default title group: title + optional subtitle.
        let mut title_group = Node::container();
        {
            let s = &mut title_group.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = region_gap;
            s.min_width = Some(0.0);
        }

        if let Some(ref title) = spec.title {
            let mut t = Node::text(title);
            t.style.descriptor.text_color = Some(title_color);
            t.style.text_size = Some(title_font);
            t.style.text_weight = Some(600);
            t.style.line_height = Some(1.2);
            t.style.no_wrap = true;
            title_group = title_group.child(t);
        }

        if let Some(ref subtitle) = spec.subtitle {
            let mut st = Node::text(subtitle);
            st.style.descriptor.text_color = Some(subtitle_color);
            st.style.text_size = Some(subtitle_font);
            st.style.line_height = Some(1.2);
            st.style.no_wrap = true;
            st.style.text_ellipsis = true;
            title_group = title_group.child(st);
        }

        identity_region = identity_region.child(title_group);
    }

    // ── Root shell ───────────────────────────────────────────────
    // Fill-width matches the contract grid column track; min-height is the
    // contract min-height — intentionally NOT width Grow, which would zero
    // the min-height (CSS min-height:0 trick) and force a vertical stretch
    // the shell must not have.
    let mut header = Node::container();
    {
        let s = &mut header.style;
        s.descriptor.background = Some(bg);
        s.border_bottom_width = Some(1.0);
        s.descriptor.border.color = border;
        s.fill_width = true;
        s.min_height = Some(min_height);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = grid_gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
    }
    let mut header = header.child(identity_region);

    // ── Actions region (grid column 2: auto) ─────────────────────
    if let Some(actions) = actions {
        let mut region = Node::container();
        {
            let s = &mut region.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = region_gap;
            s.flex_shrink_zero = true;
        }
        header = header.child(region.child(actions));
    }

    // ── Utility region (grid column 3: auto, justify-end) ────────
    if let Some(utility) = utility {
        let mut region = Node::container();
        {
            let s = &mut region.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::End;
            s.descriptor.layout.spacing.gap = region_gap;
            s.flex_shrink_zero = true;
        }
        header = header.child(region.child(utility));
    }

    // Contract: `aria-label` falls back to `title`.
    if let Some(label) = spec.aria_label.as_deref().or(spec.title.as_deref()) {
        if !label.is_empty() {
            header.a11y.label = Some(label.to_string());
        }
    }
    header
}
