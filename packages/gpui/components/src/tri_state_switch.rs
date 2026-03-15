//! PugTriStateSwitch — real GPUI component backed by TriStateSwitchSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{CheckState, TriStateSwitchSpec};

use crate::theme_ext::resolve_color;

/// A real GPUI tri-state switch component backed by `TriStateSwitchSpec`.
pub struct PugTriStateSwitch {
    spec: TriStateSwitchSpec,
    theme: GpuiThemeProvider,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl PugTriStateSwitch {
    pub fn new(spec: TriStateSwitchSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_click: None,
        }
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugTriStateSwitch {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let track_fill = resolve_color(theme, spec.track_fill_token());
        let thumb_color = resolve_color(theme, "semantic.color.text.inverse");

        // Thumb position based on state
        let thumb_offset = match spec.state {
            CheckState::Unchecked => px(2.0),
            CheckState::Mixed => px(10.0),
            CheckState::Checked => px(18.0),
        };

        let switch_id = SharedString::from("pug-tri-state-switch");

        let mut track = div()
            .id(switch_id)
            .w(px(36.0))
            .h(px(20.0))
            .rounded(px(10.0))
            .bg(track_fill)
            .relative()
            .child(
                div()
                    .absolute()
                    .top(px(2.0))
                    .left(thumb_offset)
                    .w(px(16.0))
                    .h(px(16.0))
                    .rounded(px(8.0))
                    .bg(thumb_color),
            );

        if !spec.is_disabled {
            track = track.cursor_pointer();
            if let Some(handler) = self.on_click {
                track = track.on_click(move |event, window, cx| handler(event, window, cx));
            }
        } else {
            track = track.opacity(0.5);
        }

        let mut el = div().flex().items_center().gap(px(8.0)).child(track);

        if let Some(ref label) = spec.label {
            el = el.child(
                div()
                    .text_sm()
                    .child(label.clone()),
            );
        }

        el.into_any_element()
    }
}
