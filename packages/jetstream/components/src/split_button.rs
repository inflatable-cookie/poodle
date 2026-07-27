//! SplitButton — Jetstream split button backed by SplitButtonSpec.
//!
//! Contract: `docs/contracts/components/split-button.md`
//! Reference: `packages/svelte/components/src/SplitButton.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values
//! (`rem_to_px` of a contract-exact rem is the token-faithful form here).
//!
//! NOTE: menu open/close, click-outside, and keyboard navigation live in the
//! preview event loop (Jetstream architecture); this builder renders the menu
//! panel from `spec.is_open` only. There is no accessibility channel, so the
//! contract §6 ARIA roles (menu/menuitem/separator, aria-haspopup/expanded) are
//! an accepted delta.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ButtonTone, ButtonVariant, SplitButtonSpec, SplitMenuItem};

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem, split_button_chevron_size_rem,
    split_button_toggle_width_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// Resolved variant × tone color set for the split-button halves, matching the
/// Svelte `--poodle-split-*` custom-property matrix (contract §8).
struct SplitColors {
    fill: Color,
    border: Color,
    text: Color,
}

fn resolve_split_colors(spec: &SplitButtonSpec, theme: &JetstreamThemeProvider) -> SplitColors {
    let surface: Color = resolve_color(theme, "color.background.surface").into();
    let border_subtle: Color = resolve_color(theme, "color.border.subtle").into();
    let border_default: Color = resolve_color(theme, "color.border.default").into();
    let accent: Color = resolve_color(theme, "color.accent.base").into();
    let text_primary: Color = resolve_color(theme, "color.text.primary").into();
    let text_inverse: Color = resolve_color(theme, "color.text.inverse").into();

    // Status family for danger/success/warning tone mixes; Default has no status mix.
    let status: Option<Color> = match spec.tone {
        ButtonTone::Danger => Some(resolve_color(theme, "color.status.danger").into()),
        ButtonTone::Success => Some(resolve_color(theme, "color.status.success").into()),
        ButtonTone::Warning => Some(resolve_color(theme, "color.status.warning").into()),
        ButtonTone::Default => None,
    };

    match (spec.variant, status) {
        // ── Ghost ──────────────────────────────────────────────
        // Danger/success ghost: fully transparent fill+border, status text.
        (ButtonVariant::Ghost, Some(status_color)) => SplitColors {
            fill: Color::TRANSPARENT,
            border: Color::TRANSPARENT,
            text: status_color,
        },
        // Default ghost: color-mix(surface 42%, transparent) fill,
        // color-mix(border-subtle 72%, transparent) border, text-primary.
        (ButtonVariant::Ghost, None) => SplitColors {
            fill: surface.with_alpha(surface.a * 0.42),
            border: border_subtle.with_alpha(border_subtle.a * 0.72),
            text: text_primary,
        },

        // ── Primary ────────────────────────────────────────────
        // Primary danger/success: status fill, color-mix(status 84%, black)
        // border, inverse text.
        (ButtonVariant::Primary, Some(status_color)) => SplitColors {
            fill: status_color,
            border: status_color.mix_srgb(Color::BLACK, 0.84),
            text: text_inverse,
        },
        // Primary default: accent fill, color-mix(accent 84%, black) border,
        // inverse text.
        (ButtonVariant::Primary, None) => SplitColors {
            fill: accent,
            border: accent.mix_srgb(Color::BLACK, 0.84),
            text: text_inverse,
        },

        // ── Secondary (and Danger variant alias) ───────────────
        // Secondary danger/success: color-mix(status 16%, surface) fill,
        // color-mix(status 46%, border-default) border, text-primary.
        (_, Some(status_color)) => SplitColors {
            fill: status_color.mix_srgb(surface, 0.16),
            border: status_color.mix_srgb(border_default, 0.46),
            text: text_primary,
        },
        // Secondary default: surface fill, border-default, text-primary.
        (_, None) => SplitColors {
            fill: surface,
            border: border_default,
            text: text_primary,
        },
    }
}

pub fn js_split_button(spec: &SplitButtonSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    // Primary padding: density-driven `space.control.x` (compact 0.5 / default
    // 0.75 / comfortable 1.0 rem) — equals Svelte's base `space.control.x` plus
    // the per-density ±0.25rem adjust.
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    // Contract §8 Chevron: per-size icon dimension (0.625–0.875rem).
    let chevron_size = rem_to_px(split_button_chevron_size_rem(effective_size));
    // Spinner glyph tracks the supporting-visual size (one stop down).
    let spinner_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));
    // Contract §8 Divider: 60% of control height, centered.
    let divider_h = height * 0.6;
    // Contract §8 Toggle half: per-size width (1.75–2.5rem), zero padding.
    let toggle_w = rem_to_px(split_button_toggle_width_rem(effective_size));
    // Contract §8 Primary half: gap between spinner and label = space.inline.sm.
    let primary_gap = resolve_px(theme, "space.inline.sm");

    let colors = resolve_split_colors(spec, theme);
    let elevated: Color = resolve_color(theme, "color.background.elevated").into();
    // Contract §4 hover: color-mix(split-fill 84%, background-elevated).
    let hover_fill = colors.fill.mix_srgb(elevated, 0.84);
    // Contract §4 active: fill darkened further (mix toward elevated at 72%).
    let active_fill = colors.fill.mix_srgb(elevated, 0.72);

    // Contract §8 Divider: color-mix(split-text 22%, transparent).
    let divider_color = colors.text.with_alpha(colors.text.a * 0.22);
    let radius = resolve_radius(theme, spec.radius_token());

    let is_unavailable = spec.is_unavailable();
    let label = spec.label.as_deref().unwrap_or("");

    // ── Root row ──
    let mut root = ui_element::div()
        .flex_row()
        .items_center()
        .rounded(radius);

    // ── Primary half ──
    // Spinner + label live as children so Taffy lays them out with the gap.
    let mut primary = ui_element::button("")
        // The caption is a child so Taffy can lay it out beside the spinner,
        // which leaves this button's own label empty. Name it from the spec.
        .aria_label(spec.label.clone().unwrap_or_default())
        .h(height)
        .min_w(rem_to_px(4.0)) // contract §7: min-width 4rem flat
        .pl(pad_x)
        .pr(pad_x)
        .rounded_l(radius)
        .bg(colors.fill)
        .border(1.0)
        .border_color(colors.border)
        .text_color(colors.text)
        .text_size(font_size)
        .text_weight(500)
        .flex_row()
        .items_center()
        .justify_center()
        .gap(primary_gap)
        .focusable();

    if !is_unavailable {
        primary = primary
            .hover(|s| s.bg(hover_fill))
            .active(|s| s.bg(active_fill))
            .cursor_pointer();
    }

    // Contract §4/§8: loading shows the shared ring spinner in the primary half,
    // before the label.
    if spec.is_loading {
        primary = primary.child(
            ui_element::icon("loader")
                .w(spinner_size)
                .h(spinner_size)
                .text_color(colors.text),
        );
    }
    if !label.is_empty() {
        primary = primary.child(
            ui_element::label(label)
                .text_size(font_size)
                .text_weight(500)
                .text_color(colors.text)
                .letter_spacing_em(0.01) // contract §8: letter-spacing 0.01em
                .whitespace_nowrap(),
        );
    }

    root = root.child(primary);

    // ── Divider (60% height, vertically centered by the row) ──
    root = root.child(ui_element::div().w(1.0).h(divider_h).bg(divider_color));

    // ── Toggle half (fixed per-size width, zero padding) ──
    let mut toggle = ui_element::button("")
        // Chevron-only: nothing in its subtree carries text.
        .aria_label("More actions")
        .h(height)
        .w(toggle_w)
        .rounded_r(radius)
        .bg(colors.fill)
        .border(1.0)
        .border_color(colors.border)
        .flex_row()
        .items_center()
        .justify_center()
        .child(
            ui_element::icon("chevron-down")
                .w(chevron_size)
                .h(chevron_size)
                .text_color(colors.text),
        )
        .focusable();

    if !is_unavailable {
        toggle = toggle
            .hover(|s| s.bg(hover_fill))
            .active(|s| s.bg(active_fill))
            .cursor_pointer();
    }

    root = root.child(toggle);

    // ── Disabled / loading: dim the whole control ──
    if is_unavailable {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        root = root.opacity(opacity);
    }

    // ── Menu overlay (rendered when open) ──
    // Stacked below the row inside a column wrapper; absolute positioning /
    // flipping is preview-loop owned.
    if spec.is_open && !spec.items.is_empty() {
        let menu_fill: Color = resolve_color(theme, spec.overlay_fill_token()).into();
        let menu_border: Color = resolve_color(theme, "color.border.default").into();
        let menu_radius = resolve_radius(theme, "radius.surface");
        let item_text: Color = resolve_color(theme, "color.text.primary").into();
        let accent: Color = resolve_color(theme, "color.accent.base").into();
        let sep_color: Color = resolve_color(theme, "color.border.subtle").into();
        // Contract §8 Menu: padding 0.25rem; min-width 12rem.
        let menu_pad = rem_to_px(0.25);
        let menu_min_w = rem_to_px(12.0);
        let item_pad_x = resolve_px(theme, "space.control.x");
        let item_pad_y = resolve_px(theme, "space.control.y");
        let item_min_h = resolve_px(theme, spec.control_height_token());
        // Contract §8 Item: border-radius calc(radius-control − 0.125rem).
        let item_radius = (resolve_radius(theme, "radius.control") - rem_to_px(0.125)).max(0.0);
        // Contract §8 Item hover: color-mix(accent-base 16%, transparent).
        let item_hover = accent.with_alpha(accent.a * 0.16);
        // Contract §8 Menu separator: color-mix(border-subtle 72%, transparent).
        let menu_sep_color = sep_color.with_alpha(sep_color.a * 0.72);

        // Contract: the dropdown is a `menu` of `menuitem`s.
        let mut menu = ui_element::div()
            .aria_role(jetstream_ui::accesskit::Role::Menu)
            .flex_col()
            .min_w(menu_min_w)
            .p(menu_pad)
            .mt(rem_to_px(0.375)) // contract §8 menu: top calc(100% + 0.375rem)
            .bg(menu_fill)
            .border(1.0)
            .border_color(menu_border)
            .rounded(menu_radius);

        for item in &spec.items {
            match item {
                SplitMenuItem::Action {
                    label,
                    is_disabled,
                    ..
                } => {
                    let mut item_el = ui_element::button(label)
                        .aria_role(jetstream_ui::accesskit::Role::MenuItem)
                        .min_h(item_min_h)
                        .pl(item_pad_x)
                        .pr(item_pad_x)
                        .pt(item_pad_y)
                        .pb(item_pad_y)
                        .rounded(item_radius)
                        .text_size(font_size)
                        .text_color(item_text)
                        .flex_row()
                        .items_center()
                        .focusable();
                    if *is_disabled {
                        item_el = item_el
                            .opacity(resolve_opacity(theme, spec.disabled_opacity_token()))
                            .disabled(true);
                    } else {
                        item_el = item_el.hover(|s| s.bg(item_hover)).cursor_pointer();
                    }
                    menu = menu.child(item_el);
                }
                SplitMenuItem::Separator => {
                    menu = menu.child(
                        ui_element::div()
                            // Contract §6: menu separators are announced, so a
                            // screen reader hears the grouping the divider
                            // draws rather than one undifferentiated list.
                            .aria_role(jetstream_ui::accesskit::Role::Splitter)
                            .w_full()
                            .h(rem_to_px(0.0625))
                            .my(menu_pad)
                            .bg(menu_sep_color),
                    );
                }
            }
        }

        // Wrap the row + menu in a column so the menu stacks beneath the control.
        root = ui_element::div().flex_col().child(root).child(menu);
    }

    // The visible label is the accessible name unless something
    // overrides it — the caption sits beside the control, not inside,
    // so name-from-content cannot reach it.
    crate::aria::with_aria_label(root, spec.aria_label.as_deref().or(spec.label.as_deref()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use jetstream_ui::Color;
    use poodle_specs::{ControlDensity, ControlSize};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn probe_color(c: Color) -> ProbeColor {
        ProbeColor { r: c.r, g: c.g, b: c.b, a: c.a }
    }

    fn menu_items() -> Vec<SplitMenuItem> {
        vec![
            SplitMenuItem::action("draft", "Save as draft"),
            SplitMenuItem::action("template", "Save as template"),
            SplitMenuItem::separator(),
            SplitMenuItem::action("discard", "Discard changes"),
        ]
    }

    // ── Anatomy: primary half + divider + chevron toggle ──
    #[test]
    fn renders_primary_divider_and_chevron_segments() {
        let th = theme();
        let el = js_split_button(&SplitButtonSpec::new().with_label("Save"), &th);
        let tree = probe(&el, 400.0, 120.0);

        // Primary label present.
        assert!(tree.has_text("Save"), "primary label missing: {:?}", tree.texts());
        // Chevron glyph present (trailing dropdown trigger).
        assert!(
            tree.has_text("chevron-down"),
            "chevron glyph missing: {:?}",
            tree.texts()
        );
        // Two button segments (primary + toggle).
        assert_eq!(tree.count_kind("Button"), 2, "expected primary + toggle buttons");
        // Divider Panel present at 60% control height. Default md height = 2.25rem
        // (36px) → divider 21.6px. Find the thin (1px wide) panel.
        let divider = tree
            .nodes
            .iter()
            .find(|n| n.kind == "Panel" && (n.w - 1.0).abs() < 0.01)
            .expect("1px divider panel");
        let expected = rem_to_px(control_height_rem(ControlSize::Md)) * 0.6;
        assert!(
            (divider.h - expected).abs() < 0.5,
            "divider height {} should be ~60% of control height {}",
            divider.h,
            expected
        );
    }

    // ── Open state renders the dropdown menu (items + separator) ──
    #[test]
    fn open_menu_renders_items_and_separator() {
        let th = theme();
        let el = js_split_button(
            &SplitButtonSpec::new()
                .with_label("Save")
                .with_items(menu_items())
                .with_open(true),
            &th,
        );
        let tree = probe(&el, 400.0, 400.0);

        assert!(tree.has_text("Save as draft"), "menu item 1 missing");
        assert!(tree.has_text("Save as template"), "menu item 2 missing");
        assert!(tree.has_text("Discard changes"), "menu item 3 missing");
        // primary + toggle + 3 actionable menu items = 5 buttons.
        assert_eq!(tree.count_kind("Button"), 5, "expected 2 control + 3 menu buttons");
    }

    #[test]
    fn closed_split_button_has_no_menu_items() {
        let th = theme();
        let el = js_split_button(
            &SplitButtonSpec::new()
                .with_label("Save")
                .with_items(menu_items()),
            &th,
        );
        let tree = probe(&el, 400.0, 200.0);
        assert!(!tree.has_text("Save as draft"), "menu must stay hidden when closed");
        assert_eq!(tree.count_kind("Button"), 2, "only primary + toggle when closed");
    }

    // ── Variant / tone: ghost is transparent-filled, danger differs ──
    #[test]
    fn ghost_variant_has_transparent_fill() {
        let th = theme();
        let el = js_split_button(
            &SplitButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_label("Save"),
            &th,
        );
        // Primary half fill alpha is the surface 42% mix — well below 1.
        let bg = el.style.background.unwrap_or(Color::TRANSPARENT);
        // Root has no bg; check the first child (primary) instead.
        let primary_bg = el.children[0].style.background.expect("primary bg");
        assert!(
            primary_bg.a < 0.5,
            "ghost primary fill alpha should be reduced, got {} (root {:?})",
            primary_bg.a,
            bg
        );
    }

    #[test]
    fn primary_danger_fill_is_status_danger() {
        let th = theme();
        let danger: Color = resolve_color(&th, "color.status.danger").into();
        let el = js_split_button(
            &SplitButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_tone(ButtonTone::Danger)
                .with_label("Delete"),
            &th,
        );
        let tree = probe(&el, 400.0, 120.0);
        assert!(
            tree.has_background(probe_color(danger), 0.02),
            "primary danger split-button should paint status-danger fill"
        );
    }

    #[test]
    fn primary_success_fill_is_status_success() {
        let th = theme();
        let success: Color = resolve_color(&th, "color.status.success").into();
        let el = js_split_button(
            &SplitButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_tone(ButtonTone::Success)
                .with_label("Confirm"),
            &th,
        );
        let tree = probe(&el, 400.0, 120.0);
        assert!(
            tree.has_background(probe_color(success), 0.02),
            "primary success split-button should paint status-success fill"
        );
    }

    // ── Loading renders a spinner glyph in the primary half ──
    #[test]
    fn loading_renders_spinner_glyph() {
        let th = theme();
        let el = js_split_button(
            &SplitButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_loading(true)
                .with_label("Saving..."),
            &th,
        );
        let tree = probe(&el, 400.0, 120.0);
        assert!(tree.has_text("loader"), "loading spinner glyph missing: {:?}", tree.texts());
        // Loading also dims the control (is_unavailable).
        assert!(el.style.opacity < 1.0, "loading should dim the control");
    }

    #[test]
    fn disabled_dims_control() {
        let th = theme();
        let el = js_split_button(
            &SplitButtonSpec::new().with_label("Save").with_disabled(true),
            &th,
        );
        assert!(el.style.opacity < 1.0, "disabled should reduce opacity");
    }

    // ── Density drives primary padding, not control height ──
    #[test]
    fn density_does_not_change_control_height() {
        let th = theme();
        let h = |d: ControlDensity| {
            let el = js_split_button(
                &SplitButtonSpec::new().with_label("Save").with_density(d),
                &th,
            );
            let tree = probe(&el, 400.0, 120.0);
            // First Button is the primary half.
            tree.nodes
                .iter()
                .find(|n| n.kind == "Button")
                .map(|n| n.h)
                .unwrap_or(0.0)
        };
        assert_eq!(
            h(ControlDensity::Compact),
            h(ControlDensity::Comfortable),
            "density must not change control height"
        );
    }
}
