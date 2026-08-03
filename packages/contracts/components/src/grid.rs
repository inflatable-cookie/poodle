use poodle_tokens::semantic;

use crate::types::{Dimension, Inset, PaddingScale};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GridSpec {
    pub columns: Dimension,
    pub rows: Option<Dimension>,
    pub gap: PaddingScale,
    pub padding: PaddingScale,
    pub role: Option<String>,
    pub aria_label: Option<String>,
}

impl Default for GridSpec {
    fn default() -> Self {
        Self {
            columns: Dimension::from("1fr"),
            rows: None,
            gap: PaddingScale::Md,
            padding: PaddingScale::None,
            role: None,
            aria_label: None,
        }
    }
}

/// One parsed column track from `grid-template-columns`.
///
/// Only the two forms the contract specimens use are modelled precisely; any
/// other CSS grid track syntax degrades to `Fr(1.0)` so it still renders.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridTrack {
    /// A fractional track (`1fr`, `2fr`). The weight drives `flex-grow`.
    Fr(f32),
    /// A fixed track in rem (`200px` is normalised to rem at parse time? no —
    /// only rem is modelled). Width is the rem value.
    Rem(f32),
}

/// Result of parsing `grid-template-columns`.
#[derive(Clone, Debug, PartialEq)]
pub enum GridColumns {
    /// Explicit ordered tracks, e.g. `1fr 2fr` → `[Fr(1.0), Fr(2.0)]`.
    Tracks(Vec<GridTrack>),
    /// `repeat(auto-fit | auto-fill, minmax(<min>rem, 1fr))` — children wrap,
    /// each at least `min_rem` wide and growing to fill the row.
    AutoFit { min_rem: f32 },
}

/// Contract §8 `SpaceScale` map for Grid `gap`/`padding`:
/// `none`→0, `sm`→`space-inline-sm`, `md`→`space-panel-y`, `lg`→`space-panel-x`.
///
/// This is distinct from `PaddingScale::inline_gap`/`stack_gap`/`layout_inset`,
/// so Grid resolves through this dedicated mapping rather than reusing those.
fn grid_space(scale: PaddingScale) -> Option<&'static str> {
    match scale {
        PaddingScale::None => None,
        PaddingScale::Sm => Some(semantic::SPACE_INLINE_SM),
        PaddingScale::Md => Some(semantic::SPACE_PANEL_Y),
        PaddingScale::Lg => Some(semantic::SPACE_PANEL_X),
    }
}

impl GridSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_columns(mut self, columns: impl Into<Dimension>) -> Self {
        self.columns = columns.into();
        self
    }

    pub fn with_rows(mut self, rows: impl Into<Dimension>) -> Self {
        self.rows = Some(rows.into());
        self
    }

    pub fn with_gap(mut self, gap: PaddingScale) -> Self {
        self.gap = gap;
        self
    }

    pub fn with_padding(mut self, padding: PaddingScale) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    /// Column gap token. Contract §8: single `gap` value, same on both axes,
    /// resolved through the Grid `SpaceScale` map (`md`→`space-panel-y`).
    pub fn resolved_column_gap(&self) -> Option<&'static str> {
        grid_space(self.gap)
    }

    /// Row gap token. CSS `gap` is one value applied to both axes, so the row
    /// gap is identical to the column gap (contract §8 documents one `gap`).
    pub fn resolved_row_gap(&self) -> Option<&'static str> {
        grid_space(self.gap)
    }

    /// Interior padding. Contract §8 maps `padding` through the same Grid
    /// `SpaceScale` (`md`→`space-panel-y`, `lg`→`space-panel-x`), applied
    /// uniformly on both axes (CSS `padding: <value>`).
    pub fn resolved_padding(&self) -> Inset {
        match grid_space(self.padding) {
            Some(token) => Inset {
                horizontal: Some(token),
                vertical: Some(token),
            },
            None => Inset::none(),
        }
    }

    /// Parse `columns` into a track model the Rust targets can approximate with
    /// flex. Recognises:
    /// - `repeat(auto-fit | auto-fill, minmax(<n>rem, 1fr))` → `AutoFit`
    /// - space-separated `<n>fr` / `<n>rem` tracks → `Tracks`
    /// - `repeat(<n>, 1fr)` → `n` equal `Fr(1.0)` tracks
    ///
    /// Anything unrecognised falls back to a single `Fr(1.0)` track so the grid
    /// still renders (equal-width approximation).
    pub fn parsed_columns(&self) -> GridColumns {
        parse_columns(self.columns.as_str())
    }
}

fn parse_columns(raw: &str) -> GridColumns {
    let s = raw.trim();

    if let Some(rest) = s.strip_prefix("repeat(") {
        let inner = rest.strip_suffix(')').unwrap_or(rest);
        // Split on the first comma into count/keyword + track spec.
        if let Some((head, tail)) = inner.split_once(',') {
            let head = head.trim();
            let tail = tail.trim();
            if head == "auto-fit" || head == "auto-fill" {
                if let Some(min_rem) = parse_minmax_min_rem(tail) {
                    return GridColumns::AutoFit { min_rem };
                }
            }
            if let Ok(n) = head.parse::<usize>() {
                if n > 0 {
                    let track = parse_track(tail).unwrap_or(GridTrack::Fr(1.0));
                    return GridColumns::Tracks(vec![track; n]);
                }
            }
        }
        return GridColumns::Tracks(vec![GridTrack::Fr(1.0)]);
    }

    let tracks: Vec<GridTrack> = s
        .split_whitespace()
        .map(|t| parse_track(t).unwrap_or(GridTrack::Fr(1.0)))
        .collect();

    if tracks.is_empty() {
        GridColumns::Tracks(vec![GridTrack::Fr(1.0)])
    } else {
        GridColumns::Tracks(tracks)
    }
}

/// Parse the `<min>` of `minmax(<min>rem, 1fr)` → rem value.
fn parse_minmax_min_rem(spec: &str) -> Option<f32> {
    let inner = spec.strip_prefix("minmax(")?;
    let inner = inner.strip_suffix(')').unwrap_or(inner);
    let min = inner.split(',').next()?.trim();
    min.strip_suffix("rem")?.trim().parse::<f32>().ok()
}

/// Parse a single track token: `1fr`/`2fr` → `Fr`, `8rem` → `Rem`.
fn parse_track(tok: &str) -> Option<GridTrack> {
    let tok = tok.trim();
    if let Some(n) = tok.strip_suffix("fr") {
        return n.trim().parse::<f32>().ok().map(GridTrack::Fr);
    }
    if let Some(n) = tok.strip_suffix("rem") {
        return n.trim().parse::<f32>().ok().map(GridTrack::Rem);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_tokens::semantic;

    #[test]
    fn gap_resolves_contract_space_scale() {
        // Contract §8: none→0, sm→inline-sm, md→panel-y, lg→panel-x.
        assert_eq!(
            GridSpec::new()
                .with_gap(PaddingScale::None)
                .resolved_column_gap(),
            None
        );
        assert_eq!(
            GridSpec::new()
                .with_gap(PaddingScale::Sm)
                .resolved_column_gap(),
            Some(semantic::SPACE_INLINE_SM)
        );
        assert_eq!(
            GridSpec::new()
                .with_gap(PaddingScale::Md)
                .resolved_column_gap(),
            Some(semantic::SPACE_PANEL_Y)
        );
        assert_eq!(
            GridSpec::new()
                .with_gap(PaddingScale::Lg)
                .resolved_column_gap(),
            Some(semantic::SPACE_PANEL_X)
        );
    }

    #[test]
    fn row_gap_equals_column_gap() {
        // CSS `gap` is one value on both axes.
        let spec = GridSpec::new().with_gap(PaddingScale::Md);
        assert_eq!(spec.resolved_column_gap(), spec.resolved_row_gap());
    }

    #[test]
    fn padding_resolves_contract_space_scale_both_axes() {
        let inset = GridSpec::new()
            .with_padding(PaddingScale::Md)
            .resolved_padding();
        assert_eq!(inset.horizontal, Some(semantic::SPACE_PANEL_Y));
        assert_eq!(inset.vertical, Some(semantic::SPACE_PANEL_Y));

        let none = GridSpec::new()
            .with_padding(PaddingScale::None)
            .resolved_padding();
        assert_eq!(none, Inset::none());
    }

    #[test]
    fn parses_equal_fr_tracks() {
        let cols = GridSpec::new().with_columns("1fr 1fr 1fr").parsed_columns();
        assert_eq!(
            cols,
            GridColumns::Tracks(vec![
                GridTrack::Fr(1.0),
                GridTrack::Fr(1.0),
                GridTrack::Fr(1.0)
            ])
        );
    }

    #[test]
    fn parses_mixed_ratio_tracks() {
        let cols = GridSpec::new().with_columns("1fr 2fr").parsed_columns();
        assert_eq!(
            cols,
            GridColumns::Tracks(vec![GridTrack::Fr(1.0), GridTrack::Fr(2.0)])
        );
    }

    #[test]
    fn parses_repeat_count() {
        let cols = GridSpec::new()
            .with_columns("repeat(3, 1fr)")
            .parsed_columns();
        assert_eq!(
            cols,
            GridColumns::Tracks(vec![
                GridTrack::Fr(1.0),
                GridTrack::Fr(1.0),
                GridTrack::Fr(1.0)
            ])
        );
    }

    #[test]
    fn parses_auto_fit_minmax() {
        let cols = GridSpec::new()
            .with_columns("repeat(auto-fit, minmax(8rem, 1fr))")
            .parsed_columns();
        assert_eq!(cols, GridColumns::AutoFit { min_rem: 8.0 });
    }

    #[test]
    fn parses_fixed_rem_track() {
        let cols = GridSpec::new().with_columns("8rem").parsed_columns();
        assert_eq!(cols, GridColumns::Tracks(vec![GridTrack::Rem(8.0)]));
    }

    #[test]
    fn unrecognised_columns_fall_back_to_single_fr() {
        let cols = GridSpec::new().with_columns("auto").parsed_columns();
        assert_eq!(cols, GridColumns::Tracks(vec![GridTrack::Fr(1.0)]));
    }

    #[test]
    fn aria_label_round_trips() {
        let spec = GridSpec::new().with_role("list").with_aria_label("Cards");
        assert_eq!(spec.role.as_deref(), Some("list"));
        assert_eq!(spec.aria_label.as_deref(), Some("Cards"));
    }
}
