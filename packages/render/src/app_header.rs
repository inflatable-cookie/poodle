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
//! An optional centre region (`center` node) is the presence-driven layout
//! switch (contract §8, g13-b017): when present, the contract grid becomes
//! the symmetric `minmax(0,1fr) auto minmax(0,1fr)` and actions/utility
//! share a trailing Grow column, justified to the end. Without it the
//! default `minmax(0,1fr) auto auto` emulation applies unchanged.
//!
//! The contract grid is emulated with flex — identity grows (`Grow` +
//! min-width 0 for truncation), actions/utility hold intrinsic width
//! (shrink 0), utility justifies to the end.

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node};
use poodle_specs::AppHeaderSpec;

use crate::color::with_alpha;
use crate::presentation::rem_to_px;

/// A region container: row, centered cross-axis, region gap, never shrinking
/// below natural size. `justify_end` packs the region's items to the end
/// (the utility posture; the trailing column group uses its own gap).
fn region_container(region_gap: f32, justify_end: bool) -> Node {
    let mut region = Node::container();
    {
        let s = &mut region.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = if justify_end {
            MainAxisAlignment::End
        } else {
            MainAxisAlignment::Start
        };
        s.descriptor.layout.spacing.gap = region_gap;
        s.flex_shrink_zero = true;
    }
    region
}

pub fn app_header(
    spec: &AppHeaderSpec,
    theme: &dyn ThemeProvider,
    identity: Option<Node>,
    center: Option<Node>,
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

    // Presence is captured before the node is consumed by its region block.
    let centered = center.is_some();

    // ── Center region (grid column 2: auto) ─────────────────────
    if let Some(center) = center {
        let region = region_container(region_gap, false);
        header = header.child(region.child(center));
    }

    // ── Trailing column ─────────────────────────────────────────
    // With a centre region, actions + utility share the third column
    // (`minmax(0, 1fr)` — symmetric with identity, which is what keeps the
    // centre truly centred). They are grouped in one Grow container that
    // justifies to the end; without a centre region each stays a direct
    // grid child exactly as before.
    if centered {
        let mut trailing = Node::container();
        {
            let s = &mut trailing.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::End;
            s.descriptor.layout.spacing.gap = grid_gap;
            s.descriptor.layout.width = LayoutSizing::Grow;
            s.min_width = Some(0.0);
        }
        let mut trailing = trailing;
        if let Some(actions) = actions {
            let region = region_container(region_gap, false);
            trailing = trailing.child(region.child(actions));
        }
        if let Some(utility) = utility {
            let region = region_container(region_gap, true);
            trailing = trailing.child(region.child(utility));
        }
        header = header.child(trailing);
    } else {
        // ── Actions region (grid column 2: auto) ────────────────
        if let Some(actions) = actions {
            let region = region_container(region_gap, false);
            header = header.child(region.child(actions));
        }

        // ── Utility region (grid column 3: auto, justify-end) ───
        if let Some(utility) = utility {
            let region = region_container(region_gap, true);
            header = header.child(region.child(utility));
        }
    }

    // Contract: `aria-label` falls back to `title`.
    if let Some(label) = spec.aria_label.as_deref().or(spec.title.as_deref()) {
        if !label.is_empty() {
            header.a11y.label = Some(label.to_string());
        }
    }
    header
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::NodeKind;
    use poodle_specs::ControlSize;

    /// The real token resolver over the ECLIPSE theme. Pure — no backend.
    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn header(
        center: bool,
        actions: bool,
        utility: bool,
    ) -> Node {
        app_header(
            &AppHeaderSpec::new()
                .with_title("Finch")
                .with_center(center),
            &theme(),
            None,
            center.then(|| Node::text("centre")),
            actions.then(|| Node::text("action")),
            utility.then(|| Node::text("utility")),
        )
    }

    #[test]
    fn default_layout_keeps_actions_and_utility_as_flat_region_children() {
        // Ruling 3: without a centre region the trailing actions/utility stay
        // direct grid children — no wrapper, no layout change. The third child
        // is the utility region itself (Fit, justify-end), not a Grow group.
        let node = header(false, true, true);
        assert_eq!(node.children.len(), 3, "identity + actions + utility");
        let utility_region = &node.children[2];
        assert!(
            !matches!(
                utility_region.style.descriptor.layout.width,
                LayoutSizing::Grow
            ),
            "utility region must not grow without a centre region"
        );
        assert_eq!(
            utility_region.style.descriptor.layout.alignment.main,
            MainAxisAlignment::End
        );
        assert_eq!(
            utility_region.style.descriptor.layout.spacing.gap,
            rem_to_px(AppHeaderSpec::new().region_gap_rem())
        );
        assert_eq!(utility_region.children.len(), 1);
        assert!(matches!(
            &utility_region.children[0].kind,
            NodeKind::Text { content } if content == "utility"
        ));

        // No centre region anywhere in the tree.
        assert!(!node.texts().iter().any(|t| *t == "centre"));
    }

    #[test]
    fn centered_layout_groups_actions_and_utility_into_a_trailing_column() {
        // Ruling 2: with a centre region the grid is symmetric — identity and
        // the trailing column both grow, so the centre stays centred; actions
        // and utility share the trailing column, justified to the end.
        let node = header(true, true, true);
        assert_eq!(node.children.len(), 3, "identity + centre + trailing");

        // Centre region sits between identity and the trailing column.
        assert!(matches!(
            &node.children[1].children[0].kind,
            NodeKind::Text { content } if content == "centre"
        ));

        let trailing = &node.children[2];
        assert!(
            matches!(
                trailing.style.descriptor.layout.width,
                LayoutSizing::Grow
            ),
            "trailing column must grow symmetrically with identity"
        );
        assert_eq!(
            trailing.style.descriptor.layout.alignment.main,
            MainAxisAlignment::End,
            "trailing group justifies to the end"
        );
        // The inter-region gap is preserved inside the group.
        assert_eq!(
            trailing.style.descriptor.layout.spacing.gap,
            rem_to_px(AppHeaderSpec::new().gap_rem())
        );
        assert_eq!(trailing.children.len(), 2, "actions + utility inside");
        assert!(matches!(
            &trailing.children[0].children[0].kind,
            NodeKind::Text { content } if content == "action"
        ));
        assert!(matches!(
            &trailing.children[1].children[0].kind,
            NodeKind::Text { content } if content == "utility"
        ));
    }

    #[test]
    fn centered_layout_without_actions_or_utility_still_emits_the_trailing_column() {
        // The trailing column exists whenever the centre region is present so
        // the symmetric grow split (and therefore the centring) holds even with
        // an empty trailing side.
        let node = header(true, false, false);
        assert_eq!(node.children.len(), 3);
        assert!(node.children[2].children.is_empty());
        assert!(matches!(
            node.children[2].style.descriptor.layout.width,
            LayoutSizing::Grow
        ));
    }

    #[test]
    fn size_resolution_and_label_are_unchanged_by_the_centre_region() {
        // The centre region must not disturb the size/density ladders or the
        // aria-label fallback (ruling 3: no pixel shift without a centre).
        let spec = AppHeaderSpec::new()
            .with_title("Finch")
            .with_size(ControlSize::Lg)
            .with_center(true);
        let node = app_header(
            &spec,
            &theme(),
            None,
            Some(Node::text("centre")),
            None,
            None,
        );
        assert_eq!(node.style.min_height, Some(rem_to_px(spec.min_height_rem())));
        assert_eq!(node.a11y.label.as_deref(), Some("Finch"));
    }
}
