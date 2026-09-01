use poodle_headless::motion_policy::MotionPolicy;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MotionPolicyProviderSpec {
    pub policy: MotionPolicy,
}

impl MotionPolicyProviderSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_policy(mut self, policy: MotionPolicy) -> Self {
        self.policy = policy;
        self
    }
}
