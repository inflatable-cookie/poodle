//! UpdateCenter — a titlebar trigger whose popover hosts UpdateStatus.
//!
//! Contract: `docs/contracts/components/update-center.md`
//!
//! Presence is an authority read. `hidden` collapses the tree entirely.
//! Attention draws the indicator dot; downloading replaces the glyph with a
//! progress ring (indeterminate when `fraction` is `None`).

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_headless::update::{update_download_label, UpdatePresence, UpdateProgressProjection};
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node, NodePosition,
};
use poodle_specs::{
    ButtonVariant, Dimension, IconButtonSpec, PopoverSpec, SemanticControlSizeRole, SpinnerSize,
    SpinnerSpec, SpinnerTone, SpinnerVariant, UpdateCenterSpec,
};

use crate::floating_overlay::floating_overlay;
use crate::icon_button::icon_button;
use crate::popover::popover_surface;
use crate::presentation::{control_height_rem, rem_to_px, resolve_semantic_size};
use crate::spinner::spinner;
use crate::update_status::{update_status, UpdateStatusHandlers};

#[derive(Default)]
pub struct UpdateCenterHandlers {
    pub instance_id: Option<String>,
    pub on_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub on_check: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_install: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_defer: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_confirm_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

pub fn update_center(
    spec: &UpdateCenterSpec,
    theme: &dyn ThemeProvider,
    handlers: UpdateCenterHandlers,
) -> Node {
    if spec.presence == UpdatePresence::Hidden {
        return Node::container();
    }

    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let anchor_size = rem_to_px(control_height_rem(effective_size));
    let open = spec.current_open();
    let downloading = matches!(
        spec.progress,
        Some(UpdateProgressProjection::Downloading { .. })
    );
    let download_fraction = match spec.progress {
        Some(UpdateProgressProjection::Downloading { fraction }) => fraction,
        _ => None,
    };
    let trigger_label = if downloading {
        update_download_label(download_fraction)
    } else {
        spec.effective_trigger_label().to_string()
    };

    let open_handler = handlers
        .on_open_change
        .clone()
        .map(|handler| Arc::new(move || handler(!open)) as Arc<dyn Fn() + Send + Sync>);
    let mut trigger_spec = IconButtonSpec::new()
        .with_variant(ButtonVariant::Ghost)
        .with_aria_label(trigger_label)
        .with_tooltip(&spec.title)
        .with_expanded(open)
        .with_size(effective_size)
        .with_size_role(SemanticControlSizeRole::Control)
        .with_density(spec.density);
    if !downloading {
        trigger_spec = trigger_spec.with_icon("download");
    }
    let mut trigger_button = icon_button(&trigger_spec, theme, open_handler);
    if let Some(scope) = &handlers.instance_id {
        trigger_button.id = Some(format!("{scope}-trigger"));
    } else {
        trigger_button.id = Some("update-center-trigger".to_string());
    }
    if let Some(fraction) = download_fraction {
        let mut ring = Node::progress_ring(fraction as f32);
        ring.style.descriptor.layout.width = LayoutSizing::Fixed(16.0);
        ring.style.descriptor.layout.height = LayoutSizing::Fixed(16.0);
        ring.style.descriptor.border.color = theme.resolve_color("color.border.subtle");
        ring.style.descriptor.text_color = Some(theme.resolve_color("color.accent.base"));
        trigger_button = trigger_button.child(ring);
    } else if downloading {
        trigger_button = trigger_button.child(spinner(
            &SpinnerSpec::new()
                .with_variant(SpinnerVariant::Ring)
                .with_size(SpinnerSize::Md)
                .with_tone(SpinnerTone::Accent),
            theme,
        ));
    }
    let trigger = trigger_with_attention(trigger_button, spec.presence, theme);

    let surface = open.then(|| {
        let mut header = Node::text(&spec.title);
        header.style.text_size = Some(rem_to_px(1.0));
        header.style.text_weight = Some(600);
        header.style.descriptor.text_color = Some(theme.resolve_color("color.text.primary"));

        let status = update_status(
            &spec.status_spec(),
            theme,
            UpdateStatusHandlers {
                instance_id: handlers.instance_id.clone(),
                on_check: handlers.on_check.clone(),
                on_install: handlers.on_install.clone(),
                on_defer: handlers.on_defer.clone(),
                on_confirm_open_change: handlers.on_confirm_open_change.clone(),
            },
        );

        let mut body = Node::container();
        {
            let s = &mut body.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = theme.resolve_space("space.stack.md");
        }
        let content = body.child(header).child(status);
        let popover_spec = PopoverSpec::new()
            .with_open(true)
            .with_placement(spec.placement)
            .with_aria_label(spec.effective_aria_label())
            .with_surface_min_width(Dimension::new("16rem"))
            .with_surface_max_width(Dimension::new("24rem"));
        popover_surface(&popover_spec, theme, Some(content))
    });

    let mut root = floating_overlay(
        trigger,
        surface,
        spec.placement,
        anchor_size,
        anchor_size,
        crate::floating_overlay::OVERLAY_GAP_PX,
    );
    root.roles.insert(
        "presence".to_owned(),
        match spec.presence {
            UpdatePresence::Hidden => "hidden".to_string(),
            UpdatePresence::Quiet => "quiet".to_string(),
            UpdatePresence::Attention => "attention".to_string(),
        },
    );
    root
}

fn trigger_with_attention(
    trigger: Node,
    presence: UpdatePresence,
    theme: &dyn ThemeProvider,
) -> Node {
    let mut wrapper = Node::container();
    wrapper.position = NodePosition::Relative;
    wrapper.style.descriptor.layout.direction = LayoutDirection::Row;
    wrapper = wrapper.child(trigger);

    if presence != UpdatePresence::Attention {
        return wrapper;
    }

    let mut dot = Node::container();
    dot.position = NodePosition::Absolute {
        top: Some(-rem_to_px(0.1)),
        left: None,
        right: Some(-rem_to_px(0.1)),
        bottom: None,
    };
    {
        let s = &mut dot.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(0.5));
        s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(0.5));
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.background = Some(theme.resolve_color("color.accent.base"));
        s.descriptor.corner_radii.top_left = 999.0;
        s.descriptor.corner_radii.top_right = 999.0;
        s.descriptor.corner_radii.bottom_right = 999.0;
        s.descriptor.corner_radii.bottom_left = 999.0;
    }
    wrapper.child(dot)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::update::{
        OfferReason, UpdateAvailabilityProjection, UpdateControllerStatus,
    };

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn texts(node: &Node) -> Vec<String> {
        node.texts()
            .into_iter()
            .map(str::to_string)
            .filter(|t| !t.is_empty())
            .collect()
    }

    fn offer() -> UpdateAvailabilityProjection {
        UpdateAvailabilityProjection::Offer {
            version: "1.4.0".to_string(),
            reason: OfferReason::Staged,
            notes: None,
        }
    }

    #[test]
    fn hidden_presence_collapses_the_tree() {
        let node = update_center(
            &UpdateCenterSpec::new(UpdatePresence::Hidden)
                .with_status(UpdateControllerStatus::Ready)
                .with_availability(offer()),
            &theme(),
            UpdateCenterHandlers::default(),
        );
        assert!(node.children.is_empty());
        assert!(texts(&node).is_empty());
        assert!(node.roles.get("presence").is_none());
    }

    #[test]
    fn quiet_renders_a_trigger_without_an_attention_dot() {
        let node = update_center(
            &UpdateCenterSpec::new(UpdatePresence::Quiet)
                .with_status(UpdateControllerStatus::Ready)
                .with_availability(offer()),
            &theme(),
            UpdateCenterHandlers::default(),
        );
        assert_eq!(
            node.roles.get("presence").map(String::as_str),
            Some("quiet")
        );
        assert_eq!(
            node.children.len(),
            1,
            "quiet is the trigger only — no attention dot"
        );
    }

    #[test]
    fn attention_draws_the_indicator_and_open_hosts_update_status() {
        let node = update_center(
            &UpdateCenterSpec::new(UpdatePresence::Attention)
                .with_status(UpdateControllerStatus::Ready)
                .with_availability(offer())
                .with_open(true),
            &theme(),
            UpdateCenterHandlers::default(),
        );
        assert_eq!(
            node.roles.get("presence").map(String::as_str),
            Some("attention")
        );
        let rendered = texts(&node);
        assert!(rendered.iter().any(|t| t == "Updates"));
        assert!(rendered.iter().any(|t| t == "Version 1.4.0 is available"));
    }

    #[test]
    fn downloading_replaces_the_glyph_with_distinct_progress_rings() {
        let determinate = update_center(
            &UpdateCenterSpec::new(UpdatePresence::Quiet)
                .with_progress(UpdateProgressProjection::Downloading {
                    fraction: Some(0.42),
                }),
            &theme(),
            UpdateCenterHandlers::default(),
        );
        fn find_label(node: &Node) -> Option<&str> {
            if node.id.as_deref() == Some("update-center-trigger") {
                return node.a11y.label.as_deref();
            }
            node.children.iter().find_map(find_label)
        }
        assert_eq!(
            find_label(&determinate),
            Some("Downloading update, 42%")
        );
        assert!(determinate
            .find(&|node| {
                matches!(node.kind, poodle_node::NodeKind::ProgressRing { fraction } if fraction == 0.42)
            })
            .is_some());
        assert!(determinate
            .find(&|node| {
                matches!(&node.kind, poodle_node::NodeKind::Icon { name, .. } if name == "download" || name == "spinner")
            })
            .is_none());

        let indeterminate = update_center(
            &UpdateCenterSpec::new(UpdatePresence::Quiet)
                .with_progress(UpdateProgressProjection::Downloading { fraction: None }),
            &theme(),
            UpdateCenterHandlers::default(),
        );
        assert!(indeterminate
            .find(&|node| {
                matches!(&node.kind, poodle_node::NodeKind::Icon { name, .. } if name == "spinner")
                    && node.style.animation.is_some()
            })
            .is_some());
        assert!(indeterminate
            .find(&|node| {
                matches!(&node.kind, poodle_node::NodeKind::Icon { name, .. } if name == "download")
                    || matches!(node.kind, poodle_node::NodeKind::ProgressRing { .. })
            })
            .is_none());
    }

    #[test]
    fn non_default_size_role_keeps_anchor_and_trigger_aligned() {
        let node = update_center(
            &UpdateCenterSpec::new(UpdatePresence::Quiet)
                .with_size(poodle_specs::ControlSize::Md)
                .with_size_role(SemanticControlSizeRole::Control),
            &theme(),
            UpdateCenterHandlers::default(),
        );
        let trigger = node
            .find(&|child| child.id.as_deref() == Some("update-center-trigger"))
            .expect("trigger");
        assert_eq!(node.style.descriptor.layout.width, trigger.style.descriptor.layout.width);
        assert_eq!(node.style.descriptor.layout.height, trigger.style.descriptor.layout.height);
    }
}
