#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InlineListSectionSpec {
    pub title: String,
    pub count: Option<String>,
    pub empty_message: Option<String>,
    pub framed: bool,
}

impl InlineListSectionSpec {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            count: None,
            empty_message: Some(String::from("No items yet.")),
            framed: true,
        }
    }

    pub fn with_count(mut self, count: impl Into<String>) -> Self {
        self.count = Some(count.into());
        self
    }

    pub fn with_empty_message(mut self, empty_message: impl Into<String>) -> Self {
        self.empty_message = Some(empty_message.into());
        self
    }

    pub fn with_framed(mut self, framed: bool) -> Self {
        self.framed = framed;
        self
    }
}
