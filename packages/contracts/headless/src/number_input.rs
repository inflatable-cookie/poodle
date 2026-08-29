//! NumberInput entry machinery. Mirror of core `number-input.ts`.
//!
//! Contract: `docs/contracts/components/number-input.md`, "Behavior Machine".
//! Pure decimal draft classification, configuration checks, committed
//! value/raw-draft transitions, and step/commit effects. Adapters own
//! focus, caret, DOM/native events, async validation, and callback
//! execution.

#[derive(Clone, Debug, PartialEq)]
pub struct NumberInputContext {
    pub committed: Option<f64>,
    pub default_value: Option<f64>,
    pub draft: Option<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    /// Authored step; `None` means omitted (effective step is `1`).
    pub step: Option<f64>,
    /// Authored precision; `None` means omitted.
    pub precision: Option<f64>,
    pub disabled: bool,
    pub read_only: bool,
}

/// Maximum portable fractional scale. Enough to express any finite IEEE-754
/// double without exponent syntax; larger values are invalid configuration.
pub const NUMBER_INPUT_MAX_PRECISION: u32 = 324;

#[derive(Clone, Debug, PartialEq)]
pub enum NumberInputEvent {
    RawEdit { text: String },
    Clear,
    Enter,
    Blur,
    Escape,
    Step { direction: i32 },
    Home,
    End,
    Replace { value: Option<f64> },
    SetDisabled { disabled: bool },
    SetReadOnly { read_only: bool },
    SetConstraints {
        min: Option<f64>,
        max: Option<f64>,
        step: Option<f64>,
        precision: Option<f64>,
        default_value: Option<f64>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum NumberInputEffect {
    EmitDraftValueChange { draft: Option<String> },
    EmitValueChange { value: Option<f64> },
    EmitCommit { value: Option<f64> },
}

pub type NumberInputResult = (NumberInputContext, Vec<NumberInputEffect>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumberDraftKind {
    Empty,
    Incomplete,
    Malformed,
    Complete,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NumberDecimal {
    pub negative: bool,
    /// Absolute digit coefficient (base 10), no leading zeros except `"0"`.
    /// value = ± digits / 10^scale.
    pub digits: String,
    pub scale: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NumberDraftClassification {
    pub kind: NumberDraftKind,
    pub decimal: Option<NumberDecimal>,
    pub fractional_digits: Option<u32>,
}

fn idle(context: NumberInputContext) -> NumberInputResult {
    (context, Vec::new())
}

fn is_finite_number(value: f64) -> bool {
    value.is_finite()
}

fn is_non_negative_integer(value: f64) -> bool {
    value.is_finite() && value >= 0.0 && value == value.trunc()
}

/// Effective step used for alignment and stepping when config is valid.
pub fn number_input_effective_step(step: Option<f64>) -> f64 {
    step.unwrap_or(1.0)
}

pub fn number_input_config_valid(context: &NumberInputContext) -> bool {
    let check_optional = |value: Option<f64>| -> bool {
        value.is_none_or(is_finite_number)
    };

    if !check_optional(context.committed) {
        return false;
    }
    if !check_optional(context.default_value) {
        return false;
    }
    if !check_optional(context.min) {
        return false;
    }
    if !check_optional(context.max) {
        return false;
    }

    if let Some(step) = context.step {
        if !is_finite_number(step) || step <= 0.0 {
            return false;
        }
    }

    if let Some(precision) = context.precision {
        if !is_non_negative_integer(precision) || precision > f64::from(NUMBER_INPUT_MAX_PRECISION)
        {
            return false;
        }
    }

    if let (Some(min), Some(max)) = (context.min, context.max) {
        if min > max {
            return false;
        }
    }

    true
}

fn strip_sign(text: &str) -> (bool, &str) {
    if let Some(body) = text.strip_prefix('-') {
        (true, body)
    } else {
        (false, text)
    }
}

/// Portable draft syntax: optional leading `-`, digits, at most one `.`.
fn draft_syntax_ok(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.is_empty() {
        return false;
    }

    let mut i = 0usize;
    if bytes[0] == b'-' {
        i = 1;
        if i >= bytes.len() {
            return false;
        }
    }

    if bytes[i] == b'.' {
        i += 1;
        if i >= bytes.len() || !bytes[i].is_ascii_digit() {
            return false;
        }
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        return i == bytes.len();
    }

    if !bytes[i].is_ascii_digit() {
        return false;
    }
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i < bytes.len() {
        if bytes[i] != b'.' {
            return false;
        }
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
    }

    i == bytes.len()
}

fn is_incomplete_literal(text: &str) -> bool {
    matches!(text, "-" | "." | "-.")
}

fn strip_leading_zeros(digits: &str) -> String {
    let trimmed = digits.trim_start_matches('0');
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn parse_number_decimal(text: &str) -> Option<NumberDecimal> {
    if !draft_syntax_ok(text) || is_incomplete_literal(text) || text.ends_with('.') {
        return None;
    }

    let (negative, body) = strip_sign(text);
    let mut parts = body.splitn(2, '.');
    let int_raw = parts.next().unwrap_or("");
    let frac_raw = parts.next().unwrap_or("");
    let int_part = if int_raw.is_empty() { "0" } else { int_raw };
    let digits_text = strip_leading_zeros(&format!("{int_part}{frac_raw}"));

    Some(NumberDecimal {
        negative: negative && digits_text != "0",
        digits: digits_text,
        scale: frac_raw.len() as u32,
    })
}

pub fn classify_number_draft(text: &str) -> NumberDraftClassification {
    if text.is_empty() {
        return NumberDraftClassification {
            kind: NumberDraftKind::Empty,
            decimal: None,
            fractional_digits: None,
        };
    }

    if is_incomplete_literal(text) || (draft_syntax_ok(text) && text.ends_with('.')) {
        return NumberDraftClassification {
            kind: NumberDraftKind::Incomplete,
            decimal: None,
            fractional_digits: None,
        };
    }

    if !draft_syntax_ok(text) {
        return NumberDraftClassification {
            kind: NumberDraftKind::Malformed,
            decimal: None,
            fractional_digits: None,
        };
    }

    let Some(decimal) = parse_number_decimal(text) else {
        return NumberDraftClassification {
            kind: NumberDraftKind::Malformed,
            decimal: None,
            fractional_digits: None,
        };
    };

    let (_, body) = strip_sign(text);
    let frac = body
        .split_once('.')
        .map(|(_, frac)| frac.len() as u32)
        .unwrap_or(0);

    NumberDraftClassification {
        kind: NumberDraftKind::Complete,
        decimal: Some(decimal),
        fractional_digits: Some(frac),
    }
}

fn cmp_unsigned_digits(a: &str, b: &str) -> std::cmp::Ordering {
    let a = strip_leading_zeros(a);
    let b = strip_leading_zeros(b);
    match a.len().cmp(&b.len()) {
        std::cmp::Ordering::Equal => a.cmp(&b),
        ord => ord,
    }
}

fn add_unsigned_digits(a: &str, b: &str) -> String {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut i = a.len();
    let mut j = b.len();
    let mut carry = 0u8;
    let mut out = Vec::with_capacity(a.len().max(b.len()) + 1);

    while i > 0 || j > 0 || carry > 0 {
        let da = if i > 0 {
            i -= 1;
            a[i] - b'0'
        } else {
            0
        };
        let db = if j > 0 {
            j -= 1;
            b[j] - b'0'
        } else {
            0
        };
        let sum = da + db + carry;
        out.push(b'0' + (sum % 10));
        carry = sum / 10;
    }

    out.reverse();
    strip_leading_zeros(std::str::from_utf8(&out).unwrap_or("0"))
}

fn sub_unsigned_digits(a: &str, b: &str) -> String {
    debug_assert!(cmp_unsigned_digits(a, b) != std::cmp::Ordering::Less);
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut i = a.len();
    let mut j = b.len();
    let mut borrow = 0i8;
    let mut out = Vec::with_capacity(a.len());

    while i > 0 {
        i -= 1;
        let da = (a[i] - b'0') as i8;
        let db = if j > 0 {
            j -= 1;
            (b[j] - b'0') as i8
        } else {
            0
        };
        let mut diff = da - db - borrow;
        if diff < 0 {
            diff += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }
        out.push(b'0' + diff as u8);
    }

    out.reverse();
    strip_leading_zeros(std::str::from_utf8(&out).unwrap_or("0"))
}

/// Absolute decimal digit coefficient with sign. Mirrors TS `bigint` magnitudes.
#[derive(Clone, Debug, PartialEq, Eq)]
struct SignedDigits {
    negative: bool,
    digits: String,
}

impl SignedDigits {
    fn zero() -> Self {
        Self {
            negative: false,
            digits: "0".to_string(),
        }
    }

    fn from_parts(negative: bool, digits: String) -> Self {
        let digits = strip_leading_zeros(&digits);
        if digits == "0" {
            Self::zero()
        } else {
            Self { negative, digits }
        }
    }

    fn is_zero(&self) -> bool {
        self.digits == "0"
    }

    fn is_negative(&self) -> bool {
        self.negative && !self.is_zero()
    }

    fn is_positive(&self) -> bool {
        !self.negative && !self.is_zero()
    }

    fn negate(self) -> Self {
        if self.is_zero() {
            self
        } else {
            Self {
                negative: !self.negative,
                digits: self.digits,
            }
        }
    }

    fn add(&self, other: &Self) -> Self {
        match (self.negative, other.negative) {
            (false, false) => Self::from_parts(false, add_unsigned_digits(&self.digits, &other.digits)),
            (true, true) => Self::from_parts(true, add_unsigned_digits(&self.digits, &other.digits)),
            (false, true) => match cmp_unsigned_digits(&self.digits, &other.digits) {
                std::cmp::Ordering::Less => {
                    Self::from_parts(true, sub_unsigned_digits(&other.digits, &self.digits))
                }
                std::cmp::Ordering::Greater => {
                    Self::from_parts(false, sub_unsigned_digits(&self.digits, &other.digits))
                }
                std::cmp::Ordering::Equal => Self::zero(),
            },
            (true, false) => other.add(self),
        }
    }

    fn sub(&self, other: &Self) -> Self {
        self.add(&other.clone().negate())
    }

    fn mul_i32(&self, factor: i32) -> Self {
        if factor == 0 || self.is_zero() {
            return Self::zero();
        }
        let negative = self.negative ^ (factor < 0);
        let abs_factor = factor.unsigned_abs();
        if abs_factor == 1 {
            return Self::from_parts(negative, self.digits.clone());
        }

        let mut acc = "0".to_string();
        for _ in 0..abs_factor {
            acc = add_unsigned_digits(&acc, &self.digits);
        }
        Self::from_parts(negative, acc)
    }

    fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        Self::from_parts(
            self.negative ^ other.negative,
            mul_unsigned_digits(&self.digits, &other.digits),
        )
    }

    /// Truncating remainder with dividend sign (JS BigInt `%`).
    fn rem(&self, divisor: &Self) -> Self {
        if divisor.is_zero() {
            return Self::zero();
        }

        let q_digits = div_unsigned_digits(&self.digits, &divisor.digits);
        let q = SignedDigits::from_parts(self.negative ^ divisor.negative, q_digits);
        self.sub(&q.mul(divisor))
    }
}

fn mul_unsigned_digits(a: &str, b: &str) -> String {
    let a = strip_leading_zeros(a);
    let b = strip_leading_zeros(b);
    if a == "0" || b == "0" {
        return "0".to_string();
    }

    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut out = vec![0u32; a.len() + b.len()];

    for (i, &da) in a.iter().rev().enumerate() {
        for (j, &db) in b.iter().rev().enumerate() {
            out[i + j] += u32::from(da - b'0') * u32::from(db - b'0');
        }
    }

    let mut carry = 0u32;
    for slot in &mut out {
        let sum = *slot + carry;
        *slot = sum % 10;
        carry = sum / 10;
    }

    while out.last() == Some(&0) {
        out.pop();
    }
    out.reverse();
    if out.is_empty() {
        return "0".to_string();
    }
    out.into_iter()
        .map(|d| char::from(b'0' + d as u8))
        .collect()
}

fn div_unsigned_digits(dividend: &str, divisor: &str) -> String {
    let dividend = strip_leading_zeros(dividend);
    let divisor = strip_leading_zeros(divisor);
    if divisor == "0" {
        return "0".to_string();
    }
    if cmp_unsigned_digits(&dividend, &divisor) == std::cmp::Ordering::Less {
        return "0".to_string();
    }
    if divisor == "1" {
        return dividend;
    }

    let mut quotient = String::new();
    let mut remainder = String::new();

    for ch in dividend.chars() {
        remainder.push(ch);
        remainder = strip_leading_zeros(&remainder);
        let mut digit = 0u8;
        while cmp_unsigned_digits(&remainder, &divisor) != std::cmp::Ordering::Less {
            remainder = sub_unsigned_digits(&remainder, &divisor);
            digit += 1;
        }
        if !quotient.is_empty() || digit > 0 {
            quotient.push(char::from(b'0' + digit));
        }
    }

    if quotient.is_empty() {
        "0".to_string()
    } else {
        quotient
    }
}

fn mul_pow10_digits(digits: &str, exp: u32) -> String {
    if digits == "0" || exp == 0 {
        return strip_leading_zeros(digits);
    }
    let mut out = strip_leading_zeros(digits);
    out.extend(std::iter::repeat('0').take(exp as usize));
    out
}

fn div_pow10_digits(digits: &str, exp: u32) -> String {
    if exp == 0 {
        return strip_leading_zeros(digits);
    }
    let digits = strip_leading_zeros(digits);
    if exp as usize >= digits.len() {
        return "0".to_string();
    }
    strip_leading_zeros(&digits[..digits.len() - exp as usize])
}

fn rescale_digits(decimal: &NumberDecimal, target_scale: u32) -> SignedDigits {
    let delta = target_scale as i32 - decimal.scale as i32;
    let magnitude = if delta >= 0 {
        mul_pow10_digits(&decimal.digits, delta as u32)
    } else {
        div_pow10_digits(&decimal.digits, (-delta) as u32)
    };
    SignedDigits::from_parts(decimal.negative, magnitude)
}

fn trim_trailing_zeros_and_dot(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut end = bytes.len();
    while end > 0 && bytes[end - 1] == b'0' {
        end -= 1;
    }
    if end > 0 && bytes[end - 1] == b'.' {
        end -= 1;
    }
    if end == 0 {
        "0".to_string()
    } else {
        text[..end].to_string()
    }
}

fn decimal_from_number(value: f64) -> Option<NumberDecimal> {
    if !is_finite_number(value) {
        return None;
    }

    parse_number_decimal(&format_shortest_decimal(value))
}

fn common_scale(scales: &[u32]) -> u32 {
    scales.iter().copied().max().unwrap_or(0)
}

pub fn format_number_decimal(decimal: NumberDecimal, precision: Option<f64>) -> String {
    if let Some(precision) = precision {
        let precision = precision as u32;
        let scaled = rescale_digits(&decimal, precision);
        let negative = scaled.is_negative();
        let raw = scaled.digits.clone();
        let pad = precision as usize + 1;
        let raw = if raw.len() < pad {
            format!("{:0>width$}", raw, width = pad)
        } else {
            raw
        };
        let (int_part, frac_part) = if precision == 0 {
            (raw.as_str(), "")
        } else {
            let split = raw.len() - precision as usize;
            let int_part = if raw[..split].is_empty() {
                "0"
            } else {
                &raw[..split]
            };
            (int_part, &raw[split..])
        };
        let body = if precision == 0 {
            int_part.to_string()
        } else {
            format!("{int_part}.{frac_part}")
        };
        if negative {
            format!("-{body}")
        } else {
            body
        }
    } else {
        if decimal.digits == "0" {
            return "0".to_string();
        }

        let raw = decimal.digits.clone();
        let pad = decimal.scale as usize + 1;
        let raw = if raw.len() < pad {
            format!("{:0>width$}", raw, width = pad)
        } else {
            raw
        };

        let (mut int_part, mut frac_part) = if decimal.scale == 0 {
            (raw.clone(), String::new())
        } else {
            let split = raw.len() - decimal.scale as usize;
            let int_part = if raw[..split].is_empty() {
                "0".to_string()
            } else {
                raw[..split].to_string()
            };
            (int_part, raw[split..].to_string())
        };

        while frac_part.ends_with('0') {
            frac_part.pop();
        }
        int_part = strip_leading_zeros(&int_part);
        let body = if frac_part.is_empty() {
            int_part
        } else {
            format!("{int_part}.{frac_part}")
        };
        if decimal.negative {
            format!("-{body}")
        } else {
            body
        }
    }
}

/// Expand `to_string`-style scientific notation into portable base-10 decimal.
fn expand_scientific_decimal(text: &str) -> Option<String> {
    let e_pos = text.find(['e', 'E'])?;
    let mantissa = &text[..e_pos];
    let exp: i32 = text[e_pos + 1..].parse().ok()?;

    let (int_part, frac_part) = match mantissa.split_once('.') {
        Some((int_part, frac_part)) => (int_part, frac_part),
        None => (mantissa, ""),
    };

    if int_part.is_empty()
        || !int_part.bytes().all(|b| b.is_ascii_digit())
        || !frac_part.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }

    let digits = format!("{int_part}{frac_part}");
    let point = int_part.len() as i32 + exp;

    let expanded = if point <= 0 {
        let zeros = usize::try_from(-point).ok()?;
        format!("0.{}{digits}", "0".repeat(zeros))
    } else {
        let point = point as usize;
        if point >= digits.len() {
            format!("{digits}{}", "0".repeat(point - digits.len()))
        } else {
            format!("{}.{}", &digits[..point], &digits[point..])
        }
    };

    let trimmed = if expanded.contains('.') {
        trim_trailing_zeros_and_dot(&expanded)
    } else {
        expanded
    };

    Some(if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed
    })
}

/// Shortest canonical decimal for a finite number. Never emits exponent syntax.
pub fn format_shortest_decimal(value: f64) -> String {
    if !is_finite_number(value) {
        return String::new();
    }

    if value == 0.0 {
        return "0".to_string();
    }

    let negative = value < 0.0;
    let abs = value.abs();
    let mut text = abs.to_string();

    if text.contains('e') || text.contains('E') {
        text = expand_scientific_decimal(&text).unwrap_or_default();
        if text.is_empty() {
            return String::new();
        }
    }

    if negative {
        format!("-{text}")
    } else {
        text
    }
}

pub fn format_number_committed(value: Option<f64>, precision: Option<f64>) -> String {
    let Some(value) = value else {
        return String::new();
    };

    let Some(decimal) = decimal_from_number(value) else {
        return String::new();
    };

    format_number_decimal(decimal, precision)
}

pub fn number_in_bounds(value: f64, min: Option<f64>, max: Option<f64>) -> bool {
    if let Some(min) = min {
        if value < min {
            return false;
        }
    }
    if let Some(max) = max {
        if value > max {
            return false;
        }
    }
    true
}

pub fn number_step_aligned(value: f64, min: Option<f64>, step: Option<f64>) -> bool {
    let effective_step = number_input_effective_step(step);

    if !is_finite_number(effective_step) || effective_step <= 0.0 {
        return false;
    }

    let Some(value_decimal) = decimal_from_number(value) else {
        return false;
    };
    let Some(origin_decimal) = decimal_from_number(min.unwrap_or(0.0)) else {
        return false;
    };
    let Some(step_decimal) = decimal_from_number(effective_step) else {
        return false;
    };

    let scale = common_scale(&[
        value_decimal.scale,
        origin_decimal.scale,
        step_decimal.scale,
    ]);
    let value_digits = rescale_digits(&value_decimal, scale);
    let origin_digits = rescale_digits(&origin_decimal, scale);
    let step_digits = rescale_digits(&step_decimal, scale);

    if step_digits.is_zero() {
        return false;
    }

    let delta = value_digits.sub(&origin_digits);
    delta.rem(&step_digits).is_zero()
}

pub fn number_precision_ok(fractional_digits: u32, precision: Option<f64>) -> bool {
    match precision {
        None => true,
        Some(precision) => fractional_digits as f64 <= precision,
    }
}

pub fn number_draft_constraint_valid(
    text: &str,
    min: Option<f64>,
    max: Option<f64>,
    step: Option<f64>,
    precision: Option<f64>,
) -> bool {
    let classified = classify_number_draft(text);

    let (Some(decimal), Some(fractional_digits)) =
        (classified.decimal, classified.fractional_digits)
    else {
        return false;
    };

    if classified.kind != NumberDraftKind::Complete {
        return false;
    }

    if !number_precision_ok(fractional_digits, precision) {
        return false;
    }

    let value = number_decimal_to_number(decimal);

    is_finite_number(value)
        && number_in_bounds(value, min, max)
        && number_step_aligned(value, min, step)
}

pub fn number_decimal_to_number(decimal: NumberDecimal) -> f64 {
    format_number_decimal(decimal, None)
        .parse::<f64>()
        .unwrap_or(f64::NAN)
}

pub fn number_value_constraint_valid(
    value: Option<f64>,
    min: Option<f64>,
    max: Option<f64>,
    step: Option<f64>,
    precision: Option<f64>,
) -> bool {
    let Some(value) = value else {
        return true;
    };

    if !is_finite_number(value) {
        return false;
    }

    let text = format_number_committed(Some(value), precision);
    let classified = classify_number_draft(&text);

    let Some(fractional_digits) = classified.fractional_digits else {
        return false;
    };

    if classified.kind != NumberDraftKind::Complete {
        return false;
    }

    number_precision_ok(fractional_digits, precision)
        && number_in_bounds(value, min, max)
        && number_step_aligned(value, min, step)
}

fn last_on_grid(min: Option<f64>, max: f64, step: Option<f64>) -> Option<f64> {
    let origin = min.unwrap_or(0.0);
    let effective_step = number_input_effective_step(step);
    let origin_decimal = decimal_from_number(origin)?;
    let max_decimal = decimal_from_number(max)?;
    let step_decimal = decimal_from_number(effective_step)?;

    let scale = common_scale(&[
        origin_decimal.scale,
        max_decimal.scale,
        step_decimal.scale,
    ]);
    let origin_digits = rescale_digits(&origin_decimal, scale);
    let max_digits = rescale_digits(&max_decimal, scale);
    let step_digits = rescale_digits(&step_decimal, scale);

    if !step_digits.is_positive() {
        return None;
    }

    let delta = max_digits.sub(&origin_digits);
    let remainder = delta
        .rem(&step_digits)
        .add(&step_digits)
        .rem(&step_digits);
    let last_digits = max_digits.sub(&remainder);
    let decimal = NumberDecimal {
        negative: last_digits.is_negative(),
        digits: last_digits.digits,
        scale,
    };
    let value = number_decimal_to_number(decimal);

    if number_value_constraint_valid(Some(value), min, Some(max), step, None) {
        Some(value)
    } else {
        None
    }
}

pub fn step_number_value(
    current: Option<f64>,
    direction: i32,
    min: Option<f64>,
    max: Option<f64>,
    step: Option<f64>,
    precision: Option<f64>,
) -> Option<f64> {
    let effective_step = number_input_effective_step(step);

    let config = NumberInputContext {
        committed: None,
        default_value: None,
        draft: None,
        min,
        max,
        step,
        precision,
        disabled: false,
        read_only: false,
    };
    if !number_input_config_valid(&config) {
        return None;
    }

    if current.is_none() {
        if direction > 0 {
            let origin = min.unwrap_or(0.0);
            return if number_value_constraint_valid(Some(origin), min, max, step, precision) {
                Some(origin)
            } else {
                None
            };
        }

        if let Some(max) = max {
            return last_on_grid(min, max, step);
        }

        let origin = min.unwrap_or(0.0);
        let candidate = origin - effective_step;
        return if number_value_constraint_valid(Some(candidate), min, max, step, precision) {
            Some(candidate)
        } else {
            None
        };
    }

    let current = current?;
    let current_decimal = decimal_from_number(current)?;
    let step_decimal = decimal_from_number(effective_step)?;

    let precision_scale = precision.map(|p| p as u32).unwrap_or(0);
    let scale = common_scale(&[current_decimal.scale, step_decimal.scale, precision_scale]);
    let current_digits = rescale_digits(&current_decimal, scale);
    let step_digits = rescale_digits(&step_decimal, scale);
    let next_digits = current_digits.add(&step_digits.mul_i32(direction));
    let next = number_decimal_to_number(NumberDecimal {
        negative: next_digits.is_negative(),
        digits: next_digits.digits,
        scale,
    });

    if !number_value_constraint_valid(Some(next), min, max, step, precision) {
        return None;
    }

    Some(next)
}

fn active_draft_text(context: &NumberInputContext) -> Option<&str> {
    context.draft.as_deref()
}

fn display_text(context: &NumberInputContext) -> String {
    if let Some(draft) = &context.draft {
        return draft.clone();
    }

    format_number_committed(context.committed, context.precision)
}

fn valid_draft_value(context: &NumberInputContext, text: &str) -> Option<f64> {
    if !number_draft_constraint_valid(
        text,
        context.min,
        context.max,
        context.step,
        context.precision,
    ) {
        return None;
    }

    parse_number_decimal(text).map(number_decimal_to_number)
}

fn push_unique(effects: &mut Vec<NumberInputEffect>, effect: NumberInputEffect) {
    if effects.last() == Some(&effect) {
        return;
    }
    effects.push(effect);
}

fn resolve_committed(
    context: NumberInputContext,
    value: Option<f64>,
    commit: bool,
    draft: Option<String>,
) -> NumberInputResult {
    let mut effects = Vec::new();
    let draft_changed = draft != context.draft;
    let value_changed = match (value, context.committed) {
        (Some(a), Some(b)) => a != b,
        (None, None) => false,
        _ => true,
    };

    let mut next = NumberInputContext {
        committed: value,
        draft: draft.clone(),
        ..context.clone()
    };

    if draft_changed {
        push_unique(
            &mut effects,
            NumberInputEffect::EmitDraftValueChange {
                draft: draft.clone(),
            },
        );
    }

    if value_changed {
        push_unique(
            &mut effects,
            NumberInputEffect::EmitValueChange { value },
        );
    }

    if commit {
        push_unique(&mut effects, NumberInputEffect::EmitCommit { value });
    }

    // After a successful resolve, normalize draft display through committed formatting.
    if draft.is_none() && value.is_some() {
        next.draft = None;
    }

    (next, effects)
}

fn set_draft(
    mut context: NumberInputContext,
    draft: String,
    emit_value: Option<Option<f64>>,
) -> NumberInputResult {
    let mut effects = Vec::new();
    let draft_changed = context.draft.as_deref() != Some(draft.as_str());

    if draft_changed {
        push_unique(
            &mut effects,
            NumberInputEffect::EmitDraftValueChange {
                draft: Some(draft.clone()),
            },
        );
    }

    context.draft = Some(draft);

    if let Some(emit_value) = emit_value {
        let value_changed = match (emit_value, context.committed) {
            (Some(a), Some(b)) => a != b,
            (None, None) => false,
            _ => true,
        };
        if value_changed {
            context.committed = emit_value;
            push_unique(
                &mut effects,
                NumberInputEffect::EmitValueChange { value: emit_value },
            );
        }
    }

    (context, effects)
}

fn discard_draft(mut context: NumberInputContext) -> NumberInputResult {
    if context.draft.is_none() {
        return idle(context);
    }

    context.draft = None;
    (
        context,
        vec![NumberInputEffect::EmitDraftValueChange { draft: None }],
    )
}

fn commit_current(context: NumberInputContext) -> NumberInputResult {
    let draft = active_draft_text(&context).map(str::to_owned);

    let Some(draft) = draft else {
        return (
            context.clone(),
            vec![NumberInputEffect::EmitCommit {
                value: context.committed,
            }],
        );
    };

    if draft.is_empty() {
        return resolve_committed(context, None, true, None);
    }

    let Some(value) = valid_draft_value(&context, &draft) else {
        return idle(context);
    };

    resolve_committed(context, Some(value), true, None)
}

pub fn number_input_invalid(context: &NumberInputContext) -> bool {
    if !number_input_config_valid(context) {
        return true;
    }

    let Some(draft) = active_draft_text(context) else {
        return false;
    };

    if draft.is_empty() {
        return false;
    }

    valid_draft_value(context, draft).is_none()
}

impl Default for NumberInputContext {
    fn default() -> Self {
        Self {
            committed: None,
            default_value: None,
            draft: None,
            min: None,
            max: None,
            step: None,
            precision: None,
            disabled: false,
            read_only: false,
        }
    }
}

pub fn number_input_context() -> NumberInputContext {
    NumberInputContext::default()
}

pub fn number_input_transition(
    context: NumberInputContext,
    event: NumberInputEvent,
) -> NumberInputResult {
    match &event {
        NumberInputEvent::SetDisabled { disabled } => {
            let mut next = context;
            next.disabled = *disabled;
            return (next, Vec::new());
        }
        NumberInputEvent::SetReadOnly { read_only } => {
            let mut next = context;
            next.read_only = *read_only;
            return (next, Vec::new());
        }
        NumberInputEvent::SetConstraints {
            min,
            max,
            step,
            precision,
            default_value,
        } => {
            let mut next = context;
            next.min = *min;
            next.max = *max;
            next.step = *step;
            next.precision = *precision;
            next.default_value = *default_value;
            return (next, Vec::new());
        }
        NumberInputEvent::Replace { value } => {
            let had_draft = context.draft.is_some();
            let mut next = context;
            next.committed = *value;
            next.draft = None;
            let effects = if had_draft {
                vec![NumberInputEffect::EmitDraftValueChange { draft: None }]
            } else {
                Vec::new()
            };
            return (next, effects);
        }
        _ => {}
    }

    if context.disabled || context.read_only {
        return idle(context);
    }

    if !number_input_config_valid(&context) {
        return idle(context);
    }

    match event {
        NumberInputEvent::RawEdit { text } => {
            if text.is_empty() {
                return set_draft(context, String::new(), Some(None));
            }

            let classified = classify_number_draft(&text);

            if classified.kind == NumberDraftKind::Complete && classified.decimal.is_some() {
                let value = valid_draft_value(&context, &text);
                return set_draft(context, text, value.map(Some));
            }

            set_draft(context, text, None)
        }
        NumberInputEvent::Clear => set_draft(context, String::new(), Some(None)),
        NumberInputEvent::Enter => commit_current(context),
        NumberInputEvent::Blur => {
            let draft = active_draft_text(&context).map(str::to_owned);

            let Some(draft) = draft else {
                return idle(context);
            };

            if draft.is_empty() || valid_draft_value(&context, &draft).is_some() {
                return commit_current(context);
            }

            discard_draft(context)
        }
        NumberInputEvent::Escape => discard_draft(context),
        NumberInputEvent::Step { direction } => {
            let direction = if direction < 0 { -1 } else { 1 };
            let draft = active_draft_text(&context);
            let mut from = context.committed;

            if let Some(draft) = draft {
                if draft.is_empty() {
                    from = None;
                } else {
                    let Some(draft_value) = valid_draft_value(&context, draft) else {
                        return idle(context);
                    };
                    from = Some(draft_value);
                }
            }

            let Some(next) = step_number_value(
                from,
                direction,
                context.min,
                context.max,
                context.step,
                context.precision,
            ) else {
                return idle(context);
            };

            resolve_committed(context, Some(next), true, None)
        }
        NumberInputEvent::Home => {
            let Some(min) = context.min else {
                return idle(context);
            };

            if !number_value_constraint_valid(
                Some(min),
                context.min,
                context.max,
                context.step,
                context.precision,
            ) {
                return idle(context);
            }

            resolve_committed(context, Some(min), true, None)
        }
        NumberInputEvent::End => {
            let Some(max) = context.max else {
                return idle(context);
            };

            if !number_value_constraint_valid(
                Some(max),
                context.min,
                context.max,
                context.step,
                context.precision,
            ) {
                return idle(context);
            }

            resolve_committed(context, Some(max), true, None)
        }
        NumberInputEvent::SetDisabled { .. }
        | NumberInputEvent::SetReadOnly { .. }
        | NumberInputEvent::SetConstraints { .. }
        | NumberInputEvent::Replace { .. } => idle(context),
    }
}

pub fn number_input_display_text(context: &NumberInputContext) -> String {
    display_text(context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_empty_incomplete_complete_and_malformed() {
        assert_eq!(classify_number_draft("").kind, NumberDraftKind::Empty);
        assert_eq!(classify_number_draft("-").kind, NumberDraftKind::Incomplete);
        assert_eq!(classify_number_draft(".").kind, NumberDraftKind::Incomplete);
        assert_eq!(classify_number_draft("-.").kind, NumberDraftKind::Incomplete);
        assert_eq!(classify_number_draft("1.").kind, NumberDraftKind::Incomplete);
        assert_eq!(classify_number_draft("01.20").kind, NumberDraftKind::Complete);
        assert_eq!(classify_number_draft(".5").kind, NumberDraftKind::Complete);
        assert_eq!(classify_number_draft("-12.5").kind, NumberDraftKind::Complete);
        assert_eq!(classify_number_draft("1e3").kind, NumberDraftKind::Malformed);
        assert_eq!(classify_number_draft(" 1").kind, NumberDraftKind::Malformed);
        assert_eq!(classify_number_draft("0x10").kind, NumberDraftKind::Malformed);
    }

    #[test]
    fn omitted_step_is_one_and_invalid_config_is_inert() {
        assert_eq!(number_input_effective_step(None), 1.0);
        assert!(number_step_aligned(3.0, None, None));

        let mut context = number_input_context();
        context.step = Some(-1.0);
        let (next, effects) =
            number_input_transition(context.clone(), NumberInputEvent::RawEdit { text: "1".into() });
        assert!(effects.is_empty());
        assert_eq!(next, context);
    }

    #[test]
    fn clear_empty_emits_draft_and_null_value() {
        let mut context = number_input_context();
        context.committed = Some(4.0);
        let (next, effects) = number_input_transition(context, NumberInputEvent::Clear);
        assert_eq!(next.draft.as_deref(), Some(""));
        assert_eq!(next.committed, None);
        assert_eq!(
            effects,
            vec![
                NumberInputEffect::EmitDraftValueChange {
                    draft: Some("".into())
                },
                NumberInputEffect::EmitValueChange { value: None },
            ]
        );
    }

    #[test]
    fn blur_discards_unresolved_draft() {
        let mut context = number_input_context();
        context.committed = Some(2.0);
        context.draft = Some("1.".into());
        let (next, effects) = number_input_transition(context, NumberInputEvent::Blur);
        assert!(next.draft.is_none());
        assert_eq!(next.committed, Some(2.0));
        assert_eq!(
            effects,
            vec![NumberInputEffect::EmitDraftValueChange { draft: None }]
        );
    }

    #[test]
    fn step_from_empty_baseline_uses_min_or_zero() {
        let context = number_input_context();
        let (next, effects) =
            number_input_transition(context, NumberInputEvent::Step { direction: 1 });
        assert_eq!(next.committed, Some(0.0));
        assert!(effects.iter().any(|e| matches!(
            e,
            NumberInputEffect::EmitCommit { value: Some(v) } if *v == 0.0
        )));
    }

    #[test]
    fn oversized_finite_draft_is_complete() {
        let text = "1234567890123456789012345678901234567890";
        let classified = classify_number_draft(text);
        assert_eq!(classified.kind, NumberDraftKind::Complete);
        let decimal = classified.decimal.expect("complete draft has decimal");
        assert_eq!(decimal.digits, text);
        assert_eq!(decimal.scale, 0);
        assert!(!decimal.negative);
    }

    #[test]
    fn format_shortest_decimal_expands_without_exponent() {
        let large = format_shortest_decimal(1e21);
        assert!(!large.contains('e') && !large.contains('E'));
        assert_eq!(large, "1000000000000000000000");

        assert_eq!(format_shortest_decimal(1e-17), "0.00000000000000001");

        // Scientific `to_string` form must expand to portable decimal.
        assert_eq!(
            expand_scientific_decimal("1e+21").as_deref(),
            Some("1000000000000000000000")
        );
        assert_eq!(
            expand_scientific_decimal("1e-17").as_deref(),
            Some("0.00000000000000001")
        );
        assert_eq!(
            expand_scientific_decimal("1.5e+3").as_deref(),
            Some("1500")
        );
        assert_eq!(
            expand_scientific_decimal("1.23e-2").as_deref(),
            Some("0.0123")
        );
    }
}
