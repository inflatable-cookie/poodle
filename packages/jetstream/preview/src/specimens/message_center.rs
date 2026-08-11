use crate::compat::js_message_center;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{MessageCenterItem, MessageCenterSpec, StatusTone};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let populated = MessageCenterSpec::new(vec![
        MessageCenterItem::new("render", "Render complete")
            .with_message("Mix preview 42 is ready for review.")
            .with_meta("Render queue")
            .with_timestamp("2026-08-11T09:40:00Z")
            .with_tone(StatusTone::Success),
        MessageCenterItem::new("mention", "Maya mentioned you")
            .with_message("Can you check the limiter settings before export?")
            .with_meta("Studio chat")
            .with_timestamp("2026-08-11T09:20:00Z"),
        MessageCenterItem::new("maintenance", "Maintenance scheduled")
            .with_message("Workstation services restart tonight at 23:00.")
            .with_meta("Operations")
            .with_read(true)
            .with_tone(StatusTone::Warning),
    ])
    .with_open(true);

    let empty = MessageCenterSpec::default()
        .with_open(true)
        .with_title("Messages")
        .with_trigger_icon("message-circle");

    div()
        .flex_col()
        .gap(32.0)
        .min_h(620.0)
        .child(js_message_center(&populated, theme))
        .child(js_message_center(&empty, theme))
}
