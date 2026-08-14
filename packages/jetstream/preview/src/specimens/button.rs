//! Button specimen — corpus projection (g14.001).
//!
//! The case corpus authored in `packages/core/src/conformance/button-cases.ts`
//! drives the specimen structure: groups, captions, axes, and fixtures are
//! rendered from the generated JSON by `conformance_support`, never restated
//! here.

use crate::conformance_support::render_corpus_groups;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

pub fn render(theme: &JetstreamThemeProvider) -> El {
    render_corpus_groups(theme)
}
