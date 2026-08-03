//! PageHeader — Jetstream page header backed by PageHeaderSpec.
//!
//! Contract: `docs/contracts/components/page-header.md`
//! Reference: `packages/svelte/components/src/PageHeader.svelte`
//!
//! Anatomy: root (stacked) → top-row (title block + actions-row) → secondary
//! content (subtitle / breadcrumbs / meta) → banner. The title block stacks
//! eyebrow, section (distinct from eyebrow), title (+ count Pill), subtitle.
//! The back link uses the `arrow-left` icon with the resolved display label and
//! an optional contextual success dot. Actions, breadcrumbs, and meta are
//! host-owned `JsEl` slots (mirrors the GPUI `with_actions`/`with_breadcrumbs`/
//! `with_meta` channels and the `js_app_header_with_slots` convention).
//!
//! Render-only / build-/probe-verified: the back-link click handler and any
//! action behavior live in the host event loop, not the component.
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    ControlSize, PageHeaderAlign, PageHeaderSpec, PillAppearance, PillSize, PillSpec, PillTone,
};

use crate::pill::js_pill;
use crate::presentation::{
    rem_to_px, resolve_semantic_size, resolve_supporting_visual_size, size_font_rem,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_px};

/// Map a status tone icon name (mirrors the Callout/banner convention).
fn tone_icon(tone: poodle_specs::StatusTone) -> &'static str {
    match tone {
        poodle_specs::StatusTone::Neutral | poodle_specs::StatusTone::Info => "info",
        poodle_specs::StatusTone::Success => "check-circle",
        poodle_specs::StatusTone::Warning => "alert-triangle",
        poodle_specs::StatusTone::Danger => "x-circle",
        poodle_specs::StatusTone::Pending => "loader",
    }
}

/// Map the resolved control size to the supporting-visual `PillSize` the count
/// badge renders at (matches Svelte `resolveSupportingVisualSize`).
fn count_pill_size(size: ControlSize) -> PillSize {
    match resolve_supporting_visual_size(size) {
        ControlSize::Xs => PillSize::Xs,
        ControlSize::Sm => PillSize::Sm,
        ControlSize::Md => PillSize::Md,
        ControlSize::Lg => PillSize::Lg,
        ControlSize::Xl => PillSize::Xl,
    }
}

/// Per-level heading scale applied to the base heading-size token. Mirrors the
/// Svelte `--poodle-page-header-title-size` ladder (level 2 = base, 3+ compact).
fn level_scale(level: u8) -> f32 {
    match level {
        1 => 1.143, // 2rem-ish over the 1.75rem level-2 base
        2 => 1.0,
        _ => 0.714, // compact heading for levels 3–6 (≈1.25rem)
    }
}

/// Convenience: render a page header with no host slots.
pub fn js_page_header(spec: &PageHeaderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    js_page_header_with_slots(spec, theme, None, None, None)
}

/// Render a page header with optional host-owned breadcrumbs / actions / meta
/// slots (each a pre-built `JsEl` cluster).
pub fn js_page_header_with_slots(
    spec: &PageHeaderSpec,
    theme: &JetstreamThemeProvider,
    breadcrumbs: Option<JsEl>,
    actions: Option<JsEl>,
    meta: Option<JsEl>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(
        effective_size,
    )));

    // ── Typography ────────────────────────────────────────────────────────────
    let heading_base = resolve_px(theme, spec.heading_size_token());
    let title_size = heading_base * level_scale(spec.level);
    let subtitle_size = rem_to_px(size_font_rem(effective_size));
    let eyebrow_size = rem_to_px(0.6875);
    let section_size = rem_to_px(0.75);
    let back_size = rem_to_px(0.8125);
    let body_font = rem_to_px(size_font_rem(effective_size));

    // ── Spacing ───────────────────────────────────────────────────────────────
    let gap = resolve_px(theme, spec.gap_token());
    let header_gap = resolve_px(theme, spec.header_gap_token());
    let title_block_gap = resolve_px(theme, spec.title_block_gap_token());
    let title_gap = resolve_px(theme, spec.title_gap_token());
    let actions_gap = resolve_px(theme, spec.actions_gap_token());
    let pad_y = resolve_px(theme, spec.padding_y_token());

    // ── Colors ────────────────────────────────────────────────────────────────
    let text_primary = resolve_color(theme, spec.title_color_token());
    let text_secondary = resolve_color(theme, spec.subtitle_color_token());
    let eyebrow_color = resolve_color(theme, spec.eyebrow_color_token());
    let section_color = resolve_color(theme, spec.section_color_token());
    let back_color = resolve_color(theme, spec.back_color_token());
    let context_dot = resolve_color(theme, spec.context_dot_color_token());
    let banner_color = resolve_color(theme, spec.banner_color_token());
    let panel = resolve_color(theme, "color.background.panel");
    let banner_radius = crate::theme_ext::resolve_radius(theme, spec.banner_radius_token());

    let primary_title = spec.primary_title();
    let resolved_subtitle = spec.resolved_subtitle();

    let mut outer = ui_element::div().flex_col().gap(gap).pb(pad_y);

    // ── Top row: title block (left) + actions row (right) ─────────────────────
    let mut left_col = ui_element::div()
        .flex_col()
        .gap(title_block_gap)
        .grow()
        .min_w(0.0);

    // Eyebrow (category tag) — always rendered first when present.
    if let Some(ref eyebrow) = spec.eyebrow {
        left_col = left_col.child(
            ui_element::label(&eyebrow.to_uppercase())
                .text_color(eyebrow_color)
                .text_size(eyebrow_size)
                .text_weight(600)
                .letter_spacing_em(0.12), // contract __eyebrow: letter-spacing 0.12em
        );
    }

    // Section label — distinct stacked row (only in default posture with split).
    if spec.has_section_title_split() && !spec.is_entity_detail_posture() {
        if let Some(ref section) = spec.section {
            left_col = left_col.child(
                ui_element::label(&section.to_uppercase())
                    .text_color(section_color)
                    .text_size(section_size)
                    .text_weight(700)
                    .letter_spacing_em(0.08), // contract __section: letter-spacing 0.08em
            );
        }
    }

    // Title row: heading + optional count Pill.
    let mut title_row = ui_element::div()
        .flex_row()
        .items_center()
        .gap(title_gap)
        .child(
            ui_element::label(&primary_title)
                .text_color(text_primary)
                .text_size(title_size)
                .text_weight(700),
        );

    if let Some(count) = spec.count {
        title_row = title_row.child(js_pill(
            &PillSpec::new()
                .with_label(format!("{count}"))
                .with_tone(PillTone::Neutral)
                .with_appearance(PillAppearance::Subtle)
                .with_size(count_pill_size(effective_size)),
            theme,
        ));
    }

    left_col = left_col.child(title_row);

    // Subtitle (in default posture; in entity-detail it is the swapped title).
    if let Some(ref subtitle) = resolved_subtitle {
        left_col = left_col.child(
            ui_element::label(subtitle)
                .text_color(text_secondary)
                .text_size(subtitle_size),
        );
    }

    // Actions row: back link (left) + actions cluster (right).
    let actions_row = if spec.has_back_link() || actions.is_some() {
        let mut row = ui_element::div()
            .flex_row()
            .items_center()
            .gap(header_gap)
            .flex_none();

        if spec.has_back_link() {
            let mut back = ui_element::div()
                .flex_row()
                .items_center()
                .gap(rem_to_px(0.35))
                .cursor_pointer()
                .child(
                    ui_element::icon("arrow-left")
                        .w(icon_size)
                        .h(icon_size)
                        .text_color(back_color),
                )
                .child(
                    ui_element::label(&spec.back_display_label())
                        .text_color(back_color)
                        .text_size(back_size),
                );
            if spec.back_is_contextual {
                back = back.child(
                    ui_element::div()
                        .w(rem_to_px(0.375))
                        .h(rem_to_px(0.375))
                        .rounded(rem_to_px(0.1875))
                        .flex_none()
                        .bg(context_dot),
                );
            }
            row = row.child(back);
        }

        if let Some(actions) = actions {
            row = row.child(
                ui_element::div()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap(actions_gap)
                    .child(actions),
            );
        }
        Some(row)
    } else {
        None
    };

    let mut top_row = ui_element::div().flex_row().items_start();
    top_row = match spec.align {
        PageHeaderAlign::Start => top_row.justify_start(),
        PageHeaderAlign::Between => top_row.justify_between(),
    };
    top_row = top_row.child(left_col);
    if let Some(actions_row) = actions_row {
        top_row = top_row.child(actions_row);
    }
    outer = outer.child(top_row);

    // ── Secondary content: breadcrumbs / meta ─────────────────────────────────
    if let Some(breadcrumbs) = breadcrumbs {
        outer = outer.child(
            ui_element::div()
                .flex_row()
                .flex_wrap()
                .items_center()
                .min_w(0.0)
                .child(breadcrumbs),
        );
    }
    if let Some(meta) = meta {
        outer = outer.child(ui_element::div().mt(rem_to_px(0.125)).child(meta));
    }

    // ── Banner ────────────────────────────────────────────────────────────────
    if spec.has_banner() {
        if let Some(ref message) = spec.banner_message {
            // Tinted banner fill: mix banner tone into the panel surface.
            let tinted_fill = color_mix(banner_color, panel, 0.12);
            let banner = ui_element::div()
                .bg(tinted_fill)
                .rounded(banner_radius)
                .border_l_1()
                .border_color(banner_color)
                .pl(rem_to_px(0.75))
                .pr(rem_to_px(0.75))
                .pt(rem_to_px(0.5))
                .pb(rem_to_px(0.5))
                .flex_row()
                .items_center()
                .gap(rem_to_px(0.5))
                .child(
                    ui_element::icon(tone_icon(spec.banner_tone))
                        .w(rem_to_px(1.0))
                        .h(rem_to_px(1.0))
                        .text_color(banner_color),
                )
                .child(
                    ui_element::label(message)
                        .text_color(text_primary)
                        .text_size(body_font)
                        .grow(),
                );
            outer = outer.child(banner);
        }
    }

    crate::aria::with_aria_label(outer, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn title_and_subtitle_render() {
        let el = js_page_header(
            &PageHeaderSpec::new("Components").with_subtitle("Browse the library."),
            &theme(),
        );
        let tree = crate::render_probe::probe(&el, 600.0, 200.0);
        assert!(
            tree.has_text("Components"),
            "title missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Browse the library."),
            "subtitle missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn eyebrow_renders_uppercased_and_distinct_from_section() {
        // Eyebrow AND section both present → both render as distinct rows.
        let spec = PageHeaderSpec::new("Button")
            .with_eyebrow("Primitive")
            .with_section("Components");
        let tree = crate::render_probe::probe(&js_page_header(&spec, &theme()), 600.0, 220.0);
        assert!(
            tree.has_text("PRIMITIVE"),
            "eyebrow should be uppercased: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("COMPONENTS"),
            "section should render distinctly: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn count_pill_and_back_link_render() {
        let spec = PageHeaderSpec::new("Media Library")
            .with_count(128)
            .with_back("/library", "Back to Library");
        let tree = crate::render_probe::probe(&js_page_header(&spec, &theme()), 600.0, 200.0);
        // Count pill carries the number.
        assert!(
            tree.has_text("128"),
            "count pill missing: {:?}",
            tree.texts()
        );
        // Back-link uses arrow-left icon and the stripped display label.
        assert!(
            tree.has_text("arrow-left"),
            "back link should use arrow-left icon: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Library"),
            "back label should be stripped to 'Library': {:?}",
            tree.texts()
        );
    }

    #[test]
    fn actions_slot_renders() {
        let actions = ui_element::div().child(ui_element::button("Edit"));
        let el = js_page_header_with_slots(
            &PageHeaderSpec::new("Settings"),
            &theme(),
            None,
            Some(actions),
            None,
        );
        let tree = crate::render_probe::probe(&el, 600.0, 160.0);
        assert!(
            tree.has_text("Edit"),
            "actions slot missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn banner_renders_with_tone_icon() {
        let spec = PageHeaderSpec::new("Account")
            .with_banner("Verify your email.", poodle_specs::StatusTone::Warning);
        let tree = crate::render_probe::probe(&js_page_header(&spec, &theme()), 600.0, 200.0);
        assert!(
            tree.has_text("Verify your email."),
            "banner message missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("alert-triangle"),
            "warning banner icon missing: {:?}",
            tree.texts()
        );
    }
}
