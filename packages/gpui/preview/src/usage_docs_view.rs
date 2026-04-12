use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::Code;
use poodle_specs::CodeSpec;

use crate::contract_usage_docs::ContractUsageDocs;
use crate::style_bridge::color_to_hsla;

pub fn render_usage_docs(
    theme: &poodle_gpui::GpuiThemeProvider,
    contract_doc: &ContractUsageDocs,
) -> Div {
    let border_subtle = theme.resolve_color("color.border.subtle");
    let mut sections: Vec<AnyElement> = Vec::new();

    if let Some(usage) = contract_doc.usage.as_ref() {
        sections.push(render_doc_code_section(theme, "Usage", "md", usage).into_any_element());
    }

    if !contract_doc.props.is_empty() {
        sections.push(render_props_table(theme, contract_doc).into_any_element());
    }

    if !contract_doc.slots.is_empty() {
        sections.push(render_slots_table(theme, contract_doc).into_any_element());
    }

    if !contract_doc.events.is_empty() {
        sections.push(render_events_table(theme, contract_doc).into_any_element());
    }

    let mut docs = div().flex().flex_col();
    for (index, section) in sections.into_iter().enumerate() {
        if index > 0 {
            docs = docs.child(div().h(px(1.0)).w_full().bg(color_to_hsla(border_subtle)));
        }
        docs = docs.child(section);
    }
    docs
}

fn render_doc_code_section(
    theme: &poodle_gpui::GpuiThemeProvider,
    title: &'static str,
    language: &'static str,
    content: &str,
) -> Div {
    let text_primary = theme.resolve_color("color.text.primary");

    div()
        .flex()
        .flex_col()
        .gap(px(10.0))
        .child(
            div()
                .text_lg()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(color_to_hsla(text_primary))
                .child(title),
        )
        .child(Code::from_spec(
            CodeSpec::new()
                .with_language(language)
                .with_content(content)
                .with_copyable(false),
            theme,
        ))
}

fn render_props_table(
    theme: &poodle_gpui::GpuiThemeProvider,
    contract_doc: &ContractUsageDocs,
) -> Div {
    let rows: Vec<AnyElement> = contract_doc
        .props
        .iter()
        .map(|prop| {
            let name = if prop.required {
                format!("{}*", prop.name)
            } else {
                prop.name.clone()
            };
            render_usage_table_row(
                theme,
                vec![
                    render_usage_name_cell(theme, name, prop.required).into_any_element(),
                    render_usage_code_cell(theme, &prop.type_name, px(220.0)).into_any_element(),
                    render_usage_code_cell(
                        theme,
                        prop.default_value.as_deref().unwrap_or("—"),
                        px(120.0),
                    )
                    .into_any_element(),
                    render_usage_description_cell(theme, &prop.description).into_any_element(),
                ],
            )
            .into_any_element()
        })
        .collect();

    render_usage_table_section(
        theme,
        "Props",
        vec![
            ("Prop", px(160.0)),
            ("Type", px(220.0)),
            ("Default", px(120.0)),
            ("Description", px(0.0)),
        ],
        rows,
    )
}

fn render_slots_table(
    theme: &poodle_gpui::GpuiThemeProvider,
    contract_doc: &ContractUsageDocs,
) -> Div {
    let rows: Vec<AnyElement> = contract_doc
        .slots
        .iter()
        .map(|slot| {
            render_usage_table_row(
                theme,
                vec![
                    render_usage_name_cell(theme, slot.name.clone(), false).into_any_element(),
                    render_usage_description_cell(theme, &slot.description).into_any_element(),
                ],
            )
            .into_any_element()
        })
        .collect();

    render_usage_table_section(
        theme,
        "Slots",
        vec![("Slot", px(160.0)), ("Description", px(0.0))],
        rows,
    )
}

fn render_events_table(
    theme: &poodle_gpui::GpuiThemeProvider,
    contract_doc: &ContractUsageDocs,
) -> Div {
    let rows: Vec<AnyElement> = contract_doc
        .events
        .iter()
        .map(|event| {
            render_usage_table_row(
                theme,
                vec![
                    render_usage_name_cell(theme, event.name.clone(), false).into_any_element(),
                    render_usage_code_cell(theme, &event.payload, px(220.0)).into_any_element(),
                    render_usage_description_cell(theme, &event.description).into_any_element(),
                ],
            )
            .into_any_element()
        })
        .collect();

    render_usage_table_section(
        theme,
        "Events",
        vec![
            ("Event", px(160.0)),
            ("Payload", px(220.0)),
            ("Description", px(0.0)),
        ],
        rows,
    )
}

fn render_usage_table_section(
    theme: &poodle_gpui::GpuiThemeProvider,
    title: &'static str,
    columns: Vec<(&'static str, Pixels)>,
    rows: Vec<AnyElement>,
) -> Div {
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border_subtle = theme.resolve_color("color.border.subtle");

    let mut header_row = div()
        .flex()
        .items_start()
        .w_full()
        .border_b_1()
        .border_color(color_to_hsla(border_subtle));
    for (label, width) in columns {
        let mut cell = div()
            .px(px(12.0))
            .py(px(8.0))
            .text_size(px(11.0))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(color_to_hsla(text_secondary))
            .child(label);
        if width > px(0.0) {
            cell = cell.w(width).flex_none();
        } else {
            cell = cell.flex_1().min_w(px(192.0));
        }
        header_row = header_row.child(cell);
    }

    div()
        .flex()
        .flex_col()
        .gap(px(10.0))
        .child(
            div()
                .text_lg()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(color_to_hsla(text_primary))
                .child(title),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .w_full()
                .overflow_hidden()
                .child(header_row)
                .children(rows),
        )
}

fn render_usage_table_row(theme: &poodle_gpui::GpuiThemeProvider, cells: Vec<AnyElement>) -> Div {
    let border_subtle = theme.resolve_color("color.border.subtle");
    div()
        .flex()
        .items_start()
        .w_full()
        .border_b_1()
        .border_color(color_to_hsla(border_subtle).opacity(0.4))
        .children(cells)
}

fn render_usage_name_cell(
    theme: &poodle_gpui::GpuiThemeProvider,
    text: String,
    required: bool,
) -> Div {
    let text_primary = theme.resolve_color("color.text.primary");
    let danger = theme.resolve_color("color.status.danger");
    let mut label = div()
        .flex()
        .items_center()
        .gap(px(2.0))
        .child(text.trim_end_matches('*').to_string());
    if required {
        label = label.child(div().text_color(color_to_hsla(danger)).child("*"));
    }
    div()
        .w(px(160.0))
        .flex_none()
        .px(px(12.0))
        .py(px(8.0))
        .font_family("SF Mono")
        .text_size(px(12.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(color_to_hsla(text_primary))
        .child(label)
}

fn render_usage_code_cell(
    theme: &poodle_gpui::GpuiThemeProvider,
    text: &str,
    width: Pixels,
) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let canvas = theme.resolve_color("color.background.canvas");
    div().w(width).flex_none().px(px(12.0)).py(px(8.0)).child(
        div()
            .px(px(4.0))
            .py(px(1.0))
            .rounded(px(3.0))
            .bg(color_to_hsla(canvas).opacity(0.8))
            .font_family("SF Mono")
            .text_size(px(12.0))
            .text_color(color_to_hsla(text_secondary))
            .child(text.to_string()),
    )
}

fn render_usage_description_cell(theme: &poodle_gpui::GpuiThemeProvider, text: &str) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    div()
        .flex_1()
        .min_w(px(192.0))
        .px(px(12.0))
        .py(px(8.0))
        .text_size(px(13.0))
        .line_height(relative(1.5))
        .text_color(color_to_hsla(text_secondary))
        .child(text.to_string())
}
