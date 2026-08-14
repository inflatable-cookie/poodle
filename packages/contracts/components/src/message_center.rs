use crate::{ControlDensity, ControlSize, OverlayPlacement, SemanticControlSizeRole, StatusTone};

/// Optional progress presentation for a live item row.
///
/// A pure projection of host-owned values; the component never advances,
/// estimates, or mutates it. `max` defaults to `100`.
#[derive(Clone, Debug, PartialEq)]
pub struct MessageCenterItemProgress {
    pub value: Option<f64>,
    pub max: f64,
    pub indeterminate: bool,
}

impl MessageCenterItemProgress {
    pub fn determinate(value: f64) -> Self {
        Self {
            value: Some(value),
            max: 100.0,
            indeterminate: false,
        }
    }

    pub fn indeterminate() -> Self {
        Self {
            value: None,
            max: 100.0,
            indeterminate: true,
        }
    }

    pub fn with_max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }
}

/// One durable message or live activity row displayed by [`MessageCenterSpec`].
#[derive(Clone, Debug, PartialEq)]
pub struct MessageCenterItem {
    pub id: String,
    pub title: String,
    pub message: Option<String>,
    pub meta: Option<String>,
    /// ISO-8601 timestamp consumed by `TimeAgo`; hosts may leave it absent.
    pub timestamp: Option<String>,
    pub read: bool,
    pub tone: StatusTone,
    pub icon: Option<String>,
    /// Compact progress presentation under the copy; absent for plain messages.
    pub progress: Option<MessageCenterItemProgress>,
    /// Whether the row may be activated through `on_item_select`.
    pub selectable: bool,
    /// Whether the row shows a remove control when `on_remove` is supplied.
    pub removable: bool,
    /// Whether the row shows a read toggle when `on_read_change` is supplied.
    pub read_control: bool,
}

impl MessageCenterItem {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            message: None,
            meta: None,
            timestamp: None,
            read: false,
            tone: StatusTone::Info,
            icon: None,
            progress: None,
            selectable: true,
            removable: true,
            read_control: true,
        }
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_meta(mut self, meta: impl Into<String>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    pub fn with_timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }

    pub fn with_read(mut self, read: bool) -> Self {
        self.read = read;
        self
    }

    pub fn with_tone(mut self, tone: StatusTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_progress(mut self, progress: MessageCenterItemProgress) -> Self {
        self.progress = Some(progress);
        self
    }

    pub fn with_selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    pub fn with_removable(mut self, removable: bool) -> Self {
        self.removable = removable;
        self
    }

    pub fn with_read_control(mut self, read_control: bool) -> Self {
        self.read_control = read_control;
        self
    }

    /// A live activity row: retained in place, not selectable, not removable,
    /// and not part of the unread/read cycle.
    pub fn as_live_row(mut self) -> Self {
        self.read = true;
        self.selectable = false;
        self.removable = false;
        self.read_control = false;
        self
    }
}

/// Host-owned state for the always-available message archive.
#[derive(Clone, Debug, PartialEq)]
pub struct MessageCenterSpec {
    pub items: Vec<MessageCenterItem>,
    pub open: Option<bool>,
    pub default_open: bool,
    pub title: String,
    pub aria_label: Option<String>,
    pub trigger_label: Option<String>,
    pub trigger_icon: String,
    pub placement: OverlayPlacement,
    pub empty_title: String,
    pub empty_message: String,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for MessageCenterSpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            open: None,
            default_open: false,
            title: "Notifications".into(),
            aria_label: None,
            trigger_label: None,
            trigger_icon: "bell".into(),
            placement: OverlayPlacement::BottomEnd,
            empty_title: "No messages".into(),
            empty_message: "New messages will appear here.".into(),
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Chrome,
            density: ControlDensity::Default,
        }
    }
}

impl MessageCenterSpec {
    pub fn new(items: Vec<MessageCenterItem>) -> Self {
        Self {
            items,
            ..Self::default()
        }
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn with_trigger_label(mut self, label: impl Into<String>) -> Self {
        self.trigger_label = Some(label.into());
        self
    }

    pub fn with_trigger_icon(mut self, icon: impl Into<String>) -> Self {
        self.trigger_icon = icon.into();
        self
    }

    pub fn with_placement(mut self, placement: OverlayPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    pub fn unread_count(&self) -> usize {
        self.items.iter().filter(|item| !item.read).count()
    }

    pub fn effective_aria_label(&self) -> &str {
        self.aria_label.as_deref().unwrap_or(&self.title)
    }

    pub fn effective_trigger_label(&self) -> String {
        self.trigger_label.clone().unwrap_or_else(|| {
            let unread = self.unread_count();
            if unread == 0 {
                self.title.clone()
            } else {
                format!("{}, {unread} unread", self.title)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_open_and_unread_presentation_from_host_state() {
        let spec = MessageCenterSpec::new(vec![
            MessageCenterItem::new("one", "First"),
            MessageCenterItem::new("two", "Second").with_read(true),
        ])
        .with_default_open(true);

        assert!(spec.current_open());
        assert_eq!(spec.unread_count(), 1);
        assert_eq!(spec.effective_trigger_label(), "Notifications, 1 unread");
    }

    #[test]
    fn items_default_to_full_interaction_policy() {
        let item = MessageCenterItem::new("one", "First");
        assert!(item.selectable);
        assert!(item.removable);
        assert!(item.read_control);
        assert!(item.progress.is_none());
    }

    #[test]
    fn live_row_opts_out_of_interactions_and_unread() {
        let item = MessageCenterItem::new("job", "Mix preview")
            .with_progress(MessageCenterItemProgress::determinate(60.0))
            .as_live_row();

        assert!(item.read);
        assert!(!item.selectable);
        assert!(!item.removable);
        assert!(!item.read_control);
        assert_eq!(item.progress.as_ref().unwrap().value, Some(60.0));
    }

    #[test]
    fn progress_preserves_max_and_indeterminate_meaning() {
        let determinate = MessageCenterItemProgress::determinate(3.0).with_max(5.0);
        assert_eq!(determinate.value, Some(3.0));
        assert_eq!(determinate.max, 5.0);
        assert!(!determinate.indeterminate);

        let indeterminate = MessageCenterItemProgress::indeterminate();
        assert!(indeterminate.value.is_none());
        assert!(indeterminate.indeterminate);
    }
}
