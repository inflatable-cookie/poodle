//! RenderComponent implementations for selection, value, feedback, and temporal primitives.
//!
//! g07.004: Checkbox, RadioGroup, Switch, Select, SegmentedControl, Slider,
//! RangeSlider, Progress, Badge, StatusIndicator, Meter, Rating, Skeleton,
//! TriStateSwitch
//!
//! NOTE: style objects (`s`) are built and mutated as proof of token resolution
//! but are not yet wired into the returned `GpuiElementHandle`. Suppressed until
//! the GPUI integration is wired up.
#![allow(unused_variables, unused_assignments)]

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_specs::{
    BadgeSpec, CheckboxSpec, MeterSpec, ProgressSpec, RadioGroupSpec, RangeSliderSpec, RatingSpec,
    SegmentedControlSpec, SelectSpec, SkeletonSpec, SliderSpec, StatusIndicatorSpec, SwitchSpec,
    TriStateSwitchSpec,
};
use poodle_style::StyleDescriptor;

use crate::style_map::map_style;
use crate::{GpuiAdapter, GpuiElementHandle, GpuiTarget};

impl RenderComponent<CheckboxSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        spec: &CheckboxSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut s = map_style(style);
        let fill = theme.resolve_color(spec.indicator_fill_token());
        s.background = Some(fill.into());
        GpuiElementHandle::new("checkbox", "CheckboxSpec")
    }
}

impl RenderComponent<RadioGroupSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        spec: &RadioGroupSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut s = map_style(style);
        s.gap = theme.resolve_space(spec.option_gap_token());
        GpuiElementHandle::new("radio-group", "RadioGroupSpec")
    }
}

impl RenderComponent<SwitchSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        spec: &SwitchSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut s = map_style(style);
        let track = theme.resolve_color(spec.track_fill_token());
        s.background = Some(track.into());
        GpuiElementHandle::new("switch", "SwitchSpec")
    }
}

impl RenderComponent<SelectSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        spec: &SelectSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut s = map_style(style);
        let overlay_fill = theme.resolve_color(spec.overlay_fill_token());
        s.background = Some(overlay_fill.into());
        GpuiElementHandle::new("select", "SelectSpec")
    }
}

impl RenderComponent<SegmentedControlSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        spec: &SegmentedControlSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut s = map_style(style);
        let selected = theme.resolve_color(spec.selected_fill_token());
        s.background = Some(selected.into());
        GpuiElementHandle::new("segmented-control", "SegmentedControlSpec")
    }
}

impl RenderComponent<SliderSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        spec: &SliderSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut s = map_style(style);
        let fill = theme.resolve_color(spec.range_fill_token());
        s.background = Some(fill.into());
        GpuiElementHandle::new("slider", "SliderSpec")
    }
}

impl RenderComponent<RangeSliderSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        spec: &RangeSliderSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut s = map_style(style);
        let range_fill = theme.resolve_color(spec.range_fill_token());
        s.background = Some(range_fill.into());
        let track_fill = theme.resolve_color(spec.track_fill_token());
        s.border_color = Some(track_fill.into());
        GpuiElementHandle::new("range-slider", "RangeSliderSpec")
    }
}

impl RenderComponent<ProgressSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        spec: &ProgressSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut s = map_style(style);
        let fill = theme.resolve_color(spec.indicator_fill_token());
        s.background = Some(fill.into());
        GpuiElementHandle::new("progress", "ProgressSpec")
    }
}

impl RenderComponent<BadgeSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        spec: &BadgeSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut s = map_style(style);
        let fill = theme.resolve_color(spec.fill_token());
        s.background = Some(fill.into());
        GpuiElementHandle::new("badge", "BadgeSpec")
    }
}

impl RenderComponent<StatusIndicatorSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        spec: &StatusIndicatorSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut s = map_style(style);
        let color = theme.resolve_color(spec.status_color_token());
        s.background = Some(color.into());
        GpuiElementHandle::new("status-indicator", "StatusIndicatorSpec")
    }
}

impl RenderComponent<MeterSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        spec: &MeterSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut s = map_style(style);
        let fill = theme.resolve_color(spec.fill_token());
        s.background = Some(fill.into());
        let track = theme.resolve_color(spec.track_fill_token());
        s.border_color = Some(track.into());
        GpuiElementHandle::new("meter", "MeterSpec")
    }
}

impl RenderComponent<RatingSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        spec: &RatingSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut s = map_style(style);
        let _var_active = theme.resolve_color(spec.active_color_token());
        let inactive = theme.resolve_color(spec.inactive_color_token());
        s.border_color = Some(inactive.into());
        GpuiElementHandle::new("rating", "RatingSpec")
    }
}

impl RenderComponent<SkeletonSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        spec: &SkeletonSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut s = map_style(style);
        let fill = theme.resolve_color(spec.fill_token());
        s.background = Some(fill.into());
        let radius = theme.resolve_space(spec.radius_token());
        s.corner_radii.top_left = radius;
        s.corner_radii.top_right = radius;
        s.corner_radii.bottom_left = radius;
        s.corner_radii.bottom_right = radius;
        GpuiElementHandle::new("skeleton", "SkeletonSpec")
    }
}

impl RenderComponent<TriStateSwitchSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        spec: &TriStateSwitchSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut s = map_style(style);
        let track = theme.resolve_color(spec.root_bg_token());
        s.background = Some(track.into());
        GpuiElementHandle::new("tri-state-switch", "TriStateSwitchSpec")
    }
}

#[cfg(test)]
mod tests {
    use crate::{theme::GpuiThemeProvider, GpuiAdapter};
    use poodle_adapter::RenderComponent;
    use poodle_specs::*;
    use poodle_style::StyleDescriptor;

    fn a() -> GpuiAdapter {
        GpuiAdapter::new(GpuiThemeProvider::default())
    }
    fn s() -> StyleDescriptor {
        StyleDescriptor::new()
    }
    fn t() -> GpuiThemeProvider {
        GpuiThemeProvider::default()
    }

    #[test]
    fn checkbox() {
        assert_eq!(
            a().render(&CheckboxSpec::new(), &s(), &t()).spec_type,
            "CheckboxSpec"
        );
    }
    #[test]
    fn radio_group() {
        assert_eq!(
            a().render(&RadioGroupSpec::new(vec![]), &s(), &t())
                .spec_type,
            "RadioGroupSpec"
        );
    }
    #[test]
    fn switch() {
        assert_eq!(
            a().render(&SwitchSpec::new(), &s(), &t()).spec_type,
            "SwitchSpec"
        );
    }
    #[test]
    fn select() {
        assert_eq!(
            a().render(&SelectSpec::new(vec![]), &s(), &t()).spec_type,
            "SelectSpec"
        );
    }
    #[test]
    fn segmented_control() {
        assert_eq!(
            a().render(
                &SegmentedControlSpec::new("selection-test", vec![]),
                &s(),
                &t()
            )
            .spec_type,
            "SegmentedControlSpec"
        );
    }
    #[test]
    fn slider() {
        assert_eq!(
            a().render(&SliderSpec::new(50.0), &s(), &t()).spec_type,
            "SliderSpec"
        );
    }
    #[test]
    fn range_slider() {
        assert_eq!(
            a().render(&RangeSliderSpec::new(0.0, 100.0), &s(), &t())
                .spec_type,
            "RangeSliderSpec"
        );
    }
    #[test]
    fn progress() {
        assert_eq!(
            a().render(&ProgressSpec::new(), &s(), &t()).spec_type,
            "ProgressSpec"
        );
    }
    #[test]
    fn badge() {
        assert_eq!(
            a().render(&BadgeSpec::new(), &s(), &t()).spec_type,
            "BadgeSpec"
        );
    }
    #[test]
    fn status_indicator() {
        assert_eq!(
            a().render(&StatusIndicatorSpec::new(), &s(), &t())
                .spec_type,
            "StatusIndicatorSpec"
        );
    }
    #[test]
    fn meter() {
        assert_eq!(
            a().render(&MeterSpec::new(), &s(), &t()).spec_type,
            "MeterSpec"
        );
    }
    #[test]
    fn rating() {
        assert_eq!(
            a().render(&RatingSpec::new(), &s(), &t()).spec_type,
            "RatingSpec"
        );
    }
    #[test]
    fn skeleton() {
        assert_eq!(
            a().render(&SkeletonSpec::new(), &s(), &t()).spec_type,
            "SkeletonSpec"
        );
    }
    #[test]
    fn tri_state_switch() {
        assert_eq!(
            a().render(&TriStateSwitchSpec::new(), &s(), &t()).spec_type,
            "TriStateSwitchSpec"
        );
    }
}
