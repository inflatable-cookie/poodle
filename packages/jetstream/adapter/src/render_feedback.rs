//! RenderComponent implementations for feedback, display, and informational primitives.
//!
//! g08.007: ProgressSpec, BadgeSpec, SpinnerSpec, StatusIndicatorSpec, SkeletonSpec,
//! MeterSpec, RatingSpec, CodeSpec, EyebrowSpec, PillSpec, TimeAgoSpec,
//! SplitButtonSpec, ColorPickerSpec, FileUploadSpec, DurationInputSpec,
//! TimeZoneSelectSpec, DateTimeZonePickerSpec, CalendarSpec,
//! DatePickerSpec, DateRangePickerSpec, DateTimePickerSpec, DateTimeRangePickerSpec

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_specs::{
    BadgeSpec, CalendarSpec, CodeSpec, ColorPickerSpec, DatePickerSpec, DateRangePickerSpec,
    DateTimePickerSpec, DateTimeRangePickerSpec, DateTimeZonePickerSpec, DurationInputSpec,
    EyebrowSpec, FileUploadSpec, MeterSpec, PillSpec, ProgressSpec, RatingSpec, SkeletonSpec,
    SpinnerSpec, SplitButtonSpec, StatusIndicatorSpec, TimeAgoSpec, TimeZoneSelectSpec,
};
use poodle_style::StyleDescriptor;

use crate::style_map::{map_style, JetstreamColor};
use crate::{JetstreamAdapter, JetstreamNodeHandle, JetstreamTarget, WidgetKind};

impl RenderComponent<ProgressSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &ProgressSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve indicator fill color
        let fill_token = spec.indicator_fill_token();
        let c = theme.resolve_color(fill_token);
        mapped.visuals.background = Some(JetstreamColor::from(c));

        JetstreamNodeHandle::new("progress", "ProgressSpec", WidgetKind::ProgressBar, mapped)
    }
}

impl RenderComponent<BadgeSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &BadgeSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve badge fill color
        let fill_token = spec.fill_token();
        let c = theme.resolve_color(fill_token);
        mapped.visuals.background = Some(JetstreamColor::from(c));

        JetstreamNodeHandle::new("badge", "BadgeSpec", WidgetKind::Label, mapped)
    }
}

impl RenderComponent<SpinnerSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &SpinnerSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        if let Some(color_token) = spec.tone_color_token() {
            let c = theme.resolve_color(color_token);
            mapped.visuals.icon_color = Some(JetstreamColor::from(c));
        }

        JetstreamNodeHandle::new("spinner", "SpinnerSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<StatusIndicatorSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &StatusIndicatorSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve status color
        let color_token = spec.status_color_token();
        let c = theme.resolve_color(color_token);
        mapped.visuals.icon_color = Some(JetstreamColor::from(c));

        // Use label for node id when available
        let node_id = spec.label.as_deref().unwrap_or("status-indicator");
        JetstreamNodeHandle::new(node_id, "StatusIndicatorSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<SkeletonSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &SkeletonSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve skeleton fill color
        let fill_token = spec.fill_token();
        let c = theme.resolve_color(fill_token);
        mapped.visuals.background = Some(JetstreamColor::from(c));

        // Resolve corner radius
        let radius_token = spec.radius_token();
        let r = theme.resolve_space(radius_token);
        mapped.visuals.corner_radii = [r; 4];

        JetstreamNodeHandle::new("skeleton", "SkeletonSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<MeterSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &MeterSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve meter fill color (indicator)
        let fill_token = spec.fill_token();
        let c = theme.resolve_color(fill_token);
        mapped.visuals.background = Some(JetstreamColor::from(c));

        // Resolve track fill color
        let track_token = spec.track_fill_token();
        let tc = theme.resolve_color(track_token);
        mapped.visuals.border_color = Some(JetstreamColor::from(tc));

        JetstreamNodeHandle::new("meter", "MeterSpec", WidgetKind::ProgressBar, mapped)
    }
}

impl RenderComponent<RatingSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &RatingSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve active (filled) star color
        let active_token = spec.active_color_token();
        let ac = theme.resolve_color(active_token);
        mapped.visuals.icon_color = Some(JetstreamColor::from(ac));

        // Resolve inactive (empty) star color
        let inactive_token = spec.inactive_color_token();
        let ic = theme.resolve_color(inactive_token);
        mapped.visuals.border_color = Some(JetstreamColor::from(ic));

        JetstreamNodeHandle::new("rating", "RatingSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<CodeSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &CodeSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve code block fill color
        let fill_token = spec.fill_token();
        let fc = theme.resolve_color(fill_token);
        mapped.visuals.background = Some(JetstreamColor::from(fc));

        // Resolve text color
        let text_token = spec.text_color_token();
        let tc = theme.resolve_color(text_token);
        mapped.visuals.text_color = Some(JetstreamColor::from(tc));

        // Resolve border color
        let border_token = spec.border_token();
        let bc = theme.resolve_color(border_token);
        mapped.visuals.border_color = Some(JetstreamColor::from(bc));

        // Font family — no typography field on JetstreamVisuals; acknowledge token
        let _font = spec.font_family_token();

        // Font size — resolve via space for proof; no typography field to apply
        let _font_size = theme.resolve_space(spec.font_size_token());

        JetstreamNodeHandle::new("code", "CodeSpec", WidgetKind::Label, mapped)
    }
}

impl RenderComponent<EyebrowSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &EyebrowSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve text color
        let text_token = spec.text_color_token();
        let tc = theme.resolve_color(text_token);
        mapped.visuals.text_color = Some(JetstreamColor::from(tc));

        // Font size — resolve via space for proof; no typography field to apply
        let _font_size = theme.resolve_space(spec.font_size_token());

        JetstreamNodeHandle::new("eyebrow", "EyebrowSpec", WidgetKind::Label, mapped)
    }
}

impl RenderComponent<PillSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &PillSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve pill fill color
        let fill_token = spec.fill_token();
        let fc = theme.resolve_color(fill_token);
        mapped.visuals.background = Some(JetstreamColor::from(fc));

        // Resolve text color
        let text_token = spec.text_color_token();
        let tc = theme.resolve_color(text_token);
        mapped.visuals.text_color = Some(JetstreamColor::from(tc));

        // Resolve focus ring color
        let ring_token = spec.focus_ring_color_token();
        let rc = theme.resolve_color(ring_token);
        mapped.visuals.focus_ring_color = Some(JetstreamColor::from(rc));

        // Use pill label for node id
        let node_id = if spec.label.is_empty() {
            "pill"
        } else {
            &spec.label
        };
        JetstreamNodeHandle::new(node_id, "PillSpec", WidgetKind::Label, mapped)
    }
}

impl RenderComponent<TimeAgoSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &TimeAgoSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve text color
        let text_token = spec.text_color_token();
        let tc = theme.resolve_color(text_token);
        mapped.visuals.text_color = Some(JetstreamColor::from(tc));

        // Font size — resolve via space for proof; no typography field to apply
        let _font_size = theme.resolve_space(spec.font_size_token());

        JetstreamNodeHandle::new("time-ago", "TimeAgoSpec", WidgetKind::Label, mapped)
    }
}

impl RenderComponent<SplitButtonSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &SplitButtonSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve fill color
        let fill_token = spec.fill_token();
        let fc = theme.resolve_color(fill_token);
        mapped.visuals.background = Some(JetstreamColor::from(fc));

        // Resolve border color
        let border_token = spec.border_token();
        let bc = theme.resolve_color(border_token);
        mapped.visuals.border_color = Some(JetstreamColor::from(bc));

        // Resolve separator color — map to icon_color as visual separator indicator
        let separator_token = spec.separator_token();
        let sc = theme.resolve_color(separator_token);
        mapped.visuals.icon_color = Some(JetstreamColor::from(sc));

        // Resolve overlay fill — secondary background for dropdown region
        let overlay_token = spec.overlay_fill_token();
        let oc = theme.resolve_color(overlay_token);
        mapped.visuals.text_color = Some(JetstreamColor::from(oc));

        // Resolve shadow (no shadow field on JetstreamVisuals — resolve for proof)
        let _shadow = theme.resolve_color(spec.shadow_token());

        JetstreamNodeHandle::new(
            "split-button",
            "SplitButtonSpec",
            WidgetKind::Button,
            mapped,
        )
    }
}

impl RenderComponent<ColorPickerSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &ColorPickerSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve border color
        let border_token = spec.border_token();
        let bc = theme.resolve_color(border_token);
        mapped.visuals.border_color = Some(JetstreamColor::from(bc));

        // Resolve overlay fill
        let overlay_token = spec.overlay_fill_token();
        let oc = theme.resolve_color(overlay_token);
        mapped.visuals.background = Some(JetstreamColor::from(oc));

        // Resolve shadow (no shadow field on JetstreamVisuals — resolve for proof)
        let _shadow = theme.resolve_color(spec.shadow_token());

        JetstreamNodeHandle::new("color-picker", "ColorPickerSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<FileUploadSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &FileUploadSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve fill color
        let fill_token = spec.fill_token();
        let fc = theme.resolve_color(fill_token);
        mapped.visuals.background = Some(JetstreamColor::from(fc));

        // Resolve border color
        let border_token = spec.border_token();
        let bc = theme.resolve_color(border_token);
        mapped.visuals.border_color = Some(JetstreamColor::from(bc));

        // Resolve text color
        let text_token = spec.text_color_token();
        let tc = theme.resolve_color(text_token);
        mapped.visuals.text_color = Some(JetstreamColor::from(tc));

        JetstreamNodeHandle::new("file-upload", "FileUploadSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<DurationInputSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &DurationInputSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve border color
        let border_token = spec.border_token();
        let bc = theme.resolve_color(border_token);
        mapped.visuals.border_color = Some(JetstreamColor::from(bc));

        // Resolve focus ring color
        let ring_token = spec.focus_ring_color_token();
        let rc = theme.resolve_color(ring_token);
        mapped.visuals.focus_ring_color = Some(JetstreamColor::from(rc));

        JetstreamNodeHandle::new(
            "duration-input",
            "DurationInputSpec",
            WidgetKind::TextInput,
            mapped,
        )
    }
}

impl RenderComponent<TimeZoneSelectSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &TimeZoneSelectSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve border color
        let border_token = spec.border_token();
        let bc = theme.resolve_color(border_token);
        mapped.visuals.border_color = Some(JetstreamColor::from(bc));

        // Resolve overlay fill
        let overlay_token = spec.overlay_fill_token();
        let oc = theme.resolve_color(overlay_token);
        mapped.visuals.background = Some(JetstreamColor::from(oc));

        JetstreamNodeHandle::new(
            "time-zone-select",
            "TimeZoneSelectSpec",
            WidgetKind::Button,
            mapped,
        )
    }
}

impl RenderComponent<DateTimeZonePickerSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &DateTimeZonePickerSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve border color
        let border_token = spec.border_token();
        let bc = theme.resolve_color(border_token);
        mapped.visuals.border_color = Some(JetstreamColor::from(bc));

        // Resolve overlay fill
        let overlay_token = spec.overlay_fill_token();
        let oc = theme.resolve_color(overlay_token);
        mapped.visuals.background = Some(JetstreamColor::from(oc));

        // Resolve shadow (no shadow field on JetstreamVisuals — resolve for proof)
        let _shadow = theme.resolve_color(spec.shadow_token());

        JetstreamNodeHandle::new(
            "date-time-zone-picker",
            "DateTimeZonePickerSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<CalendarSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &CalendarSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("calendar", "CalendarSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<DatePickerSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &DatePickerSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("date-picker", "DatePickerSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<DateRangePickerSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &DateRangePickerSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "date-range-picker",
            "DateRangePickerSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<DateTimePickerSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &DateTimePickerSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "date-time-picker",
            "DateTimePickerSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<DateTimeRangePickerSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &DateTimeRangePickerSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "date-time-range-picker",
            "DateTimeRangePickerSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{theme::JetstreamThemeProvider, JetstreamAdapter, WidgetKind};
    use poodle_adapter::RenderComponent;
    use poodle_specs::*;
    use poodle_style::StyleDescriptor;

    fn a() -> JetstreamAdapter {
        JetstreamAdapter::new(JetstreamThemeProvider::default())
    }
    fn s() -> StyleDescriptor {
        StyleDescriptor::new()
    }
    fn t() -> JetstreamThemeProvider {
        JetstreamThemeProvider::default()
    }

    #[test]
    fn progress() {
        assert_eq!(
            a().render(&ProgressSpec::new(), &s(), &t()).widget_kind,
            WidgetKind::ProgressBar
        );
    }
    #[test]
    fn badge() {
        assert_eq!(
            a().render(&BadgeSpec::new(), &s(), &t()).widget_kind,
            WidgetKind::Label
        );
    }
    #[test]
    fn spinner() {
        assert_eq!(
            a().render(&SpinnerSpec::new(), &s(), &t()).spec_type,
            "SpinnerSpec"
        );
    }
    #[test]
    fn status_indicator() {
        assert_eq!(
            a().render(&StatusIndicatorSpec::new(), &s(), &t())
                .widget_kind,
            WidgetKind::Panel
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
    fn meter() {
        assert_eq!(
            a().render(&MeterSpec::new(), &s(), &t()).widget_kind,
            WidgetKind::ProgressBar
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
    fn code() {
        assert_eq!(
            a().render(&CodeSpec::new(), &s(), &t()).widget_kind,
            WidgetKind::Label
        );
    }
    #[test]
    fn eyebrow() {
        assert_eq!(
            a().render(&EyebrowSpec::new(), &s(), &t()).widget_kind,
            WidgetKind::Label
        );
    }
    #[test]
    fn pill() {
        assert_eq!(
            a().render(&PillSpec::new(), &s(), &t()).widget_kind,
            WidgetKind::Label
        );
    }
    #[test]
    fn time_ago() {
        assert_eq!(
            a().render(&TimeAgoSpec::new(), &s(), &t()).widget_kind,
            WidgetKind::Label
        );
    }
    #[test]
    fn split_button() {
        assert_eq!(
            a().render(&SplitButtonSpec::new(), &s(), &t()).widget_kind,
            WidgetKind::Button
        );
    }
    #[test]
    fn color_picker() {
        assert_eq!(
            a().render(&ColorPickerSpec::new(), &s(), &t()).spec_type,
            "ColorPickerSpec"
        );
    }
    #[test]
    fn file_upload() {
        assert_eq!(
            a().render(&FileUploadSpec::new(), &s(), &t()).spec_type,
            "FileUploadSpec"
        );
    }
    #[test]
    fn duration_input() {
        assert_eq!(
            a().render(&DurationInputSpec::new(), &s(), &t()).spec_type,
            "DurationInputSpec"
        );
    }
    #[test]
    fn time_zone_select() {
        assert_eq!(
            a().render(&TimeZoneSelectSpec::new(), &s(), &t()).spec_type,
            "TimeZoneSelectSpec"
        );
    }
    #[test]
    fn date_time_zone_picker() {
        assert_eq!(
            a().render(&DateTimeZonePickerSpec::new(), &s(), &t())
                .spec_type,
            "DateTimeZonePickerSpec"
        );
    }
    #[test]
    fn calendar() {
        assert_eq!(
            a().render(&CalendarSpec::new(), &s(), &t()).spec_type,
            "CalendarSpec"
        );
    }
    #[test]
    fn date_picker() {
        assert_eq!(
            a().render(&DatePickerSpec::new(), &s(), &t()).spec_type,
            "DatePickerSpec"
        );
    }
    #[test]
    fn date_range_picker() {
        assert_eq!(
            a().render(&DateRangePickerSpec::new(), &s(), &t())
                .spec_type,
            "DateRangePickerSpec"
        );
    }
    #[test]
    fn date_time_picker() {
        assert_eq!(
            a().render(&DateTimePickerSpec::new(), &s(), &t()).spec_type,
            "DateTimePickerSpec"
        );
    }
    #[test]
    fn date_time_range_picker() {
        assert_eq!(
            a().render(&DateTimeRangePickerSpec::new(), &s(), &t())
                .spec_type,
            "DateTimeRangePickerSpec"
        );
    }
}
