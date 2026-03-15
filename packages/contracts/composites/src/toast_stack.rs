use pug_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToastStackSpec {
    pub toast_count: usize,
    pub max_visible: usize,
    pub placement: String,
}

impl ToastStackSpec {
    pub fn new() -> Self {
        Self {
            toast_count: 0,
            max_visible: 3,
            placement: String::from("bottom-right"),
        }
    }

    pub fn with_toast_count(mut self, toast_count: usize) -> Self {
        self.toast_count = toast_count;
        self
    }

    pub fn with_max_visible(mut self, max_visible: usize) -> Self {
        self.max_visible = max_visible;
        self
    }

    pub fn with_placement(mut self, placement: impl Into<String>) -> Self {
        self.placement = placement.into();
        self
    }

    pub fn visible_count(&self) -> usize {
        self.toast_count.min(self.max_visible)
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }
}
