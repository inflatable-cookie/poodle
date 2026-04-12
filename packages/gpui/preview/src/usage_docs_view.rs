use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::{Code, Separator};
use poodle_specs::{CodeSpec, SeparatorSpec};

use crate::contract_usage_docs::{ContractUsageDocs, UsageEvent, UsageProp, UsageSlot};
use crate::style_bridge::color_to_hsla;

pub fn render_usage_docs(
    theme: &poodle_gpui::GpuiThemeProvider,
    contract_doc: &ContractUsageDocs,
) -> Div {
    UsageDocsView::new(theme, contract_doc).render()
}

struct UsageDocsView<'a> {
    theme: &'a poodle_gpui::GpuiThemeProvider,
    contract_doc: &'a ContractUsageDocs,
}

impl<'a> UsageDocsView<'a> {
    fn new(theme: &'a poodle_gpui::GpuiThemeProvider, contract_doc: &'a ContractUsageDocs) -> Self {
        Self {
            theme,
            contract_doc,
        }
    }

    fn render(&self) -> Div {
        let mut sections: Vec<AnyElement> = Vec::new();

        if let Some(usage) = self.contract_doc.usage.as_ref() {
            sections.push(
                self.render_code_section("Usage", "md", usage)
                    .into_any_element(),
            );
        }

        if !self.contract_doc.props.is_empty() {
            sections.push(self.render_props_section().into_any_element());
        }

        if !self.contract_doc.slots.is_empty() {
            sections.push(self.render_slots_section().into_any_element());
        }

        if !self.contract_doc.events.is_empty() {
            sections.push(self.render_events_section().into_any_element());
        }

        let mut docs = div().flex().flex_col();
        for (index, section) in sections.into_iter().enumerate() {
            if index > 0 {
                docs = docs.child(self.render_separator());
            }
            docs = docs.child(section);
        }

        docs
    }

    fn render_separator(&self) -> Separator {
        Separator::from_spec(SeparatorSpec::new(), self.theme)
    }

    fn render_section_title(&self, title: &'static str) -> Div {
        let text_primary = self.theme.resolve_color("color.text.primary");

        div()
            .text_lg()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(color_to_hsla(text_primary))
            .child(title)
    }

    fn render_code_section(
        &self,
        title: &'static str,
        language: &'static str,
        content: &str,
    ) -> Div {
        div()
            .flex()
            .flex_col()
            .py(px(24.0))
            .gap(px(16.0))
            .child(self.render_section_title(title))
            .child(Code::from_spec(
                CodeSpec::new()
                    .with_language(language)
                    .with_content(content)
                    .with_copyable(false),
                self.theme,
            ))
    }

    fn render_props_section(&self) -> Div {
        let rows = self
            .contract_doc
            .props
            .iter()
            .map(|prop| {
                self.render_table_row(vec![
                    self.render_name_cell(&prop.name, prop.required)
                        .into_any_element(),
                    self.render_code_cell(&prop.type_name, px(224.0))
                        .into_any_element(),
                    self.render_code_cell(prop.default_value.as_deref().unwrap_or("—"), px(128.0))
                        .into_any_element(),
                    self.render_description_cell(&prop.description)
                        .into_any_element(),
                ])
                .into_any_element()
            })
            .collect();

        self.render_table_section(
            "Props",
            vec![
                ("Prop", px(160.0)),
                ("Type", px(224.0)),
                ("Default", px(128.0)),
                ("Description", px(0.0)),
            ],
            rows,
        )
    }

    fn render_slots_section(&self) -> Div {
        let rows = self
            .contract_doc
            .slots
            .iter()
            .map(|slot| {
                self.render_table_row(vec![
                    self.render_name_cell(&slot.name, false).into_any_element(),
                    self.render_description_cell(&slot.description)
                        .into_any_element(),
                ])
                .into_any_element()
            })
            .collect();

        self.render_table_section(
            "Slots",
            vec![("Slot", px(160.0)), ("Description", px(0.0))],
            rows,
        )
    }

    fn render_events_section(&self) -> Div {
        let rows = self
            .contract_doc
            .events
            .iter()
            .map(|event| {
                self.render_table_row(vec![
                    self.render_name_cell(&event.name, false).into_any_element(),
                    self.render_code_cell(&event.payload, px(224.0))
                        .into_any_element(),
                    self.render_description_cell(&event.description)
                        .into_any_element(),
                ])
                .into_any_element()
            })
            .collect();

        self.render_table_section(
            "Events",
            vec![
                ("Event", px(160.0)),
                ("Payload", px(224.0)),
                ("Description", px(0.0)),
            ],
            rows,
        )
    }

    fn render_table_section(
        &self,
        title: &'static str,
        columns: Vec<(&'static str, Pixels)>,
        rows: Vec<AnyElement>,
    ) -> Div {
        div()
            .flex()
            .flex_col()
            .py(px(24.0))
            .gap(px(16.0))
            .child(self.render_section_title(title))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .overflow_hidden()
                    .child(self.render_table_header(columns))
                    .children(rows),
            )
    }

    fn render_table_header(&self, columns: Vec<(&'static str, Pixels)>) -> Div {
        let text_secondary = self.theme.resolve_color("color.text.secondary");
        let border_subtle = self.theme.resolve_color("color.border.subtle");

        let mut header_row = div()
            .flex()
            .items_start()
            .w_full()
            .border_b_1()
            .border_color(color_to_hsla(border_subtle));

        for (label, width) in columns {
            header_row = header_row.child(
                self.render_header_cell(label, width, color_to_hsla(text_secondary))
                    .into_any_element(),
            );
        }

        header_row
    }

    fn render_header_cell(&self, label: &'static str, width: Pixels, color: Hsla) -> Div {
        let mut cell = div()
            .px(px(12.0))
            .py(px(8.0))
            .text_size(px(11.0))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(color)
            .child(label);

        if width > px(0.0) {
            cell = cell.w(width).flex_none();
        } else {
            cell = cell.flex_1().min_w(px(192.0));
        }

        cell
    }

    fn render_table_row(&self, cells: Vec<AnyElement>) -> Div {
        let border_subtle = self.theme.resolve_color("color.border.subtle");

        div()
            .flex()
            .items_start()
            .w_full()
            .border_b_1()
            .border_color(color_to_hsla(border_subtle).opacity(0.4))
            .children(cells)
    }

    fn render_name_cell(&self, name: &str, required: bool) -> Div {
        let text_primary = self.theme.resolve_color("color.text.primary");
        let danger = self.theme.resolve_color("color.status.danger");

        let mut label = div()
            .flex()
            .items_center()
            .gap(px(2.0))
            .child(name.to_string());
        if required {
            label = label.child(div().text_color(color_to_hsla(danger)).child("*"));
        }

        self.render_fixed_cell(px(160.0)).child(
            div()
                .font_family("SF Mono")
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(color_to_hsla(text_primary))
                .child(label),
        )
    }

    fn render_code_cell(&self, text: &str, width: Pixels) -> Div {
        let text_secondary = self.theme.resolve_color("color.text.secondary");
        let canvas = self.theme.resolve_color("color.background.canvas");

        self.render_fixed_cell(width).child(
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

    fn render_description_cell(&self, text: &str) -> Div {
        let text_secondary = self.theme.resolve_color("color.text.secondary");

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

    fn render_fixed_cell(&self, width: Pixels) -> Div {
        div().w(width).flex_none().px(px(12.0)).py(px(8.0))
    }
}

#[allow(dead_code)]
fn _type_anchor(_: (&UsageProp, &UsageSlot, &UsageEvent)) {}
