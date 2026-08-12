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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ControlSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
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
            Self::Xs => semantic::SIZE_ICON_XS,
            Self::Sm => semantic::SIZE_ICON_SM,
            Self::Md => semantic::SIZE_ICON_MD,
            Self::Lg => semantic::SIZE_ICON_LG,
            Self::Xl => semantic::SIZE_ICON_XL,
        }
    }
}

/// Semantic size role relative to the inherited presentation scale.
/// `Chrome` resolves one stop smaller, `Prominent` one stop larger,
/// `Control` maps directly to the inherited size scale.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SemanticControlSizeRole {
    Chrome,
    #[default]
    Control,
    Prominent,
}

/// Resolve a concrete control size from the inherited size scale and a
/// semantic role.
///
/// `Chrome` resolves one stop smaller (clamped at the minimum), `Prominent` one
/// stop larger (clamped at the maximum), `Control` is the identity mapping.
/// Mirrors the Svelte `resolveSemanticControlSize`.
///
/// This lived in five places — three specs and both native presentation
/// modules — each a private copy of a rule that has to match Svelte exactly.
pub fn resolve_semantic_control_size(
    size: ControlSize,
    role: SemanticControlSizeRole,
) -> ControlSize {
    match (size, role) {
        (_, SemanticControlSizeRole::Control) => size,

        (ControlSize::Xs, SemanticControlSizeRole::Chrome) => ControlSize::Xs,
        (ControlSize::Sm, SemanticControlSizeRole::Chrome) => ControlSize::Sm,
        (ControlSize::Md, SemanticControlSizeRole::Chrome) => ControlSize::Sm,
        (ControlSize::Lg, SemanticControlSizeRole::Chrome) => ControlSize::Md,
        (ControlSize::Xl, SemanticControlSizeRole::Chrome) => ControlSize::Lg,

        (ControlSize::Xs, SemanticControlSizeRole::Prominent) => ControlSize::Sm,
        (ControlSize::Sm, SemanticControlSizeRole::Prominent) => ControlSize::Md,
        (ControlSize::Md, SemanticControlSizeRole::Prominent) => ControlSize::Lg,
        (ControlSize::Lg, SemanticControlSizeRole::Prominent) => ControlSize::Xl,
        (ControlSize::Xl, SemanticControlSizeRole::Prominent) => ControlSize::Xl,
    }
}

/// Density mode controlling container padding and sibling gaps.
/// See docs/contracts/components/size-and-density.md for the global rules.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ControlDensity {
    Compact,
    #[default]
    Default,
    Comfortable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// Typography mode for inline text-like primitives.
///
/// `Default` uses the primitive's own token-owned typography metrics.
/// `Inherit` asks the runtime to scale from the surrounding inline context.
/// CSS runtimes should implement that literally. Non-CSS runtimes may use a
/// ratio-preserving approximation when true parent-relative inline layout is
/// not available yet.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum InlineTypographyMode {
    #[default]
    Default,
    Inherit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    /// Danger is kept for backward compatibility — equivalent to Primary + Danger tone.
    Danger,
}

/// Button tone (default, danger, success, or warning). The tone modifies variant
/// colors. Contract: variant × tone combinations produce different visual
/// treatments. `Success` and `Warning` mirror `Danger` but resolve the
/// `color.status.success` / `color.status.warning` families respectively
/// (button.md §8 Tone: warning; icon-button.md §8 Tone: success).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum ButtonTone {
    #[default]
    Default,
    Danger,
    Success,
    Warning,
}

impl ButtonVariant {
    /// Resolve fill token accounting for tone modifier.
    pub fn fill_token(self, tone: ButtonTone) -> &'static str {
        match (self, tone) {
            (Self::Ghost, _) => semantic::COLOR_BACKGROUND_SURFACE, // transparent in component
            (Self::Primary, ButtonTone::Danger) | (Self::Danger, _) => {
                semantic::COLOR_STATUS_DANGER
            }
            (Self::Primary, ButtonTone::Success) => semantic::COLOR_STATUS_SUCCESS,
            (Self::Primary, ButtonTone::Warning) => semantic::COLOR_STATUS_WARNING,
            // Secondary success/danger/warning return surface here; the
            // status-tinted color-mix (16% status, surface) is applied in the
            // component.
            (Self::Secondary, ButtonTone::Danger | ButtonTone::Success | ButtonTone::Warning) => {
                semantic::COLOR_BACKGROUND_SURFACE
            }
            (Self::Primary, ButtonTone::Default) => semantic::COLOR_ACCENT_BASE,
            (Self::Secondary, ButtonTone::Default) => semantic::COLOR_BACKGROUND_SURFACE,
        }
    }

    /// Resolve border token accounting for tone modifier.
    pub fn border_token(self, tone: ButtonTone) -> &'static str {
        match (self, tone) {
            (Self::Ghost, _) => semantic::COLOR_BORDER_SUBTLE, // transparent in component
            (Self::Primary, ButtonTone::Danger) | (Self::Danger, _) => {
                semantic::COLOR_STATUS_DANGER
            }
            (Self::Primary, ButtonTone::Success) => semantic::COLOR_STATUS_SUCCESS,
            (Self::Primary, ButtonTone::Warning) => semantic::COLOR_STATUS_WARNING,
            (Self::Secondary, ButtonTone::Danger) => semantic::COLOR_STATUS_DANGER,
            (Self::Secondary, ButtonTone::Success) => semantic::COLOR_STATUS_SUCCESS,
            (Self::Secondary, ButtonTone::Warning) => semantic::COLOR_STATUS_WARNING,
            (Self::Primary, ButtonTone::Default) => semantic::COLOR_ACCENT_BASE,
            (Self::Secondary, ButtonTone::Default) => semantic::COLOR_BORDER_DEFAULT,
        }
    }

    /// Resolve text token accounting for tone modifier.
    pub fn text_token(self, tone: ButtonTone) -> &'static str {
        match (self, tone) {
            (Self::Ghost, ButtonTone::Danger) => semantic::COLOR_STATUS_DANGER,
            (Self::Ghost, ButtonTone::Success) => semantic::COLOR_STATUS_SUCCESS,
            (Self::Ghost, ButtonTone::Warning) => semantic::COLOR_STATUS_WARNING,
            (Self::Ghost, ButtonTone::Default) => semantic::COLOR_TEXT_PRIMARY,
            (Self::Secondary, ButtonTone::Danger | ButtonTone::Success | ButtonTone::Warning) => {
                semantic::COLOR_TEXT_PRIMARY
            }
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
            // Info = the dedicated status-info color (matches Svelte
            // `--poodle-color-status-info`); pending stays accent.
            Self::Info => semantic::COLOR_STATUS_INFO,
            Self::Pending => semantic::COLOR_ACCENT_BASE,
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

/// The ternary value of a `TriStateSwitch`, in fixed display order.
///
/// The contract (`tri-state-switch.md`) names the three positions
/// `excluded | default | included`. `TriStateSwitchSpec` still stores a
/// `CheckState` for backward compatibility with existing call sites, but
/// this enum gives the component its real semantic surface
/// (`Unchecked → Excluded`, `Mixed → Default`, `Checked → Included`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TriStateValue {
    Excluded,
    Default,
    Included,
}

impl TriStateValue {
    /// Fixed segment index (excluded 0, default 1, included 2).
    pub fn index(self) -> usize {
        match self {
            Self::Excluded => 0,
            Self::Default => 1,
            Self::Included => 2,
        }
    }

    /// Map from the legacy `CheckState` storage.
    pub fn from_check_state(state: CheckState) -> Self {
        match state {
            CheckState::Unchecked => Self::Excluded,
            CheckState::Mixed => Self::Default,
            CheckState::Checked => Self::Included,
        }
    }

    /// Back to the legacy `CheckState` storage.
    pub fn to_check_state(self) -> CheckState {
        match self {
            Self::Excluded => CheckState::Unchecked,
            Self::Default => CheckState::Mixed,
            Self::Included => CheckState::Checked,
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

/// Descriptor for a collapsed danger action shown in the FormActions overflow
/// menu on narrow containers (contract `FormActionDangerItem`, §3). Mirrors the
/// Svelte `{ label, onSelect, value?, disabled? }` shape — `onSelect` is a host
/// callback that lives in the preview/event loop, so the spec carries only the
/// descriptor data needed to render the menu item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormActionDangerItem {
    /// Visible menu-item label.
    pub label: String,
    /// Stable value/key for the item. `None` falls back to `index:label`,
    /// matching the Svelte default.
    pub value: Option<String>,
    /// When true the item renders disabled and is not selectable.
    pub disabled: bool,
}

impl FormActionDangerItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: None,
            disabled: false,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Resolve the item's stable value, falling back to `index:label` when no
    /// explicit value was provided (matches Svelte `value ?? `${index}:${label}``).
    pub fn resolved_value(&self, index: usize) -> String {
        match &self.value {
            Some(v) => v.clone(),
            None => format!("{index}:{}", self.label),
        }
    }
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
    /// Optional group/section key. Options sharing the same `group` string are
    /// rendered under a shared section header in the dropdown. `None` means the
    /// option belongs to no explicit group (rendered at the top level).
    /// Group headers appear in the order the first option of each group is
    /// encountered in the `options` slice.
    pub group: Option<String>,
}

impl ChoiceOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            aria_label: None,
            is_disabled: false,
            group: None,
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

    /// Assign this option to a named group/section.
    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
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

/// Structured value for a `DateTimeZonePicker`: a local date, a local time, and
/// a timezone identifier. Each field is independently optional so partial values
/// (e.g. date committed but time/zone still empty) are representable, matching
/// the Svelte `ZonedDateTimeValue` shape.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct ZonedDateTimeValue {
    pub date: Option<String>,
    pub time: Option<String>,
    pub time_zone: Option<String>,
}

impl ZonedDateTimeValue {
    pub fn new(date: Option<String>, time: Option<String>, time_zone: Option<String>) -> Self {
        Self {
            date,
            time,
            time_zone,
        }
    }

    /// True when no constituent field has been committed yet.
    pub fn is_empty(&self) -> bool {
        self.date.is_none() && self.time.is_none() && self.time_zone.is_none()
    }
}

/// A curated timezone option for a `DateTimeZonePicker` / `TimeZoneSelect`,
/// matching the Svelte `TimeZoneOption` shape (`value` + display `label`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimeZoneOption {
    pub value: String,
    pub label: String,
}

impl TimeZoneOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
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

/// Surface width strategy for Popover (contract §3 `surfaceWidth`).
///
/// `Content` (default) sizes the surface to its content within the
/// min/max-width constraints; `Trigger` makes the surface match the
/// trigger width (`width: 100%` / `min-width: 100%`).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PopoverSurfaceWidth {
    #[default]
    Content,
    Trigger,
}

impl PopoverSurfaceWidth {
    pub fn is_trigger(self) -> bool {
        matches!(self, Self::Trigger)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DialogKind {
    Dialog,
    AlertDialog,
}

/// Width preset for the Dialog surface. Each preset maps to a
/// `min(<rem>, 100%)` rule in the Svelte implementation:
/// Sm = 24rem, Md = 34rem (default), Lg = 48rem, Xl = 64rem, Full = 100%.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DialogWidth {
    Sm,
    Md,
    Lg,
    Xl,
    Full,
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
    /// Flat tabs with a bottom border on the list and a pill-shaped
    /// accent-tinted selection on the tab itself. The default. Renamed from
    /// the former `Underline`/`text` member; the former `Card` member is
    /// deleted — its decorations moved to `activeEdge` / `activeFill`.
    /// Matches Svelte `data-variant="card"`.
    Card,
    Pill,
    /// Full-width tabs with vertical separators, no radius, no outer chrome.
    /// Selected item receives an accent-tinted surface fill; the former
    /// `strip` variant's look is `Block` + `activeEdge::Underline`.
    /// Matches Svelte `data-variant="block"`.
    Block,
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
    /// When true the entry renders in a destructive tone
    /// (status.danger foreground). Used for items like Delete, Remove.
    pub is_destructive: bool,
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
            is_destructive: false,
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

    pub fn with_destructive(mut self, is_destructive: bool) -> Self {
        self.is_destructive = is_destructive;
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
    /// Optional leading icon name (contract §3 `icon`). When set, the trigger
    /// renders the icon ahead of the label, separated by the trigger gap token.
    pub icon: Option<String>,
    pub description: Option<String>,
}

impl NavigationMenuEntry {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            is_disabled: false,
            icon: None,
            description: None,
        }
    }

    /// Set the optional leading icon (contract §3 `icon`).
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
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
    /// Optional leading icon name (Lucide icon identifier). When set,
    /// the tab renders the icon before the label.
    pub icon: Option<String>,
    /// Optional count shown as a badge next to the label, e.g. "12".
    pub count: Option<u32>,
}

impl TabDefinition {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            is_disabled: false,
            is_closable: false,
            icon: None,
            count: None,
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

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_count(mut self, count: u32) -> Self {
        self.count = Some(count);
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

#[cfg(test)]
mod semantic_size_tests {
    use super::*;

    /// The full table, transcribed from the Svelte `resolveSemanticControlSize`.
    /// Pinning every cell is the point: this rule had five private copies, and
    /// a spot check would not have caught one of them drifting.
    #[test]
    fn matches_the_svelte_size_role_table() {
        use ControlSize::*;
        use SemanticControlSizeRole::*;

        let cases = [
            (Xs, Chrome, Xs),
            (Sm, Chrome, Sm),
            (Md, Chrome, Sm),
            (Lg, Chrome, Md),
            (Xl, Chrome, Lg),
            (Xs, Control, Xs),
            (Sm, Control, Sm),
            (Md, Control, Md),
            (Lg, Control, Lg),
            (Xl, Control, Xl),
            (Xs, Prominent, Sm),
            (Sm, Prominent, Md),
            (Md, Prominent, Lg),
            (Lg, Prominent, Xl),
            (Xl, Prominent, Xl),
        ];

        for (size, role, expected) in cases {
            assert_eq!(
                resolve_semantic_control_size(size, role),
                expected,
                "{size:?} + {role:?}",
            );
        }
    }

    /// Chrome clamps at the bottom and Prominent at the top, so a role can
    /// never walk a size off either end of the scale.
    #[test]
    fn roles_clamp_at_both_ends() {
        assert_eq!(
            resolve_semantic_control_size(ControlSize::Xs, SemanticControlSizeRole::Chrome),
            ControlSize::Xs,
        );
        assert_eq!(
            resolve_semantic_control_size(ControlSize::Xl, SemanticControlSizeRole::Prominent),
            ControlSize::Xl,
        );
    }
}
