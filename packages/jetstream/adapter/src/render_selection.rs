//! RenderComponent implementations for selection primitives.
//!
//! g08.006: CheckboxSpec, RadioGroupSpec, SwitchSpec, SelectSpec, SliderSpec,
//! RangeSliderSpec, SegmentedControlSpec, TriStateSwitchSpec

use pug_adapter::{RenderComponent, ThemeProvider};
use pug_primitives::{
    CheckboxSpec, RadioGroupSpec, RangeSliderSpec, SegmentedControlSpec, SelectSpec, SliderSpec,
    SwitchSpec, TriStateSwitchSpec,
};
use pug_style::StyleDescriptor;

use crate::style_map::{map_style, JetstreamColor};
use crate::{JetstreamAdapter, JetstreamNodeHandle, JetstreamTarget, WidgetKind};

impl RenderComponent<CheckboxSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &CheckboxSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut s = map_style(style);
        let c = theme.resolve_color(spec.indicator_fill_token());
        s.background = Some(JetstreamColor::from(c));
        let id = match &spec.label {
            Some(label) => format!("checkbox-{}", label.to_lowercase().replace(' ', "-")),
            None => "checkbox".to_string(),
        };
        JetstreamNodeHandle::new(id, "CheckboxSpec", WidgetKind::Button)
    }
}

impl RenderComponent<RadioGroupSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &RadioGroupSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut s = map_style(style);
        s.gap = theme.resolve_space(spec.option_gap_token());
        JetstreamNodeHandle::new("radio-group", "RadioGroupSpec", WidgetKind::Panel)
    }
}

impl RenderComponent<SwitchSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &SwitchSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut s = map_style(style);
        let c = theme.resolve_color(spec.track_fill_token());
        s.background = Some(JetstreamColor::from(c));
        let id = match &spec.label {
            Some(label) => format!("switch-{}", label.to_lowercase().replace(' ', "-")),
            None => "switch".to_string(),
        };
        JetstreamNodeHandle::new(id, "SwitchSpec", WidgetKind::Button)
    }
}

impl RenderComponent<SelectSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &SelectSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut s = map_style(style);
        let c = theme.resolve_color(spec.overlay_fill_token());
        s.background = Some(JetstreamColor::from(c));
        JetstreamNodeHandle::new("select", "SelectSpec", WidgetKind::Button)
    }
}

impl RenderComponent<SliderSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &SliderSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut s = map_style(style);
        let c = theme.resolve_color(spec.range_fill_token());
        s.background = Some(JetstreamColor::from(c));
        s.opacity = spec.normalized_progress() as f32;
        JetstreamNodeHandle::new("slider", "SliderSpec", WidgetKind::Slider)
    }
}

impl RenderComponent<RangeSliderSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &RangeSliderSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut s = map_style(style);
        let range_c = theme.resolve_color(spec.range_fill_token());
        s.background = Some(JetstreamColor::from(range_c));
        let track_c = theme.resolve_color(spec.track_fill_token());
        s.border_color = Some(JetstreamColor::from(track_c));
        JetstreamNodeHandle::new("range-slider", "RangeSliderSpec", WidgetKind::Slider)
    }
}

impl RenderComponent<SegmentedControlSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &SegmentedControlSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut s = map_style(style);
        let c = theme.resolve_color(spec.selected_fill_token());
        s.background = Some(JetstreamColor::from(c));
        JetstreamNodeHandle::new("segmented-control", "SegmentedControlSpec", WidgetKind::Panel)
    }
}

impl RenderComponent<TriStateSwitchSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &TriStateSwitchSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut s = map_style(style);
        let c = theme.resolve_color(spec.track_fill_token());
        s.background = Some(JetstreamColor::from(c));
        JetstreamNodeHandle::new("tri-state-switch", "TriStateSwitchSpec", WidgetKind::Button)
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

    #[test] fn checkbox() { assert_eq!(a().render(&CheckboxSpec::new(), &s(), &t()).widget_kind, WidgetKind::Button); }
    #[test] fn checkbox_with_label() {
        let spec = CheckboxSpec::new().with_label("Accept Terms");
        assert_eq!(a().render(&spec, &s(), &t()).node_id, "checkbox-accept-terms");
    }
    #[test] fn radio_group() { assert_eq!(a().render(&RadioGroupSpec::new(vec![]), &s(), &t()).spec_type, "RadioGroupSpec"); }
    #[test] fn switch() { assert_eq!(a().render(&SwitchSpec::new(), &s(), &t()).widget_kind, WidgetKind::Button); }
    #[test] fn switch_with_label() {
        let mut spec = SwitchSpec::new();
        spec.label = Some("Dark Mode".to_string());
        assert_eq!(a().render(&spec, &s(), &t()).node_id, "switch-dark-mode");
    }
    #[test] fn select() { assert_eq!(a().render(&SelectSpec::new(vec![]), &s(), &t()).widget_kind, WidgetKind::Button); }
    #[test] fn slider() { assert_eq!(a().render(&SliderSpec::new(0.5), &s(), &t()).widget_kind, WidgetKind::Slider); }
    #[test] fn range_slider() { assert_eq!(a().render(&RangeSliderSpec::new(0.2, 0.8), &s(), &t()).widget_kind, WidgetKind::Slider); }
    #[test] fn segmented_control() { assert_eq!(a().render(&SegmentedControlSpec::new(vec![]), &s(), &t()).spec_type, "SegmentedControlSpec"); }
    #[test] fn tri_state_switch() { assert_eq!(a().render(&TriStateSwitchSpec::new(), &s(), &t()).spec_type, "TriStateSwitchSpec"); }
}
