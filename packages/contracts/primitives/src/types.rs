use poodle_tokens::semantic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Inset {
    pub horizontal: Option<&'static str>,
    pub vertical: Option<&'static str>,
}

impl Inset {
    pub const fn none() -> Self {
        Self {
            horizontal: None,
            vertical: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dimension(String);

impl Dimension {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Dimension {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Dimension {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaddingScale {
    None,
    Sm,
    Md,
    Lg,
}

impl PaddingScale {
    pub fn layout_inset(self) -> Inset {
        match self {
            Self::None => Inset::none(),
            Self::Sm => Inset {
                horizontal: Some(semantic::SPACE_INLINE_SM),
                vertical: Some(semantic::SPACE_STACK_SM),
            },
            Self::Md => Inset {
                horizontal: Some(semantic::SPACE_INLINE_MD),
                vertical: Some(semantic::SPACE_STACK_MD),
            },
            Self::Lg => Inset {
                horizontal: Some(semantic::SPACE_INLINE_LG),
                vertical: Some(semantic::SPACE_STACK_LG),
            },
        }
    }

    pub fn panel_inset(self) -> Inset {
        match self {
            Self::None => Inset::none(),
            Self::Sm => Inset {
                horizontal: Some(semantic::SPACE_INLINE_SM),
                vertical: Some(semantic::SPACE_STACK_SM),
            },
            Self::Md => Inset {
                horizontal: Some(semantic::SPACE_PANEL_X),
                vertical: Some(semantic::SPACE_PANEL_Y),
            },
            Self::Lg => Inset {
                horizontal: Some(semantic::SPACE_INLINE_LG),
                vertical: Some(semantic::SPACE_STACK_LG),
            },
        }
    }

    pub fn stack_gap(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Sm => Some(semantic::SPACE_STACK_SM),
            Self::Md => Some(semantic::SPACE_STACK_MD),
            Self::Lg => Some(semantic::SPACE_STACK_LG),
        }
    }

    pub fn inline_gap(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Sm => Some(semantic::SPACE_INLINE_SM),
            Self::Md => Some(semantic::SPACE_INLINE_MD),
            Self::Lg => Some(semantic::SPACE_INLINE_LG),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
    Clip,
    Auto,
    Scroll,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlSize {
    Sm,
    Md,
    Lg,
}

impl ControlSize {
    pub fn control_height_token(self) -> &'static str {
        semantic::SIZE_CONTROL_HEIGHT
    }

    pub fn control_min_width_token(self) -> &'static str {
        semantic::SIZE_CONTROL_MIN_WIDTH
    }

    pub fn icon_size_token(self) -> &'static str {
        match self {
            Self::Sm => semantic::SIZE_ICON_SM,
            Self::Md => semantic::SIZE_ICON_MD,
            Self::Lg => semantic::SIZE_ICON_LG,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    /// Danger is kept for backward compatibility — equivalent to Primary + Danger tone.
    Danger,
}

/// Button tone (default or danger). The tone modifies variant colors.
/// Contract: variant × tone combinations produce different visual treatments.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum ButtonTone {
    #[default]
    Default,
    Danger,
}

impl ButtonVariant {
    /// Resolve fill token accounting for tone modifier.
    pub fn fill_token(self, tone: ButtonTone) -> &'static str {
        match (self, tone) {
            (Self::Ghost, _) => semantic::COLOR_BACKGROUND_SURFACE, // transparent in component
            (Self::Primary, ButtonTone::Danger) | (Self::Danger, _) => semantic::COLOR_STATUS_DANGER,
            (Self::Secondary, ButtonTone::Danger) => semantic::COLOR_BACKGROUND_SURFACE,
            (Self::Primary, ButtonTone::Default) => semantic::COLOR_ACCENT_BASE,
            (Self::Secondary, ButtonTone::Default) => semantic::COLOR_BACKGROUND_SURFACE,
        }
    }

    /// Resolve border token accounting for tone modifier.
    pub fn border_token(self, tone: ButtonTone) -> &'static str {
        match (self, tone) {
            (Self::Ghost, _) => semantic::COLOR_BORDER_SUBTLE, // transparent in component
            (Self::Primary, ButtonTone::Danger) | (Self::Danger, _) => semantic::COLOR_STATUS_DANGER,
            (Self::Secondary, ButtonTone::Danger) => semantic::COLOR_STATUS_DANGER,
            (Self::Primary, ButtonTone::Default) => semantic::COLOR_ACCENT_BASE,
            (Self::Secondary, ButtonTone::Default) => semantic::COLOR_BORDER_DEFAULT,
        }
    }

    /// Resolve text token accounting for tone modifier.
    pub fn text_token(self, tone: ButtonTone) -> &'static str {
        match (self, tone) {
            (Self::Ghost, ButtonTone::Danger) => semantic::COLOR_STATUS_DANGER,
            (Self::Ghost, ButtonTone::Default) => semantic::COLOR_TEXT_PRIMARY,
            (Self::Secondary, ButtonTone::Danger) => semantic::COLOR_TEXT_PRIMARY,
            (Self::Primary, _) | (Self::Danger, _) => semantic::COLOR_TEXT_INVERSE,
            (Self::Secondary, ButtonTone::Default) => semantic::COLOR_TEXT_PRIMARY,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BadgeVariant {
    Accent,
    Muted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatusTone {
    Neutral,
    Info,
    Success,
    Warning,
    Danger,
    Pending,
}

impl StatusTone {
    pub fn color_token(self) -> &'static str {
        match self {
            // Contract: neutral uses text-secondary (not accent)
            Self::Neutral => semantic::COLOR_TEXT_SECONDARY,
            Self::Info | Self::Pending => semantic::COLOR_ACCENT_BASE,
            Self::Success => semantic::COLOR_STATUS_SUCCESS,
            Self::Warning => semantic::COLOR_STATUS_WARNING,
            Self::Danger => semantic::COLOR_STATUS_DANGER,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationState {
    None,
    Invalid,
    Valid,
    Pending,
}

impl ValidationState {
    pub fn border_token(self) -> &'static str {
        match self {
            Self::None => semantic::COLOR_BORDER_DEFAULT,
            Self::Invalid => semantic::COLOR_STATUS_DANGER,
            Self::Valid => semantic::COLOR_STATUS_SUCCESS,
            Self::Pending => semantic::COLOR_ACCENT_BASE,
        }
    }

    pub fn aria_invalid(self) -> Option<&'static str> {
        match self {
            Self::Invalid => Some("true"),
            _ => None,
        }
    }

    pub fn aria_busy(self) -> Option<&'static str> {
        match self {
            Self::Pending => Some("true"),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckState {
    Unchecked,
    Checked,
    Mixed,
}

impl CheckState {
    pub fn aria_checked(self) -> &'static str {
        match self {
            Self::Unchecked => "false",
            Self::Checked => "true",
            Self::Mixed => "mixed",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Alignment {
    Start,
    Center,
    Stretch,
    End,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Vertical,
    Horizontal,
    Both,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceTone {
    Canvas,
    Panel,
    Elevated,
}

impl SurfaceTone {
    pub fn background_token(self) -> &'static str {
        match self {
            Self::Canvas => semantic::COLOR_BACKGROUND_CANVAS,
            Self::Panel => semantic::COLOR_BACKGROUND_PANEL,
            Self::Elevated => semantic::COLOR_BACKGROUND_ELEVATED,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceBorder {
    None,
    Subtle,
    Default,
}

impl SurfaceBorder {
    pub fn color_token(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Subtle => Some(semantic::COLOR_BORDER_SUBTLE),
            Self::Default => Some(semantic::COLOR_BORDER_DEFAULT),
        }
    }

    pub fn width_token(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Subtle | Self::Default => Some(semantic::BORDER_WIDTH_DEFAULT),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceRole {
    Group,
    Region,
}

impl SurfaceRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Group => "group",
            Self::Region => "region",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SeparatorOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleTone {
    Subtle,
    Default,
}

impl RuleTone {
    pub fn color_token(self) -> &'static str {
        match self {
            Self::Subtle => semantic::COLOR_BORDER_SUBTLE,
            Self::Default => semantic::COLOR_BORDER_DEFAULT,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormActionAlign {
    Start,
    End,
    Between,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CalendarWeekStart {
    Sunday,
    Monday,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChoiceOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
    pub aria_label: Option<String>,
    pub is_disabled: bool,
}

impl ChoiceOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            aria_label: None,
            is_disabled: false,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateRangeValue {
    pub start: Option<String>,
    pub end: Option<String>,
}

impl DateRangeValue {
    pub fn new(start: Option<String>, end: Option<String>) -> Self {
        Self { start, end }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateTimeValue {
    pub date: Option<String>,
    pub time: Option<String>,
}

impl DateTimeValue {
    pub fn new(date: Option<String>, time: Option<String>) -> Self {
        Self { date, time }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateTimeRangeValue {
    pub start: DateTimeValue,
    pub end: DateTimeValue,
}

impl DateTimeRangeValue {
    pub fn new(start: DateTimeValue, end: DateTimeValue) -> Self {
        Self { start, end }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OverlayPlacement {
    Top,
    TopStart,
    TopEnd,
    Right,
    RightStart,
    RightEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PopoverInitialFocus {
    FirstFocusable,
    Content,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DialogKind {
    Dialog,
    AlertDialog,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DrawerEdge {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TabVariant {
    Underline,
    Card,
    Pill,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TabActivationMode {
    Automatic,
    Manual,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuItemKind {
    Action,
    Checkbox,
    Radio,
    Separator,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuEntry {
    pub value: String,
    pub label: String,
    pub is_disabled: bool,
    pub is_checked: bool,
    pub shortcut_label: Option<String>,
    pub kind: MenuItemKind,
}

impl MenuEntry {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            is_disabled: false,
            is_checked: false,
            shortcut_label: None,
            kind: MenuItemKind::Action,
        }
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_checked(mut self, is_checked: bool) -> Self {
        self.is_checked = is_checked;
        self
    }

    pub fn with_shortcut_label(mut self, shortcut_label: impl Into<String>) -> Self {
        self.shortcut_label = Some(shortcut_label.into());
        self
    }

    pub fn with_kind(mut self, kind: MenuItemKind) -> Self {
        self.kind = kind;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccordionItemSpec {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
    pub is_disabled: bool,
}

impl AccordionItemSpec {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            is_disabled: false,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccordionSelectionValue {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationMenuEntry {
    pub value: String,
    pub label: String,
    pub is_disabled: bool,
    pub description: Option<String>,
}

impl NavigationMenuEntry {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            is_disabled: false,
            description: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenubarEntry {
    pub value: String,
    pub label: String,
    pub is_disabled: bool,
    pub items: Vec<MenuEntry>,
}

impl MenubarEntry {
    pub fn new(value: impl Into<String>, label: impl Into<String>, items: Vec<MenuEntry>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            is_disabled: false,
            items,
        }
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TabDefinition {
    pub value: String,
    pub label: String,
    pub is_disabled: bool,
    pub is_closable: bool,
}

impl TabDefinition {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            is_disabled: false,
            is_closable: false,
        }
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_closable(mut self, is_closable: bool) -> Self {
        self.is_closable = is_closable;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TabStripItem {
    pub value: String,
    pub label: String,
    pub is_disabled: bool,
    pub is_closable: bool,
}

impl TabStripItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            is_disabled: false,
            is_closable: false,
        }
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_closable(mut self, is_closable: bool) -> Self {
        self.is_closable = is_closable;
        self
    }
}
