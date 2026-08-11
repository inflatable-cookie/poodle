//! The bounded expression vocabulary — typed, total, pure derivation
//! expressions.
//!
//! Serves spec 063 "The bounded expression vocabulary (normative)": the
//! operator set ruled by the spec and batch card 012 — `and`, `or`, `not`,
//! `eq`, `ne`, `is_null`, `is_present`, `coalesce`, `gt`, `gte`, `lt`,
//! `lte`, `is_empty`, `if`/`then`/`else` — over operands that reference a
//! declared prop (`CROSS-02`), a declared state field (`CROSS-04`), a
//! VisualState projection field (`CROSS-14`), a part's presence (`CROSS-12`
//! slots), or a resolved axis value (`CROSS-07`, `CROSS-08`, `CROSS-11`),
//! or that carry a literal boolean, integer, string, or shared-type member.
//!
//! The vocabulary is closed on purpose: arithmetic, string manipulation,
//! interpolation, formatting, function calls, iteration, recursion,
//! variable binding, and indexing are excluded (spec 063 "Excluded,
//! deliberately"; card 012 "Fixed By Ruling"). A derivation that needs one
//! is a projection field, a conformance vector, an adapter capability, or a
//! runtime extension — never a widened language.
//!
//! Expressions are **total** (they always evaluate — there is no error case
//! in their result type), **pure** (no side effects, no environment access,
//! no I/O), and **typed**: [`validate`](crate::validate) type-checks every
//! expression against the declared prop, state, and projection types and
//! reports a [`Finding`](crate::Finding) at the authored source, with the
//! offending identifier. Motivating derivations: `CROSS-20`
//! (`isUnavailable = disabled || loading`), `BTN-14` (`isToggle`), `BTN-17`
//! (`hasLeading`), `BTN-18` (`data-tone` omit-when-default), `TXT-06`
//! (`isMultiline`), `TXT-08` (`isSearch`, `canClear`), `TXT-12`
//! (validation indicator), `TXT-14` (char-over), `CROSS-07` (size axis
//! fallback).
//!
//! Expressions appear only in the four sanctioned slots: state-derived
//! attribute emission conditions and values, part render conditions, prop
//! default and axis fallback resolution, and guard conditions on
//! transitions/effect-intents (spec 063 "Where expressions may appear").
//! This crate models and type-checks expressions; it never evaluates them
//! against runtime values (`NEG-01`).

use serde::{Deserialize, Serialize};

use crate::Identifier;

/// A bounded, typed, total expression (spec 063 "The bounded expression
/// vocabulary (normative)"; card 012 "Fixed By Ruling").
///
/// The operator set is exactly the spec table and nothing else: arithmetic,
/// string manipulation, interpolation, formatting, function calls,
/// iteration, recursion, variable binding, and indexing are not variants
/// here and cannot be constructed or deserialized (the exclusions are
/// tested). Operators serialize under their spec names for deterministic,
/// target-readable JSON (`IR-07`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expr {
    /// A reference to declared state or a literal operand (spec 063
    /// "Operands").
    #[serde(rename = "operand")]
    Operand(ExprOperand),
    /// `not` — boolean negation (`BTN-09` icon-only `!children`; `TXT-08`
    /// `canClear`'s `!disabled`).
    #[serde(rename = "not")]
    Not(Box<Expr>),
    /// `and` — boolean conjunction (`TXT-06` isMultiline; `TXT-14`
    /// char-over).
    #[serde(rename = "and")]
    And(Box<Expr>, Box<Expr>),
    /// `or` — boolean disjunction (`CROSS-20` isUnavailable; `BTN-14`
    /// isToggle; `BTN-17` hasLeading).
    #[serde(rename = "or")]
    Or(Box<Expr>, Box<Expr>),
    /// `eq` — equality against a literal or shared-type member (`CROSS-04`
    /// `pressed === true`; `TXT-08` `type === "search"`).
    #[serde(rename = "eq")]
    Eq(Box<Expr>, Box<Expr>),
    /// `ne` — inequality against a literal or shared-type member (`TXT-12`
    /// `effectiveValidationState !== "none"`; `BTN-18` `data-tone` omitted
    /// when not default).
    #[serde(rename = "ne")]
    Ne(Box<Expr>, Box<Expr>),
    /// `is_null` — true when the referenced state is null (spec 063
    /// nullability group; the `!== null` checks of `BTN-14`, `TXT-06`,
    /// `TXT-14`).
    #[serde(rename = "is_null")]
    IsNull(Box<Expr>),
    /// `is_present` — true when the referenced state is non-null (spec 063
    /// nullability group; `BTN-14` isToggle, `TXT-06`, `TXT-14`).
    #[serde(rename = "is_present")]
    IsPresent(Box<Expr>),
    /// `coalesce` — the first non-null operand (`CROSS-07`
    /// `size ?? resolveSemanticControlSize(...)`).
    #[serde(rename = "coalesce")]
    Coalesce(Box<Expr>, Box<Expr>),
    /// `gt` — greater than; integers only (spec 063 ordering group;
    /// `TXT-14` `charCount > maxLength`, `TXT-06` `rows > 1`).
    #[serde(rename = "gt")]
    Gt(Box<Expr>, Box<Expr>),
    /// `gte` — greater than or equal; integers only (spec 063 ordering
    /// group).
    #[serde(rename = "gte")]
    Gte(Box<Expr>, Box<Expr>),
    /// `lt` — less than; integers only (spec 063 ordering group).
    #[serde(rename = "lt")]
    Lt(Box<Expr>, Box<Expr>),
    /// `lte` — less than or equal; integers only (spec 063 ordering group).
    #[serde(rename = "lte")]
    Lte(Box<Expr>, Box<Expr>),
    /// `is_empty` — true for empty strings and collections (spec 063
    /// emptiness group; `TXT-08` `currentValue.length > 0`).
    #[serde(rename = "is_empty")]
    IsEmpty(Box<Expr>),
    /// `if`/`then`/`else` — selection; the condition is boolean and both
    /// arms carry the same type (spec 063 selection group; `CROSS-04`
    /// `pressedControlled ? pressed === true : uncontrolledPressed`).
    #[serde(rename = "if")]
    If {
        /// The boolean condition selecting the arm.
        #[serde(rename = "condition")]
        condition: Box<Expr>,
        /// The arm evaluated when the condition is true.
        #[serde(rename = "then")]
        then: Box<Expr>,
        /// The arm evaluated when the condition is false.
        #[serde(rename = "else")]
        otherwise: Box<Expr>,
    },
}

/// An operand of an expression — a reference to declared state or a literal
/// (spec 063 "Operands": "a declared prop, a declared state field, a
/// VisualState projection field, a slot's presence, a resolved axis value;
/// or a literal boolean, integer, string, or shared-type member").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExprOperand {
    /// A literal boolean, integer, string, or shared-type member.
    #[serde(rename = "literal")]
    Literal(ExprLiteral),
    /// Reference to a declared prop of the enclosing component
    /// (`CROSS-02`), e.g. `disabled` (`BTN-07`), `type` (`TXT-06`),
    /// `maxLength` (`TXT-14`).
    #[serde(rename = "prop")]
    Prop(Identifier),
    /// Reference to a declared state field — a controlled-state id
    /// (`CROSS-04`), e.g. the TextInput `value` state (`TXT-02`). The
    /// state's type is the controlled prop's declared type.
    #[serde(rename = "state")]
    State(Identifier),
    /// Reference to a VisualState projection field (`CROSS-14`; `RNG-16`),
    /// e.g. `charCount` or `effectiveValidationState` (`TXT-12`, `TXT-14`).
    #[serde(rename = "visual")]
    Visual(Identifier),
    /// Reference to a part's presence — the slot is present when the named
    /// part exists (`CROSS-12`; `BTN-16` children slot, `BTN-17` leading
    /// slot). The expression form of `Boolean(slot)` and `!children`
    /// (`BTN-09`, `BTN-17`); slot presence replaces the excluded `Boolean`
    /// call.
    #[serde(rename = "slot")]
    Slot(Identifier),
    /// Reference to a resolved axis value (`CROSS-07` size, `CROSS-08`
    /// density, `CROSS-11` orientation) — the axis's own resolution, e.g.
    /// `resolveSemanticControlSize(sizeRole)` (`CROSS-07`). The resolved
    /// value's type is the axis domain shared type (`control-size`,
    /// `control-density`, or `orientation`).
    #[serde(rename = "axis")]
    ResolvedAxis(Identifier),
}

/// A literal expression operand (spec 063 "Operands": "a literal boolean,
/// integer, string, or shared-type member"). There is no null literal:
/// nullability is expressed with `is_null`/`is_present`, never with `eq` —
/// so no literal here can be null and expressions stay total.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExprLiteral {
    /// A boolean literal, e.g. `pressed === true` (`CROSS-04`).
    #[serde(rename = "bool")]
    Bool(bool),
    /// An integer literal, e.g. `rows > 1` (`TXT-06`). Ordering operates on
    /// integers only (spec 063).
    #[serde(rename = "int")]
    Int(i64),
    /// A string literal, e.g. `type === "search"` transcribed as a member
    /// of `text-input-type`; a string literal is used for plain strings.
    #[serde(rename = "string")]
    String(String),
    /// A member of a first-class shared enumerated type, e.g. `search` of
    /// `text-input-type` (`TXT-08`), `none` of the validation-state type
    /// (`TXT-12`). Carries the shared type explicitly so the member is
    /// unambiguous even when two shared types share a member name (e.g.
    /// `default` on both tone and density types) — the pair is
    /// type-checked against the other side of the comparison.
    #[serde(rename = "member")]
    Member {
        /// The shared type the member belongs to (g13-b003 R6.1).
        #[serde(rename = "shared_type")]
        shared_type: Identifier,
        /// The member id, e.g. `search`, `multiline`, `none`.
        #[serde(rename = "member")]
        member: Identifier,
    },
}

// `not` names the spec operator; the lint suggests implementing
// `std::ops::Not`, which would be misleading on a serializable IR value.
#[allow(clippy::should_implement_trait)]
impl Expr {
    /// Builds a literal operand.
    pub fn operand(operand: ExprOperand) -> Self {
        Self::Operand(operand)
    }

    /// Builds a reference to a declared prop (`CROSS-02`).
    pub fn prop(id: impl Into<Identifier>) -> Self {
        Self::Operand(ExprOperand::Prop(id.into()))
    }

    /// Builds a reference to a declared state field (`CROSS-04`).
    pub fn state(id: impl Into<Identifier>) -> Self {
        Self::Operand(ExprOperand::State(id.into()))
    }

    /// Builds a reference to a VisualState projection field (`CROSS-14`).
    pub fn visual(id: impl Into<Identifier>) -> Self {
        Self::Operand(ExprOperand::Visual(id.into()))
    }

    /// Builds a reference to a part's presence (`CROSS-12`).
    pub fn slot(id: impl Into<Identifier>) -> Self {
        Self::Operand(ExprOperand::Slot(id.into()))
    }

    /// Builds a reference to a resolved axis value (`CROSS-07`).
    pub fn axis(id: impl Into<Identifier>) -> Self {
        Self::Operand(ExprOperand::ResolvedAxis(id.into()))
    }

    /// Builds a boolean literal.
    pub fn boolean(value: bool) -> Self {
        Self::Operand(ExprOperand::Literal(ExprLiteral::Bool(value)))
    }

    /// Builds an integer literal.
    pub fn int(value: i64) -> Self {
        Self::Operand(ExprOperand::Literal(ExprLiteral::Int(value)))
    }

    /// Builds a string literal.
    pub fn string(value: impl Into<String>) -> Self {
        Self::Operand(ExprOperand::Literal(ExprLiteral::String(value.into())))
    }

    /// Builds a shared-type member literal, e.g. `search` of
    /// `text-input-type` (`TXT-08`).
    pub fn member(shared_type: impl Into<Identifier>, member: impl Into<Identifier>) -> Self {
        Self::Operand(ExprOperand::Literal(ExprLiteral::Member {
            shared_type: shared_type.into(),
            member: member.into(),
        }))
    }

    /// Builds `not` — boolean negation.
    pub fn not(inner: Expr) -> Self {
        Self::Not(Box::new(inner))
    }

    /// Builds `and` — boolean conjunction.
    pub fn and(left: Expr, right: Expr) -> Self {
        Self::And(Box::new(left), Box::new(right))
    }

    /// Builds `or` — boolean disjunction.
    pub fn or(left: Expr, right: Expr) -> Self {
        Self::Or(Box::new(left), Box::new(right))
    }

    /// Builds `eq` — equality against a literal or shared-type member.
    pub fn eq(left: Expr, right: Expr) -> Self {
        Self::Eq(Box::new(left), Box::new(right))
    }

    /// Builds `ne` — inequality against a literal or shared-type member.
    pub fn ne(left: Expr, right: Expr) -> Self {
        Self::Ne(Box::new(left), Box::new(right))
    }

    /// Builds `is_null` — true when the referenced state is null.
    pub fn is_null(inner: Expr) -> Self {
        Self::IsNull(Box::new(inner))
    }

    /// Builds `is_present` — true when the referenced state is non-null.
    pub fn is_present(inner: Expr) -> Self {
        Self::IsPresent(Box::new(inner))
    }

    /// Builds `coalesce` — the first non-null operand.
    pub fn coalesce(left: Expr, right: Expr) -> Self {
        Self::Coalesce(Box::new(left), Box::new(right))
    }

    /// Builds `gt` — greater than; integers only.
    pub fn gt(left: Expr, right: Expr) -> Self {
        Self::Gt(Box::new(left), Box::new(right))
    }

    /// Builds `gte` — greater than or equal; integers only.
    pub fn gte(left: Expr, right: Expr) -> Self {
        Self::Gte(Box::new(left), Box::new(right))
    }

    /// Builds `lt` — less than; integers only.
    pub fn lt(left: Expr, right: Expr) -> Self {
        Self::Lt(Box::new(left), Box::new(right))
    }

    /// Builds `lte` — less than or equal; integers only.
    pub fn lte(left: Expr, right: Expr) -> Self {
        Self::Lte(Box::new(left), Box::new(right))
    }

    /// Builds `is_empty` — true for empty strings and collections.
    pub fn is_empty(inner: Expr) -> Self {
        Self::IsEmpty(Box::new(inner))
    }

    /// Builds `if`/`then`/`else` selection.
    pub fn if_then_else(condition: Expr, then: Expr, otherwise: Expr) -> Self {
        Self::If {
            condition: Box::new(condition),
            then: Box::new(then),
            otherwise: Box::new(otherwise),
        }
    }
}
