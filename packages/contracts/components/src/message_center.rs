use crate::{ControlDensity, ControlSize, OverlayPlacement, SemanticControlSizeRole, StatusTone};

/// One durable message displayed by [`MessageCenterSpec`].
#[derive(Clone, Debug, Eq, PartialEq)]
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
}
