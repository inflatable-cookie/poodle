use poodle_headless::licence::LicenceSeat;

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// LicenceSeats — a list of authority-reported activation seats with inline
/// machine naming and release requests.
///
/// Contract: `docs/contracts/components/licence-seats.md`
///
/// Raw `machine_id` values are callback data and list keys only; everything
/// visible and accessible stays label-based (`Unnamed machine` for an
/// unnamed row). Rows, release names, and confirm bodies derive once through
/// `poodle_headless::licence::licence_seat_rows`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LicenceSeatsSpec {
    /// Accepts authority getters directly. Empty renders nothing.
    pub seats: Vec<LicenceSeat>,
    /// Disables that row's duplicate release.
    pub pending_machine_id: Option<String>,
    pub title: String,
    pub release_label: String,
    /// Uses the warning confirmation before emitting.
    pub confirm_release: bool,
    /// Host state for the composed per-row ConfirmAction (Rust targets own
    /// the open state, exactly like every other overlay in the vocabulary).
    pub open_confirm_machine_id: Option<String>,
    /// Host state for the composed per-row EditableLabel.
    pub editing_machine_id: Option<String>,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
}

impl Default for LicenceSeatsSpec {
    fn default() -> Self {
        Self {
            seats: Vec::new(),
            pending_machine_id: None,
            title: "Activated machines".to_string(),
            release_label: "Release".to_string(),
            confirm_release: true,
            open_confirm_machine_id: None,
            editing_machine_id: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }
}

impl LicenceSeatsSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_seats(mut self, seats: Vec<LicenceSeat>) -> Self {
        self.seats = seats;
        self
    }

    pub fn with_seat(mut self, seat: LicenceSeat) -> Self {
        self.seats.push(seat);
        self
    }

    pub fn with_pending_machine_id(mut self, machine_id: Option<String>) -> Self {
        self.pending_machine_id = machine_id;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_release_label(mut self, release_label: impl Into<String>) -> Self {
        self.release_label = release_label.into();
        self
    }

    pub fn with_confirm_release(mut self, confirm_release: bool) -> Self {
        self.confirm_release = confirm_release;
        self
    }

    pub fn with_open_confirm(mut self, machine_id: Option<String>) -> Self {
        self.open_confirm_machine_id = machine_id;
        self
    }

    pub fn with_editing_machine(mut self, machine_id: Option<String>) -> Self {
        self.editing_machine_id = machine_id;
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }
}
