use crate::ast::expression::{Expression, Literal};

/// Represents a numeric literal in an abstract syntax tree (AST).
///
/// The `Number` struct is used to model numeric literals within an AST. It stores a 64-bit integer
/// (`i64`) value, along with associated metadata of type `T`. The metadata can provide additional
/// context, such as source code location or type information, and is used to enrich the expression
/// with supplementary details.
///
/// # Type Parameters
/// - `T`: The type of metadata associated with the numeric literal.
///
/// # Fields
/// - `value`: The numeric value of the literal, stored as an `i64`.
/// - `metadata`: The associated metadata of type `T`, which provides additional context for the
///     literal.
///
/// # Example
/// Creating a `Number` literal with metadata:
/// ```rust
/// let num = Number {
///     value: 42,
///     metadata: "line 1, column 5".to_string(),
/// };
/// ```
///
/// In this example, a `Number` literal is created with a value of `42` and metadata indicating
/// its source code location.
pub(crate) struct Number<T> {
    value: i64,
    metadata: T,
}

impl<T> Number<T> {
    /// Creates a new `Number` literal with the specified value and metadata.
    ///
    /// # Parameters
    /// - `value`: The numeric value of the literal.
    /// - `metadata`: The metadata associated with the literal.
    ///
    /// # Returns
    /// A new `Number` instance with the specified value and metadata.
    pub fn new(value: i64, metadata: T) -> Self {
        Self { value, metadata }
    }
}

impl<T> Literal<T> for Number<T> {}

impl<T> Expression<T> for Number<T> {
    /// Returns the metadata associated with this `Number` literal.
    ///
    /// The `metadata` method provides access to the metadata of type `T`, which can contain
    /// information such as the source code location or type annotations related to the literal.
    ///
    /// # Returns
    /// A reference to the metadata of type `T`.
    fn metadata(&self) -> T {
        self.metadata.as_ref()
    }
}
