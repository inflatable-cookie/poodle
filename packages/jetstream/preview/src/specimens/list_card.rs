//! ListCard specimen — card items for list views.
//!
//! Mirrors `packages/gpui/preview/src/specimens/list_card.rs`. Every card is a
//! real `js_list_card` / `js_list_card_with_slots(&ListCardSpec, theme, …)`
//! resolving all visuals from tokens — no hand-rolled boxes. Slot content is
//! composed from real components (`js_pill`, `js_list_card_counter`, `js_button`,
//! `js_icon`), matching the GPUI specimen's `with_leading` / `with_trailing` /
//! `with_footer` compositions.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::button::js_button;
use poodle_jetstream_components::icon::js_icon;
use poodle_jetstream_components::list_card::{js_list_card, js_list_card_with_slots};
use poodle_jetstream_components::list_card_counter::js_list_card_counter;
use poodle_jetstream_components::pill::js_pill;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlSize, IconSize, IconSpec, LeadingFill, LeadingShape,
    ListCardCounterSpec, ListCardLayout, ListCardSpec, PillAppearance, PillSize, PillSpec, PillTone,
    SelectionIndicator,
};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // -- Interactive list cards (interactive + disabled) --
        .child(group("Interactive list cards", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("design-system-v2.figma")
                        .with_subtitle("Updated by Clay \u{00b7} 2h ago")
                        .with_meta("14.2 MB")
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("component-specs.pdf")
                        .with_subtitle("Shared with team \u{00b7} Yesterday")
                        .with_meta("2.8 MB")
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("brand-assets.zip")
                        .with_subtitle("Archived")
                        .with_meta("48 MB")
                        .with_disabled(true),
                    theme,
                )),
        ))
        // -- Rounded-square leading (thumbnails) --
        .child(group("Rounded-square leading (thumbnails)", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("hero-banner.png")
                        .with_subtitle("Uploaded by Jamie \u{00b7} 4h ago")
                        .with_meta("3.1 MB")
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("onboarding-flow.mp4")
                        .with_subtitle("Screen recording \u{00b7} Today")
                        .with_meta("128 MB")
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_interactive(true),
                    theme,
                )),
        ))
        // -- With badges (header-accessories slot) --
        .child(group("With badges", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card_with_slots(
                    &ListCardSpec::new()
                        .with_title("API integration guide")
                        .with_subtitle("Authentication and webhooks")
                        .with_meta("Draft")
                        .with_interactive(true),
                    theme,
                    None,
                    vec![js_pill(
                        &PillSpec::new()
                            .with_label("New")
                            .with_tone(PillTone::Info)
                            .with_size(PillSize::Sm),
                        theme,
                    )],
                    None,
                    None,
                    None,
                    None,
                ))
                .child(js_list_card_with_slots(
                    &ListCardSpec::new()
                        .with_title("Q4 planning deck")
                        .with_subtitle("Revenue targets and roadmap")
                        .with_interactive(true),
                    theme,
                    None,
                    vec![
                        js_pill(
                            &PillSpec::new()
                                .with_label("3")
                                .with_appearance(PillAppearance::Badge)
                                .with_tone(PillTone::Neutral)
                                .with_size(PillSize::Sm),
                            theme,
                        ),
                        js_pill(
                            &PillSpec::new()
                                .with_label("Review")
                                .with_tone(PillTone::Warning)
                                .with_size(PillSize::Sm),
                            theme,
                        ),
                    ],
                    None,
                    None,
                    None,
                    None,
                )),
        ))
        // -- With footer counters (footer slot) --
        .child(group("With footer counters", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card_with_slots(
                    &ListCardSpec::new()
                        .with_title("Design system")
                        .with_subtitle("Components and tokens")
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_interactive(true),
                    theme,
                    None,
                    vec![js_pill(
                        &PillSpec::new()
                            .with_label("Active")
                            .with_tone(PillTone::Success)
                            .with_size(PillSize::Sm),
                        theme,
                    )],
                    Some(
                        div().flex_row().items_center().gap(12.0)
                            .child(js_list_card_counter(&ListCardCounterSpec::new("file-text", 24), theme))
                            .child(js_list_card_counter(&ListCardCounterSpec::new("image", 8), theme))
                            .child(js_list_card_counter(&ListCardCounterSpec::new("folder", 3), theme)),
                    ),
                    None,
                    None,
                    None,
                ))
                .child(js_list_card_with_slots(
                    &ListCardSpec::new()
                        .with_title("Brand guidelines")
                        .with_subtitle("Logos, color, and voice")
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_interactive(true),
                    theme,
                    None,
                    Vec::new(),
                    Some(
                        div().flex_row().items_center().gap(12.0)
                            .child(js_list_card_counter(&ListCardCounterSpec::new("file-text", 6), theme))
                            .child(js_list_card_counter(&ListCardCounterSpec::new("image", 42), theme)),
                    ),
                    None,
                    None,
                    None,
                )),
        ))
        // -- With trailing (exclusive trailing lane; replaces meta + actions) --
        .child(group("With trailing (exclusive lane)", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card_with_slots(
                    &ListCardSpec::new()
                        .with_title("API Server")
                        .with_subtitle("Running on port 8080")
                        .with_interactive(true),
                    theme,
                    None,
                    Vec::new(),
                    None,
                    None,
                    Some(js_pill(
                        &PillSpec::new()
                            .with_label("Active")
                            .with_tone(PillTone::Success)
                            .with_size(PillSize::Sm),
                        theme,
                    )),
                    None,
                ))
                .child(js_list_card_with_slots(
                    &ListCardSpec::new()
                        .with_title("Background Worker")
                        .with_subtitle("High queue depth")
                        .with_interactive(true),
                    theme,
                    None,
                    Vec::new(),
                    None,
                    None,
                    Some(js_pill(
                        &PillSpec::new()
                            .with_label("Degraded")
                            .with_tone(PillTone::Warning)
                            .with_size(PillSize::Sm),
                        theme,
                    )),
                    None,
                )),
        ))
        // -- With actions (action-trigger lane, composed with meta) --
        .child(group("With actions", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card_with_slots(
                    &ListCardSpec::new()
                        .with_title("invoice-2024-q4.pdf")
                        .with_subtitle("Generated yesterday")
                        .with_meta("182 KB")
                        .with_interactive(true),
                    theme,
                    None,
                    Vec::new(),
                    None,
                    Some(
                        js_button(
                            &ButtonSpec::new()
                                .with_label("Download")
                                .with_variant(ButtonVariant::Secondary)
                                .with_size(ControlSize::Sm),
                            theme,
                        ),
                    ),
                    None,
                    None,
                ))
                .child(js_list_card_with_slots(
                    &ListCardSpec::new()
                        .with_title("contract-final.docx")
                        .with_subtitle("Awaiting signature")
                        .with_interactive(true),
                    theme,
                    None,
                    Vec::new(),
                    None,
                    Some(
                        div().flex_row().items_center().gap(4.0)
                            .child(js_icon(&IconSpec::new("download").with_size(IconSize::Sm), theme))
                            .child(js_button(
                                &ButtonSpec::new()
                                    .with_label("Sign")
                                    .with_variant(ButtonVariant::Primary)
                                    .with_size(ControlSize::Sm),
                                theme,
                            )),
                    ),
                    None,
                    None,
                )),
        ))
        // -- With corner (header-accessories corner slot, tertiary color) --
        .child(group("With corner (header-corner slot)", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card_with_slots(
                    &ListCardSpec::new()
                        .with_title("Pipeline config")
                        .with_subtitle("Build and deploy steps")
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_interactive(true),
                    theme,
                    None,
                    Vec::new(),
                    None,
                    None,
                    None,
                    // Corner: supplementary header-corner content (tertiary color).
                    Some(div().flex_row().items_center().gap(4.0)
                        .child(js_icon(&IconSpec::new("git-branch").with_size(IconSize::Sm), theme))
                        .child(label("v2.1"))),
                ))
                .child(js_list_card_with_slots(
                    &ListCardSpec::new()
                        .with_title("Release candidate")
                        .with_subtitle("Badges and corner share the cluster")
                        .with_interactive(true),
                    theme,
                    None,
                    // Badges and corner render side by side in the accessories cluster.
                    vec![js_pill(
                        &PillSpec::new()
                            .with_label("RC")
                            .with_tone(PillTone::Info)
                            .with_size(PillSize::Sm),
                        theme,
                    )],
                    None,
                    None,
                    None,
                    Some(div().flex_row().items_center().gap(4.0)
                        .child(js_icon(&IconSpec::new("clock").with_size(IconSize::Sm), theme))
                        .child(label("2d"))),
                )),
        ))
        // -- Solid fill with accent colors --
        .child(group("Solid fill with accent colors", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Design Tokens")
                        .with_subtitle("Color system")
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_leading_fill(LeadingFill::Solid)
                        .with_accent_color("#6366f1"),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Typography")
                        .with_subtitle("Font scales & families")
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_leading_fill(LeadingFill::Solid)
                        .with_accent_color("#ec4899"),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Spacing")
                        .with_subtitle("Layout grid & spacing tokens")
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_leading_fill(LeadingFill::Solid)
                        .with_accent_color("#10b981"),
                    theme,
                )),
        ))
        // -- Not live (dashed border, interactive) --
        .child(group("Not live (reduced opacity, interactive)", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Unpublished Draft")
                        .with_subtitle("Last edited 3 days ago")
                        .with_not_live(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Scheduled Post")
                        .with_subtitle("Publishes tomorrow at 9 AM")
                        .with_not_live(true)
                        .with_interactive(true),
                    theme,
                )),
        ))
        // -- Corner sash badges --
        .child(group("Corner sash badges", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Free tier plan")
                        .with_subtitle("No credit card required")
                        .with_sash("Free")
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Premium integration")
                        .with_subtitle("Unlocks advanced features")
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_leading_fill(LeadingFill::Solid)
                        .with_accent_color("#6366f1")
                        .with_sash("New")
                        .with_sash_color("#6366f1")
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Legacy connector")
                        .with_subtitle("Deprecated \u{2014} migrate by Q2")
                        .with_sash("EOL")
                        .with_sash_color("#ef4444")
                        .with_interactive(true),
                    theme,
                )),
        ))
        // -- Selectable (multi-select) --
        .child(group("Selectable (multi-select)", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Alice Chen")
                        .with_subtitle("alice@example.com")
                        .with_selectable(true)
                        .with_selected(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Bob Martinez")
                        .with_subtitle("bob@example.com")
                        .with_selectable(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Carol Patel")
                        .with_subtitle("carol@example.com")
                        .with_selectable(true)
                        .with_selected(true),
                    theme,
                )),
        ))
        // -- Reorder handle --
        .child(group("Reorder handle", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("design-system")
                        .with_subtitle("Primary rubric")
                        .with_reorder_handle(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("accessibility")
                        .with_subtitle("WCAG AA baseline")
                        .with_reorder_handle(true),
                    theme,
                )),
        ))
        // -- Link roots (href) --
        .child(group("Link roots (href)", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Billing settings")
                        .with_subtitle("Manage invoices and payment methods")
                        .with_href("#billing"),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Documentation portal")
                        .with_subtitle("Opens the external guide")
                        .with_meta("docs.example.com")
                        .with_href("https://example.com/docs"),
                    theme,
                )),
        ))
        // -- Highlighted (accent emphasis) --
        .child(group("Highlighted (accent emphasis)", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Active selection")
                        .with_subtitle("Accent-tinted border and inset ring")
                        .with_meta("Now")
                        .with_interactive(true)
                        .with_highlighted(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Highlighted with custom accent")
                        .with_subtitle("Accent gradient over the fill")
                        .with_interactive(true)
                        .with_highlighted(true)
                        .with_accent_color("#6366f1"),
                    theme,
                )),
        ))
        // -- Active: the card you are currently on --
        .child(group("Active card", secondary,
            div()
                .flex_col()
                .gap(8.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Acowtancy Build Test")
                        .with_subtitle("Registered 28/07/2026 16:35")
                        .with_interactive(true)
                        .with_active(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Acowtancy Build Test")
                        .with_subtitle("Registered 28/07/2026 16:35")
                        .with_interactive(true),
                    theme,
                ))
                // Orthogonal to selection, so both can be true at once.
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Active and selected")
                        .with_subtitle("Both states at once")
                        .with_interactive(true)
                        .with_selectable(true)
                        .with_selected(true)
                        .with_active(true),
                    theme,
                )),
        ))
        // -- Selection indicator (checkbox) --
        .child(group("Selection indicator (checkbox)", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Selected row")
                        .with_subtitle("Checkbox indicator, checked")
                        .with_selectable(true)
                        .with_selected(true)
                        .with_selection_indicator(SelectionIndicator::Checkbox),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Unselected row")
                        .with_subtitle("Checkbox indicator, unchecked")
                        .with_selectable(true)
                        .with_selection_indicator(SelectionIndicator::Checkbox),
                    theme,
                )),
        ))
        // -- Layout (default / compact / stacked) --
        .child(group("Layout (default / compact / stacked)", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Default layout")
                        .with_subtitle("Standard leading + body row")
                        .with_meta("2.0rem")
                        .with_layout(ListCardLayout::Default)
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Compact layout")
                        .with_subtitle("Denser \u{2014} smaller leading box")
                        .with_meta("1.75rem")
                        .with_layout(ListCardLayout::Compact)
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Stacked layout")
                        .with_subtitle("Leading on top, body below, bottom rail")
                        .with_layout(ListCardLayout::Stacked)
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_leading_fill(LeadingFill::Solid)
                        .with_accent_color("#6366f1")
                        .with_interactive(true),
                    theme,
                )),
        ))
        // -- Leading size offset --
        .child(group("Leading size offset", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Default leading")
                        .with_subtitle("Matches the card size ladder")
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Offset leading (+1 step)")
                        .with_subtitle("Leading block steps up by 0.25rem")
                        .with_leading_size_offset(1)
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Offset leading (+2 steps)")
                        .with_subtitle("Larger leading box, same typography")
                        .with_leading_size_offset(2)
                        .with_interactive(true),
                    theme,
                )),
        ))
        // -- Static list card --
        .child(group("Static list card", secondary,
            js_list_card(
                &ListCardSpec::new()
                    .with_title("System Configuration")
                    .with_subtitle("Read-only \u{2014} managed by admin")
                    .with_meta("v2.1.0"),
                theme,
            ),
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
