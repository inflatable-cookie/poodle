use crate::ControlDensity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DetailSectionGroupLayout {
    Grid,
    Stack,
}

impl Default for DetailSectionGroupLayout {
    fn default() -> Self {
        Self::Grid
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetailSectionGroupSpec {
    pub density: ControlDensity,
    pub layout: DetailSectionGroupLayout,
    pub min_column_width: String,
    pub item_min_column_width: String,
    pub max_columns: u8,
    pub aria_label: Option<String>,
}

impl Default for DetailSectionGroupSpec {
    fn default() -> Self {
        Self {
            density: ControlDensity::Default,
            layout: DetailSectionGroupLayout::Grid,
            min_column_width: String::from("14rem"),
            item_min_column_width: String::from("12rem"),
            max_columns: 4,
            aria_label: None,
        }
    }
}

impl DetailSectionGroupSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_layout(mut self, layout: DetailSectionGroupLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_max_columns(mut self, max_columns: u8) -> Self {
        self.max_columns = max_columns.clamp(2, 5);
        self
    }

    pub fn gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.75,
            ControlDensity::Default => 1.0,
            ControlDensity::Comfortable => 1.25,
        }
    }
}
