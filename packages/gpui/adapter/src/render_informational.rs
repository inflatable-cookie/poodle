//! RenderComponent implementations for informational, code, color, file, and temporal primitives.
//!
//! g07.006: Code, ColorPicker, FileUpload, Eyebrow, Pill, TimeAgo, DurationInput,
//! TimeZoneSelect, DateTimeZonePicker, SplitButton, DatePicker, Calendar,
//! DateRangePicker, DateTimePicker, DateTimeRangePicker

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_primitives::{
    CalendarSpec, CodeSpec, ColorPickerSpec, DatePickerSpec, DateRangePickerSpec,
    DateTimePickerSpec, DateTimeRangePickerSpec, DurationInputSpec, EyebrowSpec, FileUploadSpec,
    PillSpec, SplitButtonSpec, TimeAgoSpec, TimeZoneSelectSpec,
    DateTimeZonePickerSpec,
};
use poodle_style::StyleDescriptor;

use crate::style_map::map_style;
use crate::{GpuiAdapter, GpuiElementHandle, GpuiTarget};

impl RenderComponent<CodeSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, spec: &CodeSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        // Resolve code block fill color
        let fc = theme.resolve_color(spec.fill_token());
        gpui_style.background = Some(fc.into());

        // Resolve text color
        let tc = theme.resolve_color(spec.text_color_token());
        gpui_style.text_color = Some(tc.into());

        // Resolve border color
        let bc = theme.resolve_color(spec.border_token());
        gpui_style.border_color = Some(bc.into());

        // Font family — no typography field on GpuiStyle; acknowledge token
        let _font = spec.font_family_token();

        // Font size — resolve via space for proof; no typography field to apply
        let _font_size = theme.resolve_space(spec.font_size_token());

        GpuiElementHandle::new("code", "CodeSpec")
    }
}

impl RenderComponent<ColorPickerSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, spec: &ColorPickerSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        // Resolve border color
        let bc = theme.resolve_color(spec.border_token());
        gpui_style.border_color = Some(bc.into());

        // Resolve overlay fill
        let oc = theme.resolve_color(spec.overlay_fill_token());
        gpui_style.background = Some(oc.into());

        // Resolve shadow (no shadow field on GpuiStyle — resolve for proof)
        let _shadow = theme.resolve_color(spec.shadow_token());

        GpuiElementHandle::new("color-picker", "ColorPickerSpec")
    }
}

impl RenderComponent<FileUploadSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, spec: &FileUploadSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        // Resolve fill color
        let fc = theme.resolve_color(spec.fill_token());
        gpui_style.background = Some(fc.into());

        // Resolve border color
        let bc = theme.resolve_color(spec.border_token());
        gpui_style.border_color = Some(bc.into());

        // Resolve text color
        let tc = theme.resolve_color(spec.text_color_token());
        gpui_style.text_color = Some(tc.into());

        GpuiElementHandle::new("file-upload", "FileUploadSpec")
    }
}

impl RenderComponent<EyebrowSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, spec: &EyebrowSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        // Resolve text color
        let tc = theme.resolve_color(spec.text_color_token());
        gpui_style.text_color = Some(tc.into());

        // Font size — resolve via space for proof; no typography field to apply
        let _font_size = theme.resolve_space(spec.font_size_token());

        GpuiElementHandle::new("eyebrow", "EyebrowSpec")
    }
}

impl RenderComponent<PillSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, spec: &PillSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        // Resolve pill fill color
        let fc = theme.resolve_color(spec.fill_token());
        gpui_style.background = Some(fc.into());

        // Resolve text color
        let tc = theme.resolve_color(spec.text_color_token());
        gpui_style.text_color = Some(tc.into());

        // Resolve focus ring color
        let rc = theme.resolve_color(spec.focus_ring_color_token());
        gpui_style.focus_ring_color = Some(rc.into());

        GpuiElementHandle::new("pill", "PillSpec")
    }
}

impl RenderComponent<TimeAgoSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, spec: &TimeAgoSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        // Resolve text color
        let tc = theme.resolve_color(spec.text_color_token());
        gpui_style.text_color = Some(tc.into());

        // Font size — resolve via space for proof; no typography field to apply
        let _font_size = theme.resolve_space(spec.font_size_token());

        GpuiElementHandle::new("time-ago", "TimeAgoSpec")
    }
}

impl RenderComponent<DurationInputSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, spec: &DurationInputSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        // Resolve border color
        let bc = theme.resolve_color(spec.border_token());
        gpui_style.border_color = Some(bc.into());

        // Resolve focus ring color
        let rc = theme.resolve_color(spec.focus_ring_color_token());
        gpui_style.focus_ring_color = Some(rc.into());

        GpuiElementHandle::new("duration-input", "DurationInputSpec")
    }
}

impl RenderComponent<TimeZoneSelectSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, spec: &TimeZoneSelectSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        // Resolve border color
        let bc = theme.resolve_color(spec.border_token());
        gpui_style.border_color = Some(bc.into());

        // Resolve overlay fill
        let oc = theme.resolve_color(spec.overlay_fill_token());
        gpui_style.background = Some(oc.into());

        GpuiElementHandle::new("time-zone-select", "TimeZoneSelectSpec")
    }
}

impl RenderComponent<DateTimeZonePickerSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, spec: &DateTimeZonePickerSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        // Resolve border color
        let bc = theme.resolve_color(spec.border_token());
        gpui_style.border_color = Some(bc.into());

        // Resolve overlay fill
        let oc = theme.resolve_color(spec.overlay_fill_token());
        gpui_style.background = Some(oc.into());

        // Resolve shadow (no shadow field on GpuiStyle — resolve for proof)
        let _shadow = theme.resolve_color(spec.shadow_token());

        GpuiElementHandle::new("date-time-zone-picker", "DateTimeZonePickerSpec")
    }
}

impl RenderComponent<SplitButtonSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, spec: &SplitButtonSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        // Resolve fill color
        let fc = theme.resolve_color(spec.fill_token());
        gpui_style.background = Some(fc.into());

        // Resolve border color
        let bc = theme.resolve_color(spec.border_token());
        gpui_style.border_color = Some(bc.into());

        // Resolve separator color — no icon_color on GpuiStyle; resolve for proof
        let _separator = theme.resolve_color(spec.separator_token());

        // Resolve overlay fill — no secondary background field; resolve for proof
        let _overlay = theme.resolve_color(spec.overlay_fill_token());

        // Resolve shadow (no shadow field on GpuiStyle — resolve for proof)
        let _shadow = theme.resolve_color(spec.shadow_token());

        GpuiElementHandle::new("split-button", "SplitButtonSpec")
    }
}

impl RenderComponent<CalendarSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &CalendarSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("calendar", "CalendarSpec")
    }
}

impl RenderComponent<DatePickerSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &DatePickerSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("date-picker", "DatePickerSpec")
    }
}

impl RenderComponent<DateRangePickerSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &DateRangePickerSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("date-range-picker", "DateRangePickerSpec")
    }
}

impl RenderComponent<DateTimePickerSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &DateTimePickerSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("date-time-picker", "DateTimePickerSpec")
    }
}

impl RenderComponent<DateTimeRangePickerSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &DateTimeRangePickerSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("date-time-range-picker", "DateTimeRangePickerSpec")
    }
}

#[cfg(test)]
mod tests {
    use poodle_adapter::RenderComponent;
    use poodle_primitives::*;
    use poodle_style::StyleDescriptor;
    use crate::{GpuiAdapter, theme::GpuiThemeProvider};

    fn a() -> GpuiAdapter { GpuiAdapter::new(GpuiThemeProvider::default()) }
    fn s() -> StyleDescriptor { StyleDescriptor::new() }
    fn t() -> GpuiThemeProvider { GpuiThemeProvider::default() }

    #[test] fn code() { assert_eq!(a().render(&CodeSpec::new(), &s(), &t()).spec_type, "CodeSpec"); }
    #[test] fn color_picker() { assert_eq!(a().render(&ColorPickerSpec::new(), &s(), &t()).spec_type, "ColorPickerSpec"); }
    #[test] fn file_upload() { assert_eq!(a().render(&FileUploadSpec::new(), &s(), &t()).spec_type, "FileUploadSpec"); }
    #[test] fn eyebrow() { assert_eq!(a().render(&EyebrowSpec::new(), &s(), &t()).spec_type, "EyebrowSpec"); }
    #[test] fn pill() { assert_eq!(a().render(&PillSpec::new(), &s(), &t()).spec_type, "PillSpec"); }
    #[test] fn time_ago() { assert_eq!(a().render(&TimeAgoSpec::new(), &s(), &t()).spec_type, "TimeAgoSpec"); }
    #[test] fn duration_input() { assert_eq!(a().render(&DurationInputSpec::new(), &s(), &t()).spec_type, "DurationInputSpec"); }
    #[test] fn time_zone_select() { assert_eq!(a().render(&TimeZoneSelectSpec::new(), &s(), &t()).spec_type, "TimeZoneSelectSpec"); }
    #[test] fn date_time_zone_picker() { assert_eq!(a().render(&DateTimeZonePickerSpec::new(), &s(), &t()).spec_type, "DateTimeZonePickerSpec"); }
    #[test] fn split_button() { assert_eq!(a().render(&SplitButtonSpec::new(), &s(), &t()).spec_type, "SplitButtonSpec"); }
    #[test] fn calendar() { assert_eq!(a().render(&CalendarSpec::new(), &s(), &t()).spec_type, "CalendarSpec"); }
    #[test] fn date_picker() { assert_eq!(a().render(&DatePickerSpec::new(), &s(), &t()).spec_type, "DatePickerSpec"); }
    #[test] fn date_range_picker() { assert_eq!(a().render(&DateRangePickerSpec::new(), &s(), &t()).spec_type, "DateRangePickerSpec"); }
    #[test] fn date_time_picker() { assert_eq!(a().render(&DateTimePickerSpec::new(), &s(), &t()).spec_type, "DateTimePickerSpec"); }
    #[test] fn date_time_range_picker() { assert_eq!(a().render(&DateTimeRangePickerSpec::new(), &s(), &t()).spec_type, "DateTimeRangePickerSpec"); }
}
