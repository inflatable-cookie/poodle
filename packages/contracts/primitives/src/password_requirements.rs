use poodle_tokens::semantic;

/// Policy describing the password strength constraints to display.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PasswordRequirementsPolicy {
    pub min_length: usize,
    pub require_mixed_case: bool,
    pub require_digit: bool,
    pub require_special: bool,
    pub min_strength_score: Option<usize>,
    pub description: Option<String>,
}

impl PasswordRequirementsPolicy {
    pub fn new(min_length: usize) -> Self {
        Self {
            min_length,
            require_mixed_case: false,
            require_digit: false,
            require_special: false,
            min_strength_score: None,
            description: None,
        }
    }

    pub fn with_require_mixed_case(mut self, require: bool) -> Self {
        self.require_mixed_case = require;
        self
    }

    pub fn with_require_digit(mut self, require: bool) -> Self {
        self.require_digit = require;
        self
    }

    pub fn with_require_special(mut self, require: bool) -> Self {
        self.require_special = require;
        self
    }

    pub fn with_min_strength_score(mut self, score: usize) -> Self {
        self.min_strength_score = Some(score);
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// PasswordRequirements -- a live-feedback checklist showing password policy satisfaction.
#[derive(Clone, Debug, PartialEq)]
pub struct PasswordRequirementsSpec {
    pub password: String,
    pub requirements: Option<PasswordRequirementsPolicy>,
    pub is_loading: bool,
    pub error: Option<String>,
    pub title: String,
    pub hint: Option<String>,
    pub loading_label: String,
}

impl Default for PasswordRequirementsSpec {
    fn default() -> Self {
        Self {
            password: String::new(),
            requirements: None,
            is_loading: false,
            error: None,
            title: String::from("Password requirements"),
            hint: Some(String::from(
                "Avoid common words, patterns, and personal information.",
            )),
            loading_label: String::from("Loading requirements..."),
        }
    }
}

impl PasswordRequirementsSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = password.into();
        self
    }

    pub fn with_requirements(mut self, requirements: PasswordRequirementsPolicy) -> Self {
        self.requirements = Some(requirements);
        self
    }

    pub fn with_loading(mut self, is_loading: bool) -> Self {
        self.is_loading = is_loading;
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn with_loading_label(mut self, label: impl Into<String>) -> Self {
        self.loading_label = label.into();
        self
    }

    /// Whether the minimum-length requirement is met.
    pub fn length_met(&self) -> bool {
        match &self.requirements {
            Some(policy) => self.password.len() >= policy.min_length,
            None => false,
        }
    }

    /// Whether the mixed-case requirement is met (or not required).
    pub fn mixed_case_met(&self) -> bool {
        match &self.requirements {
            Some(policy) if policy.require_mixed_case => {
                let has_lower = self.password.chars().any(|c| c.is_ascii_lowercase());
                let has_upper = self.password.chars().any(|c| c.is_ascii_uppercase());
                has_lower && has_upper
            }
            _ => true,
        }
    }

    /// Whether the digit requirement is met (or not required).
    pub fn digit_met(&self) -> bool {
        match &self.requirements {
            Some(policy) if policy.require_digit => {
                self.password.chars().any(|c| c.is_ascii_digit())
            }
            _ => true,
        }
    }

    /// Whether the special-character requirement is met (or not required).
    pub fn special_met(&self) -> bool {
        match &self.requirements {
            Some(policy) if policy.require_special => {
                self.password.chars().any(|c| !c.is_ascii_alphanumeric())
            }
            _ => true,
        }
    }

    /// Whether all active requirements are satisfied.
    pub fn all_met(&self) -> bool {
        self.requirements.is_some()
            && self.length_met()
            && self.mixed_case_met()
            && self.digit_met()
            && self.special_met()
    }

    /// The number of individual requirement rules that are currently active.
    pub fn active_rule_count(&self) -> usize {
        match &self.requirements {
            None => 0,
            Some(policy) => {
                let mut count = 1; // min_length is always active
                if policy.require_mixed_case {
                    count += 1;
                }
                if policy.require_digit {
                    count += 1;
                }
                if policy.require_special {
                    count += 1;
                }
                count
            }
        }
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn title_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn met_color_token(&self) -> &'static str {
        semantic::COLOR_STATUS_SUCCESS
    }

    pub fn error_color_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
    }
}
