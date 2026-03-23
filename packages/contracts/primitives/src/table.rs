use poodle_tokens::semantic;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ColumnAlign {
    #[default]
    Start,
    End,
}

#[derive(Debug, Clone)]
pub struct TableColumn {
    pub id: String,
    pub label: String,
    pub align: ColumnAlign,
    pub is_row_header: bool,
}

impl TableColumn {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            align: ColumnAlign::Start,
            is_row_header: false,
        }
    }

    pub fn with_align(mut self, align: ColumnAlign) -> Self {
        self.align = align;
        self
    }

    pub fn with_row_header(mut self, is_row_header: bool) -> Self {
        self.is_row_header = is_row_header;
        self
    }
}

#[derive(Debug, Clone)]
pub struct TableRow {
    pub id: String,
    pub cells: Vec<(String, String)>,
}

impl TableRow {
    pub fn new(id: impl Into<String>, cells: Vec<(String, String)>) -> Self {
        Self {
            id: id.into(),
            cells,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableSpec {
    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
    pub caption: Option<String>,
    pub empty_message: String,
    pub aria_label: Option<String>,
}

impl Default for TableSpec {
    fn default() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            caption: None,
            empty_message: "No rows available.".to_string(),
            aria_label: None,
        }
    }
}

impl TableSpec {
    pub fn new() -> Self {
        Self::default()
    }

    // Builder methods

    pub fn with_columns(mut self, columns: Vec<TableColumn>) -> Self {
        self.columns = columns;
        self
    }

    pub fn with_rows(mut self, rows: Vec<TableRow>) -> Self {
        self.rows = rows;
        self
    }

    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn with_empty_message(mut self, message: impl Into<String>) -> Self {
        self.empty_message = message.into();
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn add_row(mut self, row: TableRow) -> Self {
        self.rows.push(row);
        self
    }

    // Token methods

    pub fn shell_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn shell_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    pub fn shell_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn header_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn header_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn header_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn cell_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn cell_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn caption_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn empty_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    // Helpers

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn cell_value<'a>(&self, row: &'a TableRow, column_id: &str) -> &'a str {
        row.cells
            .iter()
            .find(|(col_id, _)| col_id == column_id)
            .map(|(_, value)| value.as_str())
            .unwrap_or("")
    }
}
