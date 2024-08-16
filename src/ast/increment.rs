use crate::ast::expression::{Expression, UnaryExpression};

/// Represents an increment operation in an abstract syntax tree (AST).
///
/// The `Increment` struct wraps an expression and applies an increment operation to it.
/// The `metadata` field can store additional information about the operation, such as its location
/// in the source code or type information.
///
/// # Type Parameters:
/// - `T`: The type of the metadata associated with the expression.
pub(crate) struct Increment<T> {
    expression: Box<dyn Expression<T>>,
    metadata: T,
}

impl<T> Increment<T> {
    /// Creates a new `Increment` expression.
    ///
    /// # Parameters:
    /// - `expression`: A boxed expression to which the increment operation will be applied.
    /// - `metadata`: Metadata associated with this expression, such as type information or
    ///     location.
    ///
    /// # Returns:
    /// A new `Increment` instance.
    pub(crate) fn new(expression: Box<dyn Expression<T>>, metadata: T) -> Self {
        Self { expression, metadata }
    }
}

impl<T> Expression<T> for Increment<T> {
    /// Returns a reference to the metadata associated with this expression.
    ///
    /// # Returns:
    /// A reference to the metadata of type `T`.
    fn metadata(&self) -> &T {
        &self.metadata
    }
}

impl<T> UnaryExpression<T> for Increment<T> {
    /// Returns a reference to the wrapped expression.
    ///
    /// # Returns:
    /// A reference to the expression that this `Increment` operation is applied to.
    fn expr(&self) -> &dyn Expression<T> {
        self.expression.as_ref()
    }
}
