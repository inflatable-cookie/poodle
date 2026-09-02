use std::collections::BTreeMap;
use std::fmt;

const GRID_SIZE: i64 = 24;
const QUANTIZATION_SCALE: f64 = 10_000.0;
const SAMPLE_COUNT: usize = 64;
const MAX_CONTOURS: usize = 8;
const MAX_SAMPLES: usize = 512;
const CIRCLE_SEGMENTS: usize = 32;
const COST_EPSILON: f64 = 1e-12;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GeometryPoint {
    pub(crate) x: i64,
    pub(crate) y: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GeometrySegment {
    pub(crate) start: GeometryPoint,
    pub(crate) end: GeometryPoint,
    pub(crate) closing: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CanonicalContour {
    closed: bool,
    segments: Vec<GeometrySegment>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CanonicalGeometry {
    contours: Vec<CanonicalContour>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SampledContour {
    closed: bool,
    points: Vec<GeometryPoint>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SampledGeometry {
    contours: Vec<SampledContour>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GeometryTopology {
    closed: Vec<bool>,
    segment_counts: Vec<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NormalizedIconGeometry {
    canonical: CanonicalGeometry,
    sampled: SampledGeometry,
    topology: GeometryTopology,
    element_types: Vec<String>,
}

#[derive(Clone, Debug)]
struct IconGeometryInput {
    view_box: [f64; 4],
    fill: String,
    stroke: String,
    stroke_width: f64,
    stroke_linecap: String,
    stroke_linejoin: String,
    nodes: Vec<(String, BTreeMap<String, String>)>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GeometryErrorCode {
    InvalidViewBox,
    InvalidPaint,
    UnsupportedElement,
    UnsupportedAttribute,
    UnsupportedTransform,
    UnsupportedPathCommand,
    MalformedPath,
    InvalidNumber,
    RoundedRect,
    EmptyContour,
    DegenerateContour,
    OutOfBounds,
    TooManyContours,
    TooManySamples,
    PairContourCount,
    PairClosure,
    PairPlanning,
}

impl GeometryErrorCode {
    fn as_str(self) -> &'static str {
        match self {
            Self::InvalidViewBox => "invalid-view-box",
            Self::InvalidPaint => "invalid-paint",
            Self::UnsupportedElement => "unsupported-element",
            Self::UnsupportedAttribute => "unsupported-attribute",
            Self::UnsupportedTransform => "unsupported-transform",
            Self::UnsupportedPathCommand => "unsupported-path-command",
            Self::MalformedPath => "malformed-path",
            Self::InvalidNumber => "invalid-number",
            Self::RoundedRect => "rounded-rect",
            Self::EmptyContour => "empty-contour",
            Self::DegenerateContour => "degenerate-contour",
            Self::OutOfBounds => "out-of-bounds",
            Self::TooManyContours => "too-many-contours",
            Self::TooManySamples => "too-many-samples",
            Self::PairContourCount => "pair-contour-count",
            Self::PairClosure => "pair-closure",
            Self::PairPlanning => "pair-planning",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GeometryError {
    code: GeometryErrorCode,
    message: String,
}

impl GeometryError {
    fn new(code: GeometryErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for GeometryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code.as_str(), self.message)
    }
}

impl std::error::Error for GeometryError {}

type FloatPoint = [f64; 2];

#[derive(Clone, Debug)]
struct RawContour {
    points: Vec<FloatPoint>,
    closed: bool,
}

#[derive(Clone, Debug)]
enum PathToken {
    Command(char),
    Number(f64),
}

fn is_separator(byte: u8) -> bool {
    byte == b',' || byte.is_ascii_whitespace()
}

fn is_number_start(byte: u8) -> bool {
    byte.is_ascii_digit() || byte == b'+' || byte == b'-' || byte == b'.'
}

fn parse_number_at(value: &str, index: &mut usize) -> Result<f64, GeometryError> {
    let bytes = value.as_bytes();
    let start = *index;
    if matches!(bytes.get(*index), Some(b'+' | b'-')) {
        *index += 1;
    }

    let integer_start = *index;
    while bytes.get(*index).is_some_and(u8::is_ascii_digit) {
        *index += 1;
    }
    let mut digits = *index - integer_start;

    if bytes.get(*index) == Some(&b'.') {
        *index += 1;
        let fraction_start = *index;
        while bytes.get(*index).is_some_and(u8::is_ascii_digit) {
            *index += 1;
        }
        digits += *index - fraction_start;
    }
    if digits == 0 {
        return Err(GeometryError::new(
            GeometryErrorCode::MalformedPath,
            format!("unexpected number at {start}"),
        ));
    }

    if matches!(bytes.get(*index), Some(b'e' | b'E')) {
        *index += 1;
        if matches!(bytes.get(*index), Some(b'+' | b'-')) {
            *index += 1;
        }
        let exponent_start = *index;
        while bytes.get(*index).is_some_and(u8::is_ascii_digit) {
            *index += 1;
        }
        if *index == exponent_start {
            return Err(GeometryError::new(
                GeometryErrorCode::MalformedPath,
                format!("invalid exponent at {start}"),
            ));
        }
    }

    let parsed = value[start..*index]
        .parse::<f64>()
        .map_err(|_| GeometryError::new(GeometryErrorCode::InvalidNumber, &value[start..*index]))?;
    if !parsed.is_finite() {
        return Err(GeometryError::new(
            GeometryErrorCode::InvalidNumber,
            &value[start..*index],
        ));
    }
    Ok(parsed)
}

fn tokenize_path(value: &str) -> Result<Vec<PathToken>, GeometryError> {
    let bytes = value.as_bytes();
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        while index < bytes.len() && is_separator(bytes[index]) {
            index += 1;
        }
        if index == bytes.len() {
            break;
        }
        let byte = bytes[index];
        if byte.is_ascii_alphabetic() {
            let command = byte as char;
            if !matches!(
                command,
                'M' | 'm' | 'L' | 'l' | 'H' | 'h' | 'V' | 'v' | 'Z' | 'z'
            ) {
                return Err(GeometryError::new(
                    GeometryErrorCode::UnsupportedPathCommand,
                    command.to_string(),
                ));
            }
            tokens.push(PathToken::Command(command));
            index += 1;
            continue;
        }
        if !is_number_start(byte) {
            return Err(GeometryError::new(
                GeometryErrorCode::MalformedPath,
                format!("unexpected token at {index}"),
            ));
        }
        tokens.push(PathToken::Number(parse_number_at(value, &mut index)?));
    }
    Ok(tokens)
}

fn take_numbers(
    tokens: &[PathToken],
    index: &mut usize,
    count: usize,
    command: char,
) -> Result<Vec<f64>, GeometryError> {
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        match tokens.get(*index) {
            Some(PathToken::Number(value)) => values.push(*value),
            _ => {
                return Err(GeometryError::new(
                    GeometryErrorCode::MalformedPath,
                    format!("command {command} needs {count} values"),
                ));
            }
        }
        *index += 1;
    }
    Ok(values)
}

fn parse_path(value: &str) -> Result<Vec<RawContour>, GeometryError> {
    let tokens = tokenize_path(value)?;
    let mut contours = Vec::new();
    let mut token_index = 0;
    let mut command = None;
    let mut command_needs_values = false;
    let mut current: Option<RawContour> = None;
    let mut point = [0.0, 0.0];
    let mut start = [0.0, 0.0];

    while token_index < tokens.len() {
        if let PathToken::Command(next) = tokens[token_index] {
            command = Some(next);
            command_needs_values = true;
            token_index += 1;
            if matches!(next, 'Z' | 'z') {
                let contour = current.as_mut().ok_or_else(|| {
                    GeometryError::new(GeometryErrorCode::MalformedPath, "closepath has no moveto")
                })?;
                if contour.points.len() < 2 {
                    return Err(GeometryError::new(
                        GeometryErrorCode::EmptyContour,
                        "closepath has no segment",
                    ));
                }
                contour.closed = true;
                point = start;
                command = None;
                command_needs_values = false;
            }
            continue;
        }

        let active = command.ok_or_else(|| {
            GeometryError::new(
                GeometryErrorCode::MalformedPath,
                "numbers require a command",
            )
        })?;
        let absolute = active.is_ascii_uppercase();

        match active {
            'M' | 'm' => {
                let values = take_numbers(&tokens, &mut token_index, 2, active)?;
                if let Some(previous) = current.take() {
                    if previous.points.len() < 2 {
                        return Err(GeometryError::new(
                            GeometryErrorCode::EmptyContour,
                            "a contour needs at least one segment",
                        ));
                    }
                    contours.push(previous);
                }
                point = if absolute {
                    [values[0], values[1]]
                } else {
                    [point[0] + values[0], point[1] + values[1]]
                };
                start = point;
                current = Some(RawContour {
                    points: vec![point],
                    closed: false,
                });
                command = Some(if absolute { 'L' } else { 'l' });
                command_needs_values = false;
            }
            'L' | 'l' => {
                let values = take_numbers(&tokens, &mut token_index, 2, active)?;
                let contour = current.as_mut().ok_or_else(|| {
                    GeometryError::new(GeometryErrorCode::MalformedPath, "lineto has no moveto")
                })?;
                if contour.closed {
                    return Err(GeometryError::new(
                        GeometryErrorCode::MalformedPath,
                        "drawing after closepath requires moveto",
                    ));
                }
                point = if absolute {
                    [values[0], values[1]]
                } else {
                    [point[0] + values[0], point[1] + values[1]]
                };
                contour.points.push(point);
                command_needs_values = false;
            }
            'H' | 'h' => {
                let values = take_numbers(&tokens, &mut token_index, 1, active)?;
                let contour = current.as_mut().ok_or_else(|| {
                    GeometryError::new(
                        GeometryErrorCode::MalformedPath,
                        "horizontal lineto has no moveto",
                    )
                })?;
                if contour.closed {
                    return Err(GeometryError::new(
                        GeometryErrorCode::MalformedPath,
                        "drawing after closepath requires moveto",
                    ));
                }
                point = if absolute {
                    [values[0], point[1]]
                } else {
                    [point[0] + values[0], point[1]]
                };
                contour.points.push(point);
                command_needs_values = false;
            }
            'V' | 'v' => {
                let values = take_numbers(&tokens, &mut token_index, 1, active)?;
                let contour = current.as_mut().ok_or_else(|| {
                    GeometryError::new(
                        GeometryErrorCode::MalformedPath,
                        "vertical lineto has no moveto",
                    )
                })?;
                if contour.closed {
                    return Err(GeometryError::new(
                        GeometryErrorCode::MalformedPath,
                        "drawing after closepath requires moveto",
                    ));
                }
                point = if absolute {
                    [point[0], values[0]]
                } else {
                    [point[0], point[1] + values[0]]
                };
                contour.points.push(point);
                command_needs_values = false;
            }
            _ => {
                return Err(GeometryError::new(
                    GeometryErrorCode::UnsupportedPathCommand,
                    active.to_string(),
                ));
            }
        }
    }

    if command_needs_values {
        return Err(GeometryError::new(
            GeometryErrorCode::MalformedPath,
            format!("command {:?} has no values", command),
        ));
    }
    if let Some(last) = current {
        if last.points.len() < 2 {
            return Err(GeometryError::new(
                GeometryErrorCode::EmptyContour,
                "a contour needs at least one segment",
            ));
        }
        contours.push(last);
    }
    if contours.is_empty() {
        return Err(GeometryError::new(
            GeometryErrorCode::EmptyContour,
            "path has no contours",
        ));
    }
    Ok(contours)
}

fn parse_number_attribute(
    attrs: &BTreeMap<String, String>,
    key: &str,
    fallback: Option<f64>,
) -> Result<f64, GeometryError> {
    let Some(raw) = attrs.get(key) else {
        return fallback.ok_or_else(|| {
            GeometryError::new(GeometryErrorCode::InvalidNumber, format!("missing {key}"))
        });
    };
    let raw = raw.trim();
    let mut end = 0;
    let value = parse_number_at(raw, &mut end).map_err(|_| {
        GeometryError::new(
            GeometryErrorCode::InvalidNumber,
            format!("{key} is not a valid SVG number"),
        )
    })?;
    if end != raw.len() {
        return Err(GeometryError::new(
            GeometryErrorCode::InvalidNumber,
            format!("{key} is not a valid SVG number"),
        ));
    }
    if !value.is_finite() {
        return Err(GeometryError::new(
            GeometryErrorCode::InvalidNumber,
            format!("{key} is not finite"),
        ));
    }
    Ok(value)
}

fn validate_attributes(
    tag: &str,
    attrs: &BTreeMap<String, String>,
    allowed: &[&str],
) -> Result<(), GeometryError> {
    for key in attrs.keys() {
        if key == "transform" {
            return Err(GeometryError::new(
                GeometryErrorCode::UnsupportedTransform,
                format!("{tag}.{key}"),
            ));
        }
        if !allowed.contains(&key.as_str()) {
            return Err(GeometryError::new(
                GeometryErrorCode::UnsupportedAttribute,
                format!("{tag}.{key}"),
            ));
        }
    }
    Ok(())
}

fn parse_point_list(value: &str) -> Result<Vec<FloatPoint>, GeometryError> {
    let bytes = value.as_bytes();
    let mut values = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        while index < bytes.len() && is_separator(bytes[index]) {
            index += 1;
        }
        if index == bytes.len() {
            break;
        }
        if !is_number_start(bytes[index]) {
            return Err(GeometryError::new(
                GeometryErrorCode::MalformedPath,
                format!("invalid points at {index}"),
            ));
        }
        values.push(parse_number_at(value, &mut index)?);
    }
    if values.len() < 4 || values.len() % 2 != 0 {
        return Err(GeometryError::new(
            GeometryErrorCode::MalformedPath,
            "points must contain coordinate pairs",
        ));
    }
    Ok(values
        .chunks_exact(2)
        .map(|pair| [pair[0], pair[1]])
        .collect())
}

fn lower_node(
    tag: &str,
    attrs: &BTreeMap<String, String>,
) -> Result<Vec<RawContour>, GeometryError> {
    match tag {
        "path" => {
            validate_attributes(tag, attrs, &["d"])?;
            let value = attrs.get("d").ok_or_else(|| {
                GeometryError::new(GeometryErrorCode::MalformedPath, "path has no d attribute")
            })?;
            parse_path(value)
        }
        "line" => {
            validate_attributes(tag, attrs, &["x1", "x2", "y1", "y2"])?;
            Ok(vec![RawContour {
                points: vec![
                    [
                        parse_number_attribute(attrs, "x1", None)?,
                        parse_number_attribute(attrs, "y1", None)?,
                    ],
                    [
                        parse_number_attribute(attrs, "x2", None)?,
                        parse_number_attribute(attrs, "y2", None)?,
                    ],
                ],
                closed: false,
            }])
        }
        "polyline" => {
            validate_attributes(tag, attrs, &["points"])?;
            Ok(vec![RawContour {
                points: parse_point_list(attrs.get("points").map_or("", String::as_str))?,
                closed: false,
            }])
        }
        "polygon" => {
            validate_attributes(tag, attrs, &["points"])?;
            Ok(vec![RawContour {
                points: parse_point_list(attrs.get("points").map_or("", String::as_str))?,
                closed: true,
            }])
        }
        "rect" => {
            validate_attributes(tag, attrs, &["x", "y", "width", "height", "rx", "ry"])?;
            let rx = parse_number_attribute(attrs, "rx", Some(0.0))?;
            let ry = parse_number_attribute(attrs, "ry", Some(0.0))?;
            if rx != 0.0 || ry != 0.0 {
                return Err(GeometryError::new(
                    GeometryErrorCode::RoundedRect,
                    "rounded rectangles are unsupported",
                ));
            }
            let x = parse_number_attribute(attrs, "x", Some(0.0))?;
            let y = parse_number_attribute(attrs, "y", Some(0.0))?;
            let width = parse_number_attribute(attrs, "width", None)?;
            let height = parse_number_attribute(attrs, "height", None)?;
            if width <= 0.0 || height <= 0.0 {
                return Err(GeometryError::new(
                    GeometryErrorCode::DegenerateContour,
                    "rect has no area",
                ));
            }
            Ok(vec![RawContour {
                points: vec![
                    [x, y],
                    [x + width, y],
                    [x + width, y + height],
                    [x, y + height],
                ],
                closed: true,
            }])
        }
        "circle" => {
            validate_attributes(tag, attrs, &["cx", "cy", "r"])?;
            let cx = parse_number_attribute(attrs, "cx", Some(0.0))?;
            let cy = parse_number_attribute(attrs, "cy", Some(0.0))?;
            let radius = parse_number_attribute(attrs, "r", None)?;
            if radius <= 0.0 {
                return Err(GeometryError::new(
                    GeometryErrorCode::DegenerateContour,
                    "circle has no radius",
                ));
            }
            Ok(vec![RawContour {
                points: (0..CIRCLE_SEGMENTS)
                    .map(|index| {
                        let angle =
                            index as f64 * (std::f64::consts::PI * 2.0) / CIRCLE_SEGMENTS as f64;
                        [cx + radius * angle.cos(), cy + radius * angle.sin()]
                    })
                    .collect(),
                closed: true,
            }])
        }
        "ellipse" => {
            validate_attributes(tag, attrs, &["cx", "cy", "rx", "ry"])?;
            let cx = parse_number_attribute(attrs, "cx", Some(0.0))?;
            let cy = parse_number_attribute(attrs, "cy", Some(0.0))?;
            let rx = parse_number_attribute(attrs, "rx", None)?;
            let ry = parse_number_attribute(attrs, "ry", None)?;
            if rx <= 0.0 || ry <= 0.0 {
                return Err(GeometryError::new(
                    GeometryErrorCode::DegenerateContour,
                    "ellipse has no area",
                ));
            }
            Ok(vec![RawContour {
                points: (0..CIRCLE_SEGMENTS)
                    .map(|index| {
                        let angle =
                            index as f64 * (std::f64::consts::PI * 2.0) / CIRCLE_SEGMENTS as f64;
                        [cx + rx * angle.cos(), cy + ry * angle.sin()]
                    })
                    .collect(),
                closed: true,
            }])
        }
        _ => Err(GeometryError::new(
            GeometryErrorCode::UnsupportedElement,
            tag,
        )),
    }
}

fn quantize(value: f64) -> Result<i64, GeometryError> {
    if !value.is_finite() {
        return Err(GeometryError::new(
            GeometryErrorCode::InvalidNumber,
            "non-finite coordinate",
        ));
    }
    Ok((value * QUANTIZATION_SCALE).round() as i64)
}

fn quantize_point(point: FloatPoint) -> Result<GeometryPoint, GeometryError> {
    let result = GeometryPoint {
        x: quantize(point[0])?,
        y: quantize(point[1])?,
    };
    let max = GRID_SIZE * QUANTIZATION_SCALE as i64;
    if result.x < 0 || result.x > max || result.y < 0 || result.y > max {
        return Err(GeometryError::new(
            GeometryErrorCode::OutOfBounds,
            format!("{},{}", point[0], point[1]),
        ));
    }
    Ok(result)
}

fn canonicalize_contour(raw: &RawContour) -> Result<CanonicalContour, GeometryError> {
    if raw.points.is_empty() {
        return Err(GeometryError::new(
            GeometryErrorCode::EmptyContour,
            "contour has no points",
        ));
    }
    let mut points = raw
        .points
        .iter()
        .copied()
        .map(quantize_point)
        .collect::<Result<Vec<_>, _>>()?;
    if raw.closed && points.first() == points.last() {
        points.pop();
    }
    if points.len() < 2 {
        return Err(GeometryError::new(
            GeometryErrorCode::EmptyContour,
            "a contour has fewer than two points",
        ));
    }

    let mut segments = Vec::new();
    for pair in points.windows(2) {
        if pair[0] == pair[1] {
            return Err(GeometryError::new(
                GeometryErrorCode::DegenerateContour,
                "a contour contains a zero-length segment",
            ));
        }
        segments.push(GeometrySegment {
            start: pair[0],
            end: pair[1],
            closing: false,
        });
    }
    if raw.closed {
        let last = *points.last().expect("two points checked");
        let first = points[0];
        if last == first {
            return Err(GeometryError::new(
                GeometryErrorCode::DegenerateContour,
                "a closed contour has no closing segment",
            ));
        }
        segments.push(GeometrySegment {
            start: last,
            end: first,
            closing: true,
        });
    }
    Ok(CanonicalContour {
        closed: raw.closed,
        segments,
    })
}

fn canonical_points(contour: &CanonicalContour) -> Result<Vec<GeometryPoint>, GeometryError> {
    let first = contour.segments.first().ok_or_else(|| {
        GeometryError::new(GeometryErrorCode::EmptyContour, "contour has no segments")
    })?;
    let mut points = vec![first.start];
    points.extend(
        contour
            .segments
            .iter()
            .filter(|segment| !segment.closing)
            .map(|segment| segment.end),
    );
    Ok(points)
}

fn distance(left: GeometryPoint, right: GeometryPoint) -> f64 {
    let x = (left.x - right.x) as f64;
    let y = (left.y - right.y) as f64;
    (x * x + y * y).sqrt()
}

fn sample_contour(contour: &CanonicalContour) -> Result<SampledContour, GeometryError> {
    let lengths = contour
        .segments
        .iter()
        .map(|segment| distance(segment.start, segment.end))
        .collect::<Vec<_>>();
    let total = lengths.iter().sum::<f64>();
    if total <= 0.0 {
        return Err(GeometryError::new(
            GeometryErrorCode::DegenerateContour,
            "contour has no length",
        ));
    }

    let mut points = Vec::with_capacity(SAMPLE_COUNT);
    for index in 0..SAMPLE_COUNT {
        let target = if contour.closed {
            index as f64 / SAMPLE_COUNT as f64 * total
        } else {
            index as f64 / (SAMPLE_COUNT - 1) as f64 * total
        };
        let mut cursor = 0.0;
        let mut edge_index = lengths.len() - 1;
        for (candidate, length) in lengths.iter().enumerate() {
            if target <= cursor + length || candidate == lengths.len() - 1 {
                edge_index = candidate;
                break;
            }
            cursor += length;
        }
        let edge = &contour.segments[edge_index];
        let ratio = if lengths[edge_index] == 0.0 {
            0.0
        } else {
            (target - cursor) / lengths[edge_index]
        };
        points.push(GeometryPoint {
            x: ((edge.start.x as f64) + (edge.end.x - edge.start.x) as f64 * ratio).round() as i64,
            y: ((edge.start.y as f64) + (edge.end.y - edge.start.y) as f64 * ratio).round() as i64,
        });
    }
    Ok(SampledContour {
        closed: contour.closed,
        points,
    })
}

fn validate_input(input: &IconGeometryInput) -> Result<(), GeometryError> {
    if input.view_box != [0.0, 0.0, 24.0, 24.0] {
        return Err(GeometryError::new(
            GeometryErrorCode::InvalidViewBox,
            "expected 0 0 24 24",
        ));
    }
    if input.fill != "none"
        || input.stroke != "currentColor"
        || input.stroke_width != 2.0
        || input.stroke_linecap != "round"
        || input.stroke_linejoin != "round"
    {
        return Err(GeometryError::new(
            GeometryErrorCode::InvalidPaint,
            "expected the canonical Lucide stroke paint",
        ));
    }
    Ok(())
}

fn normalize_icon_geometry(
    input: &IconGeometryInput,
) -> Result<NormalizedIconGeometry, GeometryError> {
    validate_input(input)?;
    let mut raw_contours = Vec::new();
    let mut element_types = Vec::new();
    for (tag, attrs) in &input.nodes {
        element_types.push(tag.clone());
        raw_contours.extend(lower_node(tag, attrs)?);
    }
    if raw_contours.is_empty() {
        return Err(GeometryError::new(
            GeometryErrorCode::EmptyContour,
            "icon has no geometry",
        ));
    }
    if raw_contours.len() > MAX_CONTOURS {
        return Err(GeometryError::new(
            GeometryErrorCode::TooManyContours,
            format!("{} contours", raw_contours.len()),
        ));
    }
    let canonical_contours = raw_contours
        .iter()
        .map(canonicalize_contour)
        .collect::<Result<Vec<_>, _>>()?;
    let sampled_contours = canonical_contours
        .iter()
        .map(sample_contour)
        .collect::<Result<Vec<_>, _>>()?;
    let sample_total = sampled_contours
        .iter()
        .map(|contour| contour.points.len())
        .sum::<usize>();
    if sample_total > MAX_SAMPLES {
        return Err(GeometryError::new(
            GeometryErrorCode::TooManySamples,
            format!("{sample_total} samples"),
        ));
    }
    Ok(NormalizedIconGeometry {
        topology: GeometryTopology {
            closed: canonical_contours
                .iter()
                .map(|contour| contour.closed)
                .collect(),
            segment_counts: canonical_contours
                .iter()
                .map(|contour| contour.segments.len())
                .collect(),
        },
        canonical: CanonicalGeometry {
            contours: canonical_contours,
        },
        sampled: SampledGeometry {
            contours: sampled_contours,
        },
        element_types,
    })
}

#[derive(Clone, Copy, Debug)]
struct ContourMetrics {
    length: f64,
    centroid: [f64; 2],
    bounds: [i64; 4],
}

fn contour_metrics(contour: &SampledContour) -> ContourMetrics {
    let first = contour.points[0];
    let mut metrics = ContourMetrics {
        length: 0.0,
        centroid: [0.0, 0.0],
        bounds: [first.x, first.y, first.x, first.y],
    };
    for (index, point) in contour.points.iter().copied().enumerate() {
        metrics.centroid[0] += point.x as f64;
        metrics.centroid[1] += point.y as f64;
        metrics.bounds[0] = metrics.bounds[0].min(point.x);
        metrics.bounds[1] = metrics.bounds[1].min(point.y);
        metrics.bounds[2] = metrics.bounds[2].max(point.x);
        metrics.bounds[3] = metrics.bounds[3].max(point.y);
        if index + 1 < contour.points.len() || contour.closed {
            metrics.length += distance(point, contour.points[(index + 1) % contour.points.len()]);
        }
    }
    metrics.centroid[0] /= contour.points.len() as f64;
    metrics.centroid[1] /= contour.points.len() as f64;
    metrics
}

fn modulo(value: isize, divisor: usize) -> usize {
    ((value % divisor as isize + divisor as isize) % divisor as isize) as usize
}

fn oriented_points(contour: &SampledContour, reversed: bool, offset: usize) -> Vec<GeometryPoint> {
    contour
        .points
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let source = if reversed {
                modulo(offset as isize - index as isize, contour.points.len())
            } else {
                modulo(offset as isize + index as isize, contour.points.len())
            };
            contour.points[source]
        })
        .collect()
}

fn contour_cost(
    left: &SampledContour,
    right: &SampledContour,
    right_points: &[GeometryPoint],
) -> f64 {
    let left_metrics = contour_metrics(left);
    let right_metrics = contour_metrics(&SampledContour {
        closed: right.closed,
        points: right_points.to_vec(),
    });
    let grid = (GRID_SIZE * QUANTIZATION_SCALE as i64) as f64;
    let length_cost = (left_metrics.length - right_metrics.length).abs() / grid;
    let centroid_x = left_metrics.centroid[0] - right_metrics.centroid[0];
    let centroid_y = left_metrics.centroid[1] - right_metrics.centroid[1];
    let centroid_cost = (centroid_x * centroid_x + centroid_y * centroid_y).sqrt() / grid;
    let bounds_cost = (0..4)
        .map(|index| (left_metrics.bounds[index] - right_metrics.bounds[index]).abs() as f64)
        .sum::<f64>()
        / (grid * 4.0);
    let shape_cost = left
        .points
        .iter()
        .copied()
        .zip(right_points.iter().copied())
        .map(|(left, right)| distance(left, right).powi(2))
        .sum::<f64>()
        / (left.points.len() as f64 * grid * grid);
    length_cost + centroid_cost + bounds_cost + shape_cost
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ContourCorrespondence {
    left_index: usize,
    right_index: usize,
    reversed: bool,
    offset: usize,
    cost_micros: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct IconGeometryPairPlan {
    contour_mappings: Vec<ContourCorrespondence>,
    cost_micros: i64,
}

#[derive(Clone, Debug)]
struct CorrespondenceOption {
    correspondence: ContourCorrespondence,
    cost: f64,
}

fn better_option(
    candidate: &ContourCorrespondence,
    candidate_cost: f64,
    current: Option<&CorrespondenceOption>,
) -> bool {
    let Some(current) = current else { return true };
    if candidate_cost < current.cost - COST_EPSILON {
        return true;
    }
    if (candidate_cost - current.cost).abs() > COST_EPSILON {
        return false;
    }
    if candidate.reversed != current.correspondence.reversed {
        return !candidate.reversed;
    }
    candidate.offset < current.correspondence.offset
}

fn best_correspondence(
    left: &SampledContour,
    right: &SampledContour,
    left_index: usize,
    right_index: usize,
) -> CorrespondenceOption {
    let offsets = if right.closed {
        (0..right.points.len()).collect::<Vec<_>>()
    } else {
        vec![0]
    };
    let mut best = None;
    for reversed in [false, true] {
        for offset in &offsets {
            let actual_offset = if reversed && !right.closed {
                right.points.len() - 1
            } else {
                *offset
            };
            let right_points = oriented_points(right, reversed, actual_offset);
            let cost = contour_cost(left, right, &right_points);
            let correspondence = ContourCorrespondence {
                left_index,
                right_index,
                reversed,
                offset: actual_offset,
                cost_micros: (cost * 1_000_000.0).round() as i64,
            };
            if better_option(&correspondence, cost, best.as_ref()) {
                best = Some(CorrespondenceOption {
                    correspondence,
                    cost,
                });
            }
        }
    }
    best.expect("at least one correspondence option")
}

fn lexicographically_before(left: &[usize], right: &[usize]) -> bool {
    left.iter()
        .zip(right.iter())
        .find_map(|(left, right)| (*left != *right).then_some(left < right))
        .unwrap_or(false)
}

struct Assignment {
    assignment: Vec<usize>,
    cost: f64,
    mappings: Vec<ContourCorrespondence>,
}

fn visit_assignment(
    assignment: &mut Vec<usize>,
    used: &mut [bool],
    cost: f64,
    options: &[Vec<Option<CorrespondenceOption>>],
    best: &mut Option<Assignment>,
) {
    let left_index = assignment.len();
    if left_index == options.len() {
        let mappings = assignment
            .iter()
            .enumerate()
            .map(|(index, right_index)| {
                options[index][*right_index]
                    .as_ref()
                    .expect("assigned contours have compatible closure")
                    .correspondence
                    .clone()
            })
            .collect::<Vec<_>>();
        let replace = best.as_ref().is_none_or(|current| {
            cost < current.cost - COST_EPSILON
                || ((cost - current.cost).abs() <= COST_EPSILON
                    && lexicographically_before(assignment, &current.assignment))
        });
        if replace {
            *best = Some(Assignment {
                assignment: assignment.clone(),
                cost,
                mappings,
            });
        }
        return;
    }
    for right_index in 0..options.len() {
        let Some(option) = options[left_index][right_index].as_ref() else {
            continue;
        };
        if used[right_index] {
            continue;
        }
        used[right_index] = true;
        assignment.push(right_index);
        visit_assignment(assignment, used, cost + option.cost, options, best);
        assignment.pop();
        used[right_index] = false;
    }
}

fn plan_icon_geometry_pair(
    left: &NormalizedIconGeometry,
    right: &NormalizedIconGeometry,
) -> Result<IconGeometryPairPlan, GeometryError> {
    if left.sampled.contours.len() != right.sampled.contours.len() {
        return Err(GeometryError::new(
            GeometryErrorCode::PairContourCount,
            "endpoints have different contour counts",
        ));
    }
    let left_closed_count = left
        .sampled
        .contours
        .iter()
        .filter(|contour| contour.closed)
        .count();
    let right_closed_count = right
        .sampled
        .contours
        .iter()
        .filter(|contour| contour.closed)
        .count();
    if left_closed_count != right_closed_count {
        return Err(GeometryError::new(
            GeometryErrorCode::PairClosure,
            "endpoints have different closure signatures",
        ));
    }

    let options = left
        .sampled
        .contours
        .iter()
        .enumerate()
        .map(|(left_index, left_contour)| {
            right
                .sampled
                .contours
                .iter()
                .enumerate()
                .map(|(right_index, right_contour)| {
                    (left_contour.closed == right_contour.closed).then(|| {
                        best_correspondence(left_contour, right_contour, left_index, right_index)
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut best = None;
    visit_assignment(
        &mut Vec::new(),
        &mut vec![false; options.len()],
        0.0,
        &options,
        &mut best,
    );
    let best = best.ok_or_else(|| {
        GeometryError::new(GeometryErrorCode::PairPlanning, "no contour assignment")
    })?;
    Ok(IconGeometryPairPlan {
        contour_mappings: best.mappings,
        cost_micros: (best.cost * 1_000_000.0).round() as i64,
    })
}

fn reverse_pair_plan(plan: &IconGeometryPairPlan) -> IconGeometryPairPlan {
    let mut mappings = plan.contour_mappings.clone();
    mappings.sort_by_key(|mapping| mapping.right_index);
    IconGeometryPairPlan {
        contour_mappings: mappings
            .into_iter()
            .map(|mapping| ContourCorrespondence {
                left_index: mapping.right_index,
                right_index: mapping.left_index,
                reversed: mapping.reversed,
                offset: if mapping.reversed {
                    mapping.offset
                } else {
                    modulo(
                        SAMPLE_COUNT as isize - mapping.offset as isize,
                        SAMPLE_COUNT,
                    )
                },
                cost_micros: mapping.cost_micros,
            })
            .collect(),
        cost_micros: plan.cost_micros,
    }
}

fn geometry_wire_text(geometry: &NormalizedIconGeometry) -> String {
    let mut fields = vec![
        "icon-geometry-wire-v1".to_owned(),
        "schema=1".to_owned(),
        "normalizer=1.0.0".to_owned(),
        format!("elements={}", geometry.element_types.join(",")),
        format!("contours={}", geometry.canonical.contours.len()),
        format!("sample-count={SAMPLE_COUNT}"),
    ];
    for (index, contour) in geometry.canonical.contours.iter().enumerate() {
        let sampled = &geometry.sampled.contours[index];
        fields.push(format!(
            "contour-{index}-closed={}",
            if contour.closed { 1 } else { 0 }
        ));
        fields.push(format!(
            "contour-{index}-segments={}",
            contour
                .segments
                .iter()
                .map(|segment| {
                    format!(
                        "{},{},{},{},{}",
                        segment.start.x,
                        segment.start.y,
                        segment.end.x,
                        segment.end.y,
                        if segment.closing { 1 } else { 0 }
                    )
                })
                .collect::<Vec<_>>()
                .join(";")
        ));
        fields.push(format!(
            "contour-{index}-samples={}",
            sampled
                .points
                .iter()
                .map(|point| format!("{},{}", point.x, point.y))
                .collect::<Vec<_>>()
                .join(";")
        ));
    }
    fields.push(format!(
        "topology-closed={}",
        geometry
            .topology
            .closed
            .iter()
            .map(|closed| if *closed { "1" } else { "0" })
            .collect::<Vec<_>>()
            .join(",")
    ));
    fields.push(format!(
        "topology-segments={}",
        geometry
            .topology
            .segment_counts
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(",")
    ));
    fields.join("|")
}

fn fnv1a64(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3_u64);
    }
    format!("{hash:016x}")
}

fn geometry_wire_digest(geometry: &NormalizedIconGeometry) -> String {
    fnv1a64(&geometry_wire_text(geometry))
}

fn pair_wire_digest(
    left: &NormalizedIconGeometry,
    right: &NormalizedIconGeometry,
    plan: &IconGeometryPairPlan,
) -> String {
    let mappings = plan
        .contour_mappings
        .iter()
        .map(|mapping| {
            format!(
                "{},{},{},{},{}",
                mapping.left_index,
                mapping.right_index,
                if mapping.reversed { 1 } else { 0 },
                mapping.offset,
                mapping.cost_micros
            )
        })
        .collect::<Vec<_>>()
        .join(";");
    fnv1a64(&format!(
        "icon-geometry-pair-wire-v1|left={}|right={}|mappings={}|cost={}",
        geometry_wire_text(left),
        geometry_wire_text(right),
        mappings,
        plan.cost_micros
    ))
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GeometryFrame {
    contours: Vec<SampledContour>,
}

struct PlannedIconGeometryPair<'a> {
    left: &'a NormalizedIconGeometry,
    right: &'a NormalizedIconGeometry,
    plan: &'a IconGeometryPairPlan,
}

fn canonical_frame(geometry: &NormalizedIconGeometry) -> Result<GeometryFrame, GeometryError> {
    Ok(GeometryFrame {
        contours: geometry
            .canonical
            .contours
            .iter()
            .map(|contour| {
                Ok(SampledContour {
                    closed: contour.closed,
                    points: canonical_points(contour)?,
                })
            })
            .collect::<Result<Vec<_>, GeometryError>>()?,
    })
}

fn frame_at(
    pair: &PlannedIconGeometryPair<'_>,
    progress: f64,
) -> Result<GeometryFrame, GeometryError> {
    if !progress.is_finite() {
        return Err(GeometryError::new(
            GeometryErrorCode::PairPlanning,
            "progress is not finite",
        ));
    }
    if progress <= 0.0 {
        return canonical_frame(pair.left);
    }
    if progress >= 1.0 {
        return canonical_frame(pair.right);
    }
    let mut contours = Vec::with_capacity(pair.left.sampled.contours.len());
    for (left_index, left_contour) in pair.left.sampled.contours.iter().enumerate() {
        let mapping = pair
            .plan
            .contour_mappings
            .iter()
            .find(|mapping| mapping.left_index == left_index)
            .ok_or_else(|| {
                GeometryError::new(
                    GeometryErrorCode::PairPlanning,
                    format!("missing mapping for contour {left_index}"),
                )
            })?;
        let right_contour = &pair.right.sampled.contours[mapping.right_index];
        let right_points = oriented_points(right_contour, mapping.reversed, mapping.offset);
        contours.push(SampledContour {
            closed: left_contour.closed,
            points: left_contour
                .points
                .iter()
                .copied()
                .zip(right_points)
                .map(|(left, right)| GeometryPoint {
                    x: ((left.x as f64) + (right.x - left.x) as f64 * progress).round() as i64,
                    y: ((left.y as f64) + (right.y - left.y) as f64 * progress).round() as i64,
                })
                .collect(),
        });
    }
    Ok(GeometryFrame { contours })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GeneratedPairStatus {
    Candidate,
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct GeneratedGeometryContour {
    pub(crate) closed: bool,
    pub(crate) segments: &'static [GeometrySegment],
    pub(crate) samples: &'static [GeometryPoint],
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct GeneratedIconGeometry {
    pub(crate) element_types: &'static [&'static str],
    pub(crate) contours: &'static [GeneratedGeometryContour],
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct GeneratedContourCorrespondence {
    pub(crate) left_index: usize,
    pub(crate) right_index: usize,
    pub(crate) reversed: bool,
    pub(crate) offset: usize,
    pub(crate) cost_micros: i64,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct GeneratedIconGeometryPlan {
    pub(crate) cost_micros: i64,
    pub(crate) contour_mappings: &'static [GeneratedContourCorrespondence],
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct GeneratedIconGeometryPair {
    pub(crate) id: &'static str,
    pub(crate) authored_from: &'static str,
    pub(crate) authored_to: &'static str,
    pub(crate) canonical_from: &'static str,
    pub(crate) canonical_to: &'static str,
    pub(crate) semantic: &'static str,
    pub(crate) status: GeneratedPairStatus,
    pub(crate) source_digest_left: &'static str,
    pub(crate) source_digest_right: &'static str,
    pub(crate) asset_digest_left: &'static str,
    pub(crate) asset_digest_right: &'static str,
    pub(crate) normalizer_version: &'static str,
    pub(crate) schema_version: u32,
    pub(crate) quality_status: &'static str,
    pub(crate) quality_reviewer: &'static str,
    pub(crate) quality_notes: &'static str,
    pub(crate) rejection_reason: Option<&'static str>,
    pub(crate) diagnostic_code: Option<&'static str>,
    pub(crate) derived_digest: Option<&'static str>,
    pub(crate) payload_bytes: usize,
    pub(crate) geometry_left: Option<&'static GeneratedIconGeometry>,
    pub(crate) geometry_right: Option<&'static GeneratedIconGeometry>,
    pub(crate) plan: Option<&'static GeneratedIconGeometryPlan>,
}

include!("icon_geometry.generated.rs");

#[cfg(test)]
mod tests {
    use super::*;

    const VECTOR_SOURCE: &str = include_str!("../../../core/src/icons/geometry-vectors.json");

    #[derive(Clone, Debug)]
    enum JsonValue {
        Object(BTreeMap<String, JsonValue>),
        Array(Vec<JsonValue>),
        String(String),
        Number(String),
        Bool(bool),
        Null,
    }

    struct JsonParser<'a> {
        bytes: &'a [u8],
        index: usize,
    }

    impl<'a> JsonParser<'a> {
        fn parse(source: &'a str) -> Result<JsonValue, String> {
            let mut parser = Self {
                bytes: source.as_bytes(),
                index: 0,
            };
            let value = parser.value()?;
            parser.whitespace();
            if parser.index != parser.bytes.len() {
                return Err(format!("trailing JSON at {}", parser.index));
            }
            Ok(value)
        }

        fn whitespace(&mut self) {
            while self
                .bytes
                .get(self.index)
                .is_some_and(u8::is_ascii_whitespace)
            {
                self.index += 1;
            }
        }

        fn value(&mut self) -> Result<JsonValue, String> {
            self.whitespace();
            match self.bytes.get(self.index).copied() {
                Some(b'{') => self.object(),
                Some(b'[') => self.array(),
                Some(b'"') => self.string().map(JsonValue::String),
                Some(b't') => self.literal(b"true", JsonValue::Bool(true)),
                Some(b'f') => self.literal(b"false", JsonValue::Bool(false)),
                Some(b'n') => self.literal(b"null", JsonValue::Null),
                Some(b'-' | b'0'..=b'9') => self.number(),
                Some(byte) => Err(format!("unexpected JSON byte {byte} at {}", self.index)),
                None => Err("unexpected end of JSON".to_owned()),
            }
        }

        fn literal(&mut self, expected: &[u8], value: JsonValue) -> Result<JsonValue, String> {
            if self.bytes.get(self.index..self.index + expected.len()) != Some(expected) {
                return Err(format!("invalid JSON literal at {}", self.index));
            }
            self.index += expected.len();
            Ok(value)
        }

        fn object(&mut self) -> Result<JsonValue, String> {
            self.index += 1;
            let mut values = BTreeMap::new();
            self.whitespace();
            if self.bytes.get(self.index) == Some(&b'}') {
                self.index += 1;
                return Ok(JsonValue::Object(values));
            }
            loop {
                self.whitespace();
                let key = self.string()?;
                self.whitespace();
                if self.bytes.get(self.index) != Some(&b':') {
                    return Err(format!("missing object colon at {}", self.index));
                }
                self.index += 1;
                let value = self.value()?;
                values.insert(key, value);
                self.whitespace();
                match self.bytes.get(self.index).copied() {
                    Some(b',') => self.index += 1,
                    Some(b'}') => {
                        self.index += 1;
                        return Ok(JsonValue::Object(values));
                    }
                    _ => return Err(format!("invalid object separator at {}", self.index)),
                }
            }
        }

        fn array(&mut self) -> Result<JsonValue, String> {
            self.index += 1;
            let mut values = Vec::new();
            self.whitespace();
            if self.bytes.get(self.index) == Some(&b']') {
                self.index += 1;
                return Ok(JsonValue::Array(values));
            }
            loop {
                values.push(self.value()?);
                self.whitespace();
                match self.bytes.get(self.index).copied() {
                    Some(b',') => self.index += 1,
                    Some(b']') => {
                        self.index += 1;
                        return Ok(JsonValue::Array(values));
                    }
                    _ => return Err(format!("invalid array separator at {}", self.index)),
                }
            }
        }

        fn string(&mut self) -> Result<String, String> {
            if self.bytes.get(self.index) != Some(&b'"') {
                return Err(format!("expected JSON string at {}", self.index));
            }
            self.index += 1;
            let mut value = String::new();
            loop {
                let byte = *self
                    .bytes
                    .get(self.index)
                    .ok_or_else(|| "unterminated JSON string".to_owned())?;
                self.index += 1;
                match byte {
                    b'"' => return Ok(value),
                    b'\\' => {
                        let escaped = *self
                            .bytes
                            .get(self.index)
                            .ok_or_else(|| "unterminated JSON escape".to_owned())?;
                        self.index += 1;
                        match escaped {
                            b'"' => value.push('"'),
                            b'\\' => value.push('\\'),
                            b'/' => value.push('/'),
                            b'b' => value.push('\u{0008}'),
                            b'f' => value.push('\u{000c}'),
                            b'n' => value.push('\n'),
                            b'r' => value.push('\r'),
                            b't' => value.push('\t'),
                            b'u' => {
                                let mut code = 0_u32;
                                for _ in 0..4 {
                                    let digit = *self
                                        .bytes
                                        .get(self.index)
                                        .ok_or_else(|| "unterminated unicode escape".to_owned())?;
                                    self.index += 1;
                                    code = code * 16
                                        + match digit {
                                            b'0'..=b'9' => u32::from(digit - b'0'),
                                            b'a'..=b'f' => u32::from(digit - b'a' + 10),
                                            b'A'..=b'F' => u32::from(digit - b'A' + 10),
                                            _ => return Err("invalid unicode escape".to_owned()),
                                        };
                                }
                                value.push(
                                    char::from_u32(code)
                                        .ok_or_else(|| "invalid unicode scalar".to_owned())?,
                                );
                            }
                            _ => return Err("invalid JSON escape".to_owned()),
                        }
                    }
                    byte if byte >= 0x20 => value.push(byte as char),
                    _ => return Err("control byte in JSON string".to_owned()),
                }
            }
        }

        fn number(&mut self) -> Result<JsonValue, String> {
            let start = self.index;
            if self.bytes.get(self.index) == Some(&b'-') {
                self.index += 1;
            }
            match self.bytes.get(self.index).copied() {
                Some(b'0') => self.index += 1,
                Some(b'1'..=b'9') => {
                    self.index += 1;
                    while self.bytes.get(self.index).is_some_and(u8::is_ascii_digit) {
                        self.index += 1;
                    }
                }
                _ => return Err(format!("invalid JSON number at {start}")),
            }
            if self.bytes.get(self.index) == Some(&b'.') {
                self.index += 1;
                let fraction_start = self.index;
                while self.bytes.get(self.index).is_some_and(u8::is_ascii_digit) {
                    self.index += 1;
                }
                if self.index == fraction_start {
                    return Err(format!("invalid JSON fraction at {start}"));
                }
            }
            if matches!(self.bytes.get(self.index), Some(b'e' | b'E')) {
                self.index += 1;
                if matches!(self.bytes.get(self.index), Some(b'+' | b'-')) {
                    self.index += 1;
                }
                let exponent_start = self.index;
                while self.bytes.get(self.index).is_some_and(u8::is_ascii_digit) {
                    self.index += 1;
                }
                if self.index == exponent_start {
                    return Err(format!("invalid JSON exponent at {start}"));
                }
            }
            Ok(JsonValue::Number(
                String::from_utf8(self.bytes[start..self.index].to_vec())
                    .map_err(|_| "JSON number is not UTF-8".to_owned())?,
            ))
        }
    }

    fn object(value: &JsonValue) -> &BTreeMap<String, JsonValue> {
        match value {
            JsonValue::Object(value) => value,
            _ => panic!("expected JSON object"),
        }
    }

    fn array(value: &JsonValue) -> &[JsonValue] {
        match value {
            JsonValue::Array(value) => value,
            _ => panic!("expected JSON array"),
        }
    }

    fn field<'a>(value: &'a JsonValue, key: &str) -> &'a JsonValue {
        object(value)
            .get(key)
            .unwrap_or_else(|| panic!("missing JSON field {key}"))
    }

    fn optional_field<'a>(value: &'a JsonValue, key: &str) -> Option<&'a JsonValue> {
        object(value).get(key)
    }

    fn string(value: &JsonValue) -> &str {
        match value {
            JsonValue::String(value) => value,
            _ => panic!("expected JSON string"),
        }
    }

    fn number(value: &JsonValue) -> &str {
        match value {
            JsonValue::Number(value) => value,
            _ => panic!("expected JSON number"),
        }
    }

    fn number_f64(value: &JsonValue) -> f64 {
        number(value).parse().expect("valid JSON float")
    }

    fn number_i64(value: &JsonValue) -> i64 {
        number(value).parse().expect("valid JSON integer")
    }

    fn number_usize(value: &JsonValue) -> usize {
        number(value).parse().expect("valid JSON usize")
    }

    fn boolean(value: &JsonValue) -> bool {
        match value {
            JsonValue::Bool(value) => *value,
            _ => panic!("expected JSON boolean"),
        }
    }

    fn string_option(value: &JsonValue, key: &str) -> Option<String> {
        optional_field(value, key).map(string).map(str::to_owned)
    }

    fn boolean_vec(value: &JsonValue) -> Vec<bool> {
        array(value).iter().map(boolean).collect()
    }

    fn usize_vec(value: &JsonValue) -> Vec<usize> {
        array(value).iter().map(number_usize).collect()
    }

    fn points_value(value: &JsonValue) -> Vec<Vec<[i64; 2]>> {
        array(value)
            .iter()
            .map(|contour| {
                array(contour)
                    .iter()
                    .map(|point| {
                        let point = array(point);
                        [number_i64(&point[0]), number_i64(&point[1])]
                    })
                    .collect()
            })
            .collect()
    }

    fn vector_input(value: &JsonValue) -> VectorInput {
        let view_box = array(field(value, "viewBox"));
        let mut nodes = Vec::new();
        for node in array(field(value, "nodes")) {
            let node = array(node);
            let attrs = object(&node[1])
                .iter()
                .map(|(key, value)| (key.clone(), string(value).to_owned()))
                .collect();
            nodes.push((string(&node[0]).to_owned(), attrs));
        }
        VectorInput {
            view_box: [
                number_f64(&view_box[0]),
                number_f64(&view_box[1]),
                number_f64(&view_box[2]),
                number_f64(&view_box[3]),
            ],
            fill: string(field(value, "fill")).to_owned(),
            stroke: string(field(value, "stroke")).to_owned(),
            stroke_width: number_f64(field(value, "strokeWidth")),
            stroke_linecap: string(field(value, "strokeLinecap")).to_owned(),
            stroke_linejoin: string(field(value, "strokeLinejoin")).to_owned(),
            nodes,
        }
    }

    fn geometry_expectation(value: &JsonValue) -> GeometryExpectation {
        GeometryExpectation {
            status: string(field(value, "status")).to_owned(),
            code: string_option(value, "code"),
            contour_count: optional_field(value, "contourCount").map(number_usize),
            closed: optional_field(value, "closed").map(boolean_vec),
            segment_counts: optional_field(value, "segmentCounts").map(usize_vec),
            canonical_points: optional_field(value, "canonicalPoints").map(points_value),
            wire_digest: string_option(value, "wireDigest"),
        }
    }

    fn pair_oracle(value: &JsonValue) -> PairOracle {
        let mappings = array(field(value, "mappings"))
            .iter()
            .map(|mapping| ExpectedMapping {
                left_index: number_usize(field(mapping, "leftIndex")),
                right_index: number_usize(field(mapping, "rightIndex")),
                reversed: boolean(field(mapping, "reversed")),
                offset: number_usize(field(mapping, "offset")),
                cost_micros: number_i64(field(mapping, "costMicros")),
            })
            .collect();
        PairOracle {
            left_digest: string(field(value, "leftDigest")).to_owned(),
            right_digest: string(field(value, "rightDigest")).to_owned(),
            pair_digest: string(field(value, "pairDigest")).to_owned(),
            mappings,
            cost_micros: number_i64(field(value, "costMicros")),
        }
    }

    fn pair_expectation(value: &JsonValue) -> PairExpectation {
        PairExpectation {
            status: string(field(value, "status")).to_owned(),
            code: string_option(value, "code"),
            reversed: optional_field(value, "reversed").map(boolean_vec),
            offsets: optional_field(value, "offsets").map(usize_vec),
            oracle: optional_field(value, "oracle").map(pair_oracle),
        }
    }

    fn vector(value: &JsonValue) -> GeometryVector {
        GeometryVector {
            id: string(field(value, "id")).to_owned(),
            left: vector_input(field(value, "left")),
            right: optional_field(value, "right").map(vector_input),
            expect: VectorExpectation {
                left: geometry_expectation(field(field(value, "expect"), "left")),
                right: optional_field(field(value, "expect"), "right").map(geometry_expectation),
                pair: optional_field(field(value, "expect"), "pair").map(pair_expectation),
            },
        }
    }

    #[derive(Clone, Debug)]
    struct VectorDocument {
        schema_version: u32,
        vectors: Vec<GeometryVector>,
    }

    impl VectorDocument {
        fn parse(source: &str) -> Result<Self, String> {
            let root = JsonParser::parse(source)?;
            Ok(Self {
                schema_version: number_usize(field(&root, "schemaVersion")) as u32,
                vectors: array(field(&root, "vectors")).iter().map(vector).collect(),
            })
        }
    }

    #[derive(Clone, Debug)]
    struct GeometryVector {
        id: String,
        left: VectorInput,
        right: Option<VectorInput>,
        expect: VectorExpectation,
    }

    #[derive(Clone, Debug)]
    struct VectorInput {
        view_box: [f64; 4],
        fill: String,
        stroke: String,
        stroke_width: f64,
        stroke_linecap: String,
        stroke_linejoin: String,
        nodes: Vec<(String, BTreeMap<String, String>)>,
    }

    #[derive(Clone, Debug)]
    struct VectorExpectation {
        left: GeometryExpectation,
        right: Option<GeometryExpectation>,
        pair: Option<PairExpectation>,
    }

    #[derive(Clone, Debug)]
    struct GeometryExpectation {
        status: String,
        code: Option<String>,
        contour_count: Option<usize>,
        closed: Option<Vec<bool>>,
        segment_counts: Option<Vec<usize>>,
        canonical_points: Option<Vec<Vec<[i64; 2]>>>,
        wire_digest: Option<String>,
    }

    #[derive(Clone, Debug)]
    struct ExpectedMapping {
        left_index: usize,
        right_index: usize,
        reversed: bool,
        offset: usize,
        cost_micros: i64,
    }

    #[derive(Clone, Debug)]
    struct PairOracle {
        left_digest: String,
        right_digest: String,
        pair_digest: String,
        mappings: Vec<ExpectedMapping>,
        cost_micros: i64,
    }

    #[derive(Clone, Debug)]
    struct PairExpectation {
        status: String,
        code: Option<String>,
        reversed: Option<Vec<bool>>,
        offsets: Option<Vec<usize>>,
        oracle: Option<PairOracle>,
    }

    fn input(value: VectorInput) -> IconGeometryInput {
        IconGeometryInput {
            view_box: value.view_box,
            fill: value.fill,
            stroke: value.stroke,
            stroke_width: value.stroke_width,
            stroke_linecap: value.stroke_linecap,
            stroke_linejoin: value.stroke_linejoin,
            nodes: value.nodes,
        }
    }

    fn points(geometry: &NormalizedIconGeometry) -> Vec<Vec<[i64; 2]>> {
        geometry
            .canonical
            .contours
            .iter()
            .map(|contour| {
                canonical_points(contour)
                    .expect("test geometry has canonical segments")
                    .into_iter()
                    .map(|point| [point.x, point.y])
                    .collect()
            })
            .collect()
    }

    fn assert_geometry(
        result: Result<NormalizedIconGeometry, GeometryError>,
        expected: &GeometryExpectation,
    ) {
        if expected.status == "rejected" {
            let error = result.expect_err("vector should reject");
            assert_eq!(error.code.as_str(), expected.code.as_deref().unwrap());
            return;
        }
        let geometry = result.expect("vector should normalize");
        assert_eq!(geometry.topology.closed, expected.closed.clone().unwrap());
        assert_eq!(
            geometry.topology.segment_counts,
            expected.segment_counts.clone().unwrap()
        );
        assert_eq!(
            geometry.canonical.contours.len(),
            expected.contour_count.unwrap()
        );
        if let Some(expected_points) = &expected.canonical_points {
            assert_eq!(&points(&geometry), expected_points);
        }
        if let Some(expected_digest) = &expected.wire_digest {
            assert_eq!(geometry_wire_digest(&geometry), *expected_digest);
        }
        assert!(geometry
            .sampled
            .contours
            .iter()
            .all(|contour| contour.points.len() == SAMPLE_COUNT));
    }

    #[test]
    fn shared_vectors_cover_both_normalization_and_pair_planning() {
        let document = VectorDocument::parse(VECTOR_SOURCE).expect("valid vectors");
        assert_eq!(document.schema_version, 1);
        for vector in document.vectors {
            let left = normalize_icon_geometry(&input(vector.left));
            assert_geometry(left.clone(), &vector.expect.left);
            let Some(right_input) = vector.right else {
                continue;
            };
            let right = normalize_icon_geometry(&input(right_input));
            assert_geometry(right.clone(), vector.expect.right.as_ref().unwrap());
            let pair = vector.expect.pair.as_ref().unwrap();
            if pair.status == "rejected" {
                let left = left.expect("left endpoint should normalize");
                let right = right.expect("right endpoint should normalize");
                let error = plan_icon_geometry_pair(&left, &right).expect_err("pair should reject");
                assert_eq!(
                    error.code.as_str(),
                    pair.code.as_deref().unwrap(),
                    "{}",
                    vector.id
                );
                continue;
            }
            let left = left.expect("left endpoint should normalize");
            let right = right.expect("right endpoint should normalize");
            let plan = plan_icon_geometry_pair(&left, &right).expect("pair should plan");
            if let Some(expected_reversed) = &pair.reversed {
                assert_eq!(
                    plan.contour_mappings
                        .iter()
                        .map(|mapping| mapping.reversed)
                        .collect::<Vec<_>>(),
                    *expected_reversed,
                    "{}",
                    vector.id
                );
            }
            if let Some(expected_offsets) = &pair.offsets {
                assert_eq!(
                    plan.contour_mappings
                        .iter()
                        .map(|mapping| mapping.offset)
                        .collect::<Vec<_>>(),
                    *expected_offsets,
                    "{}",
                    vector.id
                );
            }
            for mapping in &plan.contour_mappings {
                assert_eq!(
                    left.sampled.contours[mapping.left_index].closed,
                    right.sampled.contours[mapping.right_index].closed,
                    "{}",
                    vector.id
                );
            }
            let oracle = pair.oracle.as_ref().expect("missing exact pair oracle");
            assert_eq!(
                geometry_wire_digest(&left),
                oracle.left_digest,
                "{}",
                vector.id
            );
            assert_eq!(
                geometry_wire_digest(&right),
                oracle.right_digest,
                "{}",
                vector.id
            );
            assert_eq!(plan.cost_micros, oracle.cost_micros, "{}", vector.id);
            assert_eq!(
                pair_wire_digest(&left, &right, &plan),
                oracle.pair_digest,
                "{}",
                vector.id
            );
            assert_eq!(
                plan.contour_mappings
                    .iter()
                    .map(|mapping| {
                        (
                            mapping.left_index,
                            mapping.right_index,
                            mapping.reversed,
                            mapping.offset,
                            mapping.cost_micros,
                        )
                    })
                    .collect::<Vec<_>>(),
                oracle
                    .mappings
                    .iter()
                    .map(|mapping| {
                        (
                            mapping.left_index,
                            mapping.right_index,
                            mapping.reversed,
                            mapping.offset,
                            mapping.cost_micros,
                        )
                    })
                    .collect::<Vec<_>>(),
                "{}",
                vector.id
            );
            let planned = PlannedIconGeometryPair {
                left: &left,
                right: &right,
                plan: &plan,
            };
            assert_eq!(
                frame_at(&planned, 0.0)
                    .expect("left endpoint")
                    .contours
                    .into_iter()
                    .map(|contour| contour.points)
                    .collect::<Vec<_>>(),
                left.canonical
                    .contours
                    .iter()
                    .map(|contour| canonical_points(contour).unwrap())
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                frame_at(&planned, 1.0)
                    .expect("right endpoint")
                    .contours
                    .into_iter()
                    .map(|contour| contour.points)
                    .collect::<Vec<_>>(),
                right
                    .canonical
                    .contours
                    .iter()
                    .map(|contour| canonical_points(contour).unwrap())
                    .collect::<Vec<_>>()
            );
            let reverse_plan = reverse_pair_plan(&plan);
            let reverse_pair = PlannedIconGeometryPair {
                left: &right,
                right: &left,
                plan: &reverse_plan,
            };
            assert_eq!(
                frame_at(&reverse_pair, 0.0)
                    .expect("reverse left endpoint")
                    .contours
                    .into_iter()
                    .map(|contour| contour.points)
                    .collect::<Vec<_>>(),
                right
                    .canonical
                    .contours
                    .iter()
                    .map(|contour| canonical_points(contour).unwrap())
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                frame_at(&reverse_pair, 1.0)
                    .expect("reverse right endpoint")
                    .contours
                    .into_iter()
                    .map(|contour| contour.points)
                    .collect::<Vec<_>>(),
                left.canonical
                    .contours
                    .iter()
                    .map(|contour| canonical_points(contour).unwrap())
                    .collect::<Vec<_>>()
            );
            for progress in [0.25, 0.5, 0.75] {
                let forward = frame_at(&planned, progress).expect("forward frame");
                let reverse = frame_at(&reverse_pair, 1.0 - progress).expect("reverse frame");
                for mapping in &plan.contour_mappings {
                    let forward_points = &forward.contours[mapping.left_index].points;
                    let reverse_points = &reverse.contours[mapping.right_index].points;
                    for (index, forward_point) in forward_points.iter().enumerate() {
                        let reverse_index = if mapping.reversed {
                            modulo(
                                mapping.offset as isize - index as isize,
                                reverse_points.len(),
                            )
                        } else {
                            modulo(
                                mapping.offset as isize + index as isize,
                                reverse_points.len(),
                            )
                        };
                        assert_eq!(
                            reverse_points[reverse_index], *forward_point,
                            "{}",
                            vector.id
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn generated_registry_has_complete_lineage_and_explicit_states() {
        assert_eq!(ICON_GEOMETRY_REGISTRY_SCHEMA_VERSION, 1);
        assert_eq!(ICON_GEOMETRY_NORMALIZER_VERSION, "1.0.0");
        assert_eq!(ICON_GEOMETRY_SOURCE_PACKAGE, "lucide-static");
        assert_eq!(ICON_GEOMETRY_SOURCE_VERSION, "1.31.0");
        assert_eq!(ICON_GEOMETRY_NOTICE_ID, "lucide-static-isc-feather-mit");
        assert!(ICON_GEOMETRY_REGISTRY.len() >= 8);
        assert!(ICON_GEOMETRY_REGISTRY
            .iter()
            .any(|pair| pair.status == GeneratedPairStatus::Candidate));
        assert!(ICON_GEOMETRY_REGISTRY
            .iter()
            .any(|pair| pair.status == GeneratedPairStatus::Accepted));
        assert!(ICON_GEOMETRY_REGISTRY
            .iter()
            .any(|pair| pair.status == GeneratedPairStatus::Rejected));
        assert_eq!(
            ICON_GEOMETRY_REGISTRY
                .iter()
                .filter(|pair| pair.status == GeneratedPairStatus::Accepted)
                .count(),
            5
        );
        assert_eq!(
            ICON_GEOMETRY_REGISTRY
                .iter()
                .filter(|pair| pair.status == GeneratedPairStatus::Candidate)
                .count(),
            1
        );
        assert_eq!(
            ICON_GEOMETRY_REGISTRY
                .iter()
                .filter(|pair| pair.status == GeneratedPairStatus::Rejected)
                .count(),
            6
        );
        let candidate = ICON_GEOMETRY_REGISTRY
            .iter()
            .find(|pair| pair.status == GeneratedPairStatus::Candidate)
            .expect("candidate pair");
        assert_eq!(candidate.id, "circle-to-dot");
        assert!(candidate.geometry_left.is_some());
        assert!(candidate.geometry_right.is_some());
        assert!(candidate.plan.is_some());
        for pair in ICON_GEOMETRY_REGISTRY {
            assert!(!pair.id.is_empty());
            assert!(!pair.source_digest_left.is_empty());
            assert!(!pair.source_digest_right.is_empty());
            assert!(!pair.asset_digest_left.is_empty());
            assert!(!pair.asset_digest_right.is_empty());
            assert!(!pair.quality_notes.is_empty());
            if pair.status != GeneratedPairStatus::Rejected {
                assert!(pair.geometry_left.is_some());
                assert!(pair.geometry_right.is_some());
                assert!(pair.plan.is_some());
            }
            assert!(pair.payload_bytes <= 16 * 1024);
        }
    }
}
