//! DatePicker — a trigger and a calendar popover.
//!
//! Contract: `docs/contracts/components/date-picker.md`
//! Ported from: `packages/jetstream/components/src/date_picker.rs`.
//!
//! Open/close, outside-click dismissal, Escape and calendar selection are
//! host-owned; the component renders at the current spec state
//! (`current_open()` decides whether the calendar surface is composed). The
//! calendar is composed rather than reimplemented, so `on_select` /
//! `on_navigate` forward to it — a day pressed in the popover is the same
//! event `calendar` already raises.

use std::{fmt, sync::Arc};

use poodle_node::{DismissReason, LayoutDirection, Node, NodeRole};
use poodle_specs::{
    CalendarSpec, DatePickerSpec, DateRangeValue, DateTimeRangeValue, DateTimeValue,
};

use crate::calendar::{calendar_with_identity, CalendarHandlers};
use crate::color::{mix_linear, with_alpha};
use crate::context::RenderContext;
use crate::picker_trigger::{
    configure_picker_surface, configure_picker_trigger, picker_trigger, PickerTrigger,
};
use crate::presentation::{date_picker_indicator_font_rem, rem_to_px};

/// Existing host callbacks, kept source-compatible for current callers.
#[derive(Default)]
pub struct DatePickerHandlers {
    pub on_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_navigate: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

/// Additional mounted interaction callbacks shared by the date-picker family.
/// These are additive beside [`DatePickerHandlers`] so existing handler literals
/// keep compiling while new hosts can provide stable identity and value events.
#[derive(Default)]
pub struct DatePickerCallbacks {
    pub instance_id: String,
    pub on_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub on_dismiss: Option<Arc<dyn Fn(DismissReason) + Send + Sync>>,
    pub on_range_change: Option<Arc<dyn Fn(&DateRangeValue) + Send + Sync>>,
    pub on_date_time_change: Option<Arc<dyn Fn(&DateTimeValue) + Send + Sync>>,
    pub on_date_time_range_change: Option<Arc<dyn Fn(&DateTimeRangeValue) + Send + Sync>>,
}

impl fmt::Debug for DatePickerCallbacks {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DatePickerCallbacks")
            .field("instance_id", &self.instance_id)
            .field("on_open_change", &self.on_open_change.is_some())
            .field("on_dismiss", &self.on_dismiss.is_some())
            .field("on_range_change", &self.on_range_change.is_some())
            .field("on_date_time_change", &self.on_date_time_change.is_some())
            .field(
                "on_date_time_range_change",
                &self.on_date_time_range_change.is_some(),
            )
            .finish()
    }
}

impl DatePickerCallbacks {
    pub fn new(instance_id: impl Into<String>) -> Self {
        let instance_id = instance_id.into();
        assert!(
            !instance_id.trim().is_empty(),
            "DatePickerCallbacks requires a non-empty lifetime-stable instance_id"
        );
        Self {
            instance_id,
            ..Self::default()
        }
    }
}

pub(crate) struct ComposedDatePickerHandlers {
    pub instance_id: String,
    pub on_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub on_dismiss: Option<Arc<dyn Fn(DismissReason) + Send + Sync>>,
    pub on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_range_change: Option<Arc<dyn Fn(&DateRangeValue) + Send + Sync>>,
    pub on_date_time_change: Option<Arc<dyn Fn(&DateTimeValue) + Send + Sync>>,
    pub on_date_time_range_change: Option<Arc<dyn Fn(&DateTimeRangeValue) + Send + Sync>>,
    pub on_navigate: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

pub(crate) fn picker_child_scope(instance_id: &str, part: &str) -> String {
    if instance_id.is_empty() {
        String::new()
    } else {
        format!("{instance_id}:{part}")
    }
}

pub(crate) fn compose_handlers(
    handlers: DatePickerHandlers,
    callbacks: DatePickerCallbacks,
) -> ComposedDatePickerHandlers {
    ComposedDatePickerHandlers {
        instance_id: callbacks.instance_id,
        on_toggle: handlers.on_toggle,
        on_open_change: callbacks.on_open_change,
        on_dismiss: callbacks.on_dismiss,
        on_select: handlers.on_select,
        on_range_change: callbacks.on_range_change,
        on_date_time_change: callbacks.on_date_time_change,
        on_date_time_range_change: callbacks.on_date_time_range_change,
        on_navigate: handlers.on_navigate,
    }
}

pub(crate) fn picker_toggle_handler(
    handlers: &ComposedDatePickerHandlers,
    open: bool,
) -> Arc<dyn Fn() + Send + Sync> {
    let on_toggle = handlers.on_toggle.clone();
    let on_open_change = handlers.on_open_change.clone();
    Arc::new(move || {
        if let Some(on_toggle) = &on_toggle {
            on_toggle();
        }
        if let Some(on_open_change) = &on_open_change {
            on_open_change(!open);
        }
    })
}

pub(crate) fn picker_dismiss_handler(
    handlers: &ComposedDatePickerHandlers,
) -> Option<Arc<dyn Fn(DismissReason) + Send + Sync>> {
    if handlers.on_dismiss.is_none() && handlers.on_open_change.is_none() {
        return None;
    }
    let on_dismiss = handlers.on_dismiss.clone();
    let on_open_change = handlers.on_open_change.clone();
    Some(Arc::new(move |reason| {
        if let Some(on_dismiss) = &on_dismiss {
            on_dismiss(reason);
        }
        if let Some(on_open_change) = &on_open_change {
            on_open_change(false);
        }
    }))
}

pub fn date_picker(
    spec: &DatePickerSpec,
    ctx: &RenderContext<'_>,
    handlers: DatePickerHandlers,
) -> Node {
    date_picker_with_callbacks(spec, ctx, handlers, DatePickerCallbacks::default())
}

pub fn date_picker_with_callbacks(
    spec: &DatePickerSpec,
    ctx: &RenderContext<'_>,
    handlers: DatePickerHandlers,
    callbacks: DatePickerCallbacks,
) -> Node {
    let handlers = compose_handlers(handlers, callbacks);
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let base_size = ctx.base_size(spec.size);
    let theme = ctx.theme();

    let elevated = theme.resolve_color("color.background.elevated");
    let border = theme.resolve_color("color.border.default");
    let indicator_size = rem_to_px(date_picker_indicator_font_rem(effective_size));

    // ── Display text: current_value() (honors default_value); placeholder otherwise ──
    let display = spec
        .current_value()
        .map(|v| v.to_string())
        .unwrap_or_else(|| spec.placeholder.clone());
    let open = spec.current_open();
    let toggle = picker_toggle_handler(&handlers, open);
    let dismiss = picker_dismiss_handler(&handlers);
    let mut trigger = picker_trigger(
        ctx,
        PickerTrigger {
            display: &display,
            has_value: spec.current_value().is_some(),
            open,
            disabled: spec.is_disabled,
            size: base_size,
            size_role: spec.size_role,
            indicator: "chevron-down",
            indicator_size: Some(indicator_size),
            elevated,
            border_color: border,
            on_toggle: Some(&toggle),
        },
    );
    configure_picker_trigger(
        &mut trigger,
        &handlers.instance_id,
        spec.aria_label
            .as_deref()
            .filter(|label| !label.trim().is_empty())
            .unwrap_or(&display),
        open,
        dismiss.clone(),
    );

    // ── Root wrapper: contract §7/§8 min-width 14rem ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        // Closed: single trigger child in the old tier's default Row.
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.fill_width = true;
    }
    let mut root = root.child(trigger);

    // ── Calendar surface when open (contract §2 Surface + composed Calendar) ──
    if open {
        let mut cal_spec = CalendarSpec::new().with_week_start(spec.week_starts_on);
        if let Some(val) = spec.current_value() {
            cal_spec = cal_spec.with_value(val).with_visible_month(val);
        }

        let panel_bg = theme.resolve_color("color.background.panel");
        let surface_radius = theme.resolve_radius("radius.surface");
        // Surface border: color-mix(border-default 72%, transparent).
        let surface_border = with_alpha(border, border.3 * 0.72);
        // Surface background: color-mix(elevated 98%, panel) — linear lerp.
        let surface_bg = mix_linear(elevated, panel_bg, 0.98);

        let mut surface = Node::container();
        // Contract: the open picker surface is a `dialog`.
        surface.a11y.role = Some(NodeRole::Dialog);
        {
            let s = &mut surface.style;
            // Explicit Row (see switch.rs): one calendar child in old default.
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.corner_radii.top_left = surface_radius;
            s.descriptor.corner_radii.top_right = surface_radius;
            s.descriptor.corner_radii.bottom_right = surface_radius;
            s.descriptor.corner_radii.bottom_left = surface_radius;
            s.descriptor.background = Some(surface_bg);
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = surface_border;
            // Token-accurate elevation.overlay.
            s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
            let pad = &mut s.descriptor.layout.spacing.padding;
            let surface_pad = theme.resolve_space("space.inline.md");
            pad.top = surface_pad;
            pad.bottom = surface_pad;
            pad.left = surface_pad;
            pad.right = surface_pad;
        }
        configure_picker_surface(&mut surface, &handlers.instance_id, open, dismiss);
        let calendar_handlers = CalendarHandlers {
            on_select: Some({
                let on_select = handlers.on_select.clone();
                let on_open_change = handlers.on_open_change.clone();
                Arc::new(move |date: &str| {
                    if let Some(on_select) = &on_select {
                        on_select(date);
                    }
                    if let Some(on_open_change) = &on_open_change {
                        on_open_change(false);
                    }
                }) as Arc<dyn Fn(&str) + Send + Sync>
            }),
            on_range_select: None,
            on_navigate: handlers.on_navigate.clone(),
        };
        let surface = surface.child(calendar_with_identity(
            &cal_spec,
            ctx,
            calendar_handlers,
            (!handlers.instance_id.is_empty())
                .then(|| format!("{}:calendar", handlers.instance_id)),
        ));

        // Trigger + anchored-below surface stack (overlay anchoring is a
        // platform delta; rendered as a flow column with the contract gap).
        root.style.descriptor.layout.direction = LayoutDirection::Column;
        root.style.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.xs");
        root = root.child(surface);
    }

    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
        root.interaction.disabled = true;
    }

    root
}
