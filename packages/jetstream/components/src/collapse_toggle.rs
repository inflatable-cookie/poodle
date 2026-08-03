//! CollapseToggle — Jetstream collapse toggle backed by CollapseToggleSpec.
//!
//! Contract: `docs/contracts/components/collapse-toggle.md`
//! Reference: `packages/svelte/components/src/CollapseToggle.svelte`
//!
//! Anatomy: a single `<button>` (root) wrapping the chevron `Icon`. Per contract
//! §7 the button is a compact inline-flex sized to icon + padding (NOT a fixed
//! control-height square): vertical padding from the size table, horizontal
//! padding from the density `padding-inline`. The chevron name swaps with the
//! collapsed state (JsEl has no rotation — Tier-3 freedom, like the other
//! Jetstream disclosure components). Idle background is transparent; hover sets
//! the surface-hover background + default text color.
//!
//! JsEl gap: there is no `focus`/`outline` primitive, so the contract §6 accent
//! focus ring is not rendered (`.focusable()` marks the node for the runtime's
//! own focus handling). Noted in the parity doc.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::CollapseToggleSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// CollapseToggle — the chevron that collapses a region.
///
/// Mirrors the GPUI target's shape: `from_spec` then `.on_toggle(handler)`.
pub struct CollapseToggle {
    spec: CollapseToggleSpec,
    theme: JetstreamThemeProvider,
    on_toggle: Option<crate::element::ToggleHandler>,
}

impl CollapseToggle {
    pub fn from_spec(spec: CollapseToggleSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_toggle: None,
        }
    }

    /// Fires with the collapsed state the region is moving **to**.
    pub fn on_toggle(mut self, handler: impl Fn(bool) + Send + Sync + 'static) -> Self {
        self.on_toggle = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for CollapseToggle {
    fn into_js_el(self) -> JsEl {
        let el = js_collapse_toggle(&self.spec, &self.theme);

        match (self.spec.is_disabled, self.on_toggle) {
            (false, Some(handler)) => {
                let next = !self.spec.is_collapsed;
                el.on_click(move |_event| handler(next))
            }
            _ => el,
        }
    }
}

pub fn js_collapse_toggle(spec: &CollapseToggleSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // Icon scales with the effective control size (Svelte `<Icon size={resolvedSize}>`).
    let icon_size = resolve_px(theme, spec.icon_size_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let text_color = resolve_color(theme, spec.text_color_token());
    let hover_fill = resolve_color(theme, spec.hover_fill_token());
    let hover_text = resolve_color(theme, spec.text_color_hover_token());

    // Contract §8: vertical padding = size table; horizontal padding = density
    // padding-inline. Density never changes button height.
    let pad_y = rem_to_px(spec.padding_rem());
    let pad_x = rem_to_px(spec.padding_inline_rem());

    let chevron_icon = spec.effective_icon_name();

    let mut el = ui_element::button("")
        // A bare chevron. Which way it points is visual, not announced, so the
        // name says what pressing it does and `aria_expanded` says the state.
        .aria_label("Toggle section")
        .rounded(radius)
        .px(pad_x)
        .py(pad_y)
        .flex_row()
        .items_center()
        .justify_center()
        .line_height(1.0)
        .focusable()
        .child(
            ui_element::icon(chevron_icon)
                .w(icon_size)
                .h(icon_size)
                .text_color(text_color),
        );

    if spec.is_disabled {
        // Contract §8 disabled: 0.4 opacity + cursor default (non-interactive).
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity).cursor_default();
    } else {
        // Contract §8 idle → hover: surface-hover bg + default text color.
        el = el
            .cursor_pointer()
            .hover(move |s| s.bg(hover_fill).text_color(hover_text));
    }

    crate::aria::with_aria_label(el, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{CollapseDirection, ControlDensity, ControlSize};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    /// Expanded points toward `direction`; collapsed flips to the opposite.
    #[test]
    fn chevron_direction_flips_with_collapsed_state() {
        let th = theme();

        let expanded = js_collapse_toggle(
            &CollapseToggleSpec::new().with_direction(CollapseDirection::Left),
            &th,
        );
        let exp_tree = probe(&expanded, 200.0, 200.0);
        assert!(
            exp_tree.has_text("chevron-left"),
            "expanded left toggle must show chevron-left: {:?}",
            exp_tree.texts()
        );

        let collapsed = js_collapse_toggle(
            &CollapseToggleSpec::new()
                .with_direction(CollapseDirection::Left)
                .with_collapsed(true),
            &th,
        );
        let col_tree = probe(&collapsed, 200.0, 200.0);
        assert!(
            col_tree.has_text("chevron-right"),
            "collapsed left toggle must flip to chevron-right: {:?}",
            col_tree.texts()
        );
    }

    /// Each direction renders its own chevron when expanded.
    #[test]
    fn chevron_name_matches_each_direction() {
        let th = theme();
        for (dir, name) in [
            (CollapseDirection::Left, "chevron-left"),
            (CollapseDirection::Right, "chevron-right"),
            (CollapseDirection::Up, "chevron-up"),
            (CollapseDirection::Down, "chevron-down"),
        ] {
            let el = js_collapse_toggle(&CollapseToggleSpec::new().with_direction(dir), &th);
            let tree = probe(&el, 200.0, 200.0);
            assert!(
                tree.has_text(name),
                "{dir:?} toggle must render {name}: {:?}",
                tree.texts()
            );
        }
    }

    /// Comfortable density widens horizontal padding without touching vertical
    /// padding (size/density orthogonality). Asserted on the resolved layout so
    /// it does not depend on the probe root stretching to the viewport.
    #[test]
    fn comfortable_density_widens_inline_padding_only() {
        let th = theme();
        let default_el = js_collapse_toggle(
            &CollapseToggleSpec::new().with_density(ControlDensity::Default),
            &th,
        );
        let comfortable_el = js_collapse_toggle(
            &CollapseToggleSpec::new().with_density(ControlDensity::Comfortable),
            &th,
        );

        let pad = |el: &JsEl| {
            let p = &el.layout.padding;
            (p.left.into_raw().value(), p.top.into_raw().value())
        };
        let (d_x, d_y) = pad(&default_el);
        let (c_x, c_y) = pad(&comfortable_el);

        assert!(
            c_x > d_x,
            "comfortable must widen inline padding: {c_x} vs {d_x}"
        );
        assert!(
            (c_y - d_y).abs() < 0.001,
            "density must not change vertical padding: {c_y} vs {d_y}"
        );
    }

    /// Larger sizes produce more vertical padding (the size-table ladder).
    #[test]
    fn size_scales_vertical_padding() {
        let th = theme();
        let pad_y = |size| {
            let el = js_collapse_toggle(&CollapseToggleSpec::new().with_size(size), &th);
            el.layout.padding.top.into_raw().value()
        };
        assert!(
            pad_y(ControlSize::Xl) > pad_y(ControlSize::Xs),
            "xl must have more vertical padding than xs: {} vs {}",
            pad_y(ControlSize::Xl),
            pad_y(ControlSize::Xs)
        );
    }

    /// Disabled toggle still renders its chevron (non-interactive, reduced opacity).
    #[test]
    fn disabled_still_renders_chevron() {
        let th = theme();
        let el = js_collapse_toggle(&CollapseToggleSpec::new().with_disabled(true), &th);
        let tree = probe(&el, 200.0, 200.0);
        assert!(
            tree.count_kind("Icon") >= 1,
            "disabled toggle must still render the chevron"
        );
    }

    #[test]
    fn a_click_reports_the_next_collapsed_state() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        for (collapsed, expected) in [(false, true), (true, false)] {
            let seen: Arc<Mutex<Vec<bool>>> = Arc::new(Mutex::new(Vec::new()));
            let values = Arc::clone(&seen);

            let el = CollapseToggle::from_spec(
                CollapseToggleSpec::new().with_collapsed(collapsed),
                &theme(),
            )
            .on_toggle(move |next| values.lock().unwrap().push(next))
            .into_js_el();

            crate::element::click_probe::click_at(&el, 80.0, 80.0, 12.0, 12.0);

            assert_eq!(seen.lock().unwrap().as_slice(), [expected]);
        }
    }

    #[test]
    fn a_disabled_collapse_toggle_ignores_clicks() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = CollapseToggle::from_spec(CollapseToggleSpec::new().with_disabled(true), &theme())
            .on_toggle(move |_| {
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .into_js_el();

        crate::element::click_probe::click_at(&el, 80.0, 80.0, 12.0, 12.0);

        assert_eq!(hits.load(Ordering::SeqCst), 0, "a disabled toggle fired");
    }
}
