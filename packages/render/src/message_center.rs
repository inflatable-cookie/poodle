//! MessageCenter — icon trigger plus a durable, host-owned message archive.
//!
//! Contract: `docs/contracts/components/message-center.md`.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodePosition, NodeRole,
};
use poodle_specs::{
    ButtonFit, ButtonSpec, ButtonTone, ButtonVariant, ControlSize, EmptyStateSize, EmptyStateSpec,
    IconButtonSpec, InlineTypographyMode, MessageCenterItem, MessageCenterSpec, PopoverSpec,
    ProgressSpec, SemanticControlSizeRole, StatusIndicatorSpec, TimeAgoSpec,
};

use crate::button::button;
use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::empty_state::empty_state;
use crate::floating_overlay::floating_overlay;
use crate::icon_button::icon_button;
use crate::popover::popover_surface;
use crate::presentation::{control_height_rem, rem_to_px};
use crate::progress::progress;
use crate::status_indicator::status_indicator;
use crate::time_ago::time_ago;

#[derive(Default)]
pub struct MessageCenterHandlers {
    pub on_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub on_item_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_read_change: Option<Arc<dyn Fn(&str, bool) + Send + Sync>>,
    pub on_remove: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_mark_all_read: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub fn message_center(
    spec: &MessageCenterSpec,
    ctx: &RenderContext<'_>,
    handlers: MessageCenterHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let anchor_size = rem_to_px(control_height_rem(effective_size));
    let unread = spec.unread_count();
    let open = spec.current_open();

    let open_handler = handlers
        .on_open_change
        .clone()
        .map(|handler| Arc::new(move || handler(!open)) as Arc<dyn Fn() + Send + Sync>);
    let trigger_spec = IconButtonSpec::new()
        .with_variant(ButtonVariant::Secondary)
        .with_icon(&spec.trigger_icon)
        .with_aria_label(spec.effective_trigger_label())
        .with_tooltip(&spec.title)
        .with_expanded(open)
        .with_size(base_size)
        .with_size_role(spec.size_role)
        .with_density(density);
    let trigger_button = icon_button(&trigger_spec, ctx, open_handler);
    let trigger = trigger_with_indicator(trigger_button, unread, ctx);

    let surface = open.then(|| {
        let content = center_content(spec, ctx, &handlers);
        let popover_spec = PopoverSpec::new()
            .with_open(true)
            .with_placement(spec.placement)
            .with_aria_label(spec.effective_aria_label())
            .with_surface_min_width(poodle_specs::Dimension::new("24rem"))
            .with_surface_max_width(poodle_specs::Dimension::new("30rem"));
        popover_surface(&popover_spec, ctx, Some(content))
    });

    floating_overlay(
        trigger,
        surface,
        spec.placement,
        anchor_size,
        anchor_size,
        crate::floating_overlay::OVERLAY_GAP_PX,
    )
}

fn trigger_with_indicator(trigger: Node, unread: usize, ctx: &RenderContext<'_>) -> Node {
    let mut wrapper = Node::container();
    wrapper.position = NodePosition::Relative;
    wrapper.style.descriptor.layout.direction = LayoutDirection::Row;
    wrapper = wrapper.child(trigger);

    if unread == 0 {
        return wrapper;
    }

    let count = if unread > 99 {
        "99+".into()
    } else {
        unread.to_string()
    };
    let mut badge = Node::text(count);
    badge.position = NodePosition::Absolute {
        top: Some(-rem_to_px(0.2)),
        left: None,
        right: Some(-rem_to_px(0.25)),
        bottom: None,
    };
    {
        let s = &mut badge.style;
        s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(0.85));
        s.min_width = Some(rem_to_px(0.85));
        s.descriptor.layout.spacing.padding.left = rem_to_px(0.18);
        s.descriptor.layout.spacing.padding.right = rem_to_px(0.18);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.background = Some(ctx.theme().resolve_color("color.status.danger"));
        s.descriptor.text_color = Some(ctx.theme().resolve_color("color.text.inverse"));
        s.descriptor.border.width = rem_to_px(0.125);
        s.descriptor.border.color = ctx.theme().resolve_color("color.background.elevated");
        s.descriptor.corner_radii.top_left = 999.0;
        s.descriptor.corner_radii.top_right = 999.0;
        s.descriptor.corner_radii.bottom_left = 999.0;
        s.descriptor.corner_radii.bottom_right = 999.0;
        s.text_size = Some(rem_to_px(0.5));
        s.text_weight = Some(700);
        s.text_align = Some(poodle_node::TextAlign::Center);
        s.line_height = Some(1.0);
        s.overlay = true;
    }
    badge.a11y.label = Some(format!("{unread} unread"));
    wrapper.child(badge)
}

fn center_content(
    spec: &MessageCenterSpec,
    ctx: &RenderContext<'_>,
    handlers: &MessageCenterHandlers,
) -> Node {
    let density = ctx.resolve_density(spec.density);
    let mut root = Node::container();
    root.a11y.role = Some(NodeRole::Region);
    root.a11y.label = Some(spec.effective_aria_label().to_string());
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.md");
        s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(28.0));
        s.fill_width = true;
    }
    root = root.child(center_header(spec, ctx, handlers));

    if spec.items.is_empty() {
        let empty_spec = EmptyStateSpec::new(&spec.empty_title)
            .with_message(&spec.empty_message)
            .with_size(EmptyStateSize::Compact)
            .with_density(density);
        return root.child(empty_state(&empty_spec, ctx));
    }

    let mut list = Node::container();
    list.a11y.role = Some(NodeRole::List);
    {
        let s = &mut list.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = rem_to_px(0.25);
        s.max_height = Some(rem_to_px(24.0));
        s.descriptor.layout.overflow_y = LayoutOverflow::Scroll;
        s.fill_width = true;
    }
    for item in &spec.items {
        list = list.child(message_row(item, spec, ctx, handlers));
    }
    root.child(list)
}

fn center_header(
    spec: &MessageCenterSpec,
    ctx: &RenderContext<'_>,
    handlers: &MessageCenterHandlers,
) -> Node {
    let density = ctx.resolve_density(spec.density);
    let unread = spec.unread_count();
    let mut header = Node::container();
    {
        let s = &mut header.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.md");
        s.descriptor.layout.spacing.padding.bottom = ctx.theme().resolve_space("space.stack.md");
        s.border_bottom_width = Some(ctx.theme().resolve_space("border.width.default"));
        s.border_color_bottom = Some(ctx.theme().resolve_color("color.border.subtle"));
        s.fill_width = true;
    }

    let mut copy = Node::container();
    copy.style.descriptor.layout.direction = LayoutDirection::Column;
    copy.style.descriptor.layout.spacing.gap = rem_to_px(0.125);
    let mut title = Node::text(&spec.title);
    title.style.descriptor.text_color = Some(ctx.theme().resolve_color("color.text.primary"));
    title.style.text_size = Some(ctx.theme().resolve_space("typography.heading.size"));
    title.style.text_weight = Some(650);
    copy = copy.child(title);
    if unread > 0 {
        let mut summary = Node::text(format!("{unread} unread"));
        summary.style.descriptor.text_color = Some(ctx.theme().resolve_color("color.text.secondary"));
        summary.style.text_size = Some(ctx.theme().resolve_space("typography.caption.size"));
        copy = copy.child(summary);
    }
    header = header.child(copy);

    if unread > 0 {
        if let Some(handler) = handlers.on_mark_all_read.clone() {
            let button_spec = ButtonSpec::new()
                .with_label("Mark all read")
                .with_variant(ButtonVariant::Ghost)
                .with_fit(ButtonFit::Content)
                .with_size(ControlSize::Xs)
                .with_density(density);
            header = header.child(button(&button_spec, ctx, Some(handler)));
        }
    }
    header
}

fn message_row(
    item: &MessageCenterItem,
    spec: &MessageCenterSpec,
    ctx: &RenderContext<'_>,
    handlers: &MessageCenterHandlers,
) -> Node {
    let density = ctx.resolve_density(spec.density);
    let mut row = Node::container();
    row.a11y.role = Some(NodeRole::ListItem);
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.sm");
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.top = rem_to_px(0.625);
        pad.bottom = rem_to_px(0.625);
        pad.left = rem_to_px(0.625);
        pad.right = rem_to_px(0.5);
        let radius = ctx.theme().resolve_radius("radius.control");
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.fill_width = true;
        if !item.read {
            let accent = ctx.theme().resolve_color(item.tone.color_token());
            s.descriptor.background = Some(with_alpha(accent, accent.3 * 0.08));
        }
    }

    let content = message_content(
        item,
        spec,
        ctx,
        handlers.on_item_select.clone().filter(|_| item.selectable),
    );
    row = row.child(content);

    let mut actions = Node::container();
    actions.style.descriptor.layout.direction = LayoutDirection::Row;
    actions.style.descriptor.layout.spacing.gap = rem_to_px(0.125);
    actions.style.flex_shrink_zero = true;
    let mut has_actions = false;

    if item.read_control {
        if let Some(handler) = handlers.on_read_change.clone() {
            let id = item.id.clone();
            let next_read = !item.read;
            let label = if item.read {
                format!("Mark {} unread", item.title)
            } else {
                format!("Mark {} read", item.title)
            };
            let control = IconButtonSpec::new()
                .with_icon(if item.read { "mail" } else { "check" })
                .with_aria_label(label)
                .with_tooltip(if item.read {
                    "Mark unread"
                } else {
                    "Mark read"
                })
                .with_size(ControlSize::Xs)
                .with_density(density);
            actions = actions.child(icon_button(
                &control,
                ctx,
                Some(Arc::new(move || handler(&id, next_read))),
            ));
            has_actions = true;
        }
    }

    if item.removable {
        if let Some(handler) = handlers.on_remove.clone() {
            let id = item.id.clone();
            let control = IconButtonSpec::new()
                .with_icon("trash-2")
                .with_aria_label(format!("Remove {}", item.title))
                .with_tooltip("Remove")
                .with_tone(ButtonTone::Danger)
                .with_size(ControlSize::Xs)
                .with_density(density);
            actions = actions.child(icon_button(
                &control,
                ctx,
                Some(Arc::new(move || handler(&id))),
            ));
            has_actions = true;
        }
    }

    if has_actions {
        row = row.child(actions);
    }
    row
}

fn message_content(
    item: &MessageCenterItem,
    spec: &MessageCenterSpec,
    ctx: &RenderContext<'_>,
    on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let density = ctx.resolve_density(spec.density);
    let mut content = if on_select.is_some() {
        Node::button("")
    } else {
        Node::container()
    };
    {
        let s = &mut content.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.sm");
        s.flex_fill = true;
        s.fill_width = true;
        s.descriptor.background = None;
        s.descriptor.border.width = 0.0;
    }
    if let Some(handler) = on_select {
        let id = item.id.clone();
        content.interaction.focusable = true;
        content.interaction.on_activate = Some(Arc::new(move || handler(&id)));
        content.style.descriptor.cursor = CursorHint::Pointer;
        content.a11y.label = Some(item.title.clone());
    }

    let leading = if let Some(icon) = item.icon.as_deref() {
        let mut icon = Node::icon(icon, rem_to_px(1.0));
        icon.style.descriptor.text_color = Some(ctx.theme().resolve_color(item.tone.color_token()));
        icon
    } else {
        let indicator = StatusIndicatorSpec::new()
            .with_status(if item.read {
                poodle_specs::StatusTone::Neutral
            } else {
                item.tone
            })
            .with_aria_label(if item.read { "Read" } else { "Unread" })
            .with_size(ControlSize::Xs)
            .with_size_role(SemanticControlSizeRole::Control)
            .with_density(density);
        status_indicator(&indicator, ctx)
    };
    content = content.child(leading);

    let mut copy = Node::container();
    copy.style.descriptor.layout.direction = LayoutDirection::Column;
    copy.style.descriptor.layout.spacing.gap = rem_to_px(0.1875);
    copy.style.flex_fill = true;
    let mut title = Node::text(&item.title);
    title.style.descriptor.text_color = Some(ctx.theme().resolve_color("color.text.primary"));
    title.style.text_size = Some(ctx.theme().resolve_space("typography.label.size"));
    title.style.text_weight = Some(if item.read { 500 } else { 650 });
    copy = copy.child(title);

    if let Some(message) = item.message.as_deref() {
        let mut body = Node::text(message);
        body.style.descriptor.text_color = Some(ctx.theme().resolve_color("color.text.secondary"));
        body.style.text_size = Some(ctx.theme().resolve_space("typography.body.size"));
        body.style.text_wrap = true;
        body.style.line_height = Some(1.35);
        copy = copy.child(body);
    }

    if item.meta.is_some() || item.timestamp.is_some() {
        let mut meta = Node::container();
        meta.style.descriptor.layout.direction = LayoutDirection::Row;
        meta.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        meta.style.descriptor.layout.spacing.gap = rem_to_px(0.25);
        if let Some(value) = item.meta.as_deref() {
            let mut label = Node::text(value);
            label.style.descriptor.text_color = Some(ctx.theme().resolve_color("color.text.secondary"));
            label.style.text_size = Some(ctx.theme().resolve_space("typography.caption.size"));
            meta = meta.child(label);
        }
        if item.meta.is_some() && item.timestamp.is_some() {
            let mut separator = Node::text("·");
            separator.style.descriptor.text_color =
                Some(ctx.theme().resolve_color("color.text.secondary"));
            meta = meta.child(separator);
        }
        if let Some(timestamp) = item.timestamp.as_deref() {
            let time_spec = TimeAgoSpec::new()
                .with_timestamp(timestamp)
                .with_short(true)
                .with_typography(InlineTypographyMode::Inherit);
            meta = meta.child(time_ago(&time_spec, ctx));
        }
        copy = copy.child(meta);
    }

    if let Some(item_progress) = item.progress.as_ref() {
        let progress_spec = {
            let mut spec = ProgressSpec::new()
                .with_size(ControlSize::Xs)
                .with_size_role(SemanticControlSizeRole::Control)
                .with_density(density);
            if item_progress.indeterminate {
                spec = spec.with_indeterminate(true);
            } else if let Some(value) = item_progress.value {
                spec = spec.with_value(value);
            }
            if item_progress.max != 100.0 {
                spec.max = item_progress.max;
            }
            spec.aria_label = Some(format!("{} progress", item.title));
            spec
        };
        copy = copy.child(progress(&progress_spec, ctx));
    }
    content.child(copy)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn closed_center_only_renders_trigger_and_unread_indicator() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = message_center(
            &MessageCenterSpec::new(vec![MessageCenterItem::new("one", "Build complete")]),
            &ctx,
            MessageCenterHandlers::default(),
        );
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].children.len(), 2);
        assert_eq!(
            node.style.descriptor.layout.width,
            LayoutSizing::Fixed(rem_to_px(1.75))
        );
        let badge = &node.children[0].children[1];
        assert_eq!(
            badge.style.descriptor.layout.height,
            LayoutSizing::Fixed(rem_to_px(0.85))
        );
        assert_eq!(badge.style.descriptor.border.width, rem_to_px(0.125));
        assert_eq!(
            badge.style.descriptor.layout.alignment.main,
            MainAxisAlignment::Center
        );
    }

    #[test]
    fn open_center_renders_dialog_list_and_item_controls() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = MessageCenterSpec::new(vec![MessageCenterItem::new("one", "Build complete")])
            .with_open(true);
        let handlers = MessageCenterHandlers {
            on_read_change: Some(Arc::new(|_, _| {})),
            on_remove: Some(Arc::new(|_| {})),
            ..Default::default()
        };
        let node = message_center(&spec, &ctx, handlers);
        assert_eq!(node.children.len(), 2);
        assert_eq!(
            node.children[1].children[0].a11y.role,
            Some(NodeRole::Dialog)
        );
            // The surface's inner padded wrapper carries the panel spacing;
            // the message centre content sits inside it.
            let content = &node.children[1].children[0].children[0].children[0];
        assert_eq!(
            content.style.descriptor.layout.width,
            LayoutSizing::Fixed(rem_to_px(28.0))
        );
        assert!(content.children[0].style.border_bottom_width.is_some());
    }

    #[test]
    fn mixed_feed_renders_progress_and_respects_item_policies() {
        use poodle_node::NodeKind;
        use poodle_specs::MessageCenterItemProgress;

        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = MessageCenterSpec::new(vec![
            MessageCenterItem::new("job", "Mix preview")
                .with_meta("Rendering")
                .with_progress(MessageCenterItemProgress::determinate(60.0))
                .as_live_row(),
            MessageCenterItem::new("upload", "Uploading stems")
                .with_progress(MessageCenterItemProgress::indeterminate())
                .as_live_row(),
            MessageCenterItem::new("render", "Render complete")
                .with_tone(poodle_specs::StatusTone::Success),
        ])
        .with_open(true);
        let handlers = MessageCenterHandlers {
            on_item_select: Some(Arc::new(|_| {})),
            on_read_change: Some(Arc::new(|_, _| {})),
            on_remove: Some(Arc::new(|_| {})),
            ..Default::default()
        };
        let node = message_center(&spec, &ctx, handlers);

            // The surface's inner padded wrapper carries the panel spacing;
            // the message centre content sits inside it.
            let content = &node.children[1].children[0].children[0].children[0];
        let list = &content.children[1];
        assert_eq!(list.children.len(), 3);

        // Live determinate row: not a button, no action controls, progress fill.
        let job_row = &list.children[0];
        assert!(matches!(job_row.children[0].kind, NodeKind::Container));
        assert_eq!(job_row.children.len(), 1, "live row carries no actions");
        let job_copy = &job_row.children[0].children[1];
        let job_progress = job_copy
            .children
            .iter()
            .find(|child| child.a11y.role == Some(NodeRole::ProgressIndicator))
            .expect("determinate live row renders a progress node");
        assert!(matches!(
            job_progress.kind,
            NodeKind::Progress { fraction } if (fraction - 0.6).abs() < 0.001
        ));
        assert_eq!(
            job_progress.a11y.label.as_deref(),
            Some("Mix preview progress")
        );

        // Live indeterminate row: progress indicator without a fill fraction.
        let upload_row = &list.children[1];
        assert_eq!(upload_row.children.len(), 1);
        let upload_copy = &upload_row.children[0].children[1];
        let upload_progress = upload_copy
            .children
            .iter()
            .find(|child| child.a11y.role == Some(NodeRole::ProgressIndicator))
            .expect("indeterminate live row renders a progress node");
        assert!(matches!(upload_progress.kind, NodeKind::Container));

        // Durable message keeps full interaction: button content plus two controls.
        let message_row = &list.children[2];
        assert!(matches!(
            message_row.children[0].kind,
            NodeKind::Button { .. }
        ));
        assert_eq!(message_row.children.len(), 2, "durable row keeps actions");
        assert_eq!(message_row.children[1].children.len(), 2);

        // Live rows never inflate the unread count.
        assert_eq!(spec.unread_count(), 1);
    }

    #[test]
    fn progress_updates_render_in_place_without_local_authority() {
        use poodle_node::NodeKind;
        use poodle_specs::MessageCenterItemProgress;

        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec_at = |value: f64| {
            MessageCenterSpec::new(vec![MessageCenterItem::new("job", "Mix preview")
                .with_progress(MessageCenterItemProgress::determinate(value))
                .as_live_row()])
            .with_open(true)
        };

        let early = message_center(&spec_at(20.0), &ctx, MessageCenterHandlers::default());
        let late = message_center(&spec_at(80.0), &ctx, MessageCenterHandlers::default());

        let fraction = |node: &Node| {
            node.children[1].children[0].children[0].children[0].children[1].children[0].children[0].children[1]
                .children
                .iter()
                .find_map(|child| match child.kind {
                    NodeKind::Progress { fraction } => Some(fraction),
                    _ => None,
                })
                .expect("progress node present")
        };
        assert!((fraction(&early) - 0.2).abs() < 0.001);
        assert!((fraction(&late) - 0.8).abs() < 0.001);
    }
}
