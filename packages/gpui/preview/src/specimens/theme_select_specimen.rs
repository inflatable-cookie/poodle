use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, ThemeSelect};
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, ThemeOption, ThemeSelectSpec, ThemeSwatch};

fn demo_themes() -> Vec<ThemeOption> {
    vec![
        ThemeOption::new(
            "eclipse",
            "Eclipse",
            ThemeSwatch::new("#0e1012", "#15181b", "#f0b24d", "#eef2f6", "#333"),
        ),
        ThemeOption::new(
            "iceberg",
            "Iceberg",
            ThemeSwatch::new("#e7eef5", "#dbe5ef", "#2d86f3", "#131a22", "#75869b"),
        ),
        ThemeOption::new(
            "midnight",
            "Midnight",
            ThemeSwatch::new("#0b1020", "#121933", "#6d8cff", "#e6ecff", "#333"),
        ),
        ThemeOption::new(
            "nord",
            "Nord",
            ThemeSwatch::new("#2e3440", "#3b4252", "#88c0d0", "#eceff4", "#4c566a"),
        ),
        ThemeOption::new(
            "rose",
            "Rose",
            ThemeSwatch::new("#1a1114", "#241a1e", "#f65c8a", "#f6eef1", "#333"),
        ),
        ThemeOption::new(
            "forest",
            "Forest",
            ThemeSwatch::new("#0e1512", "#15201b", "#4dc98a", "#e8f3ec", "#333"),
        ),
    ]
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    // Interactive case. The popover is anchored and absolutely positioned, so
    // a permanently-open example would sit on top of whatever follows it — the
    // open state belongs to the specimen, driven by the trigger, exactly as the
    // Popover and Menu specimens do it.
    let open_key = "theme-select-open";
    let value_key = "theme-select-value";
    let is_open = state.specimens.is_on(open_key);
    let current = state
        .specimens
        .text
        .get(value_key)
        .cloned()
        .unwrap_or_else(|| "midnight".to_string());

    let interactive = ThemeSelect::from_spec(
        ThemeSelectSpec::new()
            .with_themes(demo_themes())
            .with_value(&current)
            .with_open(is_open),
        theme,
    )
    .on_open_change({
        let queue = std::sync::Arc::clone(&state.node_events);
        std::sync::Arc::new(move |open: bool| {
            queue.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                key: open_key.to_string(),
                value: open,
            });
        })
    })
    .on_change({
        let queue = std::sync::Arc::clone(&state.node_events);
        std::sync::Arc::new(move |value: &str| {
            // Record the choice and close, mirroring the component's own
            // select-then-dismiss behaviour.
            let mut events = queue.lock().unwrap();
            events.push(NodeSpecimenEvent::SetText {
                key: value_key.to_string(),
                value: value.to_string(),
            });
            events.push(NodeSpecimenEvent::SetToggle {
                key: open_key.to_string(),
                value: false,
            });
        })
    });

    let labelled = |label: &str, body: AnyElement| {
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(Eyebrow::from_spec(
                EyebrowSpec::new().with_content(label),
                theme,
            ))
            .child(body)
    };

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // `deferred` so the open popover paints above the examples below it
        // rather than behind them — the host half of the contract's portalled
        // surface, the same treatment the app header uses.
        .child(labelled(
            "Interactive — click the trigger",
            deferred(interactive).into_any_element(),
        ))
        .child(labelled(
            "Disabled",
            ThemeSelect::from_spec(
                ThemeSelectSpec::new()
                    .with_themes(demo_themes())
                    .with_value("nord")
                    .with_disabled(true),
                theme,
            )
            .into_any_element(),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "theme-select",
        examples,
        |size, theme: &GpuiThemeProvider| {
            ThemeSelect::from_spec(
                ThemeSelectSpec::new()
                    .with_themes(demo_themes())
                    .with_value("eclipse"),
                theme,
            )
            .size(size)
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            ThemeSelect::from_spec(
                ThemeSelectSpec::new()
                    .with_themes(demo_themes())
                    .with_value("eclipse"),
                theme,
            )
            .with_density(density)
            .into_any_element()
        },
    )
}
