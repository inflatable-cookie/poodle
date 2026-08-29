//! TimeInput renderer — segmented 24-hour editor.
//!
//! Contract: `docs/contracts/components/time-input.md`.
//! There is no native `input[type=time]`, so GPUI presents one labelled group
//! of hour, minute, and conditional-second spin-button segments. Keys and
//! digits run through `poodle_headless::time_input`.

use std::sync::{Arc, Mutex};

use poodle_headless::time_input::{
    format_time, parse_time, time_input_invalid, time_input_transition, time_seconds_visible,
    TimeInputContext, TimeInputEffect, TimeInputEvent, TimeSegment,
};
use poodle_node::{CrossAxisAlignment, LayoutDirection, Node, NodeRole, TextChangeHandler};
use poodle_specs::TimeInputSpec;

use crate::context::RenderContext;
use crate::presentation::{
    rem_to_px, size_font_rem, size_height_offset_rem, size_padding_x_offset_rem,
};

#[derive(Default)]
pub struct TimeInputHandlers {
    pub on_value_change: Option<Arc<dyn Fn(Option<String>) + Send + Sync>>,
    pub on_context: Option<Arc<dyn Fn(TimeInputContext) + Send + Sync>>,
    pub live_context: Option<Arc<Mutex<TimeInputContext>>>,
}

pub fn time_input(spec: &TimeInputSpec, ctx: &RenderContext<'_>) -> Node {
    let context = context_from_spec(spec);
    time_input_with_handlers(spec, ctx, &context, TimeInputHandlers::default())
}

pub fn time_input_with_change(
    spec: &TimeInputSpec,
    ctx: &RenderContext<'_>,
    on_change: Option<TextChangeHandler>,
) -> Node {
    time_input_with_persistent_context(
        spec,
        ctx,
        Arc::new(Mutex::new(context_from_spec(spec))),
        on_change,
        None,
    )
}

pub fn time_input_with_persistent_context(
    spec: &TimeInputSpec,
    ctx: &RenderContext<'_>,
    live: Arc<Mutex<TimeInputContext>>,
    on_change: Option<TextChangeHandler>,
    on_context: Option<Arc<dyn Fn(TimeInputContext) + Send + Sync>>,
) -> Node {
    let context = live.lock().expect("time input context").clone();
    time_input_with_handlers(
        spec,
        ctx,
        &context,
        TimeInputHandlers {
            live_context: Some(live),
            on_value_change: on_change.map(|handler| {
                Arc::new(move |value: Option<String>| {
                    handler(&value.unwrap_or_default());
                }) as Arc<dyn Fn(Option<String>) + Send + Sync>
            }),
            on_context,
        },
    )
}

pub fn time_input_with_handlers(
    spec: &TimeInputSpec,
    ctx: &RenderContext<'_>,
    context: &TimeInputContext,
    handlers: TimeInputHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let invalid = time_input_invalid(context);
    let show_seconds = time_seconds_visible(
        context.committed.as_deref(),
        context.default_value.as_deref(),
        context.min.as_deref(),
        context.max.as_deref(),
        context.step,
    );
    let fill = ctx.theme().resolve_color(spec.fill_token());
    let border_color = ctx.theme().resolve_color(if invalid {
        spec.invalid_border_token()
    } else {
        spec.border_token()
    });
    let text_color = ctx.theme().resolve_color(spec.text_color_token());
    let placeholder_color = ctx.theme().resolve_color(spec.placeholder_color_token());
    let radius = ctx.theme().resolve_radius(spec.radius_token());
    let min_height = ctx.theme().resolve_space("size.control.height")
        + rem_to_px(size_height_offset_rem(effective_size));
    let pad_x = ctx.theme().resolve_space("space.control.x")
        + rem_to_px(size_padding_x_offset_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let border_width = rem_to_px(0.0625);
    let label_font = rem_to_px(0.5625);
    let segment_gap = rem_to_px(0.125);
    let (hour_text, minute_text, second_text) = segment_texts(context, show_seconds);
    let on_value_change = handlers.on_value_change.clone();
    let on_context = handlers.on_context.clone();
    let live = handlers
        .live_context
        .clone()
        .unwrap_or_else(|| Arc::new(Mutex::new(context.clone())));

    let dispatch = {
        let live = Arc::clone(&live);
        Arc::new(move |event: TimeInputEvent| {
            let current = live.lock().expect("time input context").clone();
            let (next, effects) = time_input_transition(current, event);
            *live.lock().expect("time input context") = next.clone();
            if let Some(on_context) = &on_context {
                on_context(next);
            }
            if let Some(on_value_change) = &on_value_change {
                for effect in effects {
                    let TimeInputEffect::EmitValueChange { value } = effect;
                    on_value_change(value);
                }
            }
        })
    };

    let build_segment = |label: &str, a11y_name: &str, text: &str, segment: TimeSegment| -> Node {
        let mut caption = Node::text(label);
        {
            let style = &mut caption.style;
            style.text_size = Some(label_font);
            style.line_height = Some(1.0);
            style.descriptor.text_color = Some(placeholder_color);
        }

        let filled = !text.is_empty() && text != "--";
        let mut field = Node::text(text);
        {
            let style = &mut field.style;
            style.text_size = Some(font_size);
            style.line_height = Some(1.0);
            style.descriptor.text_color = Some(if filled {
                text_color
            } else {
                placeholder_color
            });
            style.text_weight = Some(600);
            style.text_align = Some(poodle_node::TextAlign::Center);
        }

        let mut seg = Node::container();
        {
            let style = &mut seg.style;
            style.descriptor.layout.direction = LayoutDirection::Column;
            style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            style.descriptor.layout.spacing.gap = rem_to_px(0.125);
        }
        seg.a11y.role = Some(NodeRole::SpinButton);
        seg.a11y.label = Some(a11y_name.to_string());
        if let Some(numeric) = parse_segment_number(text) {
            seg.a11y.value = Some(numeric);
            seg.a11y.value_text = Some(text.to_string());
        }
        match segment {
            TimeSegment::Hour => {
                seg.a11y.value_min = Some(0.0);
                seg.a11y.value_max = Some(23.0);
            }
            TimeSegment::Minute | TimeSegment::Second => {
                seg.a11y.value_min = Some(0.0);
                seg.a11y.value_max = Some(59.0);
            }
        }

        if !spec.is_disabled {
            seg.interaction.focusable = true;
            let dispatch_keys = Arc::clone(&dispatch);
            seg.interaction.on_edit_key = Some(Arc::new(move |key: &str, _mods| {
                let event = match key {
                    "up" => TimeInputEvent::Step { direction: 1 },
                    "down" => TimeInputEvent::Step { direction: -1 },
                    "escape" => TimeInputEvent::Escape,
                    "backspace" | "delete" => TimeInputEvent::ClearSegment { segment },
                    key => {
                        let mut chars = key.chars();
                        match (chars.next(), chars.next()) {
                            (Some(digit), None) if digit.is_ascii_digit() => {
                                TimeInputEvent::Digit {
                                    segment,
                                    digit: digit.to_digit(10).expect("checked ascii digit"),
                                }
                            }
                            _ => return,
                        }
                    }
                };
                dispatch_keys(event);
            }));
            let dispatch_escape = Arc::clone(&dispatch);
            seg.interaction.on_cancel = Some(Arc::new(move || {
                dispatch_escape(TimeInputEvent::Escape);
            }));
        }

        seg.child(caption).child(field)
    };

    let build_separator = || {
        let mut sep = Node::text(":");
        {
            let style = &mut sep.style;
            style.text_size = Some(font_size);
            style.line_height = Some(1.0);
            style.descriptor.text_color = Some(placeholder_color);
            style.text_weight = Some(600);
        }
        sep
    };

    let mut segments = Node::container();
    segments.style.descriptor.layout.direction = LayoutDirection::Row;
    segments.style.descriptor.layout.alignment.cross = CrossAxisAlignment::End;
    segments.style.descriptor.layout.spacing.gap = segment_gap;
    segments = segments
        .child(build_segment("H", "Hour", &hour_text, TimeSegment::Hour))
        .child(build_separator())
        .child(build_segment(
            "M",
            "Minute",
            &minute_text,
            TimeSegment::Minute,
        ));
    if show_seconds {
        segments = segments.child(build_separator()).child(build_segment(
            "S",
            "Second",
            &second_text,
            TimeSegment::Second,
        ));
    }

    let mut root = Node::container();
    {
        let style = &mut root.style;
        style.min_height = Some(min_height);
        style.self_stretch = true;
        style.descriptor.layout.direction = LayoutDirection::Row;
        style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        let pad = &mut style.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        style.descriptor.corner_radii.top_left = radius;
        style.descriptor.corner_radii.top_right = radius;
        style.descriptor.corner_radii.bottom_right = radius;
        style.descriptor.corner_radii.bottom_left = radius;
        style.descriptor.background = Some(fill);
        style.descriptor.border.width = border_width;
        style.descriptor.border.color = border_color;
    }
    root.a11y.role = Some(NodeRole::Group);
    root.a11y.label = Some(match spec.aria_label.as_deref() {
        Some(label) if !label.is_empty() => label.to_string(),
        _ => "Time".to_string(),
    });
    if let Some(value) = context.committed.as_deref() {
        root.a11y.value_text = Some(value.to_string());
    }

    if spec.is_disabled {
        root.style.descriptor.opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
        root.interaction.disabled = true;
    }

    root.child(segments)
}

pub fn context_from_spec(spec: &TimeInputSpec) -> TimeInputContext {
    TimeInputContext {
        committed: spec.current_value().map(str::to_string),
        default_value: spec.default_value.clone(),
        draft: None,
        min: spec.min.clone(),
        max: spec.max.clone(),
        step: f64::from(spec.step),
        disabled: spec.is_disabled,
    }
}

fn segment_texts(context: &TimeInputContext, show_seconds: bool) -> (String, String, String) {
    if let Some(draft) = &context.draft {
        return (
            display_segment(&draft.hour),
            display_segment(&draft.minute),
            if show_seconds {
                display_segment(&draft.second)
            } else {
                String::new()
            },
        );
    }

    match parse_time(context.committed.as_deref()) {
        Some(parts) => {
            let formatted = format_time(parts, show_seconds);
            let mut bits = formatted.split(':');
            (
                bits.next().unwrap_or("--").to_string(),
                bits.next().unwrap_or("--").to_string(),
                bits.next().unwrap_or("--").to_string(),
            )
        }
        None => (
            "--".to_string(),
            "--".to_string(),
            if show_seconds {
                "--".to_string()
            } else {
                String::new()
            },
        ),
    }
}

fn display_segment(raw: &str) -> String {
    if raw.is_empty() {
        "--".to_string()
    } else {
        raw.to_string()
    }
}

fn parse_segment_number(text: &str) -> Option<f64> {
    if text.is_empty() || text == "--" {
        return None;
    }
    text.parse::<f64>().ok()
}

#[cfg(test)]
mod tests {
    use poodle_adapter::ThemeProvider;
    use poodle_headless::time_input::TimeInputDraft;

    use super::*;
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn segments(node: &Node) -> Vec<&Node> {
        fn walk<'a>(node: &'a Node, out: &mut Vec<&'a Node>) {
            if node.a11y.role == Some(NodeRole::SpinButton) {
                out.push(node);
            }
            for child in &node.children {
                walk(child, out);
            }
        }
        let mut out = Vec::new();
        walk(node, &mut out);
        out
    }

    fn render(
        spec: &TimeInputSpec,
        context: &TimeInputContext,
    ) -> (Node, Arc<Mutex<Vec<Option<String>>>>) {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = time_input_with_handlers(
            spec,
            &ctx,
            context,
            TimeInputHandlers {
                on_value_change: Some(Arc::new(move |value| sink.lock().unwrap().push(value))),
                on_context: None,
                live_context: None,
            },
        );
        (node, seen)
    }

    #[test]
    fn the_root_is_a_labelled_group_of_spinbuttons() {
        let spec = TimeInputSpec::new()
            .with_aria_label("Start time")
            .with_default_value("14:30");
        let (node, _) = render(&spec, &context_from_spec(&spec));
        assert_eq!(node.a11y.role, Some(NodeRole::Group));
        assert_eq!(node.a11y.label.as_deref(), Some("Start time"));
        assert!(!node.interaction.focusable);
        let segs = segments(&node);
        assert_eq!(segs.len(), 2);
        assert!(segs.iter().all(|seg| seg.interaction.focusable));
        assert_eq!(segs[0].a11y.label.as_deref(), Some("Hour"));
        assert_eq!(segs[1].a11y.label.as_deref(), Some("Minute"));
        assert_eq!(segs[0].a11y.value, Some(14.0));
        assert_eq!(segs[0].a11y.value_min, Some(0.0));
        assert_eq!(segs[0].a11y.value_max, Some(23.0));
    }

    #[test]
    fn seconds_appear_when_step_is_below_a_minute() {
        let spec = TimeInputSpec::new()
            .with_step(15)
            .with_default_value("09:30:00");
        let (node, _) = render(&spec, &context_from_spec(&spec));
        assert_eq!(segments(&node).len(), 3);
        assert!(node.texts().contains(&"S"));
    }

    #[test]
    fn a_complete_digit_pair_emits_the_canonical_value() {
        let spec = TimeInputSpec::new().with_default_value("14:30");
        let drafting = TimeInputContext {
            draft: Some(TimeInputDraft {
                hour: "1".into(),
                minute: "30".into(),
                second: "00".into(),
            }),
            ..context_from_spec(&spec)
        };
        let (node, seen) = render(&spec, &drafting);
        (segments(&node)[0].interaction.on_edit_key.as_ref().unwrap())(
            "5",
            poodle_node::NodeModifiers::default(),
        );
        assert_eq!(
            seen.lock().unwrap().last().cloned(),
            Some(Some("15:30".into()))
        );
    }

    #[test]
    fn arrows_step_the_whole_time() {
        let spec = TimeInputSpec::new().with_default_value("14:30");
        let (node, seen) = render(&spec, &context_from_spec(&spec));
        (segments(&node)[0].interaction.on_edit_key.as_ref().unwrap())(
            "up",
            poodle_node::NodeModifiers::default(),
        );
        assert_eq!(
            seen.lock().unwrap().last().cloned(),
            Some(Some("14:31".into()))
        );
    }

    #[test]
    fn disabled_segments_are_not_focus_stops() {
        let spec = TimeInputSpec::new()
            .with_default_value("12:00")
            .with_disabled(true);
        let (node, _) = render(&spec, &context_from_spec(&spec));
        assert!(segments(&node).iter().all(|seg| !seg.interaction.focusable));
        assert!(node.interaction.disabled);
    }

    #[test]
    fn a_partial_digit_survives_on_the_reusable_change_path() {
        let spec = TimeInputSpec::new().with_default_value("14:30");
        let seen = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = time_input_with_change(
            &spec,
            &ctx,
            Some(Arc::new(move |value: &str| {
                sink.lock().unwrap().push(value.to_string());
            })),
        );
        let segs = segments(&node);
        (segs[0].interaction.on_edit_key.as_ref().unwrap())(
            "1",
            poodle_node::NodeModifiers::default(),
        );
        assert!(seen.lock().unwrap().is_empty());
        (segs[0].interaction.on_edit_key.as_ref().unwrap())(
            "5",
            poodle_node::NodeModifiers::default(),
        );
        assert_eq!(seen.lock().unwrap().last().cloned(), Some("15:30".into()));
    }

    #[test]
    fn an_invalid_draft_uses_the_danger_border() {
        let spec = TimeInputSpec::new()
            .with_default_value("09:00")
            .with_step(300);
        let drafting = TimeInputContext {
            committed: Some("09:00".into()),
            draft: Some(TimeInputDraft {
                hour: "09".into(),
                minute: "07".into(),
                second: "00".into(),
            }),
            step: 300.0,
            ..TimeInputContext::default()
        };
        let (node, _) = render(&spec, &drafting);
        let danger = theme().resolve_color(spec.invalid_border_token());
        assert_eq!(node.style.descriptor.border.color, danger);
    }
}
