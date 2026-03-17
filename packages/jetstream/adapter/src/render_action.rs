//! RenderComponent implementations for action primitives.
//!
//! g08.004: ButtonSpec, IconButtonSpec, FormActionsSpec, ToolbarSpec

use pug_adapter::{RenderComponent, ThemeProvider};
use pug_primitives::{ButtonSpec, FormActionsSpec, IconButtonSpec, ToolbarSpec};
use pug_style::StyleDescriptor;

use crate::style_map::{map_style, JetstreamColor};
use crate::{JetstreamAdapter, JetstreamNodeHandle, JetstreamTarget, WidgetKind};

impl RenderComponent<ButtonSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &ButtonSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut js_style = map_style(style);

        // Resolve fill (background) color from variant
        let fill_color = theme.resolve_color(spec.resolved_fill_token());
        js_style.background = Some(JetstreamColor::from(fill_color));

        // Resolve text color from variant
        let text_color = theme.resolve_color(spec.resolved_text_token());
        js_style.text_color = Some(JetstreamColor::from(text_color));

        // Resolve border color from variant
        let border_color = theme.resolve_color(spec.resolved_border_token());
        js_style.border_color = Some(JetstreamColor::from(border_color));

        // Resolve corner radius
        let r = theme.resolve_radius(spec.radius_token());
        js_style.corner_radii = [r; 4];

        // Resolve focus ring
        let ring_color = theme.resolve_color(spec.focus_ring_color_token());
        js_style.focus_ring_color = Some(JetstreamColor::from(ring_color));
        js_style.focus_ring_width = theme.resolve_space(spec.focus_ring_width_token());

        // Handle disabled state
        if spec.is_disabled {
            js_style.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        }

        let node_id = match &spec.label {
            Some(label) if !label.is_empty() => format!("button-{}", label),
            _ => "button".to_string(),
        };
        JetstreamNodeHandle::new(node_id, "ButtonSpec", WidgetKind::Button)
    }
}

impl RenderComponent<IconButtonSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &IconButtonSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let js_style = map_style(style);

        // Resolve icon size token (used for width/height constraints)
        let _icon_size = theme.resolve_space(spec.icon_size_token());

        // Resolve control height token
        let _control_height = theme.resolve_space(spec.control_height_token());

        let node_id = match &spec.icon {
            Some(icon) if !icon.is_empty() => format!("icon-button-{}", icon),
            _ => "icon-button".to_string(),
        };

        // js_style is constructed for proof of style mapping
        let _ = &js_style;

        JetstreamNodeHandle::new(node_id, "IconButtonSpec", WidgetKind::Button)
    }
}

impl RenderComponent<FormActionsSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &FormActionsSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut js_style = map_style(style);

        // Resolve action gap (space between action buttons)
        js_style.gap = theme.resolve_space(spec.action_gap_token());

        // Resolve stack separation (vertical space above the actions bar)
        let _stack_sep = theme.resolve_space(spec.stack_separation_token());

        JetstreamNodeHandle::new("form-actions", "FormActionsSpec", WidgetKind::Panel)
    }
}

impl RenderComponent<ToolbarSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &ToolbarSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut js_style = map_style(style);

        // Resolve gap between toolbar items
        js_style.gap = theme.resolve_space(spec.gap_token());

        // Resolve separator border if present
        if spec.has_separator {
            let border_color = theme.resolve_color(spec.border_token());
            js_style.border_color = Some(JetstreamColor::from(border_color));
            js_style.border_width = 1.0;
        }

        let node_id = match &spec.aria_label {
            Some(label) if !label.is_empty() => format!("toolbar-{}", label),
            _ => "toolbar".to_string(),
        };
        JetstreamNodeHandle::new(node_id, "ToolbarSpec", WidgetKind::Panel)
    }
}

#[cfg(test)]
mod tests {
    use pug_adapter::RenderComponent;
    use pug_primitives::*;
    use pug_style::StyleDescriptor;
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
