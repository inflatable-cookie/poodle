//! MotionPolicyProvider specimen — Jetstream has no motion clocks.
//!
//! Contract: `docs/contracts/components/motion-policy-provider.md`

use crate::compat::js_spinner;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::SpinnerSpec;

pub fn render(theme: &JetstreamThemeProvider) -> El {
    div()
        .flex_col()
        .child(div().child("full / reduced / frozen — Jetstream paints a static spinner"))
        .child(js_spinner(&SpinnerSpec::new(), theme))
}
