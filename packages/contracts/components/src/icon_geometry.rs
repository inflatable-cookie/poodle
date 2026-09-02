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
    let value = raw.parse::<f64>().map_err(|_| {
        GeometryError::new(
            GeometryErrorCode::InvalidNumber,
            format!("{key} is not a number"),
        )
    })?;
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
    options: &[Vec<CorrespondenceOption>],
    best: &mut Option<Assignment>,
) {
    let left_index = assignment.len();
    if left_index == options.len() {
        let mappings = assignment
            .iter()
            .enumerate()
            .map(|(index, right_index)| options[index][*right_index].correspondence.clone())
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
        if used[right_index] {
            continue;
        }
        used[right_index] = true;
        assignment.push(right_index);
        visit_assignment(
            assignment,
            used,
            cost + options[left_index][right_index].cost,
            options,
            best,
        );
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
    if left
        .sampled
        .contours
        .iter()
        .zip(right.sampled.contours.iter())
        .any(|(left, right)| left.closed != right.closed)
    {
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
                    best_correspondence(left_contour, right_contour, left_index, right_index)
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
    use serde::Deserialize;

    const VECTOR_SOURCE: &str = include_str!("../../../core/src/icons/geometry-vectors.json");

    #[derive(Deserialize)]
    struct VectorDocument {
        #[serde(rename = "schemaVersion")]
        schema_version: u32,
        vectors: Vec<GeometryVector>,
    }

    #[derive(Deserialize)]
    struct GeometryVector {
        id: String,
        left: VectorInput,
        right: Option<VectorInput>,
        expect: VectorExpectation,
    }

    #[derive(Deserialize)]
    struct VectorInput {
        #[serde(rename = "viewBox")]
        view_box: [f64; 4],
        fill: String,
        stroke: String,
        #[serde(rename = "strokeWidth")]
        stroke_width: f64,
        #[serde(rename = "strokeLinecap")]
        stroke_linecap: String,
        #[serde(rename = "strokeLinejoin")]
        stroke_linejoin: String,
        nodes: Vec<(String, BTreeMap<String, String>)>,
    }

    #[derive(Deserialize)]
    struct VectorExpectation {
        left: GeometryExpectation,
        right: Option<GeometryExpectation>,
        pair: Option<PairExpectation>,
    }

    #[derive(Deserialize)]
    struct GeometryExpectation {
        status: String,
        code: Option<String>,
        #[serde(rename = "contourCount")]
        contour_count: Option<usize>,
        closed: Option<Vec<bool>>,
        #[serde(rename = "segmentCounts")]
        segment_counts: Option<Vec<usize>>,
        #[serde(rename = "canonicalPoints")]
        canonical_points: Option<Vec<Vec<[i64; 2]>>>,
    }

    #[derive(Deserialize)]
    struct PairExpectation {
        status: String,
        code: Option<String>,
        reversed: Option<Vec<bool>>,
        offsets: Option<Vec<usize>>,
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
        assert!(geometry
            .sampled
            .contours
            .iter()
            .all(|contour| contour.points.len() == SAMPLE_COUNT));
    }

    #[test]
    fn shared_vectors_cover_both_normalization_and_pair_planning() {
        let document: VectorDocument = serde_json::from_str(VECTOR_SOURCE).expect("valid vectors");
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
            assert_eq!(
                plan.contour_mappings
                    .iter()
                    .map(|mapping| mapping.reversed)
                    .collect::<Vec<_>>(),
                pair.reversed.clone().unwrap(),
                "{}",
                vector.id
            );
            assert_eq!(
                plan.contour_mappings
                    .iter()
                    .map(|mapping| mapping.offset)
                    .collect::<Vec<_>>(),
                pair.offsets.clone().unwrap(),
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
        for pair in ICON_GEOMETRY_REGISTRY {
            assert!(!pair.id.is_empty());
            assert!(!pair.source_digest_left.is_empty());
            assert!(!pair.source_digest_right.is_empty());
            assert!(!pair.asset_digest_left.is_empty());
            assert!(!pair.asset_digest_right.is_empty());
            assert!(!pair.quality_notes.is_empty());
            if pair.status == GeneratedPairStatus::Accepted {
                assert!(pair.geometry_left.is_some());
                assert!(pair.geometry_right.is_some());
                assert!(pair.plan.is_some());
            }
            assert!(pair.payload_bytes <= 16 * 1024);
        }
    }
}
