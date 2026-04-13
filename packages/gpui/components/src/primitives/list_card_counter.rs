//! ListCardCounter — icon + numeric count for `ListCard` footers (`list-card-counter.md`).
//!
//! Tooltip wrapping matches the contract when `spec.tooltip` is set and
//! [`ListCardCounter::on_tooltip_open_change`] is wired (GPUI needs explicit open
//! state). `font-variant-numeric: tabular-nums` is not expressed in GPUI yet.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{IconSpec, ListCardCounterSpec, OverlayPlacement, TooltipSpec};

use std::rc::Rc;

use crate::theme_ext::{resolve_color, resolve_px};
use crate::{Icon, Tooltip};

/// Compact footer counter backed by [`ListCardCounterSpec`].
pub struct ListCardCounter {
    spec: ListCardCounterSpec,
    theme: GpuiThemeProvider,
    on_link_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    tooltip_open: bool,
    on_tooltip_open_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for ListCardCounter {
    type Target = ListCardCounterSpec;
    fn deref(&self) -> &Self::Target {
        &self.spec
    }
}

impl ListCardCounter {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: ListCardCounterSpec::new("file-text", 0),
            theme: theme.clone(),
            on_link_click: None,
            tooltip_open: false,
            on_tooltip_open_change: None,
        }
    }

    pub fn from_spec(spec: ListCardCounterSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_link_click: None,
            tooltip_open: false,
            on_tooltip_open_change: None,
        }
    }

    pub fn on_link_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_link_click = Some(Rc::new(handler));
        self
    }

    /// Controlled tooltip open state (used with [`Self::on_tooltip_open_change`]).
    pub fn tooltip_open(mut self, open: bool) -> Self {
        self.tooltip_open = open;
        self
    }

    pub fn on_tooltip_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_tooltip_open_change = Some(Rc::new(handler));
        self
    }

    fn build_row(&self) -> AnyElement {
        let theme = &self.theme;
        let spec = &self.spec;
        let gap = resolve_px(theme, ListCardCounterSpec::gap_token());
        let font_size = resolve_px(theme, ListCardCounterSpec::font_size_token());
        let secondary = resolve_color(theme, ListCardCounterSpec::text_secondary_token());
        let primary = resolve_color(theme, ListCardCounterSpec::text_primary_token());

        let icon_el = Icon::from_spec(
            IconSpec::new(spec.icon.clone()).with_size(ListCardCounterSpec::icon_size()),
            theme,
        )
        .with_color(secondary);

        let count = div().child(format!("{}", spec.count));

        let row = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(gap)
            .text_size(font_size)
            .text_color(secondary)
            .child(icon_el)
            .child(count);

        if spec.is_linked() {
            // `on_click` is only available on `Stateful<Div>`; `id` promotes `Div` → `Stateful`.
            let link_id = SharedString::from(format!(
                "poodle-lcc-{}-{}",
                spec.icon,
                spec.count
            ));
            let mut linked = row
                .id(link_id)
                .cursor_pointer()
                .hover(move |style| style.text_color(primary));
            if let Some(h) = self.on_link_click.clone() {
                linked = linked.on_click(move |ev, w, a| {
                    h(ev, w, a);
                });
            }
            return linked.into_any_element();
        }

        row.into_any_element()
    }
}

impl IntoElement for ListCardCounter {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let row = self.build_row();

        if let Some(ref tip) = spec.tooltip {
            let tspec = TooltipSpec::new()
                .with_content(tip.clone())
                .with_placement(OverlayPlacement::Top)
                .with_open(self.tooltip_open);
            if let Some(handler) = self.on_tooltip_open_change.clone() {
                return Tooltip::from_spec(tspec, theme)
                    .on_open_change(move |open, w, a| handler(open, w, a))
                    .with_trigger(row)
                    .into_any_element();
            }
            // Without hover handler the tooltip never opens; still render row-only
            // so specimens stay readable.
            return row.into_any_element();
        }

        row.into_any_element()
    }
}
