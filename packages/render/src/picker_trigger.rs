use std::sync::Arc;

use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, StylePatch,
};
use poodle_specs::{ControlSize, SemanticControlSizeRole};

use crate::color::mix_srgb;
use crate::context::RenderContext;
use crate::presentation::{
    rem_to_px, size_font_rem, size_height_offset_rem, size_padding_x_offset_rem,
};

pub(crate) struct PickerTrigger<'a> {
    pub display: &'a str,
    pub has_value: bool,
    pub open: bool,
    pub disabled: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub indicator: &'static str,
    pub indicator_size: Option<f32>,
    pub elevated: ColorValue,
    pub border_color: ColorValue,
    pub on_toggle: Option<&'a Arc<dyn Fn() + Send + Sync>>,
}

pub(crate) fn picker_trigger(ctx: &RenderContext<'_>, config: PickerTrigger<'_>) -> Node {
    let effective_size = ctx.resolve_size(Some(config.size), config.size_role);
    let height = ctx.theme().resolve_space("size.control.height")
        + rem_to_px(size_height_offset_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = ctx.theme().resolve_space("space.inline.md")
        + rem_to_px(size_padding_x_offset_rem(effective_size));
    let inline_gap = ctx.theme().resolve_space("space.inline.sm");
    let indicator_size = config
        .indicator_size
        .unwrap_or_else(|| ctx.theme().resolve_space("size.icon.sm"));
    let fill = ctx.theme().resolve_color("color.background.surface");
    let accent = ctx.theme().resolve_color("color.accent.base");
    let radius = ctx.theme().resolve_radius("radius.control");
    let display_color = ctx.theme().resolve_color(if config.has_value {
        "color.text.primary"
    } else {
        "color.text.secondary"
    });
    let icon_color = ctx.theme().resolve_color("color.icon.muted");
    let hover_bg = mix_srgb(fill, config.elevated, 0.14);

    let mut trigger = Node::container();
    {
        let style = &mut trigger.style;
        style.fill_width = true;
        style.descriptor.background = Some(fill);
        style.descriptor.border.width = 1.0;
        style.descriptor.border.color = if config.open {
            accent
        } else {
            config.border_color
        };
        style.descriptor.corner_radii.top_left = radius;
        style.descriptor.corner_radii.top_right = radius;
        style.descriptor.corner_radii.bottom_right = radius;
        style.descriptor.corner_radii.bottom_left = radius;
        style.descriptor.layout.height = LayoutSizing::Fixed(height);
        let padding = &mut style.descriptor.layout.spacing.padding;
        padding.left = pad_x;
        padding.right = pad_x;
        style.descriptor.layout.direction = LayoutDirection::Row;
        style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        style.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        style.descriptor.layout.spacing.gap = inline_gap;
    }
    trigger.interaction.focusable = true;

    let mut value_label = Node::text(config.display);
    value_label.style.descriptor.text_color = Some(display_color);
    value_label.style.text_size = Some(font_size);
    value_label.style.descriptor.layout.width = LayoutSizing::Grow;
    let mut indicator = Node::icon(config.indicator, indicator_size);
    indicator.style.descriptor.text_color = Some(icon_color);
    let mut trigger = trigger.child(value_label).child(indicator);

    if !config.disabled {
        trigger.style.descriptor.cursor = CursorHint::Pointer;
        trigger.style.hover = Some(StylePatch {
            background: Some(hover_bg),
            border_color: None,
            text_color: None,
            opacity: None,
        });

        if let Some(handler) = config.on_toggle {
            let handler = Arc::clone(handler);
            trigger.interaction.on_activate = Some(Arc::new(move || handler()));
        }
    }

    trigger
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::NodeKind;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn config<'a>(ctx: &RenderContext<'_>, display: &'a str) -> PickerTrigger<'a> {
        PickerTrigger {
            display,
            has_value: false,
            open: false,
            disabled: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            indicator: "calendar",
            indicator_size: None,
            elevated: ctx.theme().resolve_color("color.background.elevated"),
            border_color: ctx.theme().resolve_color("color.border.default"),
            on_toggle: None,
        }
    }

    #[test]
    fn shared_trigger_preserves_metrics_and_indicator() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let mut trigger_config = config(&ctx, "Choose a date");
        trigger_config.indicator_size = Some(13.0);
        let trigger = picker_trigger(&ctx, trigger_config);

        assert_eq!(
            trigger.style.descriptor.layout.height,
            LayoutSizing::Fixed(36.0)
        );
        let indicator = trigger
            .find(&|node| matches!(&node.kind, NodeKind::Icon { name, .. } if name == "calendar"))
            .expect("indicator");
        assert!(matches!(&indicator.kind, NodeKind::Icon { size, .. } if *size == 13.0));
        let label = trigger
            .find(&|node| {
                matches!(&node.kind, NodeKind::Text { content } if content == "Choose a date")
            })
            .expect("value label");
        assert_eq!(
            label.style.descriptor.text_color,
            Some(ctx.theme().resolve_color("color.text.secondary"))
        );
    }

    #[test]
    fn value_open_and_disabled_states_follow_the_contract() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let mut trigger_config = config(&ctx, "2026-08-09 12:00");
        trigger_config.has_value = true;
        trigger_config.open = true;
        trigger_config.disabled = true;
        let trigger = picker_trigger(&ctx, trigger_config);

        assert_eq!(
            trigger.style.descriptor.border.color,
            ctx.theme().resolve_color("color.accent.base")
        );
        let label = trigger
            .find(&|node| {
                matches!(&node.kind, NodeKind::Text { content } if content == "2026-08-09 12:00")
            })
            .expect("value label");
        assert_eq!(
            label.style.descriptor.text_color,
            Some(ctx.theme().resolve_color("color.text.primary"))
        );
        assert!(trigger.interaction.on_activate.is_none());
        assert!(trigger.style.hover.is_none());
    }
}
