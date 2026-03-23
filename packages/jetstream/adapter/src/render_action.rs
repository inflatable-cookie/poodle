//! RenderComponent implementations for action primitives.
//!
//! g08.004: ButtonSpec, IconButtonSpec, FormActionsSpec, ToolbarSpec

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_primitives::{ButtonSpec, FormActionsSpec, IconButtonSpec, ToolbarSpec};
use poodle_style::StyleDescriptor;

use crate::style_map::{map_style, JetstreamColor};
use crate::{JetstreamAdapter, JetstreamNodeHandle, JetstreamTarget, WidgetKind};

impl RenderComponent<ButtonSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &ButtonSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve fill (background) color from variant
        let fill_color = theme.resolve_color(spec.resolved_fill_token());
        mapped.visuals.background = Some(JetstreamColor::from(fill_color));

        // Resolve text color from variant
        let text_color = theme.resolve_color(spec.resolved_text_token());
        mapped.visuals.text_color = Some(JetstreamColor::from(text_color));

        // Resolve border color from variant
        let border_color = theme.resolve_color(spec.resolved_border_token());
        mapped.visuals.border_color = Some(JetstreamColor::from(border_color));

        // Resolve corner radius
        let r = theme.resolve_radius(spec.radius_token());
        mapped.visuals.corner_radii = [r; 4];

        // Resolve focus ring
        let ring_color = theme.resolve_color(spec.focus_ring_color_token());
        mapped.visuals.focus_ring_color = Some(JetstreamColor::from(ring_color));
        mapped.visuals.focus_ring_width = theme.resolve_space(spec.focus_ring_width_token());

        // Handle disabled state
        if spec.is_disabled {
            mapped.visuals.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        }

        let node_id = match &spec.label {
            Some(label) if !label.is_empty() => format!("button-{}", label),
            _ => "button".to_string(),
        };
        JetstreamNodeHandle::new(node_id, "ButtonSpec", WidgetKind::Button, mapped)
    }
}

impl RenderComponent<IconButtonSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &IconButtonSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mapped = map_style(style);

        // Resolve icon size token (used for width/height constraints)
        let _icon_size = theme.resolve_space(spec.icon_size_token());

        // Resolve control height token
        let _control_height = theme.resolve_space(spec.control_height_token());

        let node_id = match &spec.icon {
            Some(icon) if !icon.is_empty() => format!("icon-button-{}", icon),
            _ => "icon-button".to_string(),
        };

        JetstreamNodeHandle::new(node_id, "IconButtonSpec", WidgetKind::Button, mapped)
    }
}

impl RenderComponent<FormActionsSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &FormActionsSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve action gap (space between action buttons)
        let gap = theme.resolve_space(spec.action_gap_token());
        mapped.layout.gap = taffy::Size { width: taffy::LengthPercentage::length(gap), height: taffy::LengthPercentage::length(gap) };

        // Resolve stack separation (vertical space above the actions bar)
        let _stack_sep = theme.resolve_space(spec.stack_separation_token());

        JetstreamNodeHandle::new("form-actions", "FormActionsSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<ToolbarSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &ToolbarSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve gap between toolbar items
        let gap = theme.resolve_space(spec.gap_token());
        mapped.layout.gap = taffy::Size { width: taffy::LengthPercentage::length(gap), height: taffy::LengthPercentage::length(gap) };

        // Resolve separator border if present
        if spec.has_separator {
            let border_color = theme.resolve_color(spec.border_token());
            mapped.visuals.border_color = Some(JetstreamColor::from(border_color));
            mapped.visuals.border_width = 1.0;
        }

        let node_id = match &spec.aria_label {
            Some(label) if !label.is_empty() => format!("toolbar-{}", label),
            _ => "toolbar".to_string(),
        };
        JetstreamNodeHandle::new(node_id, "ToolbarSpec", WidgetKind::Panel, mapped)
    }
}

#[cfg(test)]
mod tests {
    use poodle_adapter::RenderComponent;
    use poodle_primitives::*;
    use poodle_style::StyleDescriptor;
    use crate::{JetstreamAdapter, WidgetKind, theme::JetstreamThemeProvider};

    fn a() -> JetstreamAdapter { JetstreamAdapter::new(JetstreamThemeProvider::default()) }
    fn s() -> StyleDescriptor { StyleDescriptor::new() }
    fn t() -> JetstreamThemeProvider { JetstreamThemeProvider::default() }

    #[test]
    fn button_with_label_uses_label_in_id() {
        let spec = ButtonSpec::new().with_label("Save");
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "button-Save");
        assert_eq!(h.spec_type, "ButtonSpec");
        assert_eq!(h.widget_kind, WidgetKind::Button);
    }

    #[test]
    fn button_without_label_defaults_id() {
        let h = a().render(&ButtonSpec::new(), &s(), &t());
        assert_eq!(h.node_id, "button");
    }

    #[test]
    fn button_disabled_renders() {
        let spec = ButtonSpec::new().with_disabled(true).with_label("Submit");
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "button-Submit");
        assert_eq!(h.widget_kind, WidgetKind::Button);
    }

    #[test]
    fn icon_button_with_icon_uses_icon_in_id() {
        let spec = IconButtonSpec::new().with_icon("close");
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "icon-button-close");
        assert_eq!(h.spec_type, "IconButtonSpec");
        assert_eq!(h.widget_kind, WidgetKind::Button);
    }

    #[test]
    fn icon_button_without_icon_defaults_id() {
        let h = a().render(&IconButtonSpec::new(), &s(), &t());
        assert_eq!(h.node_id, "icon-button");
    }

    #[test]
    fn form_actions_renders_with_gap() {
        let h = a().render(&FormActionsSpec::new(), &s(), &t());
        assert_eq!(h.node_id, "form-actions");
        assert_eq!(h.spec_type, "FormActionsSpec");
        assert_eq!(h.widget_kind, WidgetKind::Panel);
    }

    #[test]
    fn toolbar_with_aria_label_uses_label_in_id() {
        let spec = ToolbarSpec::new().with_aria_label("Main Tools");
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "toolbar-Main Tools");
        assert_eq!(h.spec_type, "ToolbarSpec");
        assert_eq!(h.widget_kind, WidgetKind::Panel);
    }

    #[test]
    fn toolbar_without_label_defaults_id() {
        let h = a().render(&ToolbarSpec::new(), &s(), &t());
        assert_eq!(h.node_id, "toolbar");
    }

    #[test]
    fn toolbar_with_separator_renders() {
        let spec = ToolbarSpec::new().with_separator(true);
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "toolbar");
        assert_eq!(h.widget_kind, WidgetKind::Panel);
    }
}
